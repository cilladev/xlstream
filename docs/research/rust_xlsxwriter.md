# rust_xlsxwriter — write-side research

**VERDICT: suitable** for streaming xlsx writes.

## What it is

`rust_xlsxwriter` is a pure-Rust xlsx writer by John McNamara (the same maintainer behind `libxlsxwriter` C, Python's `XlsxWriter`, and Perl's `Excel::Writer::XLSX`). Very active — v0.94+ as of April 2026. Dual licensed MIT / Apache-2.0.

## Streaming support — YES, via `constant_memory` feature

Default mode buffers the whole workbook in memory. The `constant_memory` feature flag turns on streaming writes: each row is flushed to a tempfile on disk the moment you advance past it.

### Enabling

```toml
[dependencies]
rust_xlsxwriter = { version = "0.94", features = [
    "constant_memory",   # streaming writes — required
    "zlib",              # fast deflate via libz-sys
    "ryu",               # fast float-to-string
] }
```

### Using

```rust
use rust_xlsxwriter::{Workbook, Worksheet};

let mut workbook = Workbook::new();
let ws = workbook.add_worksheet_with_constant_memory();
// Streaming: write rows strictly in increasing row order.
ws.write_string(0, 0, "Revenue")?;
ws.write_string(0, 1, "Region")?;
ws.write_number(1, 0, 5000.0)?;
ws.write_string(1, 1, "EMEA")?;
// Once you advance past row N you cannot go back.
workbook.save("out.xlsx")?;
```

### Memory profile

- Standard mode (default): ~72 MB RAM for 400k cells (400k × 1 col).
- `constant_memory`: flat ~20 MB RAM regardless of row count.
- `constant_memory` + `zlib` + `ryu`: ~150 ms to write 200k cells on a modern machine.

## Cell type coverage

```rust
ws.write_number(row, col, 42.0)?;
ws.write_string(row, col, "hello")?;
ws.write_boolean(row, col, true)?;
ws.write_formula(row, col, "=A1+B1")?;
ws.write_blank(row, col, &format)?;
ws.write_datetime(row, col, &ExcelDateTime::from_ymd(2026, 4, 17)?)?;
// Polymorphic: dispatches on IntoExcelData trait.
ws.write(row, col, value)?;
```

## Formula cells with cached values — critical

Excel stores both the formula text and its last-computed value ("cached result"). If a workbook has formulas but no cached values, Excel forces a recompute on open — which defeats our entire purpose (we've already computed; we don't want Excel doing it again).

```rust
use rust_xlsxwriter::Formula;

ws.write_formula(
    row, col,
    Formula::new("=A1+B1").set_result("5"),
)?;
```

`set_result` stores the cached value as a string. Excel reads it and displays it immediately on open.

For LibreOffice compatibility (which ignores cached results by default), we can call `worksheet.set_formula_result_default("")` to force recalc. Our default: write real cached values; let Excel trust them.

## Performance

Per the maintainer's hyperfine benchmarks on 4000 × 50 (200k cells):

| Writer | Time |
|---|---|
| rust_xlsxwriter + zlib | 152 ms |
| libxlsxwriter (C) | 211 ms |
| rust_xlsxwriter (stock) | 240 ms |
| Python XlsxWriter | 919 ms |

Scales linearly up to 1M cells. Our 400k × 20 (~8M cells) extrapolates to ~5–7 s of pure write time in constant-memory mode. Flat ~20 MB RAM throughout.

The `ryu` feature flag adds another 20–30 % speedup on numeric-heavy files.

No head-to-head benchmark vs umya-spreadsheet published by the maintainer, but community threads (Reddit r/rust, umya GitHub issues) consistently report rust_xlsxwriter 2–5× faster for large writes and using dramatically less memory.

## Multi-sheet

Just call `add_worksheet` / `add_worksheet_with_constant_memory` repeatedly on the same `Workbook`. Each sheet gets its own tempfile in constant-memory mode. Order of sheet creation is order of appearance in the output.

## Formatting

```rust
use rust_xlsxwriter::Format;

let date_fmt = Format::new().set_num_format("yyyy-mm-dd");
let pct_fmt = Format::new().set_num_format("0.00%");
ws.write_with_format(row, col, &date, &date_fmt)?;
```

Number formats (dates, percentages, currency) are cheap. Full cell-level formatting (fonts, colours, borders) is supported but we only use it for type preservation in v0.1.

Works in `constant_memory` mode — format applies to the current row.

## Ordering constraint

`constant_memory` requires strictly increasing row indices per sheet. Once you write row N+1, you cannot write row N.

Implications for our streaming pipeline: the writer must be fed rows in order. With row parallelism, workers produce out-of-order; we use a reorder buffer in the single writer thread.

## Errors

`rust_xlsxwriter::XlsxError` enum: IoError, InvalidParameter, Overflow, others. All `Result`-returning; no panics.

## Maintainer activity

- v0.91 Dec 2025, v0.92 Jan 2026, v0.93 Feb 2026, v0.94 Apr 2026. Monthly cadence.
- Same bug fixes ported across C/Python/Perl family — inherited test coverage.
- 552+ GitHub stars, growing.

## API reference

- Docs: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/
- GitHub: https://github.com/jmcnamara/rust_xlsxwriter
- Performance guide: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/performance/index.html

## Version pin

Track `^0.94`. API is stable; minor versions add features without breaking.

## What we use

- ✅ `constant_memory` writing mode.
- ✅ `Formula::set_result()` for cached values.
- ✅ `zlib` + `ryu` feature flags.
- ✅ `Format::set_num_format` for date / percent / currency rendering.
- ✅ Multi-sheet output.

## What we defer

- ❌ Rich styling (fonts, colours, borders) — v0.1 drops most styling from input.
- ❌ Charts, tables, pivot tables — v0.1 loses these from input.
- ❌ Conditional formatting — same.
- ❌ VBA / macros — same.

## Alternatives we considered

- **umya-spreadsheet** — buffers whole workbook. No constant-memory mode.
- **simple_excel_writer** — stale, write-only with smaller feature set.
- **xlsxwriter (via PyO3)** — running Python xlsxwriter from Rust is absurd.

rust_xlsxwriter is the answer.
