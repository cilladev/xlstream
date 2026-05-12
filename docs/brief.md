# xlstream — Project Brief

## Vision (one sentence)

Build the fastest, leanest Excel formula evaluation engine in Python's reach, by committing to a streaming architecture that trades feature breadth for memory and speed on the 90% of workloads that are row-local with shared lookup sheets that fit in memory.

## Problem

Data scientists, analysts, and finance teams generate Excel workbooks programmatically, often with hundreds of thousands of rows and a handful of formula columns. Existing Python-callable evaluators do not scale:

| Engine | Approach | Measured on 700k × 20 reference workload |
|---|---|---|
| `formualizer` (Rust, graph-based) | Full dependency DAG | **3.3 GB RSS, 5h 40m wall-clock** (measured 2026-04-17: Deals 700,001×20 + Thresholds 26×4 + Region Info 6×4) |
| `pycel`, `xlcalculator`, `formulas` (pure Python) | Graph + interpreter | Hours → days; OOM on lookups at scale |
| `xlwings` | Drives real Excel via COM | Requires Excel installed; slow |
| LibreOffice `soffice --headless` | Shells out to LO | 1–3 GB RAM, minutes-to-tens-of-minutes; installs 600 MB of LibreOffice |

None of these is satisfying for an automated pipeline that processes large workbooks repeatedly. xlstream's target on the same workload: **< 250 MB peak RSS, < 3 min wall-clock**. That's roughly **13× less memory and 100× faster** than formualizer.

## Insight

In real business workbooks, formula columns overwhelmingly fall into three shapes:

1. **Row-local**: `=Deal Value * Quantity`, `=IF(Revenue > 10000, "Gold", "Silver")`. Only reference cells on the same row.
2. **Lookup-into-loaded-sheet**: `=VLOOKUP(Region, 'Region Info'!A:C, 2, FALSE)`. Lookup sheets are loaded fully into memory once during prelude and hash-indexed. Size bounded by RAM — tens to tens-of-thousands of rows is routine; a 100k-row lookup sheet still works if RAM allows.
3. **Whole-column aggregate**: `=Deal Value / SUM(Deal Value:Deal Value) * 100`. Aggregates reduce a column to a scalar.

None of these requires a full dependency graph. They can all be evaluated in a two-pass stream:

- **Pass 1 (prelude):** compute whole-column aggregates and hash-index lookup sheets.
- **Pass 2 (stream):** read main sheet one row at a time, evaluate formula cells using current row values + prelude scalars + lookup indexes, write output row.

Peak memory: `file_shared_strings + lookup_sheets + one_row + writer_buffer` — bounded by the sum of input sizes, *not* by main-sheet row count. A 5M-row main sheet with a 10k-row lookup uses the same memory as a 50k-row main sheet with the same lookup.

## Target users

- Data / analytics engineers building ETL pipelines that touch Excel.
- Finance / FP&A teams running batch model recalculation.
- Anyone who has hit the memory wall on `formualizer` or the speed wall on `pycel`/`xlcalculator`.

## Non-target users

- Anyone who needs cross-row circular references or live recalculation. Point them at LibreOffice or Excel. (Self-referential and same-row circular refs are supported via iterative calc.)
- Anyone writing new formulas interactively — use Excel.

## Guiding values

1. **Correctness first.** A fast wrong answer is worthless. Every function lands with Excel-parity tests.
2. **Honesty about scope.** If we can't handle a formula shape, we refuse it with a clear error, never silently fall back to a slow path.
3. **Documentation is part of the feature.** Undocumented public APIs are broken.
4. **Small dependencies, vetted.** Each dependency's cost is scrutinised.
5. **Tests are code.** Reviewed as carefully as library code.
6. **No premature abstraction.** Traits and generics earn their keep with at least two concrete implementations before being introduced.
