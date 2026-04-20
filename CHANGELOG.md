# Changelog

All notable changes to xlstream. Format: [Keep a Changelog](https://keepachangelog.com/).
Semver.

## [Unreleased]

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
- No named ranges or table references
- No external workbook references
- No cell formatting preservation
- xlsx only (no xls, xlsb, ods)
- `-2^2 = -4` (standard math precedence, not Excel's)
- RSS dominated by I/O libraries (~10 MB evaluator, rest is calamine + rust_xlsxwriter)
- IRR/NPV: flat cell-ref args only, no range expansion
- NETWORKDAYS/WORKDAY: holidays arg ignored
