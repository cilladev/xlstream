# Phase 2 Parser Integration Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking. The xlstream workflow overrides the skill's parallel-dispatch default with serial PR-per-chunk execution; respect it.

**Goal:** Replace Phase 1's parser stubs with a production parser + classification + AST-rewrite pipeline. After Phase 2, `xlstream_parse::parse(expr)` returns a real AST, `extract_references` surfaces every cell/range/sheet/function the formula touches, `classify(ast, ctx)` returns one of {RowLocal, AggregateOnly, LookupOnly, Mixed, Unsupported{reason, doc_link}}, and `rewrite(ast, ctx, classification)` substitutes supported aggregate/lookup sub-expressions with `PreludeRef` nodes ready for the Phase 4 evaluator. The CLI gains a `classify` subcommand that lists every formula in an xlsx with its verdict.

**Architecture:** Thin adapter over upstream `formualizer-parse 1.1.2`. Public API is opaque — no `formualizer_parse` types in our public signatures (per `docs/architecture/parse-reuse.md`). Internally we mirror upstream's AST as our own `Node` enum so we can splice in a `PreludeRef` variant during rewrite. Classification is a pure function of the tree plus a `ClassificationContext` (column headers, lookup-sheet set, current-cell address, function whitelist). All refusals carry an `UnsupportedReason` enum + `&'static str` doc link, never a free-form `String`.

**Tech stack:** Rust 1.88 (per ADR `docs/decisions/2026-04-18-phase-02-toolchain-bump.md`), `formualizer-parse = "=1.1.2"`, `formualizer-common = "=1.1.2"`, `smallvec` (already a workspace dep), `phf` (already a workspace dep) for case-insensitive function-name sets, existing xlstream-core / xlstream-cli crates.

---

## PR sequence for Phase 2 (six PRs)

| # | Chunk | Branch | Net new lines (est.) |
|---|---|---|---|
| 1 | Chunk 0 — upstream wiring + opaque `Ast` + `Node` mirror + real `parse()` | `feature/phase-02-chunk-0-parse` | ~350 |
| 2 | Chunk 1 — `Reference` types + `extract_references` + tests | `feature/phase-02-chunk-1-references` | ~300 |
| 3 | Chunk 2 — `UnsupportedReason` + `Classification` upgrade + support sets | `feature/phase-02-chunk-2-classification-types` | ~250 |
| 4 | Chunk 3 — `classify` impl + 30+ tests | `feature/phase-02-chunk-3-classify-impl` | ~700 |
| 5 | Chunk 4 — AST rewrite + `PreludeRef` + `PreludeKey` + golden tests | `feature/phase-02-chunk-4-ast-rewrite` | ~350 |
| 6 | Chunk 5 — CLI `classify` subcommand + canonical fixture + phase doc tick | `feature/phase-02-chunk-5-cli-classify` | ~250 |

Each PR ≤ ~400 changed lines (per `docs/standards/commits.md`). Chunk 3 sails close to the budget; if review surfaces it as too large, split it into 3a (algorithm) + 3b (tests batched per shape) before pushing.

---

## PR sequencing policy (read before opening any PR)

1. Branch each PR off `main`. Wait for the previous PR to merge before branching the next.
2. **Stacked PRs are the exception, not the rule.** Allowed only if the prior PR has been in review for > 2 hours (`docs/standards/commits.md` PR-workflow rules). Hard limit: one layer deep. Mark stacked PR descriptions with `"stacked on #N, will rebase onto main after #N merges."`
3. Never tick a phase-02 checkbox in `docs/phases/phase-02-parser.md` until the enabling code lands on `main`.
4. Each PR runs `make check` locally before push. CI must be green before merge.
5. PR title and squash-commit message both use `<prefix>: <imperative, lowercase, no period>`. Prefix is `xlstream-parse` for crate-scoped work, `xlstream-cli` for the CLI chunk, `docs` for docs-only follow-ups.
6. Same template as Phase 1 PRs (see `docs/standards/commits.md` PR section).

## Ground rules (reference, do not skip)

- **No `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`** in library code. Tests may use `unwrap`/`expect`/`panic!` inside the existing `#![allow(...)]` block at the top of the test module — same Phase 1 convention.
- **No `unsafe`.**
- **No new dependencies** beyond `formualizer-parse` and `formualizer-common` (declared in ADR #10, accepted by Priscilla in chat). If a chunk surfaces a need for any other crate, stop and ask in the kickoff chat with the four-bullet "When blocked" template (`docs/standards/commits.md`).
- **Every public item gets rustdoc + `# Examples` doctest.** No exceptions; missing docs fail CI via `#![warn(missing_docs)]`.
- **Use Context7 first** for every `formualizer_parse` / `formualizer_common` / `smallvec` / `phf` / `clap` / `calamine` / `rust_xlsxwriter` API call. Note: `formualizer-parse` has thin docs.rs presence; if Context7 returns nothing useful, grep `refs/formualizer/crates/formualizer-parse/src/` and `refs/formualizer/crates/formualizer-common/src/`. **Never guess from training data.** Each chunk lists Context7 checkpoints.
- **`std::sync::OnceLock`, never `once_cell`** — `once_cell` is in our graph only as upstream's transitive dep (per ADR #10).
- All six Phase 1 ratified clippy divergences apply unchanged:
  1. `keywords` + `categories` in every `[package]`.
  2. `#![allow(clippy::module_name_repetitions, clippy::cargo_common_metadata)]` at every crate root.
  3. `#![allow(clippy::float_cmp)]` in test modules comparing `f64`.
  4. Test literals avoid well-known constants (no `3.14`, `2.71828`, `1.414`).
  5. `Cargo.lock` committed.
  6. `#![allow(clippy::multiple_crate_versions)]` where upstream dep graph forces it.
- **Integration test bar from PR #8:** every meaningful behaviour has a unit test (direct fn call) AND an integration test (via the public API path). Phase 2 integration tests live in `crates/xlstream-parse/tests/`. Workspace-level end-to-end is deferred to Phase 4 (no evaluator yet).
- **No emojis**, no `Co-Authored-By: Claude`, no `Generated with Claude Code` — `commit-msg` hook rejects all three.
- **Streaming invariant is sacred** — classification is the gate that makes the invariant hold. Any classifier change that admits a formula needing future-row data is a bug, not a feature.

---

## File structure (locked-in before Chunk 0)

```
xlstream/
├── Cargo.toml                                       ← + formualizer-{parse,common} (Chunk 0)
├── crates/
│   └── xlstream-parse/
│       ├── Cargo.toml                               ← + formualizer-{parse,common}, phf (Chunks 0, 2)
│       └── src/
│           ├── lib.rs                               ← re-exports each chunk's public surface
│           ├── ast.rs                               ← opaque `Ast` + internal `Node` enum mirror (Chunk 0); `PreludeRef` variant (Chunk 4)
│           ├── parser.rs                            ← real `parse()` over upstream + error mapping (Chunk 0)
│           ├── references.rs                        ← `Reference`, `Refs`, `extract_references` (Chunk 1)
│           ├── classify.rs                          ← `UnsupportedReason`, `Classification`, `ClassificationContext`, `classify` (Chunks 2–3)
│           ├── sets.rs                              ← `phf::Set` constants for fn-name whitelists (Chunk 2)
│           └── rewrite.rs                           ← `PreludeKey`, `rewrite()` (Chunk 4)
│       └── tests/                                   ← integration tests via public API
│           ├── parse.rs                             ← Chunk 0
│           ├── references.rs                        ← Chunk 1
│           ├── classify_row_local.rs                ← Chunk 3
│           ├── classify_aggregate.rs                ← Chunk 3
│           ├── classify_lookup.rs                   ← Chunk 3
│           ├── classify_mixed.rs                    ← Chunk 3
│           ├── classify_unsupported.rs              ← Chunk 3
│           └── rewrite.rs                           ← Chunk 4
├── crates/xlstream-cli/
│   └── src/main.rs                                  ← + `classify` subcommand (Chunk 5)
├── fixtures/
│   ├── canonical/
│   │   └── benchmark_small.xlsx                     ← 3-sheet smoke fixture (Chunk 5)
│   └── scripts/
│       └── gen_benchmark_small/                     ← reproducible generator binary crate (Chunk 5)
│           ├── Cargo.toml
│           └── src/main.rs
├── docs/phases/phase-02-parser.md                   ← ticked progressively per chunk
├── docs/phases/README.md                            ← Phase 2 status flipped (Chunk 5)
└── docs/superpowers/plans/2026-04-18-phase-02-parser.md   ← this file; tick tasks as they land
```

### Why the file split

- **`ast.rs`** owns the AST shape; `parser.rs` owns input → AST conversion. They change at different times (AST mirror evolves with PreludeRef in Chunk 4; parser is stable after Chunk 0).
- **`references.rs`** is a pure walker. Cleanly reusable from `classify.rs` and `rewrite.rs`. Splitting it from `classify.rs` keeps each file tight (Phase 1 file-size convention).
- **`sets.rs`** is `phf` boilerplate; isolating it keeps `classify.rs` readable.
- **`rewrite.rs`** is a pure tree transform; lives next to AST but separate from classification logic.

### Why `fixtures/scripts/gen_benchmark_small/` is its own binary crate

Workspace-internal tools that need crates (`rust_xlsxwriter`) without polluting library deps. Pattern: a `[[bin]]` in a small isolated crate, **not** a `[workspace] members` entry — keeps the main workspace `cargo test` fast. Invoked manually when the fixture needs regenerating; output committed.

---

## Chunk 0: upstream wiring + opaque `Ast` + `Node` mirror + real `parse()`

**Goal:** `xlstream_parse::parse("SUM(A1:A10)")` returns `Ok(Ast)`. Internal `Node` enum mirrors `formualizer_parse::ASTNodeType` so Chunk 4 can splice in `PreludeRef` without breaking earlier chunks. No upstream types appear in any public signature.

**Branch:** `feature/phase-02-chunk-0-parse`.

### Task 0.1: Create feature branch

- [ ] **Step 1: Sync main and branch off it.**

```bash
git fetch origin main
git checkout main && git pull --ff-only
git checkout -b feature/phase-02-chunk-0-parse
```

### Task 0.2: Add upstream deps to workspace

**Files:**
- Modify: `Cargo.toml` (workspace root)

- [ ] **Step 1: Replace the commented-out `formualizer-parse` block with exact pins.**

In `[workspace.dependencies]`, remove the explanatory comment and the commented `formualizer-parse = "0.5"` line, then add:

```toml
# formula parser — pinned exactly per ADR docs/decisions/2026-04-18-phase-02-toolchain-bump.md.
formualizer-parse = "=1.1.2"
formualizer-common = "=1.1.2"
```

Both crates declared explicitly even though `formualizer-common` would otherwise come transitively through `formualizer-parse` — we control the pin. Exact-version (`=1.1.2`) per `docs/architecture/parse-reuse.md` ("Pin to an exact version until we know upstream's stability policy").

- [ ] **Step 2: Confirm the workspace still resolves.**

```bash
cargo update -p formualizer-parse 2>&1
cargo update -p formualizer-common 2>&1
cargo check --workspace
```

Expected: clean build, `Cargo.lock` updated. Commit `Cargo.lock` with the next change.

### Task 0.3: Add deps to `xlstream-parse/Cargo.toml`

**Files:**
- Modify: `crates/xlstream-parse/Cargo.toml`

- [ ] **Step 1: Add formualizer crates and `smallvec` to `[dependencies]`.**

Replace the existing comment block ("formualizer-parse: deferred to Phase 2 ...") with:

```toml
[dependencies]
xlstream-core = { path = "../xlstream-core", version = "0.1.0" }
formualizer-parse = { workspace = true }
formualizer-common = { workspace = true }
smallvec = { workspace = true }
```

`smallvec` is needed in Chunk 1; declaring it now keeps the dep diff in one PR. `phf` lands in Chunk 2 (different review cycle, easier to reason about).

### Task 0.4: Context7 checkpoint — formualizer-parse public API

- [ ] **Step 1: Confirm the parser entry-point shape via Context7.**

Query: `formualizer-parse` public `parse` function and `ASTNode` type.

If Context7 returns nothing usable (likely — upstream is small), fall back to `refs/formualizer/crates/formualizer-parse/src/parser.rs:2953` (`pub fn parse<T: AsRef<str>>(formula: T) -> Result<ASTNode, ParserError>`) and `parser.rs:1474` (`ASTNode { node_type: ASTNodeType, source_token: Option<Token>, contains_volatile: bool }`).

Lock-in: we call `formualizer_parse::parse(expr)`; the result is converted to our internal `Node` and wrapped in `Ast`.

### Task 0.5: Define internal `Node` enum (TDD)

**Files:**
- Modify: `crates/xlstream-parse/src/ast.rs`

The enum mirrors `formualizer_parse::ASTNodeType` plus a `PreludeRef` variant added in Chunk 4. Keep `Node` and its sub-types `pub(crate)` — the public surface remains the opaque `Ast` newtype.

- [ ] **Step 1: Write the failing test.**

Append to the existing `mod tests` block in `ast.rs`:

```rust
#[test]
fn ast_constructs_from_a_literal_node() {
    use crate::ast::{Ast, Node, NumLiteral};
    let ast = Ast::from_root_for_tests(Node::Literal(NumLiteral::Number(42.0)));
    assert_eq!(format!("{ast:?}").contains("42"), true, "expected literal in debug: {ast:?}");
}
```

Run: `cargo test -p xlstream-parse ast::tests::ast_constructs_from_a_literal_node`. Expected: FAIL — `Node`/`NumLiteral`/`from_root_for_tests` undefined.

- [ ] **Step 2: Define `Node` and supporting types.**

Replace the body of `ast.rs` with:

```rust
//! The opaque [`Ast`] — wraps an internal tree of [`Node`]s mirroring the
//! upstream parser's AST plus the `PreludeRef` variant used by the Phase 4
//! evaluator. Public consumers see only [`Ast`]; the tree itself is
//! crate-internal so we can extend variants without breaking semver.

use smallvec::SmallVec;

use crate::references::Reference;

/// Numeric or boolean literal carried by [`Node::Literal`]. Mirrors the
/// subset of `formualizer_common::LiteralValue` we actually use; everything
/// else (text, error, dates) flows through dedicated variants below.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NumLiteral {
    Number(f64),
    Bool(bool),
}

/// One node in the parsed-formula tree. Mirrors upstream's
/// `formualizer_parse::ASTNodeType` so we can splice in `PreludeRef` (Chunk
/// 4) without leaking upstream types in our public API.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Node {
    /// A scalar literal.
    Literal(NumLiteral),
    /// A text literal (kept distinct from `NumLiteral` because Excel's text
    /// comparison rules differ from numeric).
    Text(String),
    /// A reference to one or more cells.
    Reference(Reference),
    /// Unary operator (`-`, `%`).
    UnaryOp { op: String, expr: Box<Node> },
    /// Binary operator (`+ - * / ^ & = <> < > <= >= :`).
    BinaryOp { op: String, left: Box<Node>, right: Box<Node> },
    /// Function call. Name is preserved with original case but compared
    /// case-insensitively in classification.
    Function { name: String, args: Vec<Node> },
    /// Array constant `{1,2;3,4}`.
    Array(SmallVec<[SmallVec<[Node; 4]>; 2]>),
    // PreludeRef variant added in Chunk 4.
}

/// Parsed formula. Opaque by design — internals are crate-internal so we
/// can evolve the [`Node`] tree across phases.
///
/// Build one with [`crate::parse`].
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// let ast = parse("1+2").unwrap();
/// // Ast prints its node tree under Debug.
/// assert!(format!("{ast:?}").contains("BinaryOp"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub(crate) root: Node,
}

impl Ast {
    /// Test-only constructor that lets tests build trees without going
    /// through [`crate::parse`]. Hidden from rustdoc; intentionally not
    /// part of the stable public API.
    #[doc(hidden)]
    #[must_use]
    pub fn from_root_for_tests(root: Node) -> Self {
        Self { root }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    #[test]
    fn ast_constructs_from_a_literal_node() {
        let ast = Ast::from_root_for_tests(Node::Literal(NumLiteral::Number(42.0)));
        assert!(format!("{ast:?}").contains("42"), "expected literal in debug: {ast:?}");
    }

    #[test]
    fn node_literal_compares_by_value() {
        assert_eq!(Node::Literal(NumLiteral::Bool(true)), Node::Literal(NumLiteral::Bool(true)));
        assert_ne!(Node::Literal(NumLiteral::Bool(true)), Node::Literal(NumLiteral::Bool(false)));
    }
}
```

> Note the forward dep on `crate::references::Reference`. The `references` module exists as a stub in this chunk (Task 0.6) and is filled in Chunk 1.

- [ ] **Step 3: Run tests; expect compile error on `crate::references::Reference`.**

```bash
cargo test -p xlstream-parse
```

Expected: compile error — `references` module missing. Resolves with Task 0.6.

### Task 0.6: Stub `references` module

**Files:**
- Create: `crates/xlstream-parse/src/references.rs`

- [ ] **Step 1: Add a minimal stub.**

```rust
//! Reference types and the [`extract_references`] walker.
//!
//! Phase 2 Chunk 1 fills this in. Chunk 0 ships only the [`Reference`] enum
//! shape so [`crate::ast::Node::Reference`] compiles.

use smallvec::SmallVec;

/// One Excel reference: cell, range, or named range.
///
/// Phase 2 supports `Cell`, `Range`, and `NamedRange`. Upstream's
/// `External` and `Table` variants are mapped to `Unsupported` at
/// classification time (Chunk 3).
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    /// `Sheet1!$A$2` — fully-resolved cell address.
    Cell {
        /// Sheet name. `None` for an unqualified ref on the active sheet.
        sheet: Option<String>,
        /// 1-based row.
        row: u32,
        /// 1-based column.
        col: u32,
    },
    /// `A:A`, `A1:B10`, `Sheet2!A:C`. Whole-column refs use
    /// `start_row = None`, `end_row = None`.
    Range {
        /// Sheet name. `None` for an unqualified range on the active sheet.
        sheet: Option<String>,
        /// 1-based start row, or `None` for whole-column refs.
        start_row: Option<u32>,
        /// 1-based end row, or `None` for whole-column refs.
        end_row: Option<u32>,
        /// 1-based start column, or `None` for whole-row refs.
        start_col: Option<u32>,
        /// 1-based end column, or `None` for whole-row refs.
        end_col: Option<u32>,
    },
    /// `MyRange` — workbook-level named range. Resolved by classification
    /// against `ClassificationContext::named_ranges` in Chunk 3.
    Named(String),
}

/// References surfaced by [`extract_references`]. Sized by P99 expectation
/// in `docs/architecture/parse-reuse.md`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Refs {
    /// Every `CellRef` reachable from the AST root.
    pub cells: SmallVec<[Reference; 8]>,
    /// Every `RangeRef` (and named-range alias).
    pub ranges: SmallVec<[Reference; 4]>,
    /// Every distinct sheet name mentioned (de-duplicated).
    pub sheets: SmallVec<[String; 2]>,
    /// Every function name called (case-preserved).
    pub functions: SmallVec<[String; 8]>,
}
```

> No `extract_references` function in this chunk; Chunk 1 adds it. Doctest deferred until then so we don't ship a fake example.

- [ ] **Step 2: Wire the module into `lib.rs`.**

Edit `crates/xlstream-parse/src/lib.rs`. Add `mod references;` next to the existing `mod ast;`. Add a re-export: `pub use references::{Reference, Refs};`. Order matters for doc indexing — keep alphabetical within `pub use` blocks.

- [ ] **Step 3: Re-run tests.**

```bash
cargo test -p xlstream-parse
```

Expected: PASS (the two `ast::tests` we wrote in Task 0.5).

### Task 0.7: Implement real `parse()` (TDD)

**Files:**
- Modify: `crates/xlstream-parse/src/parser.rs`

The parser converts upstream `formualizer_parse::ASTNodeType` into our `Node`. Errors map to `XlStreamError::FormulaParse { address, formula, message }`. The `address` field is supplied by the caller (parser-layer doesn't know which cell the formula came from); the chunk picks the **caller-enriches** strategy: `parse(expr)` populates `address: String::new()` and `formula: expr.into()`, and the caller wraps with `map_err` to set `address`. This matches `docs/architecture/parse-reuse.md`'s "no upstream types in public signatures" rule and the phase doc's `parse(expr: &str)` form. (See Open Question 1 — surface to Priscilla before implementing if uncertain.)

- [ ] **Step 1: Write the success-case integration test first.**

Create `crates/xlstream-parse/tests/parse.rs`:

```rust
//! Integration tests for `xlstream_parse::parse`. Drive the public API
//! exactly as a downstream consumer would.

use xlstream_parse::parse;

#[test]
fn simple_arithmetic_parses() {
    let ast = parse("1+2").expect("parse failed");
    let dbg = format!("{ast:?}");
    assert!(dbg.contains("BinaryOp"), "expected BinaryOp in debug: {dbg}");
    assert!(dbg.contains('1'), "expected literal 1: {dbg}");
    assert!(dbg.contains('2'), "expected literal 2: {dbg}");
}

#[test]
fn function_call_parses() {
    let ast = parse("SUM(A1, B2)").expect("parse failed");
    let dbg = format!("{ast:?}");
    assert!(dbg.contains("Function"), "expected Function: {dbg}");
    assert!(dbg.contains("SUM"), "expected SUM name: {dbg}");
}

#[test]
fn whole_column_range_parses() {
    let ast = parse("SUM(A:A)").expect("parse failed");
    assert!(format!("{ast:?}").contains("Range"));
}

#[test]
fn cross_sheet_reference_parses() {
    let ast = parse("'Tax Rates'!A1").expect("parse failed");
    assert!(format!("{ast:?}").contains("Tax Rates"));
}

#[test]
fn malformed_input_returns_formula_parse_error() {
    let err = parse("SUM(").unwrap_err();
    assert!(
        matches!(err, xlstream_core::XlStreamError::FormulaParse { ref formula, .. } if formula == "SUM("),
        "expected FormulaParse with original formula text, got {err:?}"
    );
}
```

Run: `cargo test -p xlstream-parse --test parse`. Expected: FAIL (still the unimplemented stub).

- [ ] **Step 2: Replace the stub `parse()` with the real impl.**

Replace `crates/xlstream-parse/src/parser.rs` body with:

```rust
//! The [`parse`] entry point.

use xlstream_core::XlStreamError;

use crate::ast::{Ast, Node, NumLiteral};
use crate::references::Reference;

/// Parse an Excel formula expression into an [`Ast`].
///
/// The input must **not** include a leading `=`; that's an I/O concern,
/// stripped before the parser sees the text.
///
/// # Errors
///
/// [`XlStreamError::FormulaParse`] for malformed input. The `address`
/// field is left empty; callers that have the cell address must enrich
/// with `map_err`. (Convention chosen by `docs/architecture/parse-reuse.md`
/// — keep `formualizer_parse` types out of the public signature.)
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// let ast = parse("SUM(A1:A10)").expect("parse failed");
/// assert!(format!("{ast:?}").contains("Function"));
/// ```
pub fn parse(expr: &str) -> Result<Ast, XlStreamError> {
    let upstream = formualizer_parse::parse(expr).map_err(|e| XlStreamError::FormulaParse {
        address: String::new(),
        formula: expr.to_owned(),
        message: e.to_string(),
    })?;

    Ok(Ast {
        root: lower(&upstream),
    })
}

/// Lower an upstream `ASTNode` into our internal [`Node`].
///
/// Stays `pub(crate)` — never re-exported. Errors are infallible because
/// the upstream tree is already valid by the time we get here; unsupported
/// reference kinds (`External`, `Table`) become `Reference::Named` with a
/// recognisable sentinel that classification refuses in Chunk 3.
pub(crate) fn lower(node: &formualizer_parse::ASTNode) -> Node {
    use formualizer_parse::ASTNodeType as T;
    use formualizer_parse::parser::ReferenceType as R;

    match &node.node_type {
        T::Literal(lv) => lower_literal(lv),
        T::UnaryOp { op, expr } => Node::UnaryOp {
            op: op.clone(),
            expr: Box::new(lower(expr)),
        },
        T::BinaryOp { op, left, right } => Node::BinaryOp {
            op: op.clone(),
            left: Box::new(lower(left)),
            right: Box::new(lower(right)),
        },
        T::Function { name, args } => Node::Function {
            name: name.clone(),
            args: args.iter().map(lower).collect(),
        },
        T::Array(rows) => Node::Array(
            rows.iter()
                .map(|r| r.iter().map(lower).collect())
                .collect(),
        ),
        T::Reference { reference, .. } => Node::Reference(match reference {
            R::Cell { sheet, row, col, .. } => Reference::Cell {
                sheet: sheet.clone(),
                row: *row,
                col: *col,
            },
            R::Range {
                sheet,
                start_row,
                end_row,
                start_col,
                end_col,
                ..
            } => Reference::Range {
                sheet: sheet.clone(),
                start_row: *start_row,
                end_row: *end_row,
                start_col: *start_col,
                end_col: *end_col,
            },
            R::NamedRange(name) => Reference::Named(name.clone()),
            // External + Table get a sentinel name that classification refuses.
            R::External(_) => Reference::Named("__xlstream_external__".to_owned()),
            R::Table(_) => Reference::Named("__xlstream_table__".to_owned()),
        }),
    }
}

fn lower_literal(lv: &formualizer_common::LiteralValue) -> Node {
    use formualizer_common::LiteralValue as L;
    match lv {
        L::Number(n) => Node::Literal(NumLiteral::Number(*n)),
        L::Int(i) => Node::Literal(NumLiteral::Number(*i as f64)),
        L::Boolean(b) => Node::Literal(NumLiteral::Bool(*b)),
        L::Text(s) => Node::Text(s.clone()),
        // Empty / Error / Date / Time / DateTime / Array — represented
        // as Text(literal-form) for now. Phase 4 evaluator handles
        // them; classification in Chunk 3 only cares about Reference
        // and Function nodes.
        other => Node::Text(format!("{other:?}")),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    #[test]
    fn parse_returns_ast_for_simple_arithmetic() {
        let ast = parse("1+2").unwrap();
        assert!(format!("{ast:?}").contains("BinaryOp"));
    }

    #[test]
    fn parse_returns_formula_parse_on_garbage() {
        let err = parse("SUM(").unwrap_err();
        assert!(matches!(err, XlStreamError::FormulaParse { .. }));
    }

    #[test]
    fn lower_handles_whole_column_range() {
        let upstream = formualizer_parse::parse("SUM(A:A)").unwrap();
        let node = lower(&upstream);
        let dbg = format!("{node:?}");
        assert!(dbg.contains("start_row: None"), "expected open range: {dbg}");
        assert!(dbg.contains("end_row: None"), "expected open range: {dbg}");
    }
}
```

> The doctest in the rustdoc above is the only public-facing example. Internal `lower()` is `pub(crate)` so it gets unit tests but no doctest.

- [ ] **Step 3: Verify upstream API names against `refs/`.**

Before running tests, grep `refs/formualizer/crates/formualizer-common/src/value.rs:85` for `LiteralValue` variants — confirm the variants we destructure (`Number`, `Int`, `Boolean`, `Text`) match. If upstream uses different names (e.g., `Float` instead of `Number`), update the `lower_literal` arms accordingly. **Do not guess.**

```bash
grep -nE "^\s+(Number|Int|Boolean|Text|Empty|Error|Date|Time|DateTime|Array)" \
  refs/formualizer/crates/formualizer-common/src/value.rs
```

- [ ] **Step 4: Run all tests.**

```bash
cargo test -p xlstream-parse --all-targets
```

Expected: all PASS (5 unit tests in parser.rs + 5 integration tests in tests/parse.rs + 2 in ast.rs + the existing classify stub tests + doctests).

### Task 0.8: Drop the placeholder doctest in the old `parse()` rustdoc

The Phase 1 stub doctest asserted `unwrap_err().to_string().contains("unimplemented")`. Replaced in Task 0.7's rustdoc with a real success example. Verify no stale doctest survives.

- [ ] **Step 1: `cargo test --doc -p xlstream-parse`** — must pass without referencing "unimplemented".

### Task 0.9: Run the full local gate

- [ ] **Step 1: `make check`** — fmt, clippy `-D warnings`, all tests, all doctests.

- [ ] **Step 2: If clippy fires `multiple_crate_versions`** (likely, with formualizer pulling chrono → potentially conflicting with workspace), add `#![allow(clippy::multiple_crate_versions)]` at the top of `crates/xlstream-parse/src/lib.rs`. This is one of the six ratified Phase 1 divergences.

### Task 0.10: Tick boxes in `docs/phases/phase-02-parser.md`

- [ ] **Step 1: Tick** in the **Parser** section:
  - `[x] Integrate formualizer-parse: call its parser, map its AST to our Ast type.`
  - `[x] Define our Ast as either a re-export of the upstream type or a thin wrapper. Keep wrapper if we'll extend with extra fields ...`
  - `[x] parse(expr: &str) returns Result<Ast, XlStreamError> with FormulaParse variant on error.`
  - `[x] Include source-error context: line/column if available from upstream, otherwise the offending substring.` — done via `e.to_string()` carrying upstream's diagnostic; the `formula` field holds the offending substring.
  - `[x] Rustdoc + doctest.`

Do **not** tick reference-extraction or classification boxes; those land later.

### Task 0.11: Commit

- [ ] **Step 1: Stage scoped files.**

```bash
git add Cargo.toml Cargo.lock \
  crates/xlstream-parse/Cargo.toml \
  crates/xlstream-parse/src/ast.rs \
  crates/xlstream-parse/src/parser.rs \
  crates/xlstream-parse/src/references.rs \
  crates/xlstream-parse/src/lib.rs \
  crates/xlstream-parse/tests/parse.rs \
  docs/phases/phase-02-parser.md
```

- [ ] **Step 2: Commit with the prefix the change touches most.**

```bash
git commit -m "xlstream-parse: integrate formualizer-parse 1.1.2 and ship real parse()"
```

### Task 0.12: Open PR 1

- [ ] **Step 1: Push.**

```bash
git push -u origin feature/phase-02-chunk-0-parse
```

- [ ] **Step 2: Open PR with the standard template.**

`gh pr create --title "xlstream-parse: integrate formualizer-parse 1.1.2 and ship real parse()" --body @body.md` — body covers What / Why / How / Testing / Docs / Checklist sections per `docs/standards/commits.md`. Reference ADR `docs/decisions/2026-04-18-phase-02-toolchain-bump.md` and PR #11.

- [ ] **Step 3: Ping Priscilla.** Wait for review + merge before starting Chunk 1.

---

## Chunk 1: `Reference` types + `extract_references` + tests

**Goal:** `extract_references(&ast)` returns a `Refs` containing every `Reference` and function name reachable from the AST. Pure walker; no allocation in the success case besides the `SmallVec` spill thresholds.

**Branch:** `feature/phase-02-chunk-1-references` (off main, after Chunk 0 merges).

### Task 1.1: Branch off main

- [ ] **Step 1: Sync + branch.**

```bash
git checkout main && git pull --ff-only
git checkout -b feature/phase-02-chunk-1-references
```

### Task 1.2: Reference helper methods (TDD)

**Files:**
- Modify: `crates/xlstream-parse/src/references.rs`

The `Reference` enum needs an accessor for "which sheet does this ref point to" so `extract_references` can de-dupe sheets, and a "is whole-column" predicate that `classify` will reuse in Chunk 3.

- [ ] **Step 1: Write the failing test.**

```rust
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn cell_reference_reports_its_sheet() {
        let r = Reference::Cell { sheet: Some("Sheet1".into()), row: 2, col: 3 };
        assert_eq!(r.sheet(), Some("Sheet1"));
    }

    #[test]
    fn whole_column_range_is_detected() {
        let r = Reference::Range {
            sheet: None,
            start_row: None, end_row: None,
            start_col: Some(1), end_col: Some(1),
        };
        assert!(r.is_whole_column());
    }

    #[test]
    fn bounded_range_is_not_whole_column() {
        let r = Reference::Range {
            sheet: None,
            start_row: Some(1), end_row: Some(10),
            start_col: Some(1), end_col: Some(1),
        };
        assert!(!r.is_whole_column());
    }
}
```

Run; expect compile error: methods undefined.

- [ ] **Step 2: Implement helpers.**

Append to `references.rs` (above the `#[cfg(test)]` block):

```rust
impl Reference {
    /// Sheet name this reference points to, if explicit.
    #[must_use]
    pub fn sheet(&self) -> Option<&str> {
        match self {
            Self::Cell { sheet, .. } | Self::Range { sheet, .. } => sheet.as_deref(),
            Self::Named(_) => None,
        }
    }

    /// `true` for `A:A`-style refs (no row bounds; column bounds present).
    /// Phase 2 classification accepts these only inside aggregates.
    #[must_use]
    pub fn is_whole_column(&self) -> bool {
        matches!(
            self,
            Self::Range {
                start_row: None, end_row: None,
                start_col: Some(_), end_col: Some(_),
                ..
            }
        )
    }

    /// `true` for `1:1`-style whole-row refs. Same Phase 2 acceptance.
    #[must_use]
    pub fn is_whole_row(&self) -> bool {
        matches!(
            self,
            Self::Range {
                start_col: None, end_col: None,
                start_row: Some(_), end_row: Some(_),
                ..
            }
        )
    }
}
```

Each method gets a one-line doc; Rust will fail CI without docs on `pub` items. Add `# Examples` doctests on each.

- [ ] **Step 3: Run unit tests.** Expect PASS.

### Task 1.3: `extract_references` walker (TDD)

**Files:**
- Modify: `crates/xlstream-parse/src/references.rs`
- Create: `crates/xlstream-parse/tests/references.rs`

- [ ] **Step 1: Write the integration test first.**

```rust
//! Integration tests for `extract_references`.

use xlstream_parse::{extract_references, parse, Reference};

#[test]
fn single_cell_extracts_one_cell_ref() {
    let ast = parse("A1").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.cells.len(), 1);
    assert!(matches!(refs.cells[0], Reference::Cell { row: 1, col: 1, .. }));
    assert!(refs.ranges.is_empty());
    assert!(refs.functions.is_empty());
}

#[test]
fn range_extracts_one_range_ref() {
    let ast = parse("A1:B10").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.ranges.len(), 1);
}

#[test]
fn cross_sheet_dedupes_sheet_names() {
    let ast = parse("'Tax Rates'!A1 + 'Tax Rates'!B2").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.sheets.iter().filter(|s| s.as_str() == "Tax Rates").count(), 1);
}

#[test]
fn whole_column_extracts_range() {
    let ast = parse("SUM(A:A)").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.ranges.len(), 1);
    assert!(refs.ranges[0].is_whole_column());
}

#[test]
fn nested_function_collects_all_function_names() {
    let ast = parse("IF(SUM(A1:A10) > 0, VLOOKUP(B1, X:Y, 2, FALSE), 0)").unwrap();
    let refs = extract_references(&ast);
    let names: Vec<&str> = refs.functions.iter().map(String::as_str).collect();
    assert!(names.contains(&"IF"));
    assert!(names.contains(&"SUM"));
    assert!(names.contains(&"VLOOKUP"));
}

#[test]
fn function_names_preserve_case_of_original_input() {
    let ast = parse("Sum(A1)").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.functions[0], "Sum");
}
```

Run: `cargo test -p xlstream-parse --test references`. Expected: FAIL — `extract_references` undefined.

- [ ] **Step 2: Implement the walker.**

Append to `references.rs`:

```rust
use crate::ast::{Ast, Node};

/// Walk `ast` and collect every reference, sheet, and function name.
///
/// Pure function. Output `Refs` uses [`smallvec::SmallVec`] sized to P99
/// formula widths per `docs/architecture/parse-reuse.md`; a `SUM(A:A)`
/// fits with no heap allocation, a 50-arg `IFS(...)` spills.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{extract_references, parse};
/// let ast = parse("SUM(A:A)").unwrap();
/// let refs = extract_references(&ast);
/// assert_eq!(refs.ranges.len(), 1);
/// ```
#[must_use]
pub fn extract_references(ast: &Ast) -> Refs {
    let mut out = Refs::default();
    walk(&ast.root, &mut out);
    out
}

fn walk(node: &Node, out: &mut Refs) {
    match node {
        Node::Literal(_) | Node::Text(_) => {}
        Node::Reference(r) => collect_ref(r, out),
        Node::UnaryOp { expr, .. } => walk(expr, out),
        Node::BinaryOp { left, right, .. } => {
            walk(left, out);
            walk(right, out);
        }
        Node::Function { name, args } => {
            // Push function name (case preserved), de-duped by linear scan
            // — function-name lists are short (P99 < 10).
            if !out.functions.iter().any(|n| n == name) {
                out.functions.push(name.clone());
            }
            for a in args {
                walk(a, out);
            }
        }
        Node::Array(rows) => {
            for row in rows {
                for cell in row {
                    walk(cell, out);
                }
            }
        }
    }
}

fn collect_ref(r: &Reference, out: &mut Refs) {
    if let Some(s) = r.sheet() {
        if !out.sheets.iter().any(|existing| existing == s) {
            out.sheets.push(s.to_owned());
        }
    }
    match r {
        Reference::Cell { .. } => out.cells.push(r.clone()),
        Reference::Range { .. } => out.ranges.push(r.clone()),
        Reference::Named(_) => out.ranges.push(r.clone()),
    }
}
```

- [ ] **Step 3: Re-export from `lib.rs`.**

Add `extract_references` to the existing `pub use references::{...}` line.

- [ ] **Step 4: Run all tests.**

```bash
cargo test -p xlstream-parse --all-targets
```

Expected: all PASS (Chunk 0 + new references unit tests + new integration tests).

### Task 1.4: Tick checklist + commit + PR

- [ ] **Step 1: Tick** in `docs/phases/phase-02-parser.md` under **Reference extraction**:
  - `[x] extract_references(ast: &Ast) -> References walks the AST and returns: All CellRefs / All RangeRefs / All SheetRefs / All function names called.`
  - `[x] References uses SmallVec<[T; N]> with N sized to the P99 case.`
  - `[x] Tests cover: single cell, range, cross-sheet, whole-column, nested functions.`

- [ ] **Step 2: `make check`.**

- [ ] **Step 3: Commit.**

```bash
git commit -am "xlstream-parse: add reference extraction walker"
```

- [ ] **Step 4: Push + PR (same template as Chunk 0).** Wait for merge.

---

## Chunk 2: `UnsupportedReason` + `Classification` upgrade + support sets

**Goal:** Replace `Classification::Unsupported(String)` with `Unsupported { reason: UnsupportedReason, doc_link: &'static str }`. Add the `phf::Set` constants for `UNSUPPORTED_FUNCTIONS`, `AGGREGATE_FUNCTIONS`, and `LOOKUP_FUNCTIONS`. No classification logic yet — that's Chunk 3. This chunk is purely the type surface.

**Branch:** `feature/phase-02-chunk-2-classification-types`.

### Task 2.1: Branch off main

- [ ] **Step 1.** Standard sync + branch.

### Task 2.2: Add `phf` to xlstream-parse

**Files:** `crates/xlstream-parse/Cargo.toml`.

- [ ] **Step 1: Add to `[dependencies]`.**

```toml
phf = { workspace = true }
```

`phf` is already in `[workspace.dependencies]` with `features = ["macros"]`; we inherit.

### Task 2.3: Define `UnsupportedReason` (TDD)

**Files:** `crates/xlstream-parse/src/classify.rs`.

- [ ] **Step 1: Write the failing test.**

```rust
#[test]
fn unsupported_reason_renders_human_message() {
    let r = UnsupportedReason::UnsupportedFunction("OFFSET".into());
    let s = r.to_string();
    assert!(s.contains("OFFSET"), "expected OFFSET in message: {s}");
}

#[test]
fn unsupported_reason_doc_link_is_stable_url() {
    let r = UnsupportedReason::ForwardRowRef;
    assert!(r.doc_link().starts_with("https://"));
}
```

Expected: FAIL — `UnsupportedReason` not defined.

- [ ] **Step 2: Define the enum + `Display` + `doc_link()`.**

Insert into `classify.rs` (above the existing `Classification` enum):

```rust
/// The specific reason a formula was rejected. Replaces Phase 1's
/// `Unsupported(String)`.
///
/// Each variant maps to a `&'static str` doc link via [`Self::doc_link`]
/// so the user-facing error message can deep-link to the explanation.
///
/// # Examples
///
/// ```
/// use xlstream_parse::UnsupportedReason;
/// let r = UnsupportedReason::UnsupportedFunction("OFFSET".into());
/// assert!(r.to_string().contains("OFFSET"));
/// assert!(r.doc_link().starts_with("https://"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsupportedReason {
    /// Reference to a row the streaming pass has not yet seen.
    ForwardRowRef,
    /// Cell references itself (directly or transitively).
    CircularRef,
    /// Function not in any of the supported sets.
    UnsupportedFunction(String),
    /// Bare `A:A` outside an aggregate (would force whole-column materialisation).
    UnboundedRange,
    /// Aggregate criteria (`SUMIF`, `COUNTIF`, …) computed at row time, not pre-known.
    NonStaticCriteria,
    /// Dynamic-array (`FILTER`, `UNIQUE`, …) that spills to multiple cells.
    DynamicArray,
    /// Volatile function not in the streaming-OK set.
    VolatileUnsupported,
    /// `External` or `Table` reference upstream surfaced. Out of Phase 2 scope.
    UnsupportedReferenceKind,
    /// Sub-expression nested under another unsupported sub-expression.
    NestedUnsupported,
}

impl std::fmt::Display for UnsupportedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForwardRowRef =>
                write!(f, "references a row the streaming pass has not yet seen"),
            Self::CircularRef =>
                write!(f, "circular reference"),
            Self::UnsupportedFunction(name) =>
                write!(f, "function {name} is not supported"),
            Self::UnboundedRange =>
                write!(f, "whole-column reference outside an aggregate"),
            Self::NonStaticCriteria =>
                write!(f, "aggregate criteria computed per-row are not supported"),
            Self::DynamicArray =>
                write!(f, "dynamic-array spill is not supported"),
            Self::VolatileUnsupported =>
                write!(f, "volatile function is not in the streaming-OK set"),
            Self::UnsupportedReferenceKind =>
                write!(f, "external or table references are not supported"),
            Self::NestedUnsupported =>
                write!(f, "contains an unsupported sub-expression"),
        }
    }
}

