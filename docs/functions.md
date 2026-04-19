# Supported functions — canonical list

Authoritative catalogue of every formula function xlstream supports. Pure Excel only. No custom extensions.

## Legend

| Column | Meaning |
|---|---|
| **Tier** | `v0.1` = release-gate for the first stable release; `v0.2` = stretch, ships when time allows |
| **Phase** | Which phase doc implements it (see [`phases/README.md`](phases/README.md)) |
| **Status** | `[ ]` not implemented, `[x]` implemented, tested, shipped |

When you land a function, tick the box **here** in the same PR that lands the implementation. This page is the single source of truth.

**v0.1 ship gate:** every `v0.1` row ticked, `cargo test` green, benchmark budget met. `v0.2` rows may ship later.

## Operators

All v0.1. Non-negotiable.

| Symbol | Kind | Name | Tier | Phase | Status |
|---|---|---|---|---|---|
| `+` | binary | add / unary plus | v0.1 | 5 | [ ] |
| `-` | binary / unary | subtract / negate | v0.1 | 5 | [ ] |
| `*` | binary | multiply | v0.1 | 5 | [ ] |
| `/` | binary | divide | v0.1 | 5 | [ ] |
| `^` | binary | exponent | v0.1 | 5 | [ ] |
| `&` | binary | text concatenation | v0.1 | 5 | [ ] |
| `%` | postfix unary | percent (divide by 100) | v0.1 | 5 | [ ] |
| `=` | binary | equality | v0.1 | 5 | [ ] |
| `<>` | binary | inequality | v0.1 | 5 | [ ] |
| `<` | binary | less-than | v0.1 | 5 | [ ] |
| `>` | binary | greater-than | v0.1 | 5 | [ ] |
| `<=` | binary | less-or-equal | v0.1 | 5 | [ ] |
| `>=` | binary | greater-or-equal | v0.1 | 5 | [ ] |

## Logical (Phase 6)

All v0.1. Fundamental — nothing useful without these.

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `TRUE` | `()` | bool literal | v0.1 | [x] |
| `FALSE` | `()` | bool literal | v0.1 | [x] |
| `IF` | `(cond, then, else?)` | short-circuit | v0.1 | [x] |
| `IFS` | `(cond1, val1, cond2, val2, ...)` | first match wins; no match → `#N/A` | v0.1 | [x] |
| `SWITCH` | `(expr, val1, result1, ..., default?)` | expr evaluated once | v0.1 | [x] |
| `IFERROR` | `(expr, fallback)` | catches any `CellError` | v0.1 | [x] |
| `IFNA` | `(expr, fallback)` | catches only `#N/A` | v0.1 | [x] |
| `AND` | `(a, b, ...)` | short-circuit on false | v0.1 | [x] |
| `OR` | `(a, b, ...)` | short-circuit on true | v0.1 | [x] |
| `NOT` | `(x)` | boolean invert | v0.1 | [x] |
| `XOR` | `(a, b, ...)` | parity (odd-true) | v0.1 | [x] |

## Aggregates / Statistics (Phase 7)

