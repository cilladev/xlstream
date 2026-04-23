# Feature: Table References

**Branch:** `feat/table-references`
**Effort:** ~2 days
**Crates:** xlstream-io, xlstream-parse, xlstream-eval

## What

Excel structured references let formulas address table columns by name instead of cell coordinates. A formula on a row inside `Table1` can say `=[@Price]*[@Quantity]` (current row's Price and Quantity columns) or `=SUM(Table1[Amount])` (entire Amount column). The syntax is parsed by `formualizer-parse` already; xlstream currently refuses all table references at classification with `UnsupportedReason::TableReference`.

```
=[@Price]*1.1          → current row's Price cell times 1.1
=SUM(Table1[Amount])   → sum of entire Amount column
=Table1[@Region]       → current row's Region cell
```

Current behavior: any formula containing a table reference returns `UnsupportedFormula` error.

## What already exists

- `formualizer-parse` parses table references and emits `RefView::Table { name, specifier }` with a rich `TableSpecifier` enum (Column, ColumnRange, All, Data, Headers, Totals, ThisRow, Combination). See `refs/formualizer/crates/formualizer-parse/src/parser.rs:65`.
- `xlstream-parse` extracts `Reference::Table { name, specifier }` in `collect_view()` at `crates/xlstream-parse/src/references.rs:143-148`. The specifier is stored as `Option<String>` via `Display`.
- Classification refuses `Reference::Table` at `crates/xlstream-parse/src/classify.rs:440`.
- `UnsupportedReason::TableReference` variant exists with doc link at `crates/xlstream-parse/src/classify.rs:42-43`.
- Calamine has full table metadata support: `load_tables()`, `table_names()`, `table_by_name()`, `table_names_in_sheet()`. Metadata includes table name, sheet name, column names, and data dimensions. See `refs/calamine/src/xlsx/mod.rs:1086-1204` and `TableMetadata` struct at line 1541.
- Named range resolution (`crates/xlstream-parse/src/resolve.rs`) provides the pattern: AST rewriting that replaces symbolic references with resolved cell/range references before classification.
- `Reader` wraps `calamine::Xlsx<BufReader<File>>` at `crates/xlstream-io/src/reader.rs:26-28`.

## Where to look

- `crates/xlstream-parse/src/references.rs` — `Reference::Table` variant (line 62), `collect_view` (line 143)
- `crates/xlstream-parse/src/classify.rs` — refusal at line 440, `UnsupportedReason::TableReference` at line 42
- `crates/xlstream-parse/src/resolve.rs` — named range resolution pattern to follow
- `crates/xlstream-io/src/reader.rs` — `Reader` struct, `defined_names()` at line 107
- `crates/xlstream-eval/src/evaluate.rs` — `build_plan()` at line 151, named range loading at line 154, `build_eval_plan()` call at line 171
- `refs/calamine/src/xlsx/mod.rs` — `load_tables()` at line 1086, `table_names()` at line 1133, `get_table_meta()` at line 862, `TableMetadata` at line 1541
- `refs/formualizer/crates/formualizer-parse/src/parser.rs` — `TableSpecifier` enum at line 65, `SpecialItem` at line 107, `RefView::Table` at line 1801

## Resolution / Evaluation behavior

Table references resolve at **classification time** (before prelude), following the same pattern as named ranges:

1. **IO layer** (`xlstream-io`): `Reader` exposes table metadata — a map from table name to `{ sheet_name, columns: Vec<String>, data_dimensions }`. Calls calamine's `load_tables()` + `get_table_meta()` / `table_names()` internally.

2. **Parse layer** (`xlstream-parse`): A new `resolve_table_references()` function rewrites the AST. For each `Reference::Table { name, specifier }`:
   - Look up the table metadata by name (case-insensitive).
   - `Table[Column]` (whole column) → `Reference::Range` over the column's data rows on the table's sheet.
   - `Table[@Column]` or `[@Column]` (current row) → `Reference::Cell` pointing to the column index on the current sheet. Row is unresolved at classification time — the evaluator fills it per-row.
   - `Table` (whole table, no specifier) → `Reference::Range` over all data columns/rows.
   - Unknown table name → leave as `Reference::Table`, classifier rejects with a new `UnsupportedReason::UnknownTable` or similar.

3. **Eval layer** (`xlstream-eval`): `build_plan()` loads table metadata from the reader (same location as named ranges loading), passes it to the parse layer for resolution before classification.

**Streaming invariant compliance:** `Table[Column]` resolves to a whole-column range — handled by prelude aggregates, same as `A:A`. `[@Column]` resolves to a same-row cell reference — row-local by definition. No new data access patterns are introduced.

### The `@` (ThisRow) specifier

This is the nuanced case. `[@Column]` means "the value in this column on the current row." At classification time the row number is unknown. Two approaches:

**Option A:** Resolve `[@Column]` to a cell reference with a sentinel row (e.g., row 0) and have the evaluator substitute the actual row at eval time. Requires the evaluator to recognize and rewrite sentinels.

**Option B:** Introduce a new AST node or Reference variant (e.g., `Reference::CurrentRowCell { sheet, col }`) that the evaluator resolves per-row. Cleaner but wider surface area.

The agent should evaluate both against the existing evaluator's row-dispatch code (`crates/xlstream-eval/src/evaluate.rs` row loop and `crates/xlstream-eval/src/interpreter.rs`) and pick the one that requires fewer changes.

## Tests

### Classification (unit tests)

**Happy path:**
- `Table1[Amount]` with table metadata resolves and classifies as Aggregate (whole-column range)
- `[@Price]*1.1` with table metadata resolves and classifies as RowLocal
- `SUM(Table1[Amount])` classifies as Aggregate
- `VLOOKUP([@Key], Table2[Data], 2, FALSE)` classifies as Lookup
- `IF([@Status]="Active", [@Amount], 0)` classifies as RowLocal

**Edge cases:**
- Case insensitivity: `table1[amount]` resolves same as `Table1[Amount]`
- Unknown table name: `UnknownTable[Col]` produces a clear classification error (not a panic)
- Unknown column name: `Table1[NonexistentCol]` produces a classification error
- Empty specifier: `Table1` (whole table reference) resolves to full data range
- Column range: `Table1[[Col1]:[Col3]]` resolves to bounded range
- Nested in SUMIF: `SUMIF(Table1[Region], "EMEA", Table1[Amount])` classifies as Aggregate
- Nested in IF: `IF([@A]>0, [@B], [@C])` classifies as RowLocal
- Mixed with named ranges: `SUM(MyRange) + [@Price]` — both resolve
- Mixed with regular cell refs: `[@Price] + B2` classifies as RowLocal
- Error propagation: table reference inside a formula that also has an unsupported function — unsupported function error takes precedence
- `[@Column]` outside any table context — should produce a classification error

**Regression guards:**
- All existing classification tests pass unchanged
- Named range resolution still works
- Formulas without table references are unaffected

### Evaluation (integration tests)

- Programmatic xlsx with a table (created via openpyxl with `ws.add_table()`), formula `=[@Price]*[@Qty]` on each row — verify correct per-row multiplication
- `=SUM(Table1[Amount])` on a non-table sheet referencing a table column — verify correct aggregate
- `=VLOOKUP([@Key], Table2[[Key]:[Value]], 2, FALSE)` — table reference as lookup table array
- Table on a non-main sheet with `[@Column]` formulas — verify evaluation (related to issue #42 if resolved)
- Unknown table name in formula — verify `UnsupportedFormula` error with actionable message

### Golden-file regression

The golden regression fixture will need new formula columns exercising table references. The fixture must be created in Excel (not openpyxl) to have correct cached values. Alternatively, add a separate table-reference regression fixture.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add table reference entry under `[Unreleased]` |
| `docs/functions.md` | Tick the table reference checkbox (line 607) |
| `docs/roadmap/v0.2/README.md` | Tick the table references checkbox |
| `docs/architecture/streaming-model.md` | Update table reference row from "Unsupported" to supported with resolution description (line 75) |

## Streaming invariant

Does not violate the invariant. `Table[Column]` resolves to a whole-column range (prelude-computed). `[@Column]` resolves to a same-row cell (row-local). All resolution happens before or during the existing two-pass model.
