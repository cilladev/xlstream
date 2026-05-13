# xlstream — Project Brief

## Vision (one sentence)

Build the fastest, leanest Excel formula evaluation engine in Python's reach, by committing to a streaming architecture that trades feature breadth for memory and speed on the 90% of workloads that are row-local with shared lookup sheets that fit in memory.

## Problem

Data scientists, analysts, and finance teams generate Excel workbooks programmatically, often with hundreds of thousands of rows and a handful of formula columns. Existing Python-callable evaluators do not scale:

Measured 2026-05-13 on 100k rows × 50 cols (20 data + 30 formula). Intel i9-10910, 128 GB RAM.

| Engine | Version | Wall-clock | Peak RSS | Architecture |
|---|---|---|---|---|
| **xlstream (1 worker)** | 0.2.1 | **26.5s** | **643 MB** | Streaming (2-pass) |
| **xlstream (4 workers)** | 0.2.1 | **23.0s** | **681 MB** | Streaming (2-pass) |
| LibreOffice | 26.2 | 31.9s | 2,081 MB | Graph |
| Excel | 16.108.2 | ~99s | ~430 MB | Graph (20 threads) |
| formualizer | 0.5.6 | 2h 8m | 11,322 MB | Full dependency graph |
| `pycel`, `xlcalculator`, `formulas` | — | Hours → days | OOM at scale | Graph + interpreter |
| `xlwings` | — | Slow | — | Drives real Excel via COM |

None of these is satisfying for an automated pipeline that processes large workbooks repeatedly. xlstream is **3× less memory and 1.4× faster than LibreOffice**, and **290× faster than formualizer**.

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
