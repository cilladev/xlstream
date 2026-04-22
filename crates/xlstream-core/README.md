# xlstream-core

Foundation types for the xlstream streaming Excel evaluator. Every other crate in the workspace depends on this one.

## What it owns

- **`Value`** -- cell-value enum: `Number(f64)`, `Integer(i64)`, `Text(Box<str>)`, `Bool(bool)`, `Date(ExcelDate)`, `Error(CellError)`, `Empty`
- **`CellError`** -- Excel in-cell errors: `Div0`, `Value`, `Ref`, `Name`, `Na`, `Num`, `Null`
- **`ExcelDate`** -- wraps Excel serial date (days since 1900 epoch, leap-year bug preserved)
- **`XlStreamError`** -- library-level errors that stop evaluation: `Io`, `Xlsx`, `XlsxWrite`, `Unsupported`, `FormulaParse`, `Classification`, `CircularReference`, `Internal`
- **`col_row_to_a1`** -- convert 1-based (col, row) to A1 notation
- **Constants** -- `EXCEL_MAX_ROWS` (1,048,576), `EXCEL_MAX_COLS` (16,384)

## What it does NOT own

- Parsing, I/O, evaluation, or builtin functions. Those live in sibling crates.

## Dependencies

`thiserror`. Nothing else.

## Why separate

Breaking core types into their own crate avoids circular-dep pain, reduces recompile scope, and gives a small stable ABI surface for the public Rust API.
