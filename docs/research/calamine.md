# calamine — read-side research

**VERDICT: suitable** for streaming xlsx row iteration.

## What it is

`calamine` is a pure-Rust read-only spreadsheet parser. Handles xlsx, xlsm, xls, xlsb, ods. MIT licensed. Actively maintained (tafia/calamine on GitHub). Widely used — it's the engine behind Python's `fastexcel` / `polars-read-excel`.

## Streaming support — YES

Two API tiers:

### Eager (default, not for us)

```rust
use calamine::{open_workbook, Xlsx, Reader};
let mut wb: Xlsx<_> = open_workbook("file.xlsx")?;
let range = wb.worksheet_range("Sheet1")?;
for row in range.rows() { /* ... */ }
```

Materialises the whole sheet into a dense `Vec`. Memory ~proportional to sheet cell count. Good for small files; terrible for our 400k × 20 workload.

### Streaming (the one we use)

```rust
use calamine::{open_workbook, Xlsx, Reader};
let mut wb: Xlsx<_> = open_workbook("file.xlsx")?;
let mut reader = wb.worksheet_cells_reader("Sheet1")?;
while let Some(cell) = reader.next_cell()? {
    let (row, col) = cell.get_position();
    match cell.get_value() {
        calamine::DataRef::Int(i)        => { /* i64 */ }
        calamine::DataRef::Float(f)      => { /* f64 */ }
        calamine::DataRef::SharedString(s) |
        calamine::DataRef::String(s)     => { /* &str */ }
        calamine::DataRef::Bool(b)       => { }
        calamine::DataRef::DateTime(dt)  => { }
        calamine::DataRef::Error(e)      => { }
        calamine::DataRef::Empty         => { }
        _ => {}
    }
}
```

`XlsxCellReader::next_cell` yields cells one at a time. O(1) in sheet size (shared-strings table is loaded up front; that's the only one-time cost).

## Formula access

Separately:

```rust
while let Some(formula_cell) = reader.next_formula()? {
    // cell.get_value() is a String — the formula text, without leading '='.
}
```

Typical pattern: make two passes (one cells, one formulas), merge by `(row, col)` — or use `XlsxCellReader` twice.

## Performance

- Published benchmark (calamine README): 186 MB xlsx (NYC 311 dataset, 1M rows) reads in ~20s.
- Comparable Python readers: openpyxl ~several minutes, ClosedXML ~100s.
- Memory with `worksheet_cells_reader`: dominated by the shared-strings table. For a typical 100 MB xlsx, expect tens–low-hundreds of MB.

## Limitations

### Shared strings are loaded upfront

`xl/sharedStrings.xml` must be parsed before streaming cells. This is an xlsx format quirk, not a calamine limitation. For our workload it's fine — a few MB of strings.

### No `sheet["A5"]` stringly access

Coordinates are `(u32, u16)` row/col. We write a `parse_address("A5") -> (4, 0)` helper. Trivial.

### Date values

Excel serial numbers under the hood, surfaced as `DataRef::DateTime(ExcelDateTime)`. With the `chrono` feature flag, `ExcelDateTime::as_datetime()` returns `chrono::NaiveDateTime`.

### No concurrent readers on one handle

Each reader opens its own file handle. For our row-parallel design (N workers), that's N file opens — acceptable. Shared-strings is re-parsed N times; at a few MB each, tolerable. Optimisation for v0.2: share the table via `Arc`.

### Errors

`XlsxError` enum variants include malformed ZIP, invalid XML, unsupported cell type. All returned as `Result`, no panics on malformed input (tested by their fuzz suite).

## API reference

- Docs: https://docs.rs/calamine/latest/calamine/
- GitHub: https://github.com/tafia/calamine
- Key types:
  - [`Xlsx`](https://docs.rs/calamine/latest/calamine/struct.Xlsx.html)
  - [`XlsxCellReader`](https://docs.rs/calamine/latest/calamine/struct.XlsxCellReader.html) (via `worksheet_cells_reader`)
  - [`Data`](https://docs.rs/calamine/latest/calamine/enum.Data.html) / [`DataRef`](https://docs.rs/calamine/latest/calamine/enum.DataRef.html)
  - [`ExcelDateTime`](https://docs.rs/calamine/latest/calamine/struct.ExcelDateTime.html)

## Version pin

Track stable (`^0.34` at time of writing). Not pinned exactly — calamine's surface is conservative.

## What we use, what we don't

- ✅ `worksheet_cells_reader` + `next_cell` + `next_formula`.
- ✅ `sheet_names` for metadata.
- ✅ `Data::Error` propagation.
- ❌ `worksheet_range` (not for our main sheet).
- ❌ xls / xlsb / ods readers (we target xlsx only in v0.1).

## Alternatives we considered

- **`rust-xlsx`** — abandoned.
- **`umya-spreadsheet`** — used by formualizer. Buffers whole workbook. Not streaming.
- **`simple_excel_reader`** — stale.
- **Writing our own xlsx parser** — laughable ROI.

calamine is the answer.
