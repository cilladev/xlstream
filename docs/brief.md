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

## Scope

### In scope (v0.1)

- `.xlsx` input and output (no `.xls`, `.xlsm`, `.ods` in v0.1).
- Single- and multi-sheet workbooks.
- Pre-computed whole-column aggregates: `SUM`, `COUNT`, `COUNTA`, `AVERAGE`, `MIN`, `MAX`, `SUMIF`, `COUNTIF`, `AVERAGEIF`.
- Lookups via hash-indexed tables loaded into memory at prelude: `VLOOKUP` (exact), `XLOOKUP` (exact), `MATCH` (exact), `HLOOKUP` (exact), `XMATCH`, `INDEX`, `CHOOSE`. Multi-key lookups handled via concatenated keys (`VLOOKUP(A&B, ...)`) into lookup sheets that have pre-computed helper columns — pure Excel idiom.
- Approximate-match lookups via sorted binary search.
- Per-row eval: arithmetic, comparison, `&` concat, `IF`, `IFS`, `AND`, `OR`, `NOT`, `XOR`, `IFERROR`, `IFNA`.
- String functions: `LEFT`, `RIGHT`, `MID`, `LEN`, `UPPER`, `LOWER`, `TRIM`, `CONCAT`, `CONCATENATE`, `FIND`, `SEARCH`, `SUBSTITUTE`, `REPLACE`, `TEXT`.
- Math: `ROUND`, `ROUNDUP`, `ROUNDDOWN`, `INT`, `MOD`, `ABS`, `POWER`, `SQRT`, `SIGN`.
- Dates: Excel serial conversions, `TODAY`, `NOW` (evaluated once per run), `DATE`, `YEAR`, `MONTH`, `DAY`, `WEEKDAY`, `EDATE`, `EOMONTH`, `DATEDIF`.
- Errors: `#DIV/0!`, `#VALUE!`, `#REF!`, `#NAME?`, `#N/A`, `#NUM!`, `#NULL!`, propagated per Excel semantics.
- Python binding: `xlstream.evaluate(input_path, output_path=None)` + streaming API.
- Peak RSS < 250 MB on 700k × 20 reference workload.
- Wall-clock < 3 minutes on same workload, 8-core machine.

### Explicitly out of scope for v0.1

- Circular references / iterative calculation — refused with a clear error.
- Full dynamic-array spills (`FILTER`, `UNIQUE`, `SORT`, `SORTBY`, `SEQUENCE`). `UNIQUE` over a pre-computed column aggregate may be revisited for v0.2.
- Volatile re-evaluation across opens — `TODAY()`, `NOW()`, `RAND()` are evaluated **once per run**.
- Formulas that reference cells from future rows (forward row refs). Refused with a clear error.
- `.xlsm` macro execution.
- Preserving original cell formatting with perfect fidelity — we preserve number formats where cheap, drop custom styles otherwise. Documented trade-off.
- Excel's 1900-leap-year bug — we match Excel's output exactly for compatibility, not the calendar.
- OLE / embedded objects / pivot tables / charts — read-through passthrough only (copied to output without modification where possible).

### May land in v0.2

- `.xlsb` input.
- Streaming `UNIQUE` and `FILTER` with a row-hashing pass.
- User-defined functions registered from Python.
- Incremental re-evaluation (load previously evaluated xlsx, patch one row, write).

## Non-goals

- We are **not** competing with Excel or LibreOffice on feature coverage. Breadth is their advantage; speed and memory are ours.
- We are **not** building a graph-based engine. Every request to "support circular refs" or "arbitrary backward-row refs" will be declined.
- We are **not** building an authoring tool. We don't create formulas; we evaluate them.
- We implement **only pure Excel functions**. No custom extensions (e.g., no `MLOOKUP` or other xlformula-style helpers). Users should use standard Excel idioms — e.g., helper columns in lookup sheets for multi-key joins.

## Success criteria

v0.1 ships when all of the following hold:

1. Reference workload (700k × 20, 10 formula cols including 4 VLOOKUP into hash-indexed lookup sheets) evaluates correctly against Excel-computed ground truth in < 3 min wall-clock and < 250 MB peak RSS on a desktop workstation (10-core Intel i9 or equivalent). For context: formualizer on the same workload takes 5h 40m at 3.3 GB peak.
2. `pip install xlstream` on Linux, macOS, Windows across Python 3.9–3.14 works.
3. Every public Rust API has rustdoc with a doctested example.
4. All built-in functions have unit tests referencing Excel ground truth.
5. `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `pytest` pass in CI.
6. Published to crates.io and PyPI with a changelog and an announcement.

## Target users

- Data / analytics engineers building ETL pipelines that touch Excel.
- Finance / FP&A teams running batch model recalculation.
- Anyone who has hit the memory wall on `formualizer` or the speed wall on `pycel`/`xlcalculator`.

## Non-target users

- Anyone who needs a full spreadsheet engine with circular refs, iterative calc, or live recalculation. Point them at LibreOffice or Excel.
- Anyone writing new formulas interactively — use Excel.

## Guiding values

1. **Correctness first.** A fast wrong answer is worthless. Every function lands with Excel-parity tests.
2. **Honesty about scope.** If we can't handle a formula shape, we refuse it with a clear error, never silently fall back to a slow path.
3. **Documentation is part of the feature.** Undocumented public APIs are broken.
4. **Small dependencies, vetted.** Each dependency's cost is scrutinised.
5. **Tests are code.** Reviewed as carefully as library code.
6. **No premature abstraction.** Traits and generics earn their keep with at least two concrete implementations before being introduced.
