# xlstream-io

[![Crates.io](https://img.shields.io/crates/v/xlstream-io.svg)](https://crates.io/crates/xlstream-io)
[![docs.rs](https://docs.rs/xlstream-io/badge.svg)](https://docs.rs/xlstream-io)

xlsx read/write layer for the [xlstream](https://github.com/cilladev/xlstream) streaming Excel evaluator. Calamine-backed reader, rust_xlsxwriter-backed writer (constant-memory mode), and a row-oriented cell stream abstraction.

This is an internal crate. Most users should depend on [`xlstream-eval`](https://crates.io/crates/xlstream-eval), which calls xlstream-io internally.

## What it provides

- **`Reader`** -- open xlsx for streaming. `open(path)`, `sheet_names()`, `defined_names()`, `cells(sheet) -> CellStream`, `formulas(sheet)`
- **`Writer`** -- create xlsx in constant-memory mode. `create(path)`, `add_sheet(name) -> SheetHandle`, `finish()`
- **`SheetHandle`** -- write rows to a single sheet. Enforces strictly-increasing row order.
- **`CellStream`** -- row-oriented iterator. `next_row() -> Option<(row_idx, Vec<Value>)>`

## When to use directly

Only if you need raw xlsx streaming I/O without formula evaluation — e.g., building a custom pipeline that reads cell values and writes transformed output. For formula evaluation, use `xlstream-eval`.

## Dependencies

`calamine`, `rust_xlsxwriter` (features: `constant_memory`, `zlib`, `ryu`), `xlstream-core`.

## License

Dual-licensed under Apache-2.0 or MIT, at your option.
