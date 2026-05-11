# Feature: ROW / COLUMN / ROWS / COLUMNS

**Branch:** `feat/row-column`
**Effort:** ~1 day
**Crates:** xlstream-parse, xlstream-eval

## What

Four related functions that return positional metadata about cell references:

- `ROW(ref)` — 1-based row number of a cell. No args = current row.
- `COLUMN(ref)` — 1-based column number of a cell. No args = current column.
- `ROWS(range)` — number of rows in a range.
- `COLUMNS(range)` — number of columns in a range.

```
=ROW(B5)            → 5
=ROW()              → (row of the formula cell)
=COLUMN(C3)         → 3
=COLUMN()           → (column of the formula cell)
=ROWS(A1:A10)       → 10
=ROWS(A:A)          → 1048576
=COLUMNS(A1:C5)     → 3
=COLUMNS(1:1)       → 16384
```

Current behavior: ROWS and COLUMNS are in `UNSUPPORTED_FUNCTIONS` in `sets.rs:21` — parser refuses them. ROW and COLUMN are not in the unsupported set but have no dispatch entry — they silently return `#VALUE!` from the fallback in `interp.rs:123`.

Note: the v0.2 roadmap checkbox for ROWS/COLUMNS is ticked but the implementation (on `remotes/origin/refactor/test-conformance`) was never merged to main.

## What already exists

- `EXCEL_MAX_ROWS` (1,048,576) and `EXCEL_MAX_COLS` (16,384) constants in `crates/xlstream-core/src/lib.rs:43-45`
- `NodeView::CellRef { sheet, row, col }` in `crates/xlstream-parse/src/view.rs:34-41` — row and col are 1-based
- `NodeView::RangeRef { sheet, start_row, end_row, start_col, end_col }` in `crates/xlstream-parse/src/view.rs:43-54` — Option fields (None for whole-column/whole-row)
- `RowScope::row_idx()` in `crates/xlstream-eval/src/scope.rs:82` — 0-based row index of the current streaming row
- `ISREF` dispatch in `builtins/mod.rs:190-196` — existing example of lazy dispatch (receives `NodeRef` args, inspects AST without evaluating)
- `ClassificationContext` carries `current_row` and `current_col` (1-based) at classification time, but not at eval time

## Where to look

- `crates/xlstream-parse/src/sets.rs:21` — remove ROWS and COLUMNS from `UNSUPPORTED_FUNCTIONS`
- `crates/xlstream-parse/src/classify.rs` — verify all four classify as RowLocal
- `crates/xlstream-eval/src/builtins/mod.rs:87-209` — `dispatch()` function, add four new arms
- `crates/xlstream-eval/src/builtins/info.rs` — implementation home (info/metadata functions)
- `crates/xlstream-eval/src/interp.rs:118-124` — `Function` dispatch site
- `crates/xlstream-eval/src/scope.rs:23-26` — `RowScope` struct (may need `col_idx` or formula column)
- `crates/xlstream-eval/src/evaluate.rs:892-897` — `eval_column()` function (has `fcol` — 0-based formula column)
- `crates/xlstream-parse/src/view.rs:34-54` — `CellRef` and `RangeRef` field definitions

## Resolution / Evaluation behavior

All four are pure functions of reference metadata — they don't read cell values, don't need prelude, don't need lookup sheets.

**Classification:** Remove ROWS and COLUMNS from `UNSUPPORTED_FUNCTIONS`. All four classify as RowLocal — no whole-column aggregate pattern, no lookup pattern, no cross-row references.

**Prelude:** Nothing needed.

**Row eval:**

### ROW(ref) and COLUMN(ref) — with argument

Lazy dispatch (receive `NodeRef`, not `&[Value]`). Inspect the AST node:

- `NodeView::CellRef { row, col, .. }` → return `row` (for ROW) or `col` (for COLUMN). Already 1-based.
- `NodeView::RangeRef { start_row, start_col, .. }` → return `start_row` (for ROW) or `start_col` (for COLUMN). Excel returns the first row/col of the range. If None (whole-column/whole-row), return 1.
- Anything else → `#VALUE!`

### ROW() and COLUMN() — no argument

Returns the 1-based row/column of the formula cell itself. This requires knowing the formula's position at eval time:

- **ROW()**: `scope.row_idx() + 1` (row_idx is 0-based, Excel rows are 1-based). Already available.
- **COLUMN()**: needs the formula's 0-based column index. Currently `RowScope` doesn't carry this. Two options:
  1. Add `col_idx: u32` to `RowScope` (set from `fcol` in `eval_column`).
  2. Pass the column index through the dispatch chain (add to `dispatch()` signature or a context struct).

  Option 1 is simpler. `RowScope` already carries `row_idx` for the same reason. Adding `col_idx` is a parallel change. The `RowScope::new(values, row_idx)` call in `eval_column` becomes `RowScope::new(values, row_idx, fcol)`.

### ROWS(range) and COLUMNS(range)

Lazy dispatch. Inspect the AST node:

