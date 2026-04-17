# Phase 3 — I/O layer

**Goal:** streaming xlsx read + streaming xlsx write, with round-trip fidelity for data cells.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phase 1 complete.

**Reading:** [`docs/architecture/io.md`](../architecture/io.md), [`docs/research/calamine.md`](../research/calamine.md), [`docs/research/rust_xlsxwriter.md`](../research/rust_xlsxwriter.md).

**Output:** `xlstream-io::Reader` streams rows from an xlsx. `xlstream-io::Writer` streams rows out. A no-op roundtrip (read → write) preserves the data in every cell.

## Checklist

### Reader

- [ ] `Reader::open(path: &Path) -> Result<Self, XlStreamError>` wraps calamine's `open_workbook::<Xlsx<_>>`.
- [ ] `Reader::sheet_names() -> Vec<String>`.
- [ ] `Reader::cells(sheet: &str) -> Result<CellStream<'_>, XlStreamError>` returns a streaming iterator.
- [ ] `CellStream::next_row() -> Result<Option<Vec<Value>>, XlStreamError>` yields one row at a time.
  - [ ] Dense rows: missing cells become `Value::Empty`.
  - [ ] Length: `max_col` for the sheet (detected during open).
  - [ ] Reuses an internal buffer where possible; pay attention to hot-path allocation.
- [ ] `Reader::formulas(sheet: &str)` streams formula cells (address + formula text).
- [ ] Value conversion: calamine `DataRef` → our `Value` per the table in [`io.md`](../architecture/io.md).
- [ ] Rustdoc + doctests.

### Writer

- [ ] `Writer::create(path: &Path) -> Result<Self, XlStreamError>` creates a new `Workbook`.
- [ ] `Writer::add_sheet(name: &str) -> Result<SheetHandle, XlStreamError>` calls `add_worksheet_with_constant_memory`.
- [ ] `SheetHandle::write_row(row_idx: u32, values: &[Value]) -> Result<(), XlStreamError>`:
  - [ ] Enforces strictly-increasing `row_idx`.
  - [ ] Dispatches on `Value` variant: `write_number`, `write_string`, `write_boolean`, `write_datetime`, `write_blank`.
  - [ ] `Value::Error(e)` → writes the error's Excel string (e.g. `"#DIV/0!"`).
- [ ] `SheetHandle::write_formula(row, col, formula, cached_value)` wraps `Formula::new().set_result(...)`.
- [ ] `Writer::finish(self) -> Result<(), XlStreamError>` calls `workbook.save(path)`.
- [ ] Rustdoc + doctests.

### Round-trip tests

- [ ] Create a small xlsx fixture with every `Value` variant: Empty, Number, Integer (as Number in xlsx), Text, Bool, Date, Error.
- [ ] Round-trip test:
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
- [ ] Assertion: the output is round-trip-equivalent at the `Value` level for every cell.

### Multi-sheet

- [ ] Input with 3 sheets (main + 2 lookups). Round-trip preserves all three.
- [ ] Test: sheet order is preserved.

### Error paths

- [ ] File not found → `XlStreamError::Io`.
- [ ] Malformed xlsx → `XlStreamError::Xlsx`.
- [ ] Writing out of order → `XlStreamError::Internal` (constant-memory constraint).

### Performance smoke

- [ ] Read a 100k-row xlsx: < 5 seconds.
- [ ] Write the same: < 3 seconds (zlib + ryu).
- [ ] Round-trip: < 10 seconds total.
- [ ] Peak RSS: < 80 MB.

### Date handling

- [ ] `Value::Date` round-trips via `write_datetime`.
- [ ] Tests for both pre-1900 dates (the 1900 leap bug is relevant) and contemporary dates.
- [ ] 1900-02-29 (Excel serial 60) survives round-trip as serial 60.

### Formula cell round-trip

- [ ] When input has `=A1+B1` with cached `5.0`, reading gives us the formula string and cached value.
- [ ] Writing the same formula + cached value via `write_formula` produces an output that, re-read, gives the same pair.

## Verification

```bash
cargo test -p xlstream-io --all-features
cargo test --doc -p xlstream-io
cargo run -p xlstream-cli -- evaluate fixtures/canonical/arithmetic.xlsx --output /tmp/out.xlsx
# Even without eval implemented, --no-eval flag should copy input to output.
```

## Done when

Round-trip tests pass for every Value variant. 100k-row workload reads in < 5 s and writes in < 3 s. Peak RSS < 80 MB. All CI checks pass.
