# Changelog

All notable changes to xlstream. Format: [Keep a Changelog](https://keepachangelog.com/).
Semver.

## [Unreleased]

### Changed
- Centralize formula registration — single registry replaces 9 scattered function-name registration sites. `classify()`, `rewrite()`, and `collect_lookup_keys()` now take a `fn_lookup` callback. Dispatch uses `registry::dispatch()` instead of 200-arm match.

## [0.3.0] - 2026-05-19

### Added

**Statistical functions (30)**
- STDEV.S, STDEV.P, VAR.S, VAR.P statistical functions
- SKEW, SKEW.P, KURT higher-order moment statistics
- AVEDEV average absolute deviation
- MODE.SNGL most frequent value (tie-broken by first occurrence)
- PERCENTILE.INC, PERCENTILE.EXC, QUARTILE.INC, QUARTILE.EXC
- LARGE, SMALL k-th largest/smallest value
- RANK.EQ, RANK.AVG rank functions
- EXPON.DIST exponential distribution (PDF and CDF)
- POISSON.DIST Poisson probability distribution (PMF and CDF)
- T.DIST, T.DIST.RT, T.DIST.2T, T.INV, T.INV.2T Student's t-distribution functions
- BINOM.DIST, BINOM.INV binomial distribution PMF/CDF and inverse
- NORM.DIST, NORM.INV normal distribution (CDF, PDF, inverse CDF)
- NORM.S.DIST, NORM.S.INV standard normal distribution and inverse
- CORREL Pearson correlation coefficient
- COVARIANCE.P, COVARIANCE.S covariance (population and sample)
- SLOPE, INTERCEPT, RSQ linear regression statistics
- FORECAST.LINEAR linear prediction via regression

**Engineering functions (26)**
- HEX2DEC, DEC2HEX hexadecimal conversion (40-bit two's complement)
- BIN2DEC, DEC2BIN binary conversion (10-bit two's complement)
- OCT2DEC, DEC2OCT octal conversion (30-bit two's complement)
- HEX2BIN, BIN2HEX, HEX2OCT, OCT2HEX, BIN2OCT, OCT2BIN cross-base conversion
- BASE general base conversion (radix 2-36, non-negative)
- COMPLEX, IMREAL, IMAGINARY complex number creation and extraction
- DELTA, GESTEP Kronecker delta and unit step
- ERF, ERFC, ERF.PRECISE, ERFC.PRECISE error function and complement
- CONVERT unit conversion (~100 base units, SI/binary prefixes, 13 categories)
- BITAND, BITOR, BITXOR, BITLSHIFT, BITRSHIFT bitwise operations (48-bit non-negative integers)

**Math extras (27)**
- ACOSH, ASINH, ATANH inverse hyperbolic functions
- COSH, SINH, TANH hyperbolic functions
- COT, CSC, SEC, COTH, CSCH, SECH reciprocal trig/hyperbolic functions
- DEGREES, RADIANS angle conversion functions
- FACT, FACTDOUBLE factorial functions
- PERMUT, PERMUTATIONA permutation functions
- COMBIN, COMBINA combination functions
- EVEN, ODD round to next even/odd integer (away from zero)
- TRUNC truncation toward zero (1-2 args, Decimal-based)
- MROUND round to nearest multiple
- CEILING.MATH, FLOOR.MATH rounding with mode parameter
- CEILING.PRECISE, FLOOR.PRECISE, ISO.CEILING direction-fixed rounding variants
- GCD, LCM greatest common divisor / least common multiple (variadic)
- ROMAN, ARABIC Roman numeral conversion (forms 0-4)

**Infrastructure**
- ROW, COLUMN, ROWS, COLUMNS positional metadata functions
- SUBTOTAL multi-mode aggregate (function_num 1-11, 101-111)
- AGGREGATE extended multi-mode aggregate (function_num 1-13, options 0-7)

### Fixed
- Cross-sheet cell refs (e.g. `=EVEN(Sheet2!A2)`) silently returned wrong values from the main sheet (#136)
- Cross-sheet simple aggregates (`SUM(Sheet2!A:A)`) read from the main sheet instead of the referenced sheet (#137)
- Bounded aggregate ranges (`SUM(A2:A10)`) ignored row bounds and summed the whole column (#138)
- Interpreter fallback for unloaded sheets changed from silent wrong value to `#REF!` (defense-in-depth)
- Self-referencing cross-sheet refs (`=Sheet1!A1` on Sheet1) now resolve from streaming row instead of `#REF!`

## [0.2.1] - 2026-05-11

### Added
- Table reference support (`Table[Column]`, `[@Column]`)
- SUMPRODUCT, MINIFS, MAXIFS functions
- Self-referential formula support via iterative calculation
- Formula preservation: output includes `<f>` and `<v>` by default; `--values-only` for static output
- Iterative calculation controls: `--max-iterations`, `--max-change`, `--no-iterative-calc`

### Fixed
- Formulas on secondary sheets now evaluated correctly (#42)
- Mixed-column formulas with per-row variations now handled

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