- `NodeView::RangeRef { start_row, end_row, .. }` for ROWS: if both Some, return `end_row - start_row + 1`. If either None (whole-column), return `EXCEL_MAX_ROWS`.
- `NodeView::RangeRef { start_col, end_col, .. }` for COLUMNS: if both Some, return `end_col - start_col + 1`. If either None (whole-row), return `EXCEL_MAX_COLS`.
- `NodeView::CellRef { .. }` → return 1 (single cell = 1 row, 1 column).
- No args or wrong type → `#VALUE!`

## Tests

### Classification (unit tests)

**Happy path:**
- `ROW(A5)` — classifies as RowLocal
- `COLUMN(C3)` — classifies as RowLocal
- `ROW()` — no args, classifies as RowLocal
- `ROWS(A1:A10)` — classifies as RowLocal
- `COLUMNS(A1:C1)` — classifies as RowLocal

**Edge cases:**
- `row(a5)` — case insensitive
- `ROWS(Sheet2!A1:A10)` — cross-sheet range
- `COLUMNS(1:5)` — whole-row ref
- `IF(ROW()>5, "big", "small")` — nested in IF
- `ROW()+COLUMN()` — in arithmetic
- `ROWS()` — no args, should produce `#VALUE!`
- `COLUMNS()` — no args, should produce `#VALUE!`
- `ROWS(A1)` — single cell ref, returns 1
- `COLUMNS(A1)` — single cell ref, returns 1

**Regression guards:**
- Existing info functions (ISBLANK, ISNUMBER, etc.) unchanged
- Existing conformance tests all pass

### Conformance (per-function xlsx fixtures)

Create `tests/fixtures/info/row-column.xlsx`:

**Data needed:** Sheet1 with some values in A1:D10 (for context). Sheet2 with values in A1:A20 (for cross-sheet tests).

**Sheet1 layout:**
- A1:D1 headers: "X", "Y", "Z", "W"
- A2:D10 numeric data (any values — the functions don't read cell values, just metadata)

**Formulas (place in column E or further right, rows 2+):**

Happy path — ROW:
- `=ROW(A1)` → 1
- `=ROW(A5)` → 5
- `=ROW(B10)` → 10
- `=ROW(A100)` → 100
- `=ROW()` → (row of this formula cell — varies by placement)

Happy path — COLUMN:
- `=COLUMN(A1)` → 1
- `=COLUMN(C3)` → 3
- `=COLUMN(Z1)` → 26
- `=COLUMN(AA1)` → 27
- `=COLUMN()` → (column of this formula cell — varies by placement)

Happy path — ROWS:
- `=ROWS(A1:A10)` → 10
- `=ROWS(A1:C5)` → 5
- `=ROWS(B3:B7)` → 5
- `=ROWS(A1:A100)` → 100

Happy path — COLUMNS:
- `=COLUMNS(A1:C5)` → 3
- `=COLUMNS(A1:Z1)` → 26
- `=COLUMNS(B3:E3)` → 4

Single cell:
- `=ROW(A1)` → 1
- `=COLUMN(A1)` → 1
- `=ROWS(A1)` → 1
- `=COLUMNS(A1)` → 1
- `=ROWS(A1:A1)` → 1
- `=COLUMNS(A1:A1)` → 1

Whole-column / whole-row:
- `=ROWS(A:A)` → 1048576
- `=ROWS(B:B)` → 1048576
- `=COLUMNS(A:C)` → 3
- `=COLUMNS(1:1)` → 16384
- `=ROW(A:A)` → 1 (first row of whole-column ref)
- `=COLUMN(1:1)` → 1 (first column of whole-row ref)

Range arg for ROW/COLUMN (returns first row/col):
- `=ROW(A3:A10)` → 3
- `=COLUMN(C1:E1)` → 3

Nested usage:
- `=ROW()+COLUMN()` → sum of formula cell's row and column
- `=IF(ROW()>5,"below","above")` → depends on placement
- `=ROWS(A1:A10)+COLUMNS(A1:C1)` → 13
- `=ROWS(A1:A3)*COLUMNS(A1:C3)` → 9

Cross-sheet:
- `=ROW(Sheet2!A5)` → 5
- `=COLUMN(Sheet2!C1)` → 3
- `=ROWS(Sheet2!A1:A20)` → 20
- `=COLUMNS(Sheet2!A1:D1)` → 4

Error cases:
- `=ROW("text")` → `#VALUE!`
- `=COLUMN(123)` → `#VALUE!`
- `=ROWS("text")` → `#VALUE!`

Fixture workflow:
1. Generate xlsx with openpyxl (formulas + data)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn row_column()` in `conformance/info.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add ROW/COLUMN/ROWS/COLUMNS entry |
| `docs/functions.md` | Tick all four as implemented |
| `docs/roadmap/v0.2/README.md` | Verify ROWS/COLUMNS checkbox (was ticked prematurely — confirm after actual merge) |
| `docs/roadmap/v0.3/README.md` | Tick the ROW/COLUMN checkbox, add ROWS/COLUMNS if not already listed |

## Streaming invariant

Does not violate. All four functions inspect reference metadata in the AST or the current row/column position. They don't read cell values from any row. `ROW()` and `COLUMN()` (no-arg) use the formula cell's position, which is known at eval time without cross-row access.