impl UnsupportedReason {
    /// Stable documentation URL for this refusal.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::UnsupportedReason;
    /// assert!(UnsupportedReason::ForwardRowRef.doc_link().starts_with("https://"));
    /// ```
    #[must_use]
    pub fn doc_link(&self) -> &'static str {
        const BASE: &str =
            "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md";
        match self {
            Self::ForwardRowRef | Self::CircularRef =>
                concat!(
                    "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md",
                    "#classification-algorithm"
                ),
            Self::UnsupportedFunction(_)
            | Self::DynamicArray
            | Self::VolatileUnsupported =>
                concat!(
                    "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md",
                    "#why-offset-and-indirect-are-unsupported"
                ),
            Self::UnboundedRange =>
                concat!(
                    "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md",
                    "#aggregate-of-a-column"
                ),
            Self::NonStaticCriteria =>
                concat!(
                    "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md",
                    "#aggregate-pre-pass"
                ),
            Self::UnsupportedReferenceKind | Self::NestedUnsupported => BASE,
        }
    }
}
```

> The `BASE` constant is unused in the match if every arm has its own anchor — but kept here as the documented fallback for the last two variants. Avoid the `clippy::const_is_empty` warning by referencing `BASE` directly from the last arms (already done above).

- [ ] **Step 3: Run unit tests.** Expect PASS.

### Task 2.4: Upgrade `Classification` (TDD; this is the breaking change)

**Files:**
- Modify: `crates/xlstream-parse/src/classify.rs`
- Indirect: any callsite that pattern-matched `Classification::Unsupported(String)`.

The grep done in plan-prep confirms only `classify.rs` itself (the stub `classify()` and its two tests) matches. No callers in `xlstream-cli`, `xlstream-eval`, or `xlstream-io`.

- [ ] **Step 1: Update the existing two tests** in `classify.rs::tests` to expect the new shape:

```rust
#[test]
fn classify_returns_unsupported_stub() {
    let ast = Ast::from_root_for_tests(/* simplest node */);
    let ctx = ClassificationContext::default();
    match classify(&ast, &ctx) {
        Classification::Unsupported { reason, doc_link } => {
            assert!(matches!(reason, UnsupportedReason::NestedUnsupported));
            assert!(doc_link.starts_with("https://"));
        }
        other => panic!("expected Unsupported, got {other:?}"),
    }
}
```

(The `from_root_for_tests` constructor exists from Chunk 0.)

Run; expect FAIL — enum still on the old shape.

- [ ] **Step 2: Update the enum.**

```rust
/// The verdict returned by [`classify`] for a single formula.
///
/// [...]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Classification {
    /// Formula reads only the current row; streamable without prelude.
    RowLocal,
    /// Formula reads only prelude-computed aggregates.
    AggregateOnly,
    /// Formula reads only prelude-loaded lookup sheets.
    LookupOnly,
    /// Formula mixes row-local, aggregate, and/or lookup reads — still
    /// streamable via prelude + row data.
    Mixed,
    /// Formula cannot be streamed.
    Unsupported {
        /// Specific reason for refusal.
        reason: UnsupportedReason,
        /// Stable documentation URL explaining the refusal class.
        doc_link: &'static str,
    },
}
```

Update the stub `classify()` body:

```rust
#[must_use]
pub fn classify(_ast: &Ast, _ctx: &ClassificationContext) -> Classification {
    let reason = UnsupportedReason::NestedUnsupported;
    let doc_link = reason.doc_link();
    Classification::Unsupported { reason, doc_link }
}
```

Update the existing `Classification` doctest to use `matches!(..., Classification::Unsupported { .. })`.

- [ ] **Step 3: Re-export `UnsupportedReason` from `lib.rs`.**

Append to the existing `pub use classify::{...}` line: `..., UnsupportedReason};`.

- [ ] **Step 4: Run all tests.** Expect PASS.

### Task 2.5: Define support-set constants in `sets.rs` (TDD)

**Files:** Create `crates/xlstream-parse/src/sets.rs`.

- [ ] **Step 1: Write the failing test.**

```rust
#[test]
fn aggregate_set_recognises_sum_case_insensitively() {
    assert!(is_aggregate("SUM"));
    assert!(is_aggregate("sum"));
    assert!(is_aggregate("Sum"));
}

