# Phase 9 — Strings, dates, math

**Goal:** fill out the MVP function coverage — string manipulation, date arithmetic, extra math.

**Estimated effort:** 3–4 days (parallelisable into 3 sub-phases).

**Prerequisites:** Phase 5 (for coercions).

**Output:** ~40 more builtin functions. Full MVP function surface complete.

## Checklist — strings

In `xlstream-eval/src/builtins/string.rs`:

- [x] `LEFT(text, n?)` — leftmost n chars; n defaults to 1.
- [x] `RIGHT(text, n?)`.
- [x] `MID(text, start, n)` — start is 1-based.
- [x] `LEN(text)`.
- [x] `UPPER(text)`, `LOWER(text)`, `PROPER(text)`.
- [x] `TRIM(text)` — removes leading/trailing and collapses internal runs of spaces.
- [x] `CLEAN(text)` — strips non-printable.
- [x] `CONCAT(a, b, ...)`, `CONCATENATE(a, b, ...)` — joins.
- [x] `TEXTJOIN(delim, ignore_empty, ...)`.
- [x] `FIND(needle, haystack, start?)` — 1-based, case-sensitive.
- [x] `SEARCH(needle, haystack, start?)` — case-insensitive, supports `?` `*` wildcards.
- [x] `SUBSTITUTE(text, old, new, which?)`.
- [x] `REPLACE(text, start, n, new)`.
- [x] `TEXT(value, format)` — numeric/date to formatted string. **Complex**; v0.1 supports common formats: `"0.00"`, `"#,##0"`, `"yyyy-mm-dd"`, `"0.00%"`. Doc the subset.
- [x] `VALUE(text)` — text-to-number.
- [x] `EXACT(a, b)` — case-sensitive equality.

### String tests

Each function ≥ 5 tests. Particular care for:
- [x] `LEFT("", 5)` → `""` not error.
- [x] `MID("abc", 5, 2)` → `""`.
- [x] `FIND` returns `#VALUE!` on not-found.
- [x] `SEARCH` returns `#VALUE!` on not-found.
- [x] `SUBSTITUTE` with `which=2` replaces only the 2nd occurrence.
- [x] Text case-insensitive comparison for `SEARCH`, case-sensitive for `FIND`.

## Checklist — dates

In `xlstream-eval/src/builtins/date.rs`. See also `xlstream-core::ExcelDate`.

- [x] `ExcelDate::from_serial(f64) -> ExcelDate` — including the 1900-leap-bug preservation.
- [x] `ExcelDate::year_month_day() -> (i32, u32, u32)`.
- [x] `ExcelDate::from_ymd(y, m, d) -> ExcelDate`.
- [x] `TODAY()` — single-evaluation-per-run. Result stored in `Prelude::volatile` at run start.
- [x] `NOW()` — same.
- [x] `DATE(y, m, d)`.
- [x] `YEAR`, `MONTH`, `DAY`.
- [x] `WEEKDAY(date, type?)`.
- [x] `EDATE(date, months)` — date + n months.
- [x] `EOMONTH(date, months)` — end of month.
- [x] `DATEDIF(start, end, unit)` — `"y"`, `"m"`, `"d"`, `"ym"`, `"yd"`, `"md"`.
- [x] `NETWORKDAYS(start, end, holidays?)`.
- [x] `WORKDAY(start, n, holidays?)`.

### Date tests

- [x] 1900 leap bug: `ExcelDate::from_serial(60).year_month_day() == (1900, 2, 29)`.
- [x] Serial 61 → (1900, 3, 1).
- [x] Serial 1 → (1900, 1, 1).
- [x] `DATE(2026, 13, 1) = DATE(2027, 1, 1)` (Excel rolls over).
- [x] `EOMONTH(2026-01-15, 2) = 2026-03-31`.
- [x] `DATEDIF` each unit.
- [x] `WEEKDAY` each type variant.
- [x] `NETWORKDAYS` skips weekends.

## Checklist — math

In `xlstream-eval/src/builtins/math.rs`:

- [x] `ROUND(x, digits)` — banker's rounding? Excel uses round-half-away-from-zero.
- [x] `ROUNDUP(x, digits)`, `ROUNDDOWN(x, digits)`.
- [x] `INT(x)` — floor toward negative infinity (yes, Excel does this for INT).
- [x] `MOD(x, y)` — Excel's sign-of-divisor convention.
- [x] `ABS`, `SIGN`, `SQRT`, `POWER`.
- [x] `CEILING(x, significance)`, `FLOOR(x, significance)`.
- [x] `LN`, `LOG` (log10 by default in Excel), `LOG10`, `LOG(x, base)`, `EXP`.
- [x] `SIN`, `COS`, `TAN`, `ASIN`, `ACOS`, `ATAN`, `ATAN2`.
- [x] `PI()`.
- [ ] `RAND()` — with the deterministic-per-run rule via prelude volatiles. (unsupported: volatile functions, deterministic seeding deferred)
- [ ] `RANDBETWEEN(low, high)`. (unsupported: volatile functions, deterministic seeding deferred)

### Math tests

- [x] `ROUND(0.5, 0) = 1` (not `0` — Excel is round-half-away-from-zero, not banker's).
- [x] `ROUND(-0.5, 0) = -1`.
- [x] `INT(-1.5) = -2` (floor).
- [x] `MOD(-3, 2) = 1` (Excel sign-of-divisor).
- [x] `POWER(-2, 0.5)` → `#NUM!`.
- [x] `LN(0)` → `#NUM!`.

## Checklist — info

In `xlstream-eval/src/builtins/info.rs`:

- [x] `ISBLANK`, `ISNUMBER`, `ISTEXT`, `ISERROR`, `ISNA`, `ISLOGICAL`, `ISNONTEXT`.
- [x] `ISREF` — always FALSE for our use (we've resolved).
- [x] `NA()` — returns `Value::Error(CellError::Na)`.
- [x] `TYPE(x)` — Excel-style type code.

## Checklist — financial (v0.1 minimum)

In `xlstream-eval/src/builtins/financial.rs`:

- [x] `PMT(rate, nper, pv, fv?, type?)`.
- [x] `PV(rate, nper, pmt, fv?, type?)`.
- [x] `FV(rate, nper, pmt, pv?, type?)`.
- [x] `NPV(rate, v1, v2, ...)`.
- [x] `IRR(values, guess?)` — iterative, stable up to 100 iterations.
- [x] `RATE(nper, pmt, pv, fv?, type?, guess?)`.

### Financial tests

- [x] PMT / PV / FV / NPV match Excel for standard textbook examples.
- [x] IRR converges on a canonical cashflow.

## Done when

All 80+ builtin functions from the MVP list (`docs/research/prior-art.md`) work. Each has ≥ 5 tests. The 1900 leap bug test passes. Excel-precision tests pass.
