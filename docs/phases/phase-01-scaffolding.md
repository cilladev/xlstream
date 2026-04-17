# Phase 1 — Scaffolding

**Goal:** all crates exist, compile, and export empty-but-valid public APIs. Cargo workspace dep graph wired (crate-to-crate, not formula-to-formula — xlstream does not build a formula dependency graph; see [`../architecture/streaming-model.md`](../architecture/streaming-model.md)).

**Estimated effort:** 1 day.

**Prerequisites:** Phase 0 complete.

**Output:** `cargo build --workspace` produces artefacts; all crates type-check; tests run (though they assert nothing useful yet).

## Checklist

### Create crates

- [ ] `crates/xlstream-core/` with `Cargo.toml` + `src/lib.rs`.
- [ ] `crates/xlstream-parse/` with `Cargo.toml` + `src/lib.rs`.
- [ ] `crates/xlstream-io/` with `Cargo.toml` + `src/lib.rs`.
- [ ] `crates/xlstream-eval/` with `Cargo.toml` + `src/lib.rs`.
- [ ] `crates/xlstream-cli/` with `Cargo.toml` + `src/main.rs`.
- [ ] Register all five in workspace root `Cargo.toml` `[workspace] members`.

### `xlstream-core`

- [ ] Empty public types:
  - [ ] `pub enum Value { Empty, Number(f64), Integer(i64), Text(Box<str>), Bool(bool), Date(ExcelDate), Error(CellError) }` (stubs; full impl in later phases).
  - [ ] `pub enum CellError { Div0, Value, Ref, Name, Na, Num, Null }`.
  - [ ] `pub enum XlStreamError { ... }` — variants from `docs/architecture/errors.md`, using `thiserror`.
  - [ ] `pub struct ExcelDate { pub serial: f64 }` — just a newtype for now.
- [ ] Every type `#[derive(Debug, Clone, PartialEq)]` where it makes sense.
- [ ] Rustdoc on every public item.
- [ ] One doctest per type.
- [ ] Unit tests verifying construction and basic equality.

### `xlstream-parse`

- [ ] Dep on `formualizer-parse` (pin exact: `formualizer-parse = "=0.5.0"` or whatever latest 0.5 is).
- [ ] Dep on `xlstream-core`.
- [ ] Stub functions:
  - [ ] `pub fn parse(expr: &str) -> Result<Ast, XlStreamError>` — stub returns `Err(Internal("unimplemented"))`.
  - [ ] `pub enum Classification { RowLocal, AggregateOnly, LookupOnly, Mixed, Unsupported(String) }`.
  - [ ] `pub fn classify(ast: &Ast, ctx: &ClassificationContext) -> Classification` — stub returns `Unsupported`.
  - [ ] Placeholder `pub struct Ast` — wraps `formualizer_parse::Ast` once integration lands (Phase 2).
- [ ] Rustdoc + one stub doctest per item.

### `xlstream-io`

- [ ] Deps: `calamine`, `rust_xlsxwriter` (with `constant_memory`, `zlib`, `ryu` features), `xlstream-core`.
- [ ] Stub types:
  - [ ] `pub struct Reader;` with `open(path)` stub.
  - [ ] `pub struct Writer;` with `create(path)` stub.
  - [ ] `pub struct CellStream;` with `next_row()` stub.
- [ ] Stubs return `Err(Internal("unimplemented"))`.
- [ ] Rustdoc.

### `xlstream-eval`

- [ ] Deps: `xlstream-core`, `xlstream-parse`, `xlstream-io`, `rayon`, `tracing`, `phf`.
- [ ] Stubs:
  - [ ] `pub fn evaluate(input: &Path, output: &Path, workers: Option<usize>) -> Result<EvaluateSummary, XlStreamError>`.
  - [ ] `pub struct EvaluateSummary { pub rows_processed: u32, pub duration_ms: u64, pub peak_rss_bytes: u64 }`.
  - [ ] Stubs return an error or zero summary.
- [ ] Rustdoc.

### `xlstream-cli`

- [ ] Dep on `clap` (derive feature), `xlstream-eval`, `xlstream-io`, `tracing-subscriber`.
- [ ] Stub binary with a single `evaluate` subcommand.
- [ ] `cargo run -p xlstream-cli -- evaluate --help` works.

### Tests

- [ ] Each crate has `#[cfg(test)]` unit tests — at least one passing test, even if it just asserts `2 + 2 == 4`. Establishes the test harness runs.
- [ ] Run `cargo test --workspace` — all green.

### CI

- [ ] Verify workflows run and pass on the scaffolding commit.

## Cargo crate dependency graph check

After this phase, `cargo tree -p xlstream-eval` should show:

```
xlstream-eval v0.1.0
├── xlstream-core v0.1.0
├── xlstream-parse v0.1.0
│   ├── xlstream-core
│   └── formualizer-parse v0.5.x
├── xlstream-io v0.1.0
│   ├── xlstream-core
│   ├── calamine v0.34.x
│   └── rust_xlsxwriter v0.94.x
├── rayon v1.10.x
├── tracing
└── phf
```

No cycles. Arrows upward only.

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