#[test]
fn unsupported_set_lists_offset_indirect_filter() {
    assert!(is_unsupported("OFFSET"));
    assert!(is_unsupported("INDIRECT"));
    assert!(is_unsupported("FILTER"));
}

#[test]
fn lookup_set_lists_vlookup_xlookup_index() {
    assert!(is_lookup("VLOOKUP"));
    assert!(is_lookup("XLOOKUP"));
    assert!(is_lookup("INDEX"));
}
```

Expected: FAIL — module/functions undefined.

- [ ] **Step 2: Implement.**

```rust
//! Function-name sets used by the classifier.
//!
//! Sets are stored in upper-case; lookups normalise the incoming name to
//! upper-case to give Excel-style case-insensitivity.

use phf::{phf_set, Set};

/// Functions xlstream cannot stream. Listed in
/// `docs/phases/phase-02-parser.md`.
pub static UNSUPPORTED: Set<&'static str> = phf_set! {
    "OFFSET", "INDIRECT", "FILTER", "UNIQUE", "SORT", "SORTBY",
    "SEQUENCE", "RANDARRAY", "LAMBDA", "LET", "HYPERLINK",
    "WEBSERVICE", "CUBEVALUE", "CUBEMEMBER", "CUBESET",
};

/// Functions evaluable in a single column pre-pass.
pub static AGGREGATE: Set<&'static str> = phf_set! {
    "SUM", "COUNT", "COUNTA", "AVERAGE", "MIN", "MAX", "PRODUCT",
    "SUMIF", "COUNTIF", "AVERAGEIF",
    "SUMIFS", "COUNTIFS", "AVERAGEIFS", "MINIFS", "MAXIFS",
    "MEDIAN",
};

