# Phase 9 — Strings, dates, math

**Goal:** fill out the MVP function coverage — string manipulation, date arithmetic, extra math.

**Estimated effort:** 3–4 days (parallelisable into 3 sub-phases).

**Prerequisites:** Phase 5 (for coercions).

**Output:** ~40 more builtin functions. Full MVP function surface complete.

## Checklist — strings

In `xlstream-eval/src/builtins/string.rs`:

- [ ] `LEFT(text, n?)` — leftmost n chars; n defaults to 1.
- [ ] `RIGHT(text, n?)`.
- [ ] `MID(text, start, n)` — start is 1-based.
- [ ] `LEN(text)`.
- [ ] `UPPER(text)`, `LOWER(text)`, `PROPER(text)`.
- [ ] `TRIM(text)` — removes leading/trailing and collapses internal runs of spaces.
- [ ] `CLEAN(text)` — strips non-printable.
- [ ] `CONCAT(a, b, ...)`, `CONCATENATE(a, b, ...)` — joins.
- [ ] `TEXTJOIN(delim, ignore_empty, ...)`.
- [ ] `FIND(needle, haystack, start?)` — 1-based, case-sensitive.
- [ ] `SEARCH(needle, haystack, start?)` — case-insensitive, supports `?` `*` wildcards.
- [ ] `SUBSTITUTE(text, old, new, which?)`.
- [ ] `REPLACE(text, start, n, new)`.
- [ ] `TEXT(value, format)` — numeric/date to formatted string. **Complex**; v0.1 supports common formats: `"0.00"`, `"#,##0"`, `"yyyy-mm-dd"`, `"0.00%"`. Doc the subset.
- [ ] `VALUE(text)` — text-to-number.
- [ ] `EXACT(a, b)` — case-sensitive equality.

### String tests

Each function ≥ 5 tests. Particular care for:
- [ ] `LEFT("", 5)` → `""` not error.
- [ ] `MID("abc", 5, 2)` → `""`.
- [ ] `FIND` returns `#VALUE!` on not-found.
- [ ] `SEARCH` returns `#VALUE!` on not-found.
- [ ] `SUBSTITUTE` with `which=2` replaces only the 2nd occurrence.
- [ ] Text case-insensitive comparison for `SEARCH`, case-sensitive for `FIND`.

## Checklist — dates

In `xlstream-eval/src/builtins/date.rs`. See also `xlstream-core::ExcelDate`.

- [ ] `ExcelDate::from_serial(f64) -> ExcelDate` — including the 1900-leap-bug preservation.
- [ ] `ExcelDate::year_month_day() -> (i32, u32, u32)`.
- [ ] `ExcelDate::from_ymd(y, m, d) -> ExcelDate`.
- [ ] `TODAY()` — single-evaluation-per-run. Result stored in `Prelude::volatile` at run start.
- [ ] `NOW()` — same.
- [ ] `DATE(y, m, d)`.
- [ ] `YEAR`, `MONTH`, `DAY`.
- [ ] `WEEKDAY(date, type?)`.
- [ ] `EDATE(date, months)` — date + n months.
- [ ] `EOMONTH(date, months)` — end of month.
- [ ] `DATEDIF(start, end, unit)` — `"y"`, `"m"`, `"d"`, `"ym"`, `"yd"`, `"md"`.
- [ ] `NETWORKDAYS(start, end, holidays?)`.
- [ ] `WORKDAY(start, n, holidays?)`.

### Date tests

- [ ] 1900 leap bug: `ExcelDate::from_serial(60).year_month_day() == (1900, 2, 29)`.
- [ ] Serial 61 → (1900, 3, 1).
- [ ] Serial 1 → (1900, 1, 1).
- [ ] `DATE(2026, 13, 1) = DATE(2027, 1, 1)` (Excel rolls over).
- [ ] `EOMONTH(2026-01-15, 2) = 2026-03-31`.
- [ ] `DATEDIF` each unit.
- [ ] `WEEKDAY` each type variant.
- [ ] `NETWORKDAYS` skips weekends.

## Checklist — math

In `xlstream-eval/src/builtins/math.rs`:

- [ ] `ROUND(x, digits)` — banker's rounding? Excel uses round-half-away-from-zero.
- [ ] `ROUNDUP(x, digits)`, `ROUNDDOWN(x, digits)`.
- [ ] `INT(x)` — floor toward negative infinity (yes, Excel does this for INT).
- [ ] `MOD(x, y)` — Excel's sign-of-divisor convention.
- [ ] `ABS`, `SIGN`, `SQRT`, `POWER`.
- [ ] `CEILING(x, significance)`, `FLOOR(x, significance)`.
- [ ] `LN`, `LOG` (log10 by default in Excel), `LOG10`, `LOG(x, base)`, `EXP`.
- [ ] `SIN`, `COS`, `TAN`, `ASIN`, `ACOS`, `ATAN`, `ATAN2`.
- [ ] `PI()`.
- [ ] `RAND()` — with the deterministic-per-run rule via prelude volatiles.
- [ ] `RANDBETWEEN(low, high)`.

### Math tests

- [ ] `ROUND(0.5, 0) = 1` (not `0` — Excel is round-half-away-from-zero, not banker's).
- [ ] `ROUND(-0.5, 0) = -1`.
- [ ] `INT(-1.5) = -2` (floor).
- [ ] `MOD(-3, 2) = 1` (Excel sign-of-divisor).
- [ ] `POWER(-2, 0.5)` → `#NUM!`.
- [ ] `LN(0)` → `#NUM!`.

## Checklist — info

In `xlstream-eval/src/builtins/info.rs`:

- [ ] `ISBLANK`, `ISNUMBER`, `ISTEXT`, `ISERROR`, `ISNA`, `ISLOGICAL`, `ISNONTEXT`.
- [ ] `ISREF` — always FALSE for our use (we've resolved).
- [ ] `NA()` — returns `Value::Error(CellError::Na)`.
- [ ] `TYPE(x)` — Excel-style type code.

## Checklist — financial (v0.1 minimum)

In `xlstream-eval/src/builtins/financial.rs`:

- [ ] `PMT(rate, nper, pv, fv?, type?)`.
- [ ] `PV(rate, nper, pmt, fv?, type?)`.
- [ ] `FV(rate, nper, pmt, pv?, type?)`.
- [ ] `NPV(rate, v1, v2, ...)`.
- [ ] `IRR(values, guess?)` — iterative, stable up to 100 iterations.
- [ ] `RATE(nper, pmt, pv, fv?, type?, guess?)`.

### Financial tests

- [ ] PMT / PV / FV / NPV match Excel for standard textbook examples.
- [ ] IRR converges on a canonical cashflow.

## Done when

All 80+ builtin functions from the MVP list (`docs/research/prior-art.md`) work. Each has ≥ 5 tests. The 1900 leap bug test passes. Excel-precision tests pass.