Pre-computed at prelude when the range is a whole column or bounded range in a non-streaming sheet.

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `SUM` | `(range, ...)` | numeric sum | v0.1 | [x] |
| `SUMIF` | `(range, criteria, sum_range?)` | conditional sum | v0.1 | [x] |
| `SUMIFS` | `(sum_range, (crit_range, crit)+)` | multi-criteria sum | v0.1 | [ ] |
| `SUMPRODUCT` | `(range1, range2, ...)` | sum of element-wise products | v0.2 | [ ] |
| `PRODUCT` | `(range, ...)` | numeric product | v0.1 | [x] |
| `COUNT` | `(range, ...)` | count of numerics | v0.1 | [x] |
| `COUNTA` | `(range, ...)` | count of non-empty | v0.1 | [x] |
| `COUNTBLANK` | `(range)` | count of empty | v0.1 | [x] |
| `COUNTIF` | `(range, criteria)` | conditional count | v0.1 | [x] |
| `COUNTIFS` | `((crit_range, crit)+)` | multi-criteria count | v0.1 | [ ] |
| `AVERAGE` | `(range, ...)` | mean; empty -> `#DIV/0!` | v0.1 | [x] |
| `AVERAGEIF` | `(range, criteria, avg_range?)` | conditional mean | v0.1 | [x] |
| `AVERAGEIFS` | `(avg_range, (crit_range, crit)+)` | multi-criteria mean | v0.1 | [ ] |
| `MIN` | `(range, ...)` | minimum | v0.1 | [x] |
| `MAX` | `(range, ...)` | maximum | v0.1 | [x] |
| `MINIFS` | `(min_range, (crit_range, crit)+)` | conditional min | v0.2 | [ ] |
| `MAXIFS` | `(max_range, (crit_range, crit)+)` | conditional max | v0.2 | [ ] |
| `MEDIAN` | `(range, ...)` | middle value of sorted numerics | v0.1 | [x] |

## Lookup (Phase 8)

All v0.1. Hash-indexed exact match, binary-search approx, wildcard fallback. Single-key; multi-key via pre-computed helper column in the lookup sheet.

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `VLOOKUP` | `(lookup, table, col_index, approx?)` | exact (`FALSE`) or approx (`TRUE`/default) | v0.1 | [ ] |
| `HLOOKUP` | `(lookup, table, row_index, approx?)` | row-oriented VLOOKUP | v0.1 | [ ] |
| `XLOOKUP` | `(lookup, lookup_arr, return_arr, not_found?, match_mode?, search_mode?)` | modern lookup | v0.1 | [ ] |
| `MATCH` | `(lookup, lookup_arr, match_type?)` | returns index | v0.1 | [ ] |
| `XMATCH` | `(lookup, lookup_arr, match_mode?, search_mode?)` | modern MATCH | v0.1 | [ ] |
| `INDEX` | `(array, row, col?)` | array access; no index build | v0.1 | [ ] |
| `CHOOSE` | `(index, val1, val2, ...)` | argument pick | v0.1 | [ ] |

## Text (Phase 9)

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `LEFT` | `(text, n?)` | n defaults to 1 | v0.1 | [ ] |
| `RIGHT` | `(text, n?)` | n defaults to 1 | v0.1 | [ ] |
| `MID` | `(text, start, n)` | start is 1-based | v0.1 | [ ] |
| `LEN` | `(text)` | character count | v0.1 | [ ] |
| `UPPER` | `(text)` | uppercase | v0.1 | [ ] |
| `LOWER` | `(text)` | lowercase | v0.1 | [ ] |
| `PROPER` | `(text)` | title case | v0.2 | [ ] |
| `TRIM` | `(text)` | strips leading/trailing + collapses runs of spaces | v0.1 | [ ] |
| `CLEAN` | `(text)` | strips non-printable | v0.2 | [ ] |
| `CONCAT` | `(a, b, ...)` | joins (modern) | v0.1 | [ ] |
| `CONCATENATE` | `(a, b, ...)` | joins (legacy alias) | v0.1 | [ ] |
| `TEXTJOIN` | `(delim, ignore_empty, a, b, ...)` | delimited join | v0.1 | [ ] |
| `FIND` | `(needle, haystack, start?)` | 1-based, case-sensitive | v0.1 | [ ] |
| `SEARCH` | `(needle, haystack, start?)` | case-insensitive, wildcard-enabled | v0.1 | [ ] |
| `SUBSTITUTE` | `(text, old, new, which?)` | replace by match | v0.1 | [ ] |
| `REPLACE` | `(text, start, n, new)` | replace by position | v0.1 | [ ] |
| `TEXT` | `(value, format)` | numeric/date to formatted string (subset of formats) | v0.1 | [ ] |
| `VALUE` | `(text)` | text to number | v0.1 | [ ] |
| `EXACT` | `(a, b)` | case-sensitive equality | v0.2 | [ ] |

