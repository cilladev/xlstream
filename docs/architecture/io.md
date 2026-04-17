# I/O layer

## Reading: calamine

### Why calamine

- Pure Rust, read-only, fast. (See [`../research/calamine.md`](../research/calamine.md).)
- Supports streaming cell iteration via `XlsxCellReader::next_cell` / `next_formula`.
- MIT licensed.
- Actively maintained, large user base.

### API usage in xlstream

We wrap calamine in `xlstream-io::Reader`:

```rust
pub struct Reader {
    workbook: Xlsx<BufReader<File>>,
}

impl Reader {
    pub fn open(path: &Path) -> Result<Self, XlStreamError> { ... }

    pub fn sheet_names(&self) -> Vec<String> { ... }

    /// Stream cells of a sheet, yielded in row-major order.
    pub fn cells(&mut self, sheet: &str) -> Result<CellStream<'_>, XlStreamError> { ... }

    /// Stream formula text + address for every formula cell.
    pub fn formulas(&mut self, sheet: &str) -> Result<FormulaStream<'_>, XlStreamError> { ... }
}

pub struct CellStream<'a> { inner: calamine::XlsxCellReader<'a> }

impl<'a> CellStream<'a> {
    /// Yield the next row as a dense `Vec<Value>` (length == max_col for the sheet).
    /// Missing cells become Value::Empty.
    pub fn next_row(&mut self) -> Result<Option<Vec<Value>>, XlStreamError> { ... }
}
```

### Value conversion

calamine's `Data` / `DataRef` → our `Value`:

| calamine | xlstream `Value` |
|---|---|
| `Int(i)` | `Integer(i)` |
| `Float(f)` | `Number(f)` |
| `SharedString(s)`, `String(s)` | `Text(Box<str>)` |
| `Bool(b)` | `Bool(b)` |
| `DateTime(dt)` | `Date(ExcelDate::from_serial(dt.as_f64()))` |
| `DateTimeIso(s)` | `Date(ExcelDate::parse_iso(&s)?)` or fallback to `Text` |
| `Error(e)` | `Error(map_error(e))` |
| `Empty` | `Empty` |

Notes:
- We do not store string values as `&str` despite calamine offering zero-copy refs. Lifetime complexity outweighs the allocation saving at the scale we care about. Revisit if profiling says so.
- We do not parse ISO date strings in v0.1. They become `Text`.

### Streaming invariant

`CellStream::next_row` must not materialise more than one row at a time. Under the hood it aggregates `next_cell` results until row index advances, yields the accumulated row, drops the buffer. Peak overhead: `O(max_col)` for the row buffer, reused.

## Writing: rust_xlsxwriter

### Why rust_xlsxwriter

- Same maintainer as libxlsxwriter and Python's XlsxWriter — battle-tested.
- `constant_memory` feature: flushes rows to a tempfile the moment you advance past them. Flat ~20 MB RAM.
- `Formula::set_result` — we can write both the formula text and the cached value, so Excel doesn't force a recompute on open.
- MIT/Apache dual licensed.

### Features to enable

```toml
rust_xlsxwriter = { version = "0.94", features = [
    "constant_memory",   # streaming writes
    "zlib",              # fast deflate via libz-sys
    "ryu",               # fast float-to-string
] }
```

### API usage in xlstream

```rust
pub struct Writer {
    workbook: Workbook,
    // We create each sheet via add_worksheet_with_constant_memory().
    active: Option<RefCell<Worksheet>>,
}

impl Writer {
    pub fn create(path: &Path) -> Result<Self, XlStreamError> { ... }

    pub fn add_sheet(&mut self, name: &str) -> Result<(), XlStreamError> { ... }

    /// Write one row. `row_idx` must be strictly increasing within a sheet
    /// (enforced by constant_memory mode).
    pub fn write_row(&mut self, row_idx: u32, values: &[Value]) -> Result<(), XlStreamError> { ... }

    /// Write a formula cell with its pre-computed cached value.
    pub fn write_formula(
        &mut self,
        row: u32, col: u16,
        formula: &str,          // without the leading '='
        cached: &Value,
    ) -> Result<(), XlStreamError> { ... }

    pub fn finish(self) -> Result<(), XlStreamError> { ... }
}
```

### Value → cell conversion

| `Value` | rust_xlsxwriter call |
|---|---|
| `Number(f)` | `write_number(row, col, f)` |
| `Integer(i)` | `write_number(row, col, i as f64)` |
| `Text(s)` | `write_string(row, col, &s)` |
| `Bool(b)` | `write_boolean(row, col, b)` |
| `Date(d)` | `write_datetime(row, col, &d.to_excel_datetime(), &date_fmt)` |
| `Error(e)` | write as string `"#DIV/0!"` etc. See note. |
| `Empty` | skip (leave cell default) |

**Errors as strings:** rust_xlsxwriter doesn't have a direct error-cell write API. We render the error to its Excel string (`"#DIV/0!"` etc.) and write as text. This renders identically to a real error in Excel for most purposes. If we discover it's visibly different, we switch to writing the formula `=NA()` etc.

**Formula cells:** `write_formula(row, col, Formula::new(&formula).set_result(cached_str))`. The cached value is what Excel uses on open; the formula is retained so the user can edit.

### Ordering constraint

`constant_memory` requires strictly increasing row indices per sheet. The streaming driver enforces this naturally. With row parallelism, workers write to their own in-memory buffers and a single writer thread drains them in row order — see [`parallelism.md`](parallelism.md).

## Round-trip type fidelity

Given input cell type and value, the output cell type must match. Test matrix:

| Input | Output |
|---|---|
| Number | Number |
| Integer (stored as `Int` in calamine, numbers in Excel don't distinguish, so written as Number) | Number |
| Text | Text |
| Bool | Bool |
| Date | Date (with `yyyy-mm-dd` format) |
| Empty | Empty / skipped |
| Error | Error (as string, see above) |
| Formula cell → cached Value | Formula written with Value as cached result |

Tests assert round-trip equality for every type.

## Multi-sheet

```rust
let mut w = Writer::create(path)?;
w.add_sheet("Deals")?;
w.add_sheet("Thresholds")?;
// Add all sheets up front, then write rows.
```

Why "all up front": constant_memory mode creates a tempfile per sheet; the ordering is by creation. Writing rows to a sheet finalises that sheet's tempfile.

## Non-data passthrough

For sheets that xlstream doesn't evaluate (no formulas, or all unsupported), we can:

- v0.1: copy the sheet's raw values through (read with calamine, write with rust_xlsxwriter). Styles are NOT preserved. Documented limitation.
- v0.2: investigate copying the raw xlsx sheet XML bytes for perfect fidelity.

## What we do not support

- Preserving cell formatting beyond `NumFormat` for dates/percentages.
- Preserving charts, pivot tables, OLE objects — we carry a big warning in the docs. If the input has these, the output loses them.
- Writing `.xls` (binary xlsx predecessor).
- Reading password-protected files.
