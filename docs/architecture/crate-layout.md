# Crate layout

## Workspace structure

```
xlstream/
├── Cargo.toml                       workspace root
├── rust-toolchain.toml              pinned stable Rust version
├── rustfmt.toml                     formatting rules
├── clippy.toml                      lint config
│
├── crates/
│   ├── xlstream-core/               value/error types, traits, no external deps
│   ├── xlstream-parse/              parser adapter (wraps formualizer-parse)
│   ├── xlstream-eval/               streaming evaluator + all builtin functions
│   ├── xlstream-io/                 calamine reader + rust_xlsxwriter writer
│   └── xlstream-cli/                optional binary, dev tool only
│
├── bindings/
│   └── python/                      PyO3 binding, published as `xlstream` on PyPI
│       ├── Cargo.toml
│       ├── pyproject.toml
│       ├── src/lib.rs               #[pymodule]
│       ├── py_src/xlstream/         pure-python wrapper + type stubs
│       │   ├── __init__.py
│       │   ├── py.typed
│       │   └── _xlstream.pyi
│       └── tests/                   pytest
│
├── benchmarks/
│   ├── Cargo.toml                   criterion benches
│   ├── benches/
│   └── fixtures/                    reference workbooks (generated)
│
├── fixtures/                        small test workbooks (committed, < 50 KB each)
│   ├── generated/                   produced by build script, gitignored
│   └── canonical/                   hand-crafted, committed
│
├── tests/                           workspace-level integration tests
│
└── docs/
```

## Crate responsibilities

### `xlstream-core`

The foundation. Contains nothing that can fail to compile for an external reason.

**Owns:**
- `Value` — the cell-value enum. `Number(f64) | Integer(i64) | Text(Box<str>) | Bool(bool) | Date(ExcelDate) | Error(CellError) | Empty`.
- `CellError` — the Excel error enum (`Div0`, `Value`, `Ref`, `Name`, `Na`, `Num`, `Null`).
- `XlStreamError` — the library-level error enum (parse, classification, I/O, evaluation, unsupported).
- `FormulaAddr` — `(SheetId, Row, Col)`.
- Traits: `EvalContext`, `Scope`, `RangeRef`, `LookupIndex`. These define the contracts the evaluator uses.

**Does NOT own:**
- Any parser logic.
- Any I/O.
- Any builtin function.

**Dependencies:** `thiserror`, maybe `smallvec`, maybe `compact_str`. That's it.

**Why separate:** Breaking the core types into their own crate lets everyone depend on it without circular-dep pain, and gives us a small, stable ABI surface for the public Rust API.

### `xlstream-parse`

Thin adapter over `formualizer-parse`.

**Owns:**
- `Ast` — our public AST type. Either a re-export or a wrapper of `formualizer_parse::Ast`.
- `parse(text: &str) -> Result<Ast, ParseError>`.
- `classify(ast: &Ast, context: &ClassificationContext) -> Classification`.
- `Classification` enum: `RowLocal | AggregateOnly | LookupOnly | Mixed | Unsupported(Reason)`.
- `extract_references(ast: &Ast) -> References` — the referenced cells/ranges/sheets, for prelude planning.