## Math (Phase 9)

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `ROUND` | `(x, digits)` | round half away from zero | v0.1 | [ ] |
| `ROUNDUP` | `(x, digits)` | toward +∞ | v0.1 | [ ] |
| `ROUNDDOWN` | `(x, digits)` | toward 0 | v0.1 | [ ] |
| `INT` | `(x)` | floor toward -∞ | v0.1 | [ ] |
| `MOD` | `(x, y)` | sign of divisor | v0.1 | [ ] |
| `ABS` | `(x)` | absolute value | v0.1 | [ ] |
| `SIGN` | `(x)` | -1, 0, or 1 | v0.1 | [ ] |
| `SQRT` | `(x)` | square root; negative → `#NUM!` | v0.1 | [ ] |
| `POWER` | `(base, exp)` | same as `^` | v0.1 | [ ] |
| `CEILING` | `(x, significance)` | round up to multiple | v0.2 | [ ] |
| `FLOOR` | `(x, significance)` | round down to multiple | v0.2 | [ ] |
| `LN` | `(x)` | natural log | v0.2 | [ ] |
| `LOG` | `(x, base?)` | base defaults to 10 | v0.2 | [ ] |
| `LOG10` | `(x)` | base-10 log | v0.2 | [ ] |
| `EXP` | `(x)` | e^x | v0.2 | [ ] |
| `SIN` | `(x)` | radians | v0.2 | [ ] |
| `COS` | `(x)` | radians | v0.2 | [ ] |
| `TAN` | `(x)` | radians | v0.2 | [ ] |
| `ASIN` | `(x)` | returns radians | v0.2 | [ ] |
| `ACOS` | `(x)` | returns radians | v0.2 | [ ] |
| `ATAN` | `(x)` | returns radians | v0.2 | [ ] |
| `ATAN2` | `(y, x)` | note Excel arg order (y before x) | v0.2 | [ ] |
| `PI` | `()` | constant π | v0.1 | [ ] |
| `RAND` | `()` | single-evaluation-per-run (deterministic with seed) | v0.1 | [ ] |
| `RANDBETWEEN` | `(low, high)` | single-evaluation-per-run | v0.1 | [ ] |

## Date / Time (Phase 9)

Excel 1900-based serial dates. The 1900-02-29 leap bug is preserved for compatibility.

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `TODAY` | `()` | date; evaluated once per run | v0.1 | [ ] |
| `NOW` | `()` | date+time; evaluated once per run | v0.1 | [ ] |
| `DATE` | `(y, m, d)` | rolls over (`DATE(2026,13,1) = DATE(2027,1,1)`) | v0.1 | [ ] |
| `YEAR` | `(date)` | | v0.1 | [ ] |
| `MONTH` | `(date)` | | v0.1 | [ ] |
| `DAY` | `(date)` | | v0.1 | [ ] |
| `WEEKDAY` | `(date, type?)` | type = 1/2/3 (Excel variants) | v0.1 | [ ] |
| `EDATE` | `(date, months)` | same day, N months later | v0.1 | [ ] |
| `EOMONTH` | `(date, months)` | last day of the resulting month | v0.1 | [ ] |
| `DATEDIF` | `(start, end, unit)` | `"y"`, `"m"`, `"d"`, `"ym"`, `"yd"`, `"md"` | v0.1 | [ ] |
| `NETWORKDAYS` | `(start, end, holidays?)` | skips weekends | v0.2 | [ ] |
| `WORKDAY` | `(start, days, holidays?)` | start + N working days | v0.2 | [ ] |

## Info / type (Phase 9)

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `ISBLANK` | `(x)` | true iff `Empty` | v0.1 | [ ] |
| `ISNUMBER` | `(x)` | | v0.1 | [ ] |
| `ISTEXT` | `(x)` | | v0.1 | [ ] |
| `ISLOGICAL` | `(x)` | | v0.2 | [ ] |
| `ISNONTEXT` | `(x)` | inverse of ISTEXT | v0.2 | [ ] |
| `ISERROR` | `(x)` | any `CellError` | v0.1 | [ ] |
| `ISNA` | `(x)` | `#N/A` only | v0.1 | [ ] |
| `ISREF` | `(x)` | always `FALSE` in our model | v0.2 | [ ] |
| `NA` | `()` | returns `#N/A` | v0.1 | [ ] |
| `TYPE` | `(x)` | Excel-style type code | v0.2 | [ ] |

