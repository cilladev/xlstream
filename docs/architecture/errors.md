# Errors

Two levels of errors, clearly separated.

## 1. Cell-level errors (`CellError`)

These are Excel's own errors, written to output cells and propagated by formulas. They are **values**, not exceptions.

```rust
pub enum CellError {
    Div0,   // #DIV/0!
    Value,  // #VALUE!
    Ref,    // #REF!
    Name,   // #NAME?
    Na,     // #N/A
    Num,    // #NUM!
    Null,   // #NULL!
}
```

### Semantics

- Arithmetic on a `CellError` propagates the error: `1 + #DIV/0! = #DIV/0!`.
- Functions receive `Value::Error(e)` and must either propagate, short-circuit (`IFERROR`), or return a new error.
- `CellError` is written to the output cell as-is. Excel displays it correctly.

### Do NOT

- Do not raise a Rust error when a cell evaluates to `#DIV/0!`. The user asked us to evaluate the workbook; producing `#DIV/0!` in a cell is a correct outcome.
- Do not terminate the run. One bad cell does not abort 399,999 good rows.

## 2. Library-level errors (`XlStreamError`)

These mean "the system cannot continue" — malformed input, unsupported formula shape, I/O failure.

```rust
#[derive(Debug, thiserror::Error)]
pub enum XlStreamError {
    #[error("I/O error reading {path}: {source}")]
    Io { path: PathBuf, #[source] source: std::io::Error },

    #[error("xlsx parse error: {0}")]
    Xlsx(#[from] calamine::XlsxError),

    #[error("xlsx write error: {0}")]
    XlsxWrite(#[from] rust_xlsxwriter::XlsxError),

    #[error("formula parse error at {address}: {message}\n  formula: {formula}")]
    FormulaParse { address: String, formula: String, message: String },

    #[error("unsupported formula at {address}: {reason}\n  formula: {formula}\n  see: {doc_link}")]
    Unsupported {
        address: String,
        formula: String,
        reason: String,
        doc_link: &'static str,
    },

    #[error("classification error at {address}: {message}")]
    Classification { address: String, message: String },

    #[error("circular reference involving {cells:?}")]
    CircularReference { cells: Vec<String> },

    #[error("internal invariant violation: {0}")]
    Internal(String),
}
```

### Rules

1. **Every fallible public function returns `Result<_, XlStreamError>`.** No exceptions.
2. **Error messages name the cell.** Always include the sheet name + cell address when the error is cell-scoped.
3. **Error messages quote the formula text.** Never make the user look it up themselves.
4. **Use `#[source]` for causes.** `thiserror` preserves the chain; consumers can `source()` down.
5. **No `anyhow::Error` in library code.** We own our error type.

## Mapping to Python

Each `XlStreamError` variant maps to a specific Python exception. Variants that share semantics can share an exception.

```rust
// In bindings/python/src/lib.rs
create_exception!(xlstream, XlStreamError, pyo3::exceptions::PyException);
create_exception!(xlstream, UnsupportedFormula, XlStreamError);
create_exception!(xlstream, FormulaParseError, XlStreamError);
create_exception!(xlstream, ClassificationError, XlStreamError);
create_exception!(xlstream, CircularReferenceError, XlStreamError);

impl From<xlstream_core::XlStreamError> for PyErr {
    fn from(e: xlstream_core::XlStreamError) -> PyErr {
        match e {
            XlStreamError::Io { .. }           => PyIOError::new_err(e.to_string()),
            XlStreamError::Unsupported { .. }  => UnsupportedFormula::new_err(e.to_string()),
            XlStreamError::FormulaParse { .. } => FormulaParseError::new_err(e.to_string()),
            XlStreamError::Classification{..}  => ClassificationError::new_err(e.to_string()),
            XlStreamError::CircularReference{..} => CircularReferenceError::new_err(e.to_string()),
            XlStreamError::Internal(_)         => PyRuntimeError::new_err(e.to_string()),
            _                                  => XlStreamError::new_err(e.to_string()),
        }
    }
}
```

Python users get a rich exception hierarchy they can catch selectively.

## Panic policy

Panics are allowed only when an invariant that the code itself should have guaranteed has been violated — i.e., a bug, not a bad input. These should be labelled `Internal(...)` via the error type first and only escalate to `panic!` for things the process genuinely cannot continue past.

**Never panic because of:**
- A malformed formula.
- A missing file.
- A row with the wrong number of columns.
- An out-of-range numeric value.
- An unknown function name.

**Do panic for:**
- Nothing user-facing. Almost nothing. If in doubt, return `Internal("...")`.

`debug_assert!` is fine anywhere — it's a test aid, not runtime behaviour.

## Logging

All errors and unusual states go through `tracing`. Consumers (including the Python binding) can wire up their own subscriber.

- `error!` — about to return an `XlStreamError`.
- `warn!` — unexpected-but-survivable state. Example: a formula that classified as supported but evaluates to `#VALUE!` on many rows.
- `info!` — lifecycle events. Example: "classified 12 formulas in 3 sheets".
- `debug!` / `trace!` — for local development only; never enable in production runtime by default.

## Error surface test

Every module with a public error path includes a test that triggers each variant. If you add a new variant, you add a new test.