**Does NOT own:**
- The actual evaluation of a formula (that's `xlstream-eval`).

**Dependencies:** `formualizer-parse`, `xlstream-core`.

**Why this layer exists:** Decouples the rest of the codebase from upstream parser breaking changes. If we ever have to fork or swap the parser, this is the only crate that changes.

### `xlstream-eval`

The brain. Streaming evaluator + every builtin function.

**Owns:**
- `StreamingEvaluator` — the orchestrator: open input, classify, run prelude, stream rows, write output.
- `Prelude` — aggregates + lookup indexes, shared across rows and workers.
- `Interpreter` — walks an AST node, dispatches to builtins, returns a `Value`.
- `builtins/` — one module per family: `arithmetic.rs`, `conditional.rs`, `aggregate.rs`, `lookup.rs`, `string.rs`, `date.rs`, `math.rs`, `logical.rs`. Each file contains pure functions `fn sum(args: &[Value]) -> Result<Value, CellError>`.
- `BUILTIN_REGISTRY` — a `phf::Map<&str, BuiltinFn>` compiled at build time for O(1) name → function lookup.

**Does NOT own:**
- xlsx I/O (that's `xlstream-io`).
- Value type definitions (those are in `xlstream-core`).
- Formula parsing (that's `xlstream-parse`).

**Dependencies:** `xlstream-core`, `xlstream-parse`, `xlstream-io`, `rayon`, `phf`, `tracing`.

**Why separate from `xlstream-io`:** the evaluator must be testable without hitting disk. Tests construct fake row iterators and assert evaluated values.

### `xlstream-io`

File I/O. Read with calamine, write with rust_xlsxwriter.

**Owns:**
- `Reader` — an abstraction over a calamine `XlsxCellReader` that yields rows as `Vec<Value>`.
- `Writer` — an abstraction over a `rust_xlsxwriter::Worksheet` in constant-memory mode that accepts rows as `&[Value]`.
- Type conversion between calamine's `Data` / `DataRef` and our `Value`.
- Type conversion between our `Value` and rust_xlsxwriter's `IntoExcelData` traits.
- Sheet enumeration, metadata extraction.

**Does NOT own:**
- Anything about formula evaluation or parsing.

**Dependencies:** `calamine`, `rust_xlsxwriter` (with `constant_memory`, `zlib`, `ryu` features), `xlstream-core`.

**Why separate from `xlstream-eval`:** the evaluator works against the `Reader` and `Writer` traits — tests can mock these easily.

### `xlstream-cli`

Optional binary. Used for development, smoke tests, ad-hoc evaluation from the terminal. Not published.

```
xlstream-cli evaluate input.xlsx --output out.xlsx
xlstream-cli classify input.xlsx         # list each formula + classification
xlstream-cli aggregates input.xlsx       # list pre-computed aggregates
```

**Dependencies:** `xlstream-eval`, `xlstream-io`, `clap`, `tracing-subscriber`.

### `bindings/python` (crate name: `xlstream-python`)

PyO3 crate that wraps `xlstream-eval` with a `#[pymodule]`. Built with maturin. Published to PyPI as `xlstream`.

**Owns:**
- `#[pymodule] fn _xlstream(m)` — the native extension.
- `#[pyfunction] fn evaluate(input, output=None)` — the main entry point.
- Error type mapping: our Rust errors → Python exceptions.
- `py_src/xlstream/__init__.py` — the pure-Python wrapper that re-exports the native API and adds typing-friendly helpers.

**Does NOT own:**
- Any evaluation or I/O logic — all comes from the Rust crates.

**Dependencies:** `pyo3` (Bound API), `xlstream-eval`, `xlstream-io`, `xlstream-core`.

## Dependency graph

```
          xlstream-core
           ▲       ▲
           │       │
  xlstream-parse   xlstream-io
           ▲       ▲
           │       │
          xlstream-eval
           ▲       ▲
           │       │
  xlstream-cli    bindings/python
```

Rule: an arrow goes upward only. No cycles. If you find yourself wanting to add a reverse arrow, you are modelling something wrong.

## Naming convention

- Crate names: `xlstream-<role>` (kebab-case).
- Python package: `xlstream` (underscore or dash? dash for consistency with pip, but the Python module `import xlstream` uses underscore-free name).
- Native extension module: `xlstream._xlstream` (single underscore prefix marks it private).

## Public vs internal APIs

- Every crate's `lib.rs` declares what's public via `pub use`. Internal modules are `pub(crate)`.
- The Rust crate that gets promoted to crates.io is **`xlstream-core` + `xlstream-eval` + `xlstream-io`**, each with their own version. v0.1 publishes all three simultaneously at the same version. `xlstream-parse` is internal; `xlstream-cli` is internal; `bindings/python` is internal (only reachable via PyPI).

## Why this many crates

Each crate should answer "yes" to: *could a consumer want this without the others?*

- `xlstream-core` — yes, someone writing their own evaluator could want our value types.
- `xlstream-parse` — marginal, but the classification logic is reusable.
- `xlstream-io` — yes, a pipeline might want our typed row I/O without evaluation.
- `xlstream-eval` — the main product.

This isn't academic — every split reduces compile times (changes in `eval` don't recompile `core`), narrows the test surface, and makes mocking straightforward.