## Financial (Phase 9)

| Function | Signature | Notes | Tier | Status |
|---|---|---|---|---|
| `PMT` | `(rate, nper, pv, fv?, type?)` | loan payment | v0.1 | [ ] |
| `PV` | `(rate, nper, pmt, fv?, type?)` | present value | v0.1 | [ ] |
| `FV` | `(rate, nper, pmt, pv?, type?)` | future value | v0.1 | [ ] |
| `NPV` | `(rate, v1, v2, ...)` | net present value | v0.1 | [ ] |
| `IRR` | `(values, guess?)` | internal rate of return; iterative | v0.2 | [ ] |
| `RATE` | `(nper, pmt, pv, fv?, type?, guess?)` | interest rate; iterative | v0.2 | [ ] |

## Explicitly NOT supported

These functions are parsed but **refused at classification time** with a `ClassificationError` / `UnsupportedFormula`:

- **`OFFSET`, `INDIRECT`** — resolve addresses at runtime; incompatible with streaming.
- **`FILTER`, `UNIQUE`, `SORT`, `SORTBY`, `SEQUENCE`, `RANDARRAY`** — dynamic arrays; need spill semantics.
- **`LAMBDA`, `LET`** — user-defined functions; deferred.
- **`HYPERLINK`** (as a function returning a clickable URL), **`WEBSERVICE`**, **`ENCODEURL`** — network / side-effecting.
- **`CUBE*`** family — OLAP functions.
- **Engineering** (BESSEL, HEX2BIN, etc.) — deferred; add if users ask.
- **Database** (DSUM, DGET, etc.) — deferred.

See [`architecture/streaming-model.md`](architecture/streaming-model.md) for *why* each is refused.

## Tallies

### v0.1 ship gate — 81 functions + 13 operators = 94 surfaces

| Category | v0.1 count |
|---|---|
| Operators | 13 |
| Logical | 11 |
| Aggregates | 15 |
| Lookup | 7 |
| Text | 16 |
| Math | 12 |
| Date/Time | 10 |
| Info | 6 |
| Financial | 4 |

### v0.2 stretch — 27 additional functions

| Category | v0.2 adds |
|---|---|
| Aggregates | SUMPRODUCT, MINIFS, MAXIFS |
| Text | PROPER, CLEAN, EXACT |
| Math | CEILING, FLOOR, LN, LOG, LOG10, EXP, SIN, COS, TAN, ASIN, ACOS, ATAN, ATAN2 |
| Date/Time | NETWORKDAYS, WORKDAY |
| Info | ISLOGICAL, ISNONTEXT, ISREF, TYPE |
| Financial | IRR, RATE |

### Grand total — 108 functions + 13 operators = 121 surfaces

## How to add a new function

1. Pick a home — category in this file, matching phase doc.
2. Implement in the right `xlstream-eval/src/builtins/*.rs` module.
3. Register in `BUILTIN_REGISTRY` (or `STATEFUL_BUILTINS`).
4. Write ≥ 5 unit tests (happy, empty, error-propagation, coercion, edge case).
5. Tick the box here in the same PR.
6. Update `CHANGELOG.md`.

## How to promote a v0.2 function to v0.1

Open an issue with:
- The function name.
- Why it's needed for v0.1 release (real use case, not "would be nice").
- Estimate of implementation + test effort.

Decision rule: we promote only if leaving it out would break a common workbook shape the release is meant to handle.

## How to request a new function

Open a GitHub issue with:
- The function name and Excel signature.
- A realistic use case.
- Why a workaround using existing functions isn't sufficient.

We evaluate against the project's "pure Excel only" rule and the streaming invariant.
