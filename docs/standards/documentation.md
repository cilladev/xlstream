# Documentation

Documentation is part of the feature. A function that is fast and correct but undocumented is broken.

## Three documentation surfaces

1. **Rustdoc** — API reference, generated from code. The source of truth for library users.
2. **Markdown in `docs/`** — design docs, guides, research. Read by humans and agents.
3. **README files** — per-crate, short, point to `docs/`.

## Rustdoc rules

### Every public item is documented

`missing_docs` warning is enabled. Undocumented public items don't compile.

### Every doc has an example

If it takes args, show a call. If it's a type, show construction. If it's a trait, show an impl.

Every example is a doctest. Doctests run in CI. Broken examples don't merge.

```rust
/// Convert an Excel date serial number to a calendar date.
///
/// Excel uses a 1900-based date serial. Serial 1 = 1900-01-01. Serial 60
/// is the bogus "1900-02-29" — Excel preserves Lotus 1-2-3's leap-year bug
/// for backward compatibility.
///
/// # Examples
///
/// ```
/// use xlstream_core::ExcelDate;
/// let d = ExcelDate::from_serial(44927.0);
/// assert_eq!(d.year_month_day(), (2023, 1, 1));
/// ```
pub fn from_serial(serial: f64) -> ExcelDate { ... }
```

### Panics, errors, and safety

If a function can panic, say when. If it returns `Result`, say what each error variant means. If it's `unsafe`, document the invariants.

```rust
/// # Errors
///
/// - [`XlStreamError::FormulaParse`] if `expr` contains an unknown function
///   name or is syntactically malformed.
/// - [`XlStreamError::Classification`] if the parse succeeds but the formula
///   shape isn't supported — for instance, it references `OFFSET` or `INDIRECT`.
```

### Cross-references

Use the `[link]` syntax. `rustdoc` resolves these at build time; broken links fail the build.

```rust
/// See [`crate::evaluator::Interpreter`] for how this AST is executed.
```

### Module-level docs

Each module starts with a block comment summarising its purpose:

```rust
//! # Aggregate builtins
//!
//! Functions that reduce a range of values to a scalar. These are invoked
//! both at prelude time (pre-computing column aggregates) and at row time
//! (reducing an inline range argument).
```

## Markdown docs (`docs/`)

Organised by purpose, not by module:

- `brief.md` — the project's why.
- `architecture/` — how things work, and why they were chosen this way.
- `standards/` — how we work.
- `operations/` — repo, CI, release.
- `phases/` — the roadmap.
- `research/` — background reading and competitive analysis.

Every markdown file opens with a 1–2 line statement of its purpose, so a reader can tell within 5 seconds whether it's what they need.

Link generously. If a concept is defined in another file, link to it, don't re-explain.

## Changelogs

`CHANGELOG.md` at repo root. Keep a Changelog format (https://keepachangelog.com/).

Every merge to main updates `[Unreleased]`. Release tags move items to a new versioned section.

```markdown
## [Unreleased]

### Added
- `XLOOKUP` with wildcard match mode.

### Fixed
- `VLOOKUP` no longer panics on empty lookup range.

### Changed
- `evaluate` now requires an output path; pass `None` explicitly to overwrite input.
```

## PR descriptions

Every PR's description includes:
- **What**: one paragraph on what changed.
- **Why**: link to the phase doc / issue / motivating case.
- **Tests**: what tests were added.
- **Docs**: which docs were updated. If "none," justify.

## Rustdoc conventions

| Convention | Example |
|---|---|
| Type links | `` [`Value`] `` |
| Function links | `` [`Interpreter::eval`] `` |
| Error docs | `# Errors` section |
| Panic docs | `# Panics` section — but we don't panic, so this is for test-only utilities |
| Safety docs | `# Safety` section — for `unsafe fn` or `unsafe trait` (none in v0.1) |
| Examples | `# Examples` section, at least one compilable example |

## README files

Each crate has a `README.md`. Max 100 lines. Structure:

1. One-sentence purpose.
2. Link to `docs/architecture/<area>.md`.
3. Minimal usage example.
4. Link to the full docs.

```markdown
# xlstream-eval

Streaming Excel formula evaluator. Row-by-row, bounded-memory.

Full architecture: [`docs/architecture/evaluator.md`](../../docs/architecture/evaluator.md).

```rust
use xlstream_eval::evaluate;
evaluate("input.xlsx", "output.xlsx", None)?;
```

See [`docs/`](../../docs/) for the full picture.
```

## Style

- Short sentences.
- Present tense.
- Active voice.
- Second person ("you") only in guides (README, getting-started). Third person ("the evaluator") in design docs.
- No emojis.
- No ceremony ("this document will describe..."). Say the thing.
- Use code blocks for every identifier, path, command, filename.

## Diagrams

ASCII diagrams in markdown are fine — they stay readable in `git diff`, don't require toolchains. If a concept is genuinely visual (a graph, a state machine) and ASCII is insufficient, use `mermaid`:

\`\`\`mermaid
flowchart LR
  Input --> Classify --> Prelude --> Stream --> Output
\`\`\`

GitHub renders mermaid natively.

## Documentation as you go

Rule: **any PR that changes a public API must update the rustdoc and any mention in `docs/` in the same PR.**

Docs drift is a silent project killer. We enforce at review time, not at release time.

## Final release docs

Before v0.1.0 ships:

- `README.md` polished (badges, link to docs site).
- `docs/` review for typos, broken links, outdated code.
- mdBook site built and deployed (optional for v0.1; mandatory for v1.0).
- Changelog finalised.
- Migration guide from `formualizer` if applicable.
