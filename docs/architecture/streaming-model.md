# Streaming model

The invariant:

> **After prelude, no evaluator function may read a cell value from a row it has not yet streamed over.**

Everything in this document exists to make that statement true and useful.

## Why this shape

A spreadsheet evaluator has to answer "what is cell C5?" — and the answer can depend on any other cell. A graph-based engine handles this by building the full dependency DAG and evaluating in topological order. A streaming engine cannot; it sees one row at a time. So we restrict ourselves to formulas whose answers depend only on:

1. Cells in the **same row** (row-local).
2. **Scalars** derived from a full column / sheet / workbook in a cheap single pass (aggregates).
3. **Lookup tables** that fit in memory and can be hash-indexed once (lookup sheets).

Together these cover the overwhelming majority of real business workbooks. The minority that fall outside — forward/backward row refs, cross-row circular refs, dynamic-array spills that depend on future rows — we refuse with a clear error. Self-referential formulas (a cell referencing itself) are supported via iterative calculation within the current row.

## The three formula shapes

### Row-local

```
=Deal Value * Quantity
=IF(Revenue > 10000, "Gold", "Silver")
=UPPER(LEFT(Product, 3)) & "-" & YEAR(Date)
```

References: only cells on the current row. No other data needed.

Evaluation: walk the AST with a single-row scope, resolve each cell reference to the row's pre-loaded values.

### Lookup-into-loaded-sheet

```
=VLOOKUP(Region, 'Region Info'!A:C, 2, FALSE)
=XLOOKUP(ID, Lookup!A:A, Lookup!B:B)
=VLOOKUP(Region & Business, 'Thresholds'!D:E, 2, FALSE)   ' multi-key via concat + helper column
```

References: cells on the current row + an entire lookup sheet or column range from a different sheet.

Prerequisite: the lookup sheet must fit in memory so prelude can load it fully and build a hash index. "Fit in memory" is the real constraint — a 100k-row lookup sheet with narrow text keys is a few megabytes of heap, totally fine.

Evaluation: at row time, compute the lookup key from current-row cells, probe the hash index, return the indexed value.

### Aggregate of a column

```
=Deal Value / SUM(Deal Value:Deal Value) * 100
=Quantity / SUMIF(Region:Region, "EMEA", Quantity:Quantity)
```

References: cells on the current row + an aggregate over a whole column.

Prerequisite: the aggregate must be evaluable in a single pass over the column during prelude.

Evaluation: at row time, the aggregate is a pre-computed scalar in the prelude context; treat it like a constant.

## Classification algorithm

For each formula found in the main sheet, walk the AST. Record every `CellRef`, `RangeRef`, and `SheetRef`. For each, decide:

| Reference | Disposition |
|---|---|
| `Column_Name` (implicit current row) or `A2` where 2 == current row | **RowLocal** |
| `A:A`, `Revenue:Revenue`, `Sheet1!A:A` inside an aggregate function (`SUM`, `COUNT`, ...) | **AggregateOnly** |
| `'Sheet X'!A:C` used as lookup range inside a supported lookup function | **LookupOnly** |
| `A:A` outside an aggregate function (e.g. `A:A * 2`) | **Unsupported** (would require full-column materialisation) |
| `A3` where 3 ≠ current row | **Unsupported** (forward/backward row ref) |
| Self-reference (`A2` in cell `A2`) | **RowLocal** (iterative calc using cached value as seed) |
| Function in the `VOLATILE_STREAMING_OK` set (`TODAY`, `NOW`) | **Supported** with single-evaluation-per-run semantics |
| Function in the `UNSUPPORTED` set (`OFFSET`, `INDIRECT`, `FILTER`, `UNIQUE`, `SORT`) | **Unsupported** |
| Named range (`MyRange`) | **Supported** (resolved at classification time via `defined_names()`) |
| Table reference (`Table1[Column]`) | **Unsupported** (v0.2 candidate; requires table-definition loading) |
| External reference (`[Book2.xlsx]Sheet1!A1`) | **Unsupported** (permanent; violates single-file model) |

A formula is **Supported** iff it is RowLocal, AggregateOnly, LookupOnly, or a mixture whose sub-expressions each fall into one of those shapes. Otherwise **Unsupported** → refused with a clear error.

### Why `OFFSET` and `INDIRECT` are unsupported

They can resolve to arbitrary cell addresses at runtime. Classification cannot know, at prelude, which cells the formula needs. Treating them as supported would force us to load the whole workbook, defeating the streaming model. Refuse.

Users who need these should preprocess their workbooks (statically resolve them) before handing to xlstream.

## Prelude — one pass over each column that needs it

### Aggregate pre-pass

A single streaming pass over the main sheet produces, for each required aggregate, a scalar. We only pass over columns that are referenced by an aggregate; untouched columns are ignored.

