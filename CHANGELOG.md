# Changelog

All notable changes to xlstream. Format: [Keep a Changelog](https://keepachangelog.com/).
Semver.

## [Unreleased]

### Added
- MINIFS and MAXIFS conditional aggregate functions
- SUMPRODUCT: sum of element-wise products of bounded ranges; single-array degenerate case sums the array; booleans coerce to 1/0

### Changed
- Test infrastructure: replaced monolithic golden-file regression and end-to-end tests with per-function conformance fixtures (LibreOffice as oracle, one xlsx per function)

### Fixed
- Formulas on sheets not referenced by the main sheet are now evaluated instead of producing None (#42)
- Mixed-column formulas: columns where later rows have structurally different formulas (e.g., cross-sheet ref vs same-sheet ref) now store per-row AST overrides instead of silently using the first formula's AST for all rows

## [0.2.0] - 2026-04-21

### Added
- Named range resolution: formulas using workbook-level defined names (`SalesData`, `TaxRate`, etc.) now resolve at classification time via calamine's `defined_names()` API
- Unknown named ranges return `#NAME?` cell error instead of aborting evaluation (matches Excel behavior)
- End-to-end test module consolidation (`tests/end_to_end/` per-feature structure)

## [0.1.1] - 2026-04-20

### Added
- Golden-file regression test suite comparing xlstream output against Excel-cached values (117 formula surfaces)
- Per-issue regression test framework (`regression_per_issue.rs`) with ignored-until-fixed workflow
- `EXCEL_MAX_ROWS` (1,048,576) and `EXCEL_MAX_COLS` (16,384) constants in xlstream-core

### Fixed
- SUMIF/COUNTIF/AVERAGEIF with row-local criteria (e.g., `SUMIF(A:A,A2,B:B)`) no longer rejected at classification with `NonStaticCriteria`; pre-computes grouped aggregate maps in prelude, O(1) lookup per row
- Cross-sheet conditional aggregates (e.g., `SUMIF(RefData!A:A,"EMEA",RefData!B:B)`) now correctly scan non-main sheets during prelude; previously returned `#VALUE!` or 0
- `PRODUCT(2,3,4)` with literal/cell-ref args no longer returns `#VALUE!`; aggregate functions with all-scalar args now classify as RowLocal and evaluate inline
- `COUNTA(H:H)` includes header row in prelude scan; previously undercounted by 1
- `COUNTBLANK(H:H)` uses `EXCEL_MAX_ROWS - COUNTA` for whole-column ranges to match Excel's 1M+ row grid semantics
- `FLOOR(-2.3, 1)` returns -3 instead of `#NUM!`; removed legacy sign-mismatch check to match modern Excel (2010+)
- `ISREF(A2)` returns TRUE; moved to lazy dispatch to inspect raw AST node before evaluation

### Changed
- `classify_aggregate` returns `RowLocal` when all args are scalar (no column ranges); generalizes to SUM/COUNT/MIN/MAX with literal args
- Renamed test files: `regression.rs` → `regression_base.rs` (golden-file), `regression_base.rs` → `regression_per_issue.rs` (per-issue)

### Known limitations
- Empty string cells (e.g., `MID("x",2,3)` → `""`) written as blank by `rust_xlsxwriter` (upstream library discards empty strings); downstream readers see `Empty` instead of `""`

## [0.1.0] - 2026-04-20

### Added

**Core engine**
- Streaming Excel formula evaluation engine (Rust core)
- Two-pass architecture: prelude (aggregates + lookup indexes) then row stream
- Intra-row topological sort for formula column dependencies
- 104 Excel-compatible functions across 8 categories (see docs/functions.md)
- 13 operators: arithmetic, comparison, concatenation, percent
- Prelude-computed whole-column aggregates (SUM, AVERAGE, COUNT, MIN, MAX, PRODUCT, MEDIAN)
- Conditional aggregates (SUMIF/COUNTIF/AVERAGEIF and multi-criteria *IFS variants)
- Hash-indexed lookups (VLOOKUP, HLOOKUP, XLOOKUP) with exact, approximate, and wildcard match
- Cross-sheet cell references to pre-loaded lookup sheets
- Lookup sheet formula evaluation (helper columns like `=A2&"|"&B2`)
- Excel-accurate decimal rounding via rust_decimal
- Full ECMA-376 TEXT format codes via ssfmt
- Row-parallel evaluation via rayon (auto-detect cores or explicit worker count)

**Python bindings**
- `xlstream.evaluate()` Python API with GIL release
- Custom exception hierarchy: XlStreamError, UnsupportedFormula, FormulaParseError, ClassificationError, CircularReferenceError
- Type stubs (_xlstream.pyi) for IDE support
- abi3-py39 wheels (Python 3.9+)

**CLI**
- `xlstream evaluate` with `--workers` and `--verbose` flags
- `xlstream classify` for formula inspection

**Benchmarks**
- Criterion benchmark harness with 3-tier fixture generator (10k/100k/1M rows)
- Python API benchmark

### Known limitations
- No OFFSET, INDIRECT, FILTER, UNIQUE, SORT, LAMBDA, LET (breaks streaming)
- No RAND/RANDBETWEEN (volatile functions, deterministic seeding deferred)
- No SUMPRODUCT, MINIFS, MAXIFS (v0.2)
- No table references (named ranges supported since v0.2.0)
- No external workbook references
- No cell formatting preservation
- xlsx only (no xls, xlsb, ods)
- `-2^2 = -4` (standard math precedence, not Excel's)
- RSS dominated by I/O libraries (~10 MB evaluator, rest is calamine + rust_xlsxwriter)
- IRR/NPV: flat cell-ref args only, no range expansion
- NETWORKDAYS/WORKDAY: holidays arg ignored
