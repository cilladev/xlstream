# v0.2 Roadmap

**Status:** in progress
**Target:** 2026 Q2
**Theme:** formula coverage, output fidelity, developer experience

## Features

### Formula support

- [x] **Named ranges** — resolve `MyRange` via `defined_names()` at classification time. ~1 day.
- [ ] **Table references** — `Table[Column]`, `Table[@Column]`. Parse `tables/table*.xml`, resolve to ranges. ~2 days.
- [x] **SUMPRODUCT** — sum of element-wise products. Requires bounded-range evaluation. ~1 day.
- [x] **MINIFS / MAXIFS** — conditional min/max. Same pattern as SUMIFS/COUNTIFS. ~0.5 day.
- [x] **ROWS / COLUMNS** — return row/column count. Uses `EXCEL_MAX_ROWS`/`EXCEL_MAX_COLS` constants (already in xlstream-core). ~0.5 day.

### Output fidelity

- [ ] **Keep formulas by default** — write `<f>formula</f><v>cached</v>` instead of just `<v>`. Add `--values-only` flag for static output. ~4 hours.
- [x] **PyPI description** — fix empty project page. ~30 min.
- [ ] **MID empty string workaround** — `rust_xlsxwriter` drops empty string writes. Either patch upstream or work around. ~1 hour.

### Performance

- [ ] **Memory optimization** — investigate calamine shared-strings buffering and rust_xlsxwriter string table. Target: < 250 MB for 700k-row workbook (currently 734 MB).


### Documentation

- [x] **Per-crate READMEs** — one-sentence purpose + example for each crate.
- [ ] **mdBook site** — optional.

## Out of scope (permanent)

- External workbook references (`[Book.xlsx]Sheet1!A1`) — violates single-file model
- `.xls`, `.xlsb`, `.ods` input formats
- Cell formatting preservation
- Circular references / iterative calculation
- Dynamic arrays (FILTER, UNIQUE, SORT, SEQUENCE)
- OFFSET, INDIRECT (runtime address resolution)
- LAMBDA, LET (user-defined functions)

## Out of scope (v0.3+)

- `.xlsb` input support
- User-defined functions
- Per-evaluation timeout
- Incremental re-evaluation
- RAND/RANDBETWEEN with deterministic seeding

## Done when

- All boxes ticked
- `make check` passes
- Golden-file regression passes
- Benchmark report generated (`make bench-report VERSION=0.2.0`)
- CHANGELOG promoted to `[0.2.0]`
- Tagged and released