Implementation sketch:
```rust
// For each column that needs aggregates:
let mut sum = 0.0; let mut count = 0;
for row in calamine.streaming_column_read(col) {
    if let Value::Number(n) = row {
        sum += n;
        count += 1;
    }
}
prelude.aggregates.insert((AggKind::Sum, col), sum.into());
prelude.aggregates.insert((AggKind::Count, col), (count as f64).into());
```

Multiple aggregates over the same column share the same pass (fold state).

### Lookup index pre-pass

For each lookup sheet, load it fully. Build `HashMap<Key, RowIdx>` keyed by whichever column is actually looked up. "Lookup sheet" here means a sheet we've classified as cacheable: finite, fits in memory, no formulas that depend on main-sheet rows. Sizes in the wild range from a handful of rows (region-code tables) to hundreds of thousands (product masters) — all supported as long as RAM allows.

Key types: single-value (`VLOOKUP(K, ...)`). Multi-key lookups use `&` at the call site (`VLOOKUP(A & B, sheet_with_concat_helper, ...)`), which we treat as a single-value text key. Hashes are type-aware (Excel: `1` and `"1"` are different lookup keys) but case-insensitive for strings (matches Excel default).

## Row pass — the tight loop

```rust
let writer = XlsxWriter::constant_memory(output_path)?;
let mut reader = calamine.row_iter(main_sheet)?;

// Write header row.
writer.write_row(reader.next_row()?.unwrap())?;

while let Some(raw_row) = reader.next_row()? {
    let mut row = raw_row; // Vec<Value>, one per column
    for &fcol in &topo_order {
        let ast = &asts[fcol];
        row[fcol] = evaluator.eval(ast, &row, &prelude)?;
    }
    writer.write_row(&row)?;
}

writer.close()?;
```

Key properties:
- `row` is a reused `Vec<Value>`; no per-row heap churn in the hot path.
- `evaluator.eval` takes `&prelude` (immutable, shared across rows and threads).
- `topo_order` is computed once during prelude.

### Intra-row topo order

Formula columns may reference each other. Example: column `Net Value = Deal Value * (1 - Discount)`, column `Profit = Net Value - Cost`. `Profit` depends on `Net Value`.

Build a tiny intra-row DAG over formula columns. Topo-sort. Self-edges (a column referencing itself) are filtered out and the column is marked as self-referential for iterative evaluation. If a cross-column cycle exists (A depends on B, B depends on A), it's refused during classification.

## Writer buffering

`rust_xlsxwriter` in `constant_memory` mode flushes each row to a temp file on disk. Flat ~20 MB RAM regardless of row count. Output is finalised (zip, write xlsx structure) on `writer.close()`.

For formula cells we write both the formula text and the cached value (`Formula::new("=A1+B1").set_result("5")`) so Excel doesn't re-evaluate on open — unless the user opts into "store formula only" (future flag).

## Multi-sheet workbooks

A workbook may have several sheets that each want streaming evaluation. We process them independently — one prelude + one stream per main sheet — and write each to the output. Lookup sheets are loaded once and shared.

Cross-main-sheet references (sheet A's formula referencing a cell on sheet B that is also a formula) are **supported only** if sheet B is either purely static data or a finite lookup sheet. Chains of main-sheet evaluation are not supported.

## Refusal policy

When classification refuses a formula, the error message names:
1. The sheet.
2. The cell address.
3. The formula text.
4. The specific sub-expression that disqualified it.
5. A link to a doc section explaining why that shape is unsupported.

Example:
```
xlstream: cannot evaluate formula at Sheet1!F7:
    =OFFSET($A$1, MATCH(B7, $A$1:$A$1000, 0), 0)
  OFFSET is not supported because it resolves cell addresses at runtime.
  See: https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#why-offset-and-indirect-are-unsupported
```

## Things we deliberately do not optimise in v0.1

- Lazy evaluation of function arguments (we eagerly evaluate both branches of `IF`). Simpler, easier to test. Can revisit.
- Constant folding at parse time. `2 + 3` in a formula evaluates to `5` every row. Trivial perf cost; not worth the complexity.
- SIMD. Rust's compiler auto-vectorises enough for arithmetic; explicit SIMD is a future consideration.

## Things we *do* optimise in v0.1

- **No allocation in the row loop** for the common case. Row vector reused. Value enum kept small (~16 bytes).
- **Formula AST cached by text.** Identical formula text across 400k rows parses once.
- **Hash lookup indexes.** O(1) VLOOKUP/XLOOKUP exact match vs. O(n) linear scan in formualizer.
- **Rayon row sharding.** Near-linear speedup for pure-row-local formulas.
- **rust_xlsxwriter constant memory.** Flat write memory.