/// Lookup functions allowed against pre-loaded lookup sheets.
pub static LOOKUP: Set<&'static str> = phf_set! {
    "VLOOKUP", "HLOOKUP", "XLOOKUP", "MATCH", "XMATCH", "INDEX", "CHOOSE",
};

/// `true` if `name` is in [`UNSUPPORTED`] (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_unsupported;
/// assert!(is_unsupported("offset"));
/// ```
#[must_use]
pub fn is_unsupported(name: &str) -> bool {
    UNSUPPORTED.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in [`AGGREGATE`] (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_aggregate;
/// assert!(is_aggregate("Sum"));
/// ```
#[must_use]
pub fn is_aggregate(name: &str) -> bool {
    AGGREGATE.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in [`LOOKUP`] (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_lookup;
/// assert!(is_lookup("vlookup"));
/// ```
#[must_use]
pub fn is_lookup(name: &str) -> bool {
    LOOKUP.contains(name.to_uppercase().as_str())
}

#[cfg(test)]
mod tests { /* the three tests from Step 1 */ }
```

> Note: each lookup allocates a `String` for upper-casing. Acceptable — function names are short, and classification runs once per unique formula text (the AST cache amortises across all 400k rows).

- [ ] **Step 3: Wire into `lib.rs`.**

Add `pub mod sets;` (public so consumers can reach `is_aggregate` etc., useful for diagnostics and the CLI).

- [ ] **Step 4: Run all tests.**

### Task 2.6: Tick boxes + commit + PR

- [ ] **Step 1: Tick** in `docs/phases/phase-02-parser.md` under **Classification**:
  - `[x] UnsupportedReason enum: ForwardRowRef, CircularRef, UnsupportedFunction(String), UnboundedRange, NonStaticCriteria, DynamicArray, VolatileUnsupported, etc.`
  - `[x] Support-set constants: UNSUPPORTED_FUNCTIONS / AGGREGATE_FUNCTIONS / LOOKUP_FUNCTIONS.`

Don't tick the algorithm sub-boxes (RowLocal/AggregateOnly/LookupOnly/Mixed) — those land in Chunk 3.

- [ ] **Step 2: `make check`.**

- [ ] **Step 3: Commit.**

```bash
git commit -am "xlstream-parse: upgrade Classification to structured Unsupported and add fn-name sets"
```

- [ ] **Step 4: Push + PR.** Standard template. Note in PR description: "Breaking change to `Classification::Unsupported`. No external callers (verified by grep)."

---

## Chunk 3: `classify` impl + 30+ tests

**Goal:** `classify(ast, ctx)` returns the correct verdict for every shape in `docs/phases/phase-02-parser.md`. 30+ tests across 5 test files (one per classification verdict). Largest chunk.

**Branch:** `feature/phase-02-chunk-3-classify-impl`. **Watch the line budget.** If the diff exceeds ~700 lines, split into 3a (algorithm + RowLocal/Aggregate/Lookup tests) and 3b (Mixed/Unsupported tests + edge cases) before pushing.

### Task 3.1: Branch off main; pull merged Chunks 0–2.

### Task 3.2: Expand `ClassificationContext` (TDD)

**Files:** `crates/xlstream-parse/src/classify.rs`.

The Phase 1 stub had a placeholder context. Replace with the real fields per `docs/architecture/parse-reuse.md`:

```rust
use std::collections::{HashMap, HashSet};

/// Context the classifier consults to decide a formula's verdict.
///
/// Built once per main sheet during the prelude (Phase 4); shared
/// immutably across every formula evaluation in that sheet.
///
/// # Examples
///
/// ```
/// use xlstream_parse::ClassificationContext;
/// let ctx = ClassificationContext::default();
/// assert!(ctx.headers().is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ClassificationContext {
    /// Header → 1-based column index, for the main sheet.
    headers: HashMap<String, u32>,
    /// Sheet names designated as lookup sheets (loaded fully into prelude).
    lookup_sheets: HashSet<String>,
    /// (Sheet, row, col) of the cell currently being classified —
    /// `Some` once the prelude knows the address of the formula. Used for
    /// circular-reference + forward-row-ref detection.
    current_address: Option<(String, u32, u32)>,
    /// Workbook-level named ranges and their resolved targets.
    named_ranges: HashMap<String, Reference>,
}
```

Add `with_*` builder methods + getters; each gets a doctest. Keep mutators private — `ClassificationContext` is built once per sheet by the prelude.

- [ ] **Step 1.** Write tests for each accessor (default empty, builder appends, getter returns the same).
- [ ] **Step 2.** Implement.
- [ ] **Step 3.** Run.

### Task 3.3: Skeleton classifier (TDD — start with RowLocal)

**Files:** `crates/xlstream-parse/src/classify.rs`.

The classifier is a tree walk that:
1. Collects every reference + function name.
2. Decides each ref's disposition per the table in `docs/architecture/streaming-model.md#classification-algorithm`.
3. Combines per-ref dispositions into a single `Classification`.

Implementation strategy: define an internal `Disposition` enum, walk the tree producing one `Disposition` per leaf, then **fold** dispositions up the tree.

```rust
#[derive(Debug, Clone, PartialEq)]
enum Disposition {
    RowLocal,
    Aggregate,
    Lookup,
    Unsupported(UnsupportedReason),
}
```

Folding rules:
- `RowLocal + RowLocal = RowLocal`.
- `RowLocal + Aggregate = Mixed`.
- `RowLocal + Lookup = Mixed`.
- `Aggregate + Aggregate = AggregateOnly` (when aggregates differ they're both still aggregates, the verdict is `AggregateOnly`).
- `Aggregate + Lookup = Mixed`.
- `Lookup + Lookup = LookupOnly`.
- `_ + Unsupported = Unsupported`.
- `Mixed + anything-supported = Mixed`.

Verdict from the root disposition:
- All-RowLocal → `RowLocal`.
- All-Aggregate → `AggregateOnly`.
- All-Lookup → `LookupOnly`.
- Any mix → `Mixed`.
- Any `Unsupported` → propagate.

- [ ] **Step 1: Test simplest RowLocal first.**

Create `crates/xlstream-parse/tests/classify_row_local.rs`:

```rust
use xlstream_parse::{classify, parse, Classification, ClassificationContext};

#[test]
fn literal_arithmetic_is_row_local() {
    let ast = parse("1+2").unwrap();
    let ctx = ClassificationContext::default();
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn current_row_cell_arithmetic_is_row_local() {
    let ast = parse("A2*B2").unwrap();
    let mut ctx = ClassificationContext::default();
    ctx = ctx.with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn if_with_row_local_branches_is_row_local() {
    let ast = parse(r#"IF(A2>0, "Y", "N")"#).unwrap();
    let mut ctx = ClassificationContext::default();
    ctx = ctx.with_current_address("Sheet1", 2, 3);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn upper_left_year_text_concat_is_row_local() {
    let ast = parse(r#"UPPER(LEFT(A2,3))&"-"&YEAR(B2)"#).unwrap();
    let mut ctx = ClassificationContext::default();
    ctx = ctx.with_current_address("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}
```

Expected: FAIL — `classify` still returns the placeholder `Unsupported`.

- [ ] **Step 2: Implement enough of the classifier to pass.**

Replace the stub `classify()` body with a real walker. Sketch:

```rust
#[must_use]
pub fn classify(ast: &Ast, ctx: &ClassificationContext) -> Classification {
    match disposition(&ast.root, ctx) {
        Disposition::RowLocal => Classification::RowLocal,
        Disposition::Aggregate => Classification::AggregateOnly,
        Disposition::Lookup => Classification::LookupOnly,
        Disposition::Unsupported(reason) => {
            let doc_link = reason.doc_link();
            Classification::Unsupported { reason, doc_link }
        }
    }
}

fn disposition(node: &Node, ctx: &ClassificationContext) -> Disposition { /* ... */ }
```

Implement `disposition` to handle:
- `Literal` / `Text` → `RowLocal`.
- `Reference(Cell { row, sheet, .. })` → `RowLocal` if `(sheet, row) == ctx.current_address.{sheet, row}` (or `sheet=None` matches active), else if same column same row → RowLocal, else `Unsupported(ForwardRowRef)`. Self-reference detection: `(sheet, row, col) == ctx.current_address` → `Unsupported(CircularRef)`.
- `Reference(Range)` → only valid inside an Aggregate or Lookup function; bare → `Unsupported(UnboundedRange)`. Resolution must come from parent context — simplest impl: track parent context in a recursive arg.
- `Reference(Named)` → look up; if external/table sentinel → `Unsupported(UnsupportedReferenceKind)`.
- `BinaryOp { left, right }` → fold dispositions. Verdict combination rules above.
- `UnaryOp { expr }` → recurse.
- `Function { name, args }` → check `sets::is_unsupported(name)` first → `Unsupported(UnsupportedFunction)`. Else if `is_aggregate(name)` → expect range arg, return `Disposition::Aggregate`. Else if `is_lookup(name)` → expect range arg pointing at a lookup sheet, return `Disposition::Lookup`. Else (e.g. `IF`, `UPPER`, `YEAR`, `&`, `+`) recurse into args and fold.
- `Array` → recurse into all cells; folded.

Test each shape one at a time. Keep iterations small. Run after each impl.

- [ ] **Step 3: Add the `Disposition` enum + folding helpers.** Keep `pub(crate)` — internal abstraction.

- [ ] **Step 4: Run RowLocal tests.** Iterate until green.

### Task 3.4: AggregateOnly tests + impl

**Files:** Create `crates/xlstream-parse/tests/classify_aggregate.rs`.

```rust
use xlstream_parse::{classify, parse, sets::is_aggregate, Classification, ClassificationContext};

#[test]
fn sum_whole_column_is_aggregate_only() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::default();
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn sumif_with_static_criterion_is_aggregate_only() {
    let ast = parse(r#"SUMIF(A:A, "EMEA", B:B)"#).unwrap();
    let ctx = ClassificationContext::default();
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn count_whole_column_is_aggregate_only() {
    let ast = parse("COUNT(A:A)").unwrap();
    let ctx = ClassificationContext::default();
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn nested_aggregates_are_still_aggregate_only() {
    let ast = parse("SUM(A:A)+COUNT(B:B)").unwrap();
    let ctx = ClassificationContext::default();
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn aggregate_set_recognises_each_listed_function() {
    for name in ["SUM", "AVERAGE", "MIN", "MAX", "MEDIAN", "PRODUCT"] {
        assert!(is_aggregate(name), "{name} should be aggregate");
    }
}
```

- [ ] **Step 1.** Write tests; expect failures on shapes the algorithm doesn't yet handle.
- [ ] **Step 2.** Extend `disposition` to handle aggregate functions. Each aggregate-fn arg that's a `Range` returns `Disposition::Aggregate`; mixing a `Range` with a non-aggregate function is `Unsupported(UnboundedRange)`.
- [ ] **Step 3.** Iterate to green.

### Task 3.5: LookupOnly tests + impl

**Files:** Create `crates/xlstream-parse/tests/classify_lookup.rs`.

```rust
use xlstream_parse::{classify, parse, Classification, ClassificationContext};

fn lookup_ctx() -> ClassificationContext {
    ClassificationContext::default()
        .with_lookup_sheet("Region Info")
        .with_lookup_sheet("Tax Rates")
}

#[test]
fn vlookup_into_lookup_sheet_is_lookup_only() {
    let ast = parse(r#"VLOOKUP(A2, 'Region Info'!A:C, 2, FALSE)"#).unwrap();
    let ctx = lookup_ctx().with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn xlookup_into_lookup_sheet_is_lookup_only() {
    let ast = parse(r#"XLOOKUP(A2, 'Region Info'!A:A, 'Region Info'!B:B)"#).unwrap();
    let ctx = lookup_ctx().with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn vlookup_with_concat_key_is_lookup_only() {
    let ast = parse(r#"VLOOKUP(A2&B2, 'Tax Rates'!D:E, 2, FALSE)"#).unwrap();
    let ctx = lookup_ctx().with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn match_into_lookup_sheet_is_lookup_only() {
    let ast = parse(r#"MATCH(A2, 'Region Info'!A:A, 0)"#).unwrap();
    let ctx = lookup_ctx().with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn lookup_into_unknown_sheet_is_unsupported() {
    let ast = parse(r#"VLOOKUP(A2, NotALookupSheet!A:C, 2, FALSE)"#).unwrap();
    let ctx = lookup_ctx().with_current_address("Sheet1", 2, 1);
    assert!(matches!(
        classify(&ast, &ctx),
        Classification::Unsupported { .. }
    ));
}
```

- [ ] **Step 1–3.** Same TDD pattern. Implement lookup-fn handling: if a `Range` arg's `sheet` is in `ctx.lookup_sheets`, the disposition is `Lookup`. If not, `Unsupported(UnsupportedFunction)`-ish — pick `UnboundedRange` if the disposition is the issue, or surface a new variant if a more specific one is needed (extend `UnsupportedReason` if so; document why).

### Task 3.6: Mixed tests + impl

**Files:** Create `crates/xlstream-parse/tests/classify_mixed.rs`.

```rust
use xlstream_parse::{classify, parse, Classification, ClassificationContext};

#[test]
fn deal_value_over_sum_is_mixed() {
    // Real benchmark formula: row-local cell divided by whole-column aggregate.
    let ast = parse("A2/SUM(A:A)").unwrap();
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}

#[test]
fn lookup_plus_row_local_is_mixed() {
    let ast = parse(r#"VLOOKUP(A2, 'Region Info'!A:C, 2, FALSE) + A2"#).unwrap();
    let ctx = ClassificationContext::default()
        .with_lookup_sheet("Region Info")
        .with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}

#[test]
fn aggregate_plus_lookup_is_mixed() {
    let ast = parse(r#"SUM(A:A) + VLOOKUP(B2, 'Region Info'!A:C, 2, FALSE)"#).unwrap();
    let ctx = ClassificationContext::default()
        .with_lookup_sheet("Region Info")
        .with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}

#[test]
fn nested_if_with_aggregate_branch_is_mixed() {
    let ast = parse("IF(A2>0, SUM(B:B), A2)").unwrap();
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}
```

- [ ] **Step 1–3.** TDD; the folding rules from Task 3.3 should already produce `Mixed` for these. Iterate.

### Task 3.7: Unsupported tests + impl + error-message assertions

**Files:** Create `crates/xlstream-parse/tests/classify_unsupported.rs`.

The phase doc lists the canonical Unsupported cases; expand to ≥10 in this file.

```rust
use xlstream_parse::{classify, parse, Classification, ClassificationContext, UnsupportedReason};

fn unsupported_with(text: &str, ctx: &ClassificationContext) -> (UnsupportedReason, &'static str) {
    let ast = parse(text).expect("parse");
    match classify(&ast, ctx) {
        Classification::Unsupported { reason, doc_link } => (reason, doc_link),
        other => panic!("expected Unsupported for {text:?}, got {other:?}"),
    }
}

#[test]
fn offset_is_refused_with_unsupported_function() {
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let (reason, link) = unsupported_with("OFFSET(A1,1,0)", &ctx);
    assert!(matches!(reason, UnsupportedReason::UnsupportedFunction(ref s) if s == "OFFSET"));
    assert!(link.starts_with("https://"));
}

#[test]
fn indirect_is_refused() {
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let (reason, _) = unsupported_with(r#"INDIRECT("A1")"#, &ctx);
    assert!(matches!(reason, UnsupportedReason::UnsupportedFunction(ref s) if s == "INDIRECT"));
}

#[test]
fn forward_row_ref_is_refused() {
    let ast = parse("A3").unwrap();
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 5, 1);
    let v = classify(&ast, &ctx);
    assert!(matches!(
        v,
        Classification::Unsupported { reason: UnsupportedReason::ForwardRowRef, .. }
    ));
}

#[test]
fn circular_self_reference_is_refused() {
    let ast = parse("A2+1").unwrap();
    // Cell A2 references A2 → cycle.
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let v = classify(&ast, &ctx);
    assert!(matches!(
        v,
        Classification::Unsupported { reason: UnsupportedReason::CircularRef, .. }
    ));
}

#[test]
fn filter_dynamic_array_is_refused() {
    let ctx = ClassificationContext::default();
    let (reason, _) = unsupported_with("FILTER(A:A, B:B>0)", &ctx);
    assert!(matches!(reason, UnsupportedReason::UnsupportedFunction(ref s) if s == "FILTER"));
}

#[test]
fn unique_dynamic_array_is_refused() {
    let ctx = ClassificationContext::default();
    let (reason, _) = unsupported_with("UNIQUE(A:A)", &ctx);
    assert!(matches!(reason, UnsupportedReason::UnsupportedFunction(ref s) if s == "UNIQUE"));
}

#[test]
fn bare_whole_column_is_refused() {
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let (reason, _) = unsupported_with("A:A*2", &ctx);
    assert!(matches!(reason, UnsupportedReason::UnboundedRange));
}

#[test]
fn nested_unsupported_propagates() {
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let (reason, _) = unsupported_with("IF(A2>0, OFFSET(A1,1,0), 0)", &ctx);
    assert!(matches!(reason, UnsupportedReason::UnsupportedFunction(ref s) if s == "OFFSET"));
}

#[test]
fn external_reference_is_refused() {
    let ast = parse("[OtherBook.xlsx]Sheet1!A1").unwrap();
    let ctx = ClassificationContext::default();
    let v = classify(&ast, &ctx);
    assert!(matches!(
        v,
        Classification::Unsupported { reason: UnsupportedReason::UnsupportedReferenceKind, .. }
    ));
}

#[test]
fn lookup_into_unloaded_sheet_is_refused() {
    let ast = parse("VLOOKUP(A2, NotALookup!A:C, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let v = classify(&ast, &ctx);
    assert!(matches!(v, Classification::Unsupported { .. }));
}
```

- [ ] **Step 1.** Write the file.
- [ ] **Step 2.** Implement each unsupported branch in `disposition`.
- [ ] **Step 3.** Verify error-message helpers expose the doc link properly. Add an integration check: `r.doc_link().contains("streaming-model.md")` or the like.

### Task 3.8: Reference-extraction parity tests

The phase doc requires "Reference-extraction tests for each variant." Chunk 1 already added these to `tests/references.rs`; verify coverage for cell, range, cross-sheet, whole-column, nested, named-range. Add what's missing.

- [ ] **Step 1.** Inspect existing tests; list any variant uncovered.
- [ ] **Step 2.** Add missing tests.

### Task 3.9: Tick checklist + commit + PR

- [ ] **Step 1: Tick** in `docs/phases/phase-02-parser.md` under **Classification**:
  - `[x] RowLocal — references only current-row cells; no function in UNSUPPORTED or REQUIRES_PRELUDE sets.`
  - `[x] AggregateOnly — root is a supported aggregate function; all range args are whole-column or fixed.`
  - `[x] LookupOnly — supported lookup function with cross-sheet range into a lookup sheet.`
  - `[x] Mixed — recurses; ok if every sub-expression classifies.`
  - `[x] Unsupported(UnsupportedReason) — with a specific reason + doc link.`

Under **Tests**:
  - `[x] 30+ classification tests covering: simple arithmetic / SUM(A:A) / VLOOKUP / Mixed / OFFSET / INDIRECT / forward ref / FILTER / circular / VLOOKUP concat / nested mixed.`
  - `[x] Reference-extraction tests for each variant.`

Under **Error messages**:
  - `[x] Every Unsupported path produces a user-facing message that quotes the formula text / names the specific reason / includes a doc link.` (Note: "quotes the formula text" is the consumer's responsibility — the classifier returns the reason + link; the eval-layer wraps with `XlStreamError::Unsupported { address, formula, reason: reason.to_string(), doc_link }`. Make sure the rustdoc on `Classification::Unsupported` documents this contract.)
  - `[x] Tests assert on message substrings.`

- [ ] **Step 2: `make check`.**

- [ ] **Step 3: Verify total test count.**

```bash
cargo test -p xlstream-parse 2>&1 | grep -E "test result"
```

Expected: ≥ 30 across the integration files plus ~10 unit tests in `classify.rs`.

- [ ] **Step 4: Commit + PR.**

```bash
git commit -am "xlstream-parse: implement classify with 30+ tests across all five verdicts"
```

PR description calls out: "Resolves the bulk of `docs/phases/phase-02-parser.md`'s Classification + Tests sections."

---

## Chunk 4: AST rewrite + `PreludeRef` + `PreludeKey` + golden tests

**Goal:** Add a `Node::PreludeRef(PreludeKey)` variant. `rewrite(ast, ctx, classification)` walks a Mixed/AggregateOnly/LookupOnly AST and replaces supported aggregate / lookup sub-expressions with `Node::PreludeRef(PreludeKey)`. Pure function. Golden tests assert input → expected output via `Debug` formatting.

**Branch:** `feature/phase-02-chunk-4-ast-rewrite`.

### Task 4.1: Branch off main

### Task 4.2: Add `Node::PreludeRef` variant + `PreludeKey`

**Files:**
- Modify: `crates/xlstream-parse/src/ast.rs`
- Create: `crates/xlstream-parse/src/rewrite.rs`

- [ ] **Step 1: Define `PreludeKey` in `rewrite.rs`.**

```rust
//! AST rewrite — substitutes supported aggregate / lookup sub-expressions
//! with [`crate::ast::Node::PreludeRef`] nodes for the Phase 4 evaluator.

/// Identifies a prelude-computed value the evaluator can look up by key.
///
/// Aggregates are keyed by `(kind, sheet, column)`; lookups by their
/// fully-resolved range plus the key shape.
///
/// # Examples
///
/// ```
/// use xlstream_parse::PreludeKey;
/// let k = PreludeKey::Aggregate {
///     kind: xlstream_parse::AggKind::Sum,
///     sheet: None,
///     column: 1,
/// };
/// assert!(format!("{k:?}").contains("Sum"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreludeKey {
    /// Aggregate over a single column.
    Aggregate {
        /// Which aggregate kind.
        kind: AggKind,
        /// Sheet that owns the column. `None` = active sheet.
        sheet: Option<String>,
        /// 1-based column index.
        column: u32,
    },
    /// Lookup table prepared during prelude.
    Lookup {
        /// Lookup function family.
        kind: LookupKind,
        /// Sheet the lookup table lives on.
        sheet: String,
        /// 1-based column index of the key column.
        key_column: u32,
        /// 1-based column index of the value column returned.
        value_column: u32,
    },
}

/// Aggregate function family. Mirrors the union of [`crate::sets::AGGREGATE`]
/// shapes the evaluator supports without per-row work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggKind { Sum, Count, CountA, Average, Min, Max, Product, Median }

/// Lookup function family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LookupKind { VLookup, HLookup, XLookup, IndexMatch }
```

- [ ] **Step 2: Add the variant to `Node` (in `ast.rs`).**

```rust
pub(crate) enum Node {
    // ... existing variants
    /// Reference into the prelude-computed scalar/lookup table. Inserted
    /// by [`crate::rewrite`].
    PreludeRef(crate::rewrite::PreludeKey),
}
```

Update any `match` over `Node` (in `parser::lower`, `references::walk`, future spots) to handle the new variant — `PreludeRef` is a leaf, contributes no references, and only appears post-rewrite.

- [ ] **Step 3: Re-export from `lib.rs`.**

```rust
pub use rewrite::{rewrite, AggKind, LookupKind, PreludeKey};
```

### Task 4.3: Implement `rewrite` (TDD with golden tests)

- [ ] **Step 1: Write the golden test first.**

Create `crates/xlstream-parse/tests/rewrite.rs`:

```rust
use xlstream_parse::{classify, parse, rewrite, AggKind, ClassificationContext, PreludeKey};

#[test]
fn sum_whole_column_collapses_to_prelude_ref() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::default();
    let v = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &v);
    let dbg = format!("{rewritten:?}");
    assert!(dbg.contains("PreludeRef"), "expected PreludeRef in: {dbg}");
    assert!(dbg.contains("Sum"));
    assert!(!dbg.contains("Function"), "SUM should be replaced: {dbg}");
}

#[test]
fn deal_value_over_sum_collapses_only_the_aggregate() {
    let ast = parse("A2/SUM(A:A)").unwrap();
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let v = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &v);
    let dbg = format!("{rewritten:?}");
    assert!(dbg.contains("BinaryOp"), "outer / preserved: {dbg}");
    assert!(dbg.contains("PreludeRef"), "inner SUM rewritten: {dbg}");
    // Cell ref preserved literally.
    assert!(dbg.contains("Cell"));
}

#[test]
fn unsupported_classifications_pass_through_untouched() {
    let ast = parse("OFFSET(A1,1,0)").unwrap();
    let ctx = ClassificationContext::default().with_current_address("Sheet1", 2, 1);
    let v = classify(&ast, &ctx);
    let cloned = ast.clone();
    let rewritten = rewrite(ast, &ctx, &v);
    assert_eq!(format!("{cloned:?}"), format!("{rewritten:?}"));
}

#[test]
fn vlookup_collapses_to_prelude_lookup() {
    let ast = parse(r#"VLOOKUP(A2, 'Region Info'!A:C, 2, FALSE)"#).unwrap();
    let ctx = ClassificationContext::default()
        .with_lookup_sheet("Region Info")
        .with_current_address("Sheet1", 2, 1);
    let v = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &v);
    let dbg = format!("{rewritten:?}");
    assert!(dbg.contains("Lookup"));
    assert!(dbg.contains("Region Info"));
}
```

Expected: FAIL — `rewrite` undefined.

- [ ] **Step 2: Implement `rewrite()`.**

```rust
use crate::ast::{Ast, Node};
use crate::classify::Classification;

/// Replace supported aggregate / lookup sub-expressions in `ast` with
/// [`Node::PreludeRef`] nodes. Pure function — input `ast` is consumed and
/// the rewritten tree returned.
///
/// `Unsupported` classifications pass through untouched (the evaluator
/// will reject them at run time via `XlStreamError::Unsupported`).
///
/// # Examples
///
/// ```
/// use xlstream_parse::{classify, parse, rewrite, ClassificationContext};
/// let ast = parse("SUM(A:A)").unwrap();
/// let ctx = ClassificationContext::default();
/// let v = classify(&ast, &ctx);
/// let rewritten = rewrite(ast, &ctx, &v);
/// assert!(format!("{rewritten:?}").contains("PreludeRef"));
/// ```
#[must_use]
pub fn rewrite(ast: Ast, ctx: &ClassificationContext, verdict: &Classification) -> Ast {
    if matches!(verdict, Classification::Unsupported { .. }) {
        return ast;
    }
    Ast { root: rewrite_node(ast.root, ctx) }
}

fn rewrite_node(node: Node, ctx: &ClassificationContext) -> Node {
    match &node {
        Node::Function { name, args } => {
            if let Some(key) = aggregate_key(name, args, ctx) {
                return Node::PreludeRef(PreludeKey::Aggregate(key));
            }
            if let Some(key) = lookup_key(name, args, ctx) {
                return Node::PreludeRef(PreludeKey::Lookup(key));
            }
            // Recurse into args (Mixed parents wrap row-local + prelude leaves).
            let new_args = args.iter().cloned().map(|a| rewrite_node(a, ctx)).collect();
            Node::Function { name: name.clone(), args: new_args }
        }
        Node::BinaryOp { op, left, right } => Node::BinaryOp {
            op: op.clone(),
            left: Box::new(rewrite_node((**left).clone(), ctx)),
            right: Box::new(rewrite_node((**right).clone(), ctx)),
        },
        Node::UnaryOp { op, expr } => Node::UnaryOp {
            op: op.clone(),
            expr: Box::new(rewrite_node((**expr).clone(), ctx)),
        },
        Node::Array(rows) => {
            let new = rows.iter().map(|r| {
                r.iter().cloned().map(|c| rewrite_node(c, ctx)).collect()
            }).collect();
            Node::Array(new)
        }
        // Leaves: literal, text, reference, prelude-ref — return as-is.
        _ => node,
    }
}

fn aggregate_key(name: &str, args: &[Node], ctx: &ClassificationContext) -> Option<AggregateKey> {
    use crate::sets::is_aggregate;
    if !is_aggregate(name) { return None; }
    // Inspect the first arg: must be Reference::Range, otherwise fall
    // through (function recurses normally — handled by caller).
    // Real impl: parse the range, compute (sheet, column).
    todo!("walk first arg → AggregateKey")
}

fn lookup_key(name: &str, args: &[Node], ctx: &ClassificationContext) -> Option<LookupKey> {
    use crate::sets::is_lookup;
    if !is_lookup(name) { return None; }
    todo!("walk lookup args → LookupKey")
}
```

> The two `todo!()` markers above are for **plan readability only** — replace with full implementations during execution. Library code never ships with `todo!()`.

- [ ] **Step 3: Iterate to green.** Each test exercises a distinct shape. Replace `todo!`s incrementally.

### Task 4.4: Tick boxes + commit + PR

- [ ] **Step 1: Tick** in `docs/phases/phase-02-parser.md` under **AST rewrite**:
  - `[x] After classification, rewrite the AST to replace supported aggregate/lookup sub-expressions with PreludeRef(key) nodes.`
  - `[x] Add AstNode::PreludeRef(PreludeKey) variant.`
  - `[x] PreludeKey encodes aggregate vs lookup plus parameters.`
  - `[x] Rewriting is a pure function; golden tests for input → rewritten AST.`

Under **Tests**:
  - `[x] AST rewrite golden tests.`

- [ ] **Step 2: `make check`.**

- [ ] **Step 3: Commit + PR.** `xlstream-parse: add AST rewrite to substitute prelude refs`.

---

## Chunk 5: CLI `classify` subcommand + canonical fixture + phase doc tick

**Goal:** `cargo run -p xlstream-cli -- classify path.xlsx` lists every formula in the workbook with its address, formula text, and `Classification`. Provides the verification step from `docs/phases/phase-02-parser.md`.

**Branch:** `feature/phase-02-chunk-5-cli-classify`.

### Task 5.1: Branch off main

### Task 5.2: Add `calamine` to `xlstream-cli`

**Files:** `crates/xlstream-cli/Cargo.toml`.

`xlstream-io` is still stubbed (Phase 3 territory). For Phase 2 the CLI uses calamine directly. Refactor in Phase 3 once the real `Reader` lands.

- [ ] **Step 1: Add `calamine` to `[dependencies]`.** Inherit from workspace.

```toml
calamine = { workspace = true }
```

PR description must call this out as a *temporary* CLI-local dep, to be replaced in Phase 3.

### Task 5.3: Generate the canonical fixture

**Files:**
- Create: `fixtures/scripts/gen_benchmark_small/Cargo.toml`
- Create: `fixtures/scripts/gen_benchmark_small/src/main.rs`
- Create: `fixtures/canonical/benchmark_small.xlsx`

Three sheets:
1. `Deals` — 5 rows × 4 columns (`Region`, `Deal Value`, `Discount`, `Net Value`). One formula column: `=B2*(1-C2)`. One whole-column aggregate cell: `=SUM(B:B)` (RowLocal/AggregateOnly mix). One unsupported: `=OFFSET(A1,1,0)`.
2. `Region Info` — 3 rows × 3 columns. Static lookup data.
3. `Thresholds` — 4 rows × 2 columns. Static lookup data.

The script writes the .xlsx with `rust_xlsxwriter`; commit both the script and the produced .xlsx (file < 50 KB per `docs/standards/testing.md`).

- [ ] **Step 1: Cargo.toml for the fixture-generator binary.**

```toml
[package]
name = "gen_benchmark_small"
version = "0.0.0"
edition = "2021"
publish = false
# Intentionally NOT a workspace member — see fixtures/scripts/README.md.

[[bin]]
name = "gen_benchmark_small"
path = "src/main.rs"

[dependencies]
rust_xlsxwriter = "0.94"
```

- [ ] **Step 2: `src/main.rs`** writes the workbook described above. Each formula's expected `Classification` is documented in a header comment so reviewers can sanity-check.

- [ ] **Step 3: Generate the .xlsx once, commit it.**

```bash
cargo run --manifest-path fixtures/scripts/gen_benchmark_small/Cargo.toml
ls -la fixtures/canonical/benchmark_small.xlsx
```

> File must be < 50 KB. If it's larger, drop a sheet.

### Task 5.4: Implement `classify` subcommand (TDD)

**Files:** `crates/xlstream-cli/src/main.rs`.

- [ ] **Step 1: Add CLI test.**

```rust
#[test]
fn classify_subcommand_parses_required_arg() {
    let cli = Cli::try_parse_from([
        "xlstream", "classify", "fixtures/canonical/benchmark_small.xlsx",
    ]).unwrap();

    match cli.command {
        Command::Classify { input, .. } => {
            assert_eq!(input.to_str(), Some("fixtures/canonical/benchmark_small.xlsx"));
        }
        _ => panic!("expected Classify"),
    }
}
```

Expected: FAIL — `Classify` variant doesn't exist.

- [ ] **Step 2: Add the variant + handler.**

```rust
#[derive(Debug, Subcommand)]
enum Command {
    Evaluate { /* existing */ },
    /// List every formula in the workbook with its xlstream classification.
    Classify {
        /// Input workbook.
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Increase log verbosity.
        #[arg(long, short = 'v')]
        verbose: bool,
    },
}

fn run_classify(input: &Path) -> Result<(), xlstream_core::XlStreamError> {
    use calamine::{open_workbook_auto, Reader};

    let mut wb = open_workbook_auto(input).map_err(|e| xlstream_core::XlStreamError::Io {
        path: input.to_owned(),
        source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
    })?;

    for sheet_name in wb.sheet_names() {
        let range = wb.worksheet_range(&sheet_name).map_err(/* same Io map */)?;
        for ((row, col), cell) in range.cells() {
            if let calamine::DataType::String(s) = cell {
                if let Some(expr) = s.strip_prefix('=') {
                    let address = format!("{sheet_name}!{}", a1(*row as u32, *col as u32));
                    match xlstream_parse::parse(expr) {
                        Ok(ast) => {
                            let ctx = xlstream_parse::ClassificationContext::default()
                                .with_current_address(&sheet_name, (*row as u32) + 1, (*col as u32) + 1);
                            let v = xlstream_parse::classify(&ast, &ctx);
                            tracing::info!("{address}\t{expr}\t{v:?}");
                        }
                        Err(e) => tracing::warn!("{address}\tparse error: {e}"),
                    }
                }
            }
        }
    }
    Ok(())
}

fn a1(row: u32, col: u32) -> String { /* "A1" / "B2" formatter — small helper */ }
```

> Use `tracing::info!` (with the existing subscriber) instead of `println!` — `clippy::print_stdout` is denied at the crate root.

- [ ] **Step 3: Wire into `match cli.command` in `run()`.**

- [ ] **Step 4: Manual smoke test.**

```bash
cargo run -p xlstream-cli -- classify fixtures/canonical/benchmark_small.xlsx
```

Expected output: ~5 lines, one per formula, each tagged with its `Classification`. Verify by eye against the comments in the generator.

- [ ] **Step 5: Add an integration test that exec's the binary.**

Create `crates/xlstream-cli/tests/classify_smoke.rs`:

```rust
use std::process::Command;

#[test]
fn classify_smoke_runs_on_canonical_fixture() {
    let bin = env!("CARGO_BIN_EXE_xlstream");
    let out = Command::new(bin)
        .args(["classify", "../../fixtures/canonical/benchmark_small.xlsx"])
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    // Output goes via tracing → stderr.
    assert!(stderr.contains("RowLocal") || stderr.contains("AggregateOnly"));
}
```

> Path is relative to the crate root; tweak as needed.

### Task 5.5: Tick remaining boxes + flip phase status

- [ ] **Step 1: Tick** the **CLI integration** section in `docs/phases/phase-02-parser.md`:
  - `[x] xlstream-cli classify path.xlsx lists every formula + classification.`
  - `[x] Useful smoke test for development: "what would xlstream do with this workbook?"`

- [ ] **Step 2: Verify every Phase 2 box is now `[x]`.** If any remains, reopen the relevant chunk's PR or add a small docs PR — do NOT tick prematurely.

- [ ] **Step 3: Flip status in `docs/phases/README.md`:**

```markdown
| 2 | Parser integration | [`phase-02-parser.md`](phase-02-parser.md) | ✓ complete |
| 3 | I/O layer | [`phase-03-io.md`](phase-03-io.md) | in progress |
```

- [ ] **Step 4: Update the stale "Current phase: 0 — Foundation" header** in `docs/phases/README.md` line 3 to reflect Phase 3 (Priscilla flagged this as a separate cleanup; bundle here if she hasn't already shipped it).

### Task 5.6: Commit + PR

- [ ] **Step 1: `make check`.**

- [ ] **Step 2: Commit.**

```bash
git commit -am "xlstream-cli: add classify subcommand and canonical fixture"
```

- [ ] **Step 3: PR with the standard template.** Note: includes a non-trivial new dep (`calamine` in `xlstream-cli`) — flagged in the design-note section.

### Task 5.7: Post-merge confirmation

- [ ] **Step 1: After main lands the PR**, run `cargo run -p xlstream-cli -- classify fixtures/canonical/benchmark_small.xlsx` once on a clean checkout. Verify output.
- [ ] **Step 2: Tick this plan's boxes.** Open a tiny PR `docs: tick phase 2 plan completion` if any remain (or amend in Chunk 5 PR if pre-merge).

---

## Final deliverable (what to report back to the human who kicked this off)

After Chunk 5 lands on `main`, post in the kickoff chat:

1. ADR PR link + outcome — already in this conversation (PR #10, merged).
2. Phase 2 plan PR link + approval link — this PR.
3. Implementation PR links, in order: Chunks 0 → 5.
4. Final `make check` summary on `main`:
   - Test counts: `cargo test --workspace 2>&1 | grep "test result"` — paste verbatim.
   - Doctest count: `cargo test --workspace --doc 2>&1 | grep "test result"`.
   - Clippy + fmt clean (one-line statement).
5. Anything surprising in the docs that warrants a follow-up cleanup PR (track in a tiny "post-Phase-2 punch list" comment).

Then stop. Phase 3 starts in a fresh session.

---

## Open questions

Surface to Priscilla before the chunk that depends on the answer.

1. **`parse(expr)` error context.** The phase doc says `parse(expr: &str) -> Result<Ast, XlStreamError>` with `FormulaParse { address, formula, message }`. The parser layer doesn't know `address`. Plan adopts **caller-enriches**: parser sets `address = String::new()`, callers `map_err` to add the cell address. Confirm before Chunk 0 ships, or pick alternative (parser takes `&str` + `&str` address; or define a narrower `FormulaParseError` at parse layer). **Resolution required before Chunk 0 PR.**

2. **`docs/superpowers/` tracking.** `.gitignore` does not exclude `docs/superpowers/`, but Phase 1's plan was never committed. This plan's PR will commit only the Phase 2 plan. If you want the Phase 1 plan retroactively committed for symmetry, open a tiny separate PR — out of scope here. **Resolution: optional; defer.**

3. **Aggregate first-arg shape.** Some aggregates accept a literal value as the first arg (e.g. `SUM(1, 2, A:A)`). Plan currently routes only `Range` first-args through the rewrite. Confirm: do we support multi-arg aggregates in v0.1, or is `SUM(literals, range, ...)` deferred? Phase doc lists `SUM` etc. as supported but doesn't enumerate arg shapes. **Resolution required before Chunk 4 PR.**

4. **Whole-row references (`1:1`).** Phase doc covers whole-column (`A:A`); whole-row is symmetric but uncommon. Plan assumes same handling (allowed inside aggregates only). Confirm or deprioritise. **Resolution: defer to Chunk 3 review; default = allow for symmetry.**

5. **Volatile fns (`TODAY`, `NOW`, `RAND`).** `docs/architecture/streaming-model.md` mentions a `VOLATILE_STREAMING_OK` set with single-evaluation-per-run semantics. Phase 2 doesn't include it in the support sets explicitly. Plan defers volatile handling to Phase 4 (when the evaluator can actually invoke them); classifier currently routes them through `is_unsupported` → `Unsupported(VolatileUnsupported)`. **Resolution: confirm volatile fns can wait, OR add to a fourth support set in Chunk 2.**
