# Phase 3 — I/O layer

**Goal:** streaming xlsx read + streaming xlsx write, with round-trip fidelity for data cells.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phase 1 complete.

**Reading:** [`docs/architecture/io.md`](../architecture/io.md), [`docs/research/calamine.md`](../research/calamine.md), [`docs/research/rust_xlsxwriter.md`](../research/rust_xlsxwriter.md).

**Output:** `xlstream-io::Reader` streams rows from an xlsx. `xlstream-io::Writer` streams rows out. A no-op roundtrip (read → write) preserves the data in every cell.

## Checklist

### Reader

- [x] `Reader::open(path: &Path) -> Result<Self, XlStreamError>` wraps calamine's `open_workbook::<Xlsx<_>>`.
- [x] `Reader::sheet_names() -> Vec<String>`.
- [x] `Reader::cells(sheet: &str) -> Result<CellStream<'_>, XlStreamError>` returns a streaming iterator.
- [x] `CellStream::next_row() -> Result<Option<Vec<Value>>, XlStreamError>` yields one row at a time.
  - [x] Dense rows: missing cells become `Value::Empty`.
  - [x] Length: `max_col` for the sheet (detected during open).
  - [x] Reuses an internal buffer where possible; pay attention to hot-path allocation.
- [x] `Reader::formulas(sheet: &str)` streams formula cells (address + formula text).
- [x] Value conversion: calamine `DataRef` → our `Value` per the table in [`io.md`](../architecture/io.md).
- [x] Rustdoc + doctests.

### Writer

- [x] `Writer::create(path: &Path) -> Result<Self, XlStreamError>` creates a new `Workbook`.
- [x] `Writer::add_sheet(name: &str) -> Result<SheetHandle, XlStreamError>` calls `add_worksheet_with_constant_memory`.
- [x] `SheetHandle::write_row(row_idx: u32, values: &[Value]) -> Result<(), XlStreamError>`:
  - [x] Enforces strictly-increasing `row_idx`.
  - [x] Dispatches on `Value` variant: `write_number`, `write_string`, `write_boolean`, `write_datetime`, `write_blank`.
  - [x] `Value::Error(e)` → writes the error's Excel string (e.g. `"#DIV/0!"`).
- [x] `SheetHandle::write_formula(row, col, formula, cached_value)` wraps `Formula::new().set_result(...)`.
- [x] `Writer::finish(self) -> Result<(), XlStreamError>` calls `workbook.save(path)`.
- [x] Rustdoc + doctests.

### Round-trip tests

- [x] Create a small xlsx fixture with every `Value` variant: Empty, Number, Integer (as Number in xlsx), Text, Bool, Date, Error.
- [x] Round-trip test:
  ```rust
  let input = ...;
  let reader = Reader::open(&input)?;
  let cells = reader.cells("Sheet1")?;
  let mut writer = Writer::create(&output)?;
  let mut sheet = writer.add_sheet("Sheet1")?;
  let mut row = 0;
  while let Some(r) = cells.next_row()? {
      sheet.write_row(row, &r)?;
      row += 1;
  }
  writer.finish()?;
  // Re-open output and assert equality with input.
  ```
- [x] Assertion: the output is round-trip-equivalent at the `Value` level for every cell.

### Multi-sheet

- [x] Input with 3 sheets (main + 2 lookups). Round-trip preserves all three.
- [x] Test: sheet order is preserved.

### Error paths

- [x] File not found → `XlStreamError::Xlsx` (calamine wraps the I/O error).
- [x] Malformed xlsx → `XlStreamError::Xlsx`.
- [x] Writing out of order → `XlStreamError::Internal` (constant-memory constraint).

### Performance smoke

- [x] Read a 100k-row xlsx: < 5 seconds.
- [x] Write the same: < 3 seconds (zlib + ryu).
- [x] Round-trip: < 10 seconds total.
- [x] Peak RSS: < 80 MB (verified via manual `/usr/bin/time -l` on macOS; not asserted in-process).

### Date handling

- [x] `Value::Date` round-trips via `write_datetime`.
- [x] Tests for both pre-1900 dates (the 1900 leap bug is relevant) and contemporary dates.
- [x] 1900-02-29 (Excel serial 60) survives round-trip as serial 60.

### Formula cell round-trip

- [x] When input has `=A1+B1` with cached `5.0`, reading gives us the formula string and cached value.
- [x] Writing the same formula + cached value via `write_formula` produces an output that, re-read, gives the same pair.

## Verification

```bash
cargo test -p xlstream-io --all-features
cargo test --doc -p xlstream-io
cargo run -p xlstream-cli -- evaluate fixtures/canonical/arithmetic.xlsx --output /tmp/out.xlsx
# Even without eval implemented, --no-eval flag should copy input to output.
```

## Done when

Round-trip tests pass for every Value variant. 100k-row workload reads in < 5 s and writes in < 3 s. Peak RSS < 80 MB. All CI checks pass.
