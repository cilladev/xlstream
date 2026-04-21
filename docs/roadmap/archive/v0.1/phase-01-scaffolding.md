# Phase 1 — Scaffolding

**Goal:** all crates exist, compile, and export empty-but-valid public APIs. Cargo workspace dep graph wired (crate-to-crate, not formula-to-formula — xlstream does not build a formula dependency graph; see [`../architecture/streaming-model.md`](../architecture/streaming-model.md)).

**Estimated effort:** 1 day.

**Prerequisites:** Phase 0 complete.

**Output:** `cargo build --workspace` produces artefacts; all crates type-check; tests run (though they assert nothing useful yet).

## Checklist

### Create crates

- [x] `crates/xlstream-core/` with `Cargo.toml` + `src/lib.rs`.
- [x] `crates/xlstream-parse/` with `Cargo.toml` + `src/lib.rs`.
- [x] `crates/xlstream-io/` with `Cargo.toml` + `src/lib.rs`.
- [x] `crates/xlstream-eval/` with `Cargo.toml` + `src/lib.rs`.
- [x] `crates/xlstream-cli/` with `Cargo.toml` + `src/main.rs`.
- [x] Register all five in workspace root `Cargo.toml` `[workspace] members`.

### `xlstream-core`

- [x] Empty public types:
  - [x] `pub enum Value { Empty, Number(f64), Integer(i64), Text(Box<str>), Bool(bool), Date(ExcelDate), Error(CellError) }` (stubs; full impl in later phases).
  - [x] `pub enum CellError { Div0, Value, Ref, Name, Na, Num, Null }`.
  - [x] `pub enum XlStreamError { ... }` — variants from `docs/architecture/errors.md`, using `thiserror`.
  - [x] `pub struct ExcelDate { pub serial: f64 }` — just a newtype for now.
- [x] Every type `#[derive(Debug, Clone, PartialEq)]` where it makes sense.
- [x] Rustdoc on every public item.
- [x] One doctest per type.
- [x] Unit tests verifying construction and basic equality.

### `xlstream-parse`

- [ ] Dep on `formualizer-parse` (pin exact: `formualizer-parse = "=0.5.0"` or whatever latest 0.5 is). *Deferred to Phase 2: upstream 0.5.x does not exist on crates.io, and 1.x pulls in `formualizer-common` 1.1.2 which requires Rust 1.88 (we're pinned at 1.85). Per plan Task 2.3 fallback, Ast ships as a self-contained stub.*
- [x] Dep on `xlstream-core`.
- [x] Stub functions:
  - [x] `pub fn parse(expr: &str) -> Result<Ast, XlStreamError>` — stub returns `Err(Internal("unimplemented"))`.
  - [x] `pub enum Classification { RowLocal, AggregateOnly, LookupOnly, Mixed, Unsupported(String) }`.
  - [x] `pub fn classify(ast: &Ast, ctx: &ClassificationContext) -> Classification` — stub returns `Unsupported`.
  - [x] Placeholder `pub struct Ast` — wraps `formualizer_parse::Ast` once integration lands (Phase 2).
- [x] Rustdoc + one stub doctest per item.

### `xlstream-io`

- [x] Deps: `calamine`, `rust_xlsxwriter` (with `constant_memory`, `zlib`, `ryu` features), `xlstream-core`.
- [x] Stub types:
  - [x] `pub struct Reader;` with `open(path)` stub.
  - [x] `pub struct Writer;` with `create(path)` stub.
  - [x] `pub struct CellStream;` with `next_row()` stub.
- [x] Stubs return `Err(Internal("unimplemented"))`.
- [x] Rustdoc.

### `xlstream-eval`

- [x] Deps: `xlstream-core`, `xlstream-parse`, `xlstream-io`, `tracing`. (`rayon` lands in Phase 10, `phf` lands in Phase 7 — each phase declares only what it uses.)
- [x] Stubs:
  - [x] `pub fn evaluate(input: &Path, output: &Path, workers: Option<usize>) -> Result<EvaluateSummary, XlStreamError>`.
  - [x] `pub struct EvaluateSummary { pub rows_processed: u32, pub duration_ms: u64, pub peak_rss_bytes: u64 }`.
  - [x] Stubs return an error or zero summary.
- [x] Rustdoc.

### `xlstream-cli`

- [x] Dep on `clap` (derive feature), `xlstream-eval`, `xlstream-io`, `tracing-subscriber`.
- [x] Stub binary with a single `evaluate` subcommand.
- [x] `cargo run -p xlstream-cli -- evaluate --help` works.

### Tests

- [x] Each crate has `#[cfg(test)]` unit tests — at least one passing test, even if it just asserts `2 + 2 == 4`. Establishes the test harness runs.
- [x] Run `cargo test --workspace` — all green.

### CI

- [x] Verify workflows run and pass on the scaffolding commit.

## Cargo crate dependency graph check

After this phase, `cargo tree -p xlstream-eval` should show:

```
xlstream-eval v0.1.0
├── xlstream-core v0.1.0
├── xlstream-parse v0.1.0
│   └── xlstream-core
├── xlstream-io v0.1.0
│   ├── xlstream-core
│   ├── calamine v0.34.x
│   └── rust_xlsxwriter v0.94.x
└── tracing
```

No cycles. Arrows upward only.

`rayon` and `phf` are absent — they land in Phase 10 and Phase 7 respectively, in the PR that first imports them. `formualizer-parse` is also absent: crates.io 0.5.x does not exist, and 1.x transitively requires Rust 1.88+ (`let` chains in `formualizer-common` 1.1.2). Phase 2 picks a resolution (toolchain bump, version pair, or fork).

## Verification

```bash
cargo build --workspace            # all five crates compile
cargo test --workspace             # tests pass (even if trivial)
cargo clippy --workspace -- -D warnings  # clean
cargo fmt --check                  # clean
cargo doc --workspace --no-deps    # doc builds without errors
```

## Done when

All checkboxes ticked. `cargo doc --no-deps --open` shows five crate docs, each with rustdoc on every public item. CI is green.
