# Feature: ROWS / COLUMNS

**Branch:** `feat/rows-columns`
**Effort:** ~0.5 day
**Crates:** xlstream-parse, xlstream-eval

## What

`ROWS(range)` returns the number of rows in a range. `COLUMNS(range)` returns the number of columns. For whole-column refs (`A:A`), ROWS returns `EXCEL_MAX_ROWS` (1,048,576). For whole-row refs (`1:1`), COLUMNS returns `EXCEL_MAX_COLS` (16,384).

```
=ROWS(A1:A10)      → 10
=ROWS(A:A)          → 1048576
=ROWS(A1:C5)        → 5
=COLUMNS(A1:C5)     → 3
=COLUMNS(A:C)        → 3
=COLUMNS(1:1)        → 16384
```

Current behavior: both are in `UNSUPPORTED_FUNCTIONS` in `sets.rs:21`. Parser refuses them with `Classification::Unsupported`.

## What already exists

- `EXCEL_MAX_ROWS` and `EXCEL_MAX_COLS` constants — `crates/xlstream-core/src/lib.rs:37-40`
- `NodeView::RangeRef` — `crates/xlstream-parse/src/view.rs:42-54` — has `start_row`, `end_row`, `start_col`, `end_col` as `Option<u32>` (None for whole-column/whole-row)
- These functions don't need prelude data or row values — they operate purely on the range reference structure in the AST

## Where to look

- `crates/xlstream-parse/src/sets.rs:16-22` — remove ROWS and COLUMNS from `UNSUPPORTED_FUNCTIONS`
- `crates/xlstream-parse/src/classify.rs` — verify they classify as RowLocal (pure functions of their args)
- `crates/xlstream-eval/src/builtins/mod.rs:83-203` — add dispatch arms
- `crates/xlstream-eval/src/builtins/info.rs` — logical home for these (they inspect reference metadata, not cell values)
- `crates/xlstream-parse/src/view.rs:42-54` — `RangeRef` variant fields
- `crates/xlstream-core/src/lib.rs:37-40` — `EXCEL_MAX_ROWS`, `EXCEL_MAX_COLS`

## Resolution / Evaluation behavior

These are pure functions of the AST node — they don't read cell values, don't need prelude, don't need the current row.

**Classification:** Remove from `UNSUPPORTED_FUNCTIONS`. They'll fall through to default classification as RowLocal (no whole-column aggregate pattern, no lookup pattern).

**Prelude:** Nothing needed.

**Row eval:** The builtin receives the `RangeRef` argument as an AST node (not expanded). It inspects the `start_row`, `end_row`, `start_col`, `end_col` fields directly:
- `ROWS`: if both `start_row` and `end_row` are `Some`, return `end_row - start_row + 1`. If either is `None` (whole-column ref), return `EXCEL_MAX_ROWS`.
- `COLUMNS`: if both `start_col` and `end_col` are `Some`, return `end_col - start_col + 1`. If either is `None` (whole-row ref), return `EXCEL_MAX_COLS`.

**Important:** these functions must NOT call `expand_range()`. They inspect the range reference metadata, not the range contents. The arg must be read as a raw `NodeView::RangeRef`, not evaluated to a value.

## Tests

### Classification (unit tests)

**Happy path:**
- `ROWS(A1:A10)` — bounded range, classifies as RowLocal
- `COLUMNS(A1:C1)` — bounded range, classifies as RowLocal
- `ROWS(A:A)` — whole-column, classifies as RowLocal

**Edge cases:**
- `rows(a1:a10)` — case insensitive
- `ROWS(Sheet2!A1:A10)` — cross-sheet range
- `COLUMNS(1:5)` — whole-row ref
- `IF(ROWS(A1:A10)>5, "big", "small")` — nested in IF
- `ROWS()` — no args, should produce `#VALUE!`
- `ROWS(A1)` — single cell ref, should return 1
- `COLUMNS(A1)` — single cell ref, should return 1

**Regression guards:**
- Existing functions must not change behavior
- Other info functions (ISBLANK, ISNUMBER, etc.) unchanged

### Evaluation (integration tests)

- `ROWS(A1:A10)` → 10
- `ROWS(A:A)` → 1048576
- `COLUMNS(A1:C5)` → 3
- `COLUMNS(A:C)` → 3
- `COLUMNS(1:1)` → 16384
- `ROWS(A1:A1)` → 1 (single cell)
- Combined: `ROWS(A1:A5) * COLUMNS(A1:C5)` → 15

### Conformance fixture

Create `tests/fixtures/info/rows_columns.xlsx` with data + ROWS/COLUMNS formulas. Generate with openpyxl, recalculate with LibreOffice headless. Add `#[test] fn rows_columns()` in `conformance/info.rs`.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add ROWS/COLUMNS entry under `[Unreleased]` |
| `docs/functions.md` | Tick ROWS and COLUMNS as implemented |
| `docs/roadmap/v0.2/README.md` | Tick the checkbox |

## Streaming invariant

Does not violate. ROWS/COLUMNS inspect range reference metadata in the AST. They don't read cell values from any row.
