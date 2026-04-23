# xlstream-core

[![Crates.io](https://img.shields.io/crates/v/xlstream-core.svg)](https://crates.io/crates/xlstream-core)
[![docs.rs](https://docs.rs/xlstream-core/badge.svg)](https://docs.rs/xlstream-core)

Core value and error types shared across the [xlstream](https://github.com/cilladev/xlstream) streaming Excel evaluator.

This is an internal crate. Depend on [`xlstream-eval`](https://crates.io/crates/xlstream-eval) for the evaluation API, or `pip install xlstream` for Python.

## What it provides

- **`Value`** -- cell-value enum: `Number(f64)`, `Integer(i64)`, `Text(Box<str>)`, `Bool(bool)`, `Date(ExcelDate)`, `Error(CellError)`, `Empty`
- **`CellError`** -- Excel in-cell errors: `Div0`, `Value`, `Ref`, `Name`, `Na`, `Num`, `Null`
- **`ExcelDate`** -- wraps Excel serial date (days since 1900 epoch, leap-year bug preserved)
- **`XlStreamError`** -- typed error enum for the full pipeline: `Io`, `Xlsx`, `XlsxWrite`, `Unsupported`, `FormulaParse`, `Classification`, `CircularReference`, `Internal`
- **`col_row_to_a1`** -- convert 1-based (col, row) to A1 notation
- **Constants** -- `EXCEL_MAX_ROWS` (1,048,576), `EXCEL_MAX_COLS` (16,384)

## When to use directly

Only if you're building a custom component that handles xlstream value types without pulling in the full evaluator. Most users should depend on `xlstream-eval` instead.

## Dependencies

`thiserror`. Nothing else.

## License

Dual-licensed under Apache-2.0 or MIT, at your option.
