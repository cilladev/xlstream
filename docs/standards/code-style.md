# Code style

## Formatting

`rustfmt` with our config. Run on save. CI checks.

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
use_small_heuristics = "Max"
group_imports = "StdExternalCrate"
imports_granularity = "Module"
reorder_imports = true
reorder_modules = true
```

Non-negotiable:
- 100-column soft limit.
- Imports grouped (std → external → internal), alphabetised within each group.
- No tabs. Four spaces.
- Trailing newline on every file.

## Linting

Clippy with `-D warnings` in CI. Zero tolerance.

```toml
# clippy.toml
msrv = "1.85"
avoid-breaking-exported-api = true
```

Additional lints enabled across the workspace:

```rust
// At the top of each crate's lib.rs:
#![warn(
    missing_docs,
    rust_2018_idioms,
    clippy::pedantic,
    clippy::cargo,
)]
#![deny(
    clippy::unwrap_used,          // no unwrap() in library code
    clippy::expect_used,          // no expect()
    clippy::panic,                // no panic!()
    clippy::todo,                 // no todo!()
    clippy::unimplemented,        // no unimplemented!()
    clippy::print_stdout,         // no println! in library code
    clippy::dbg_macro,            // no dbg!()
)]
```

Test modules get to relax:

```rust
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    // ...
}
```

## Naming

| Item | Convention | Example |
|---|---|---|
| Crate | `kebab-case` | `xlstream-core` |
| Module | `snake_case` | `mod streaming_evaluator;` |
| Type | `UpperCamelCase` | `StreamingEvaluator` |
| Trait | `UpperCamelCase`, verb or adjective | `Evaluate`, `Readable` |
| Function / method | `snake_case` | `fn next_row()` |
| Enum variant | `UpperCamelCase` | `Value::Number` |
| Const / static | `SCREAMING_SNAKE_CASE` | `const MAX_ROWS: u32 = ...;` |
| Lifetime | short lowercase | `<'a>`, `<'row>`, `<'ctx>` |
| Error type | `<Area>Error` | `XlStreamError`, `CellError` |

## Comments

Default: **no comments**. A well-named function + types + rustdoc on the public items is enough.

Write a comment only when:
- A hidden invariant must be stated for the reader to not break it.
- A workaround for a specific bug exists and removing it would bring the bug back.
- Non-obvious performance choice — "we use `SmallVec` here because 99% of cases have ≤ 8 elements and the heap allocation is measurable on the hot path."
- A surprising piece of behaviour that would otherwise look like a bug.

Never write a comment that:
- Restates the code.
- Describes what a function does (that's rustdoc).
- Credits the author of the change.
- References a PR, ticket, or author by name.

## Rustdoc

Every public item has a doc comment. Every public item's doc comment has at least one `# Examples` block that compiles under `cargo test --doc`.

```rust
/// Parse an Excel formula expression into an AST.
///
/// The input must **not** include a leading `=` — that's an I/O concern.
///
/// # Errors
///
/// Returns [`FormulaParseError`] if the expression is malformed, contains
/// an unknown function name the parser doesn't recognise, or exceeds the
/// nesting limit of 64.
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// let ast = parse("SUM(A1:A10)")?;
/// assert_eq!(ast.to_string(), "SUM(A1:A10)");
/// # Ok::<(), xlstream_parse::FormulaParseError>(())
/// ```
pub fn parse(expr: &str) -> Result<Ast, FormulaParseError> { ... }
```

## Type design

- **Prefer owned types over references in public APIs** unless profiling shows the copy matters. Lifetime-laden APIs are a tax on every caller.
- **Prefer `&[T]` over `&Vec<T>`**, `&str` over `&String`.
- **Prefer enums over type-flag booleans.** `enum Mode { Strict, Lenient }` beats `bool`.
- **Enum variants are data, not flags.** If a variant is always parameterless, consider whether it's really an enum or should be a unit struct.

## Error handling

- `Result<_, XlStreamError>` at every library boundary.
- `?` for propagation.
- `thiserror` for the error enum.
- `anyhow` only in the CLI binary, never in library crates.
- Never `.unwrap()` or `.expect()` in library code. If you genuinely believe a condition cannot fail, write `debug_assert!` and return the error anyway on release.

## Unsafe

Not allowed in v0.1. If future perf work needs it, a design doc is required first (`docs/design/unsafe-<area>.md`) covering:
- What invariant is being unsafely asserted.
- Why the safe alternative isn't fast enough (with numbers).
- The surface area (how much unsafe code, in how many files).
- The safety argument (formal or semi-formal reasoning).

## Module layout

```
crates/<name>/
├── Cargo.toml
├── README.md                  # short, links to docs/
└── src/
    ├── lib.rs                 # pub use only; no logic
    ├── <module>.rs            # or <module>/mod.rs + submodules
    └── tests/                 # integration tests (cross-module)
```

`lib.rs` contains only:
- The crate-level rustdoc comment.
- `pub use ...` statements exposing the public API.
- Module declarations.
- No functions, no types.

## Imports

Always absolute paths in library code:

```rust
// good
use crate::evaluator::Interpreter;
use xlstream_core::Value;

// bad
use super::super::Interpreter;
```

No `use some_crate::*;` at module scope. Bring in exactly what you use. The exception: `use crate::prelude::*` is acceptable if a `prelude` module is curated.

## Visibility

- `pub` only on items that are part of the crate's public API.
- `pub(crate)` for cross-module internal items.
- `pub(super)` rarely.
- Private (no modifier) is the default.

Every `pub` item earns its publicity — tested, documented, and stable-ish across 0.x versions.

## Types of number

- `f64` for everything numeric unless there's a reason. Excel numbers are f64.
- `u32` for row indices (Excel max row is ~1M, fits in u32).
- `u16` for column indices (Excel max column is 16384, fits in u16).
- `usize` for in-memory vec indices and lengths.
- `i64` only when we need integer semantics Excel itself requires (very rare).

## String types

- `Box<str>` for long-lived strings we own and don't mutate.
- `String` when we'll mutate.
- `&str` for borrows.
- `Cow<'_, str>` when we're sometimes borrowing, sometimes owning.
- `compact_str::CompactString` considered for small strings (< 24 bytes inline) if profiling says allocation is a hotspot. Defer.

## Performance notes in code

When you make a non-obvious perf choice, document it in code, briefly:

```rust
// SmallVec: 99% of formulas reference <=4 cells; inline avoids the heap allocation
// on the hot path.
pub struct References {
    pub cells: SmallVec<[CellRef; 4]>,
}
```

## Public API stability

We are pre-1.0. Breaking changes are allowed between minor versions (0.1 → 0.2), but:
- Document them in `CHANGELOG.md` with migration notes.
- Give deprecation warnings where possible.
- Make the change worth it.

## Forbidden in library code

- `println!`, `eprintln!`, `dbg!` — use `tracing`.
- `panic!`, `unreachable!`, `todo!`, `unimplemented!`.
- `.unwrap()`, `.expect()`.
- Any `unsafe` block.
- `lazy_static!` / `once_cell` / `OnceLock` for mutable state.
- Global singletons, process-wide caches, `thread_local!` caches.
- `std::env::var` reads — accept config via function args.
- Heap allocation in the row loop hot path.
- `Box<dyn Error>` — use `XlStreamError`.
