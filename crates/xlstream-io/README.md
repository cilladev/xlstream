# xlstream-io

xlsx I/O. Calamine-backed reader, rust_xlsxwriter-backed writer (constant-memory mode), and a row-oriented cell stream abstraction.

## Public API

- **`Reader`** -- open xlsx for streaming. Methods: `open(path)`, `sheet_names()`, `defined_names()`, `cells(sheet) -> CellStream`, `formulas(sheet)`, `sheet_dimensions(sheet)`
- **`Writer`** -- create xlsx in constant-memory mode. Methods: `create(path)`, `add_sheet(name) -> SheetHandle`, `finish()`
- **`SheetHandle`** -- write rows to a single sheet. Enforces strictly-increasing row order. Methods: `write_row(row_idx, values)`, `write_formula(row, col, formula, cached)`
- **`CellStream`** -- row-oriented iterator. `next_row() -> Option<(row_idx, Vec<Value>)>`, `seek_to_row(row)`

## Type conversions

- `calamine::DataRef` -> `Value` (read path)
- `Value` -> `rust_xlsxwriter` write calls (write path)

## What it does NOT own

- Formula parsing or evaluation. Those live in `xlstream-parse` and `xlstream-eval`.

## Dependencies

`calamine`, `rust_xlsxwriter` (features: `constant_memory`, `zlib`, `ryu`), `xlstream-core`.

## Why separate

The evaluator works against `Reader` and `Writer` -- tests can construct fake row iterators and assert evaluated values without hitting disk.

See [`docs/architecture/io.md`](../../docs/architecture/io.md).
