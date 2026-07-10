# Feature: DATEVALUE (with reusable date text parser)

**Branch:** `feat/datevalue`
**Effort:** ~1 day (parser infra + DATEVALUE)
**Crates:** xlstream-core, xlstream-eval

## What

`DATEVALUE(date_text)` parses a text string representing a date and returns its Excel serial number (integer, no time component).

```
=DATEVALUE("2026-01-15")      -> 46037  (ISO 8601)
=DATEVALUE("15-Jan-2026")     -> 46037  (named month, day first)
=DATEVALUE("Jan-15-2026")     -> 46037  (named month, month first — unambiguous)
=DATEVALUE("January 15, 2026")-> 46037  (comma format — unambiguous)
=DATEVALUE("15 January 2026") -> 46037  (full month name)
=DATEVALUE("Jan-2026")        -> 46023  (month-year, day defaults to 1)
=DATEVALUE("15-Jan")          -> current year serial (no-year, defaults to today's year)
=DATEVALUE("2026-01-15 14:30")-> 46037  (date+time, time stripped)
=DATEVALUE("01/02/2026")      -> #VALUE! (ambiguous numeric — locale-dependent MEANING)
=DATEVALUE("15-Jan-26")       -> #VALUE! (2-digit year — century-window surprise)
=DATEVALUE("not a date")      -> #VALUE!
```

Current behavior: no registry entry — returns `#VALUE!` via unknown-function path.

## Design decision: ambiguity-based acceptance

DATEVALUE in Excel delegates text parsing to the OS regional settings — not the
workbook. `"01/02/2026"` parses as Jan 2 on a US machine, Feb 1 on a UK
machine. The same file recalculated on two machines caches two different
values. There is no locale we can pin to that makes us "match Excel," because
Excel does not match itself across machines.

Locale-dependence comes in two kinds, and only one is dangerous:

- **Locale-dependent meaning** (`01/02/2026`): parsing it can produce a
  silently wrong date. Reject.
- **Locale-dependent acceptance** (`Jan-15-2026`): some Excel locales accept
  it, some reject it, but there is only one possible interpretation — the
  month token is named, so `15` can only be a day and `2026` only a year.
  Accepting it can never produce a wrong date. Accept.

One named-month case IS meaning-ambiguous, verified against Excel
(`datevalue_formats_test.xlsx`, EU-locale save): a single number FOLLOWING
the month is read by Excel as a 2-digit year, not a day — `"January 15"`
cached as Jan-1-**2015**, while `"15-Jan"` cached as day 15 of the current
year. Month-first no-year input is therefore rejected; day-first no-year
stays accepted.

**Rule: all-numeric input must be ISO year-first (`yyyy` then month then
day, `-` or `/` separators in any mix). Named-month input is accepted in
any token order, resolved by value: the named token is the month, a 4-digit
number is the year, a number <= 31 is the day — except a single number <= 31
that follows the month with no year present, which is ambiguous (day vs
Excel's 2-digit year) and rejected. 2-digit years are rejected. A pure time
string returns 0, matching Excel. Everything else is `#VALUE!`.**

This never produces a silently wrong date in any locale. Divergences are loud
(`#VALUE!` where some Excel locale computes a value) and documented.

### Accepted inputs (examples of the rule, not an exhaustive list)

| # | Example | Parsed as | Notes |
|---|---|---|---|
| 1 | `2026-01-15` | 2026-01-15 | ISO 8601. Primary. |
| 2 | `2026/01/15`, `2026-01/15` | 2026-01-15 | ISO slash variant; mixed separators allowed (Excel-verified). |
| 3 | `15-Jan-2026` | 2026-01-15 | Named month, any single separator. |
| 4 | `Jan-15-2026` | 2026-01-15 | Month first — unambiguous, order-free. |
| 5 | `2026-Jan-15` | 2026-01-15 | Year first named — unambiguous. |
| 6 | `January 15, 2026` | 2026-01-15 | Comma treated as separator. |
| 7 | `15 January 2026` | 2026-01-15 | Full month name, spaces. |
| 8 | `Jan-2026` / `January 2026` | 2026-01-01 | Month-year, day defaults to 1. |
| 9 | `15-Jan` | day 15, current year | No year, day BEFORE month only. |
| 10 | `2026-01-15 14:30` | 2026-01-15 | Trailing time parsed then ignored by DATEVALUE. |
| 11 | `14:30:00` | 0 | Pure time string — date part is 0. Matches Excel (verified). |

Month names are case-insensitive: `jan`, `Jan`, `JAN`, `JANUARY` all match.
Leading/trailing whitespace is trimmed.

### Rejected inputs

| Example | Why rejected |
|---|---|
| `1/15/2026`, `15/01/2026` | All-numeric, not year-first — locale-dependent meaning. |
| `15-Jan-26`, `1/15/26` | 2-digit year. Excel accepts named-month 2-digit years via a 29/30 century window; we reject the surprise. Documented divergence. |
| `2026.01.15`, `15.Jan.2026` | Dot separators — rejected by Excel. |
| `2026-01-15T14:30` | T separator — rejected by Excel. |
| `Jan 15`, `January 15` | Month-first no-year — Excel reads the number as a 2-digit year (Jan-1-2015), day-reading would silently differ. Meaning-ambiguous. |
| `Jan` | Month alone — no day or year. |
| `12345` | Serial-as-text — not a date format. |

### Error conditions

- Wrong arity (0 or 2+ args) -> `#VALUE!`
- Non-text arg (number, bool, date, empty) -> `#VALUE!`
- Error in arg -> propagate
- Empty string -> `#VALUE!`
- Unparseable text -> `#VALUE!`
- Day/month out of valid range -> `#VALUE!` (no rollover, unlike `DATE()`)
- Year < 1900 or > 9999 -> `#VALUE!`
- No-year format when volatile data is not configured -> `#VALUE!` (`volatile_today()` unset returns serial 0; silently producing 1900 dates is worse than erroring)
- Time-only string (e.g. `"14:30:00"`) -> `0`. Matches Excel (verified in `datevalue_formats_test.xlsx` row 50).

### Known locale limitations (documented, not mitigated)

- **Month names are English-only.** Workbooks authored in non-English Excel
  with localized month names (`15-janv.-2026`, `15-Dez-2026`) get `#VALUE!` —
  loud, never wrong.
- **Gregorian calendar only.** Thai-locale Excel writes Buddhist-era years
  (2569 BE = 2026 CE); a string like `2569-01-15` would parse here as
  Gregorian year 2569. Within the 1900-9999 window this is indistinguishable
  from a real Gregorian date — the single known case where output could be a
  different date rather than an error. Exotic; documented divergence.
- Field-order locales (mdy/dmy/ymd) and separator conventions are fully
  covered by the ambiguity rule — rejected when meaning differs by locale.

### Special case: 1900-02-29

`DATEVALUE("1900-02-29")` returns 60 (the Lotus 1-2-3 phantom date). `from_ymd(1900, 2, 29)` already handles this.

## Reusable parser design

The date text parser lives in **xlstream-core** so it's available to all crates. TIMEVALUE consumes the same parse — so the parser must return the time components, not just note their presence.

### API in `xlstream-core/src/date.rs`

```rust
/// Parsed components from a date text string.
///
/// `year` and `day` are `Option` because some formats omit them (no-year
/// defaults to the current year, month-year defaults day to 1). The
/// caller supplies defaults and validates ranges.
pub struct ParsedDate {
    pub year: Option<i32>,
    pub month: u32,
    pub day: Option<u32>,
    /// Trailing time component, if present: (hour, minute, seconds).
    /// DATEVALUE ignores it; TIMEVALUE requires it.
    pub time: Option<(u32, u32, f64)>,
}

/// Parse a date text string into components.
///
/// All-numeric input must be ISO year-first (`yyyy-mm-dd`, one separator
/// kind). Input containing a month name is accepted in any token order.
/// All-numeric non-ISO and 2-digit-year formats are rejected as
/// locale-ambiguous.
///
/// Returns `None` for unparseable input. Caller supplies defaults
/// (current year, day 1) and validates ranges.
pub fn parse_date_text(text: &str) -> Option<ParsedDate>

/// Look up a month name (case-insensitive, 3-letter or full).
/// Returns 1-12 or None.
pub fn month_from_name(name: &str) -> Option<u32>

/// Parse a pure time string (`HH:MM`, `HH:MM:SS`, `HH:MM:SS.fff`).
/// Returns (hour, minute, seconds) or None. Used by DATEVALUE for the
/// time-only -> 0 case and by TIMEVALUE directly.
pub fn parse_time_text(text: &str) -> Option<(u32, u32, f64)>
```

### Why in core, not eval

- TIMEVALUE (eval) reuses `parse_date_text` and takes `ParsedDate::time`.
- Future: `coerce::to_number` could try date text parsing for implicit coercion (Excel does this for functions like `YEAR("2026-01-15")`).
- Future: I/O crate might need date parsing for CSV import.
- Zero new dependencies — just string splitting and matching.

### Parser logic

1. Trim whitespace.
2. Strip a trailing time component (`HH:MM`, `HH:MM:SS`, `HH:MM:SS.fff` after the last space). Parse it into `(h, m, s)`: h 0-23, m 0-59, s 0-59.999. A malformed or out-of-range time fails the whole parse (`None`).
3. Try ISO numeric: split on `-` and `/` (mixing allowed — Excel-verified), 3 parts, first part 4 digits -> `yyyy-mm-dd`.
4. Try named-month: tokenize on `-`, `/`, space, comma; empty tokens from consecutive separators are dropped (Excel rejects `"January  15,  2026"` — minor permissive divergence, we accept). Exactly one token must match a month name; every remaining token must be numeric, and there must be 1 or 2 of them:
   - 4-digit token -> year. Token <= 31 -> day. Anything else (2-digit year, 3-digit, 32-99) -> `None`.
   - 1 numeric token, 4-digit -> month-year (day `None`).
   - 1 numeric token <= 31 BEFORE the month -> no-year day (`15-Jan`). AFTER the month -> `None`: Excel reads `"Jan 15"` as year 2015, a day-reading would silently differ.
   - 2 numeric tokens: must be exactly one year and one day, any order, else `None`.
5. Validate: month 1..=12, day (if present) 1..=days_in_month. Year (if present) 1900..=9999. Real Gregorian leap rules, except 1900-02-29 (Lotus bug).

### Month name lookup

Static match, no allocations:

```rust
pub fn month_from_name(name: &str) -> Option<u32> {
    const MONTHS: [(&str, &str, u32); 12] = [
        ("jan", "january", 1), ("feb", "february", 2), ("mar", "march", 3),
        ("apr", "april", 4), ("may", "may", 5), ("jun", "june", 6),
        ("jul", "july", 7), ("aug", "august", 8), ("sep", "september", 9),
        ("oct", "october", 10), ("nov", "november", 11), ("dec", "december", 12),
    ];
    MONTHS
        .iter()
        .find(|(short, full, _)| name.eq_ignore_ascii_case(short) || name.eq_ignore_ascii_case(full))
        .map(|&(_, _, n)| n)
}
```

## What already exists

- `crates/xlstream-core/src/date.rs:50-192` — `ExcelDate` with `from_serial()`, `from_ymd()`, `year_month_day()`, `weekday()`.
- `crates/xlstream-core/src/date.rs:36-48` — `month_days()` and `year_days()` private helpers. `month_days` returns `[i32; 12]` for real Gregorian calendar — needed for day validation.
- `crates/xlstream-core/src/date.rs:31-33` — `is_real_leap_year()` private helper.
- `crates/xlstream-core/src/lib.rs:47` — `EXCEL_MAX_DATE_SERIAL = 2_958_465.0`.
- `crates/xlstream-eval/src/builtins/date.rs:20-26` — `date_serial()` helper for extracting serial from Value.
- `crates/xlstream-eval/src/builtins/date.rs:61-103` — `builtin_date()` shows the pattern.
- `crates/xlstream-eval/src/builtins/mod.rs:644-706` — eager handler wrappers for date builtins.
- `crates/xlstream-eval/src/registry.rs:440-541` — all 12 date function entries.
- `crates/xlstream-eval/src/registry.rs:2379-2381` — `entry_count_is_exact` asserts 208; bump to 209.
- `docs/functions.md:284` — DATEVALUE listed as `.` (planned), target v0.4.

## Where to look

- `crates/xlstream-core/src/date.rs:31-48` — make `is_real_leap_year` and `month_days` pub(crate) or pub, since `parse_date_text` needs them for validation.
- `crates/xlstream-core/src/date.rs` — add `parse_date_text`, `ParsedDate`, `month_from_name` here.
- `crates/xlstream-core/src/lib.rs` — re-export `parse_date_text`, `ParsedDate`, `month_from_name`.
- `crates/xlstream-eval/src/builtins/date.rs` — add `builtin_datevalue` using the core parser.
- `crates/xlstream-eval/src/builtins/mod.rs:700-706` — add `handle_datevalue` wrapper.
- `crates/xlstream-eval/src/registry.rs:511-520` — add registry entry after DATEDIF.
- `crates/xlstream-eval/tests/conformance/date.rs` — add conformance test.

## DATEVALUE evaluation

**Classification:** RowLocal — pure function of a single text arg.

**Prelude:** Nothing.

**Row eval:**
1. Arity check: exactly 1 arg, else `#VALUE!`.
2. Arg must be `Value::Text`. Error propagates. Number/Bool/Date/Empty -> `#VALUE!`.
3. Call `parse_date_text(text)`. On `None`, try `parse_time_text(text)` — pure time -> `Value::Number(0.0)` (matches Excel). Both `None` -> `#VALUE!`.
4. Fill defaults: day defaults to 1. Year defaults to `prelude.volatile_today()`'s year — if volatile data is not configured, return `#VALUE!` instead of defaulting from serial 0.
5. Validate ranges: year 1900..=9999, month 1..=12, day 1..=days_in_month. Fail -> `#VALUE!`.
6. Call `ExcelDate::from_ymd(year, month, day)`. Ignore `ParsedDate::time`.
7. Return `Value::Number(serial.floor())`.

Note: step 4 means DATEVALUE needs prelude access (for today's year). Use the stateful handler pattern like `builtin_today`, not the eager `&[Value]` pattern.

## Tests

### Unit tests for parser (in `xlstream-core/src/date.rs`)

**`parse_date_text` happy path:**
- `parse_iso_dash` — `"2026-01-15"` -> `ParsedDate { year: Some(2026), month: 1, day: Some(15), time: None }`
- `parse_iso_slash` — `"2026/01/15"` -> same with slashes
- `parse_named_day_first_dash` — `"15-Jan-2026"` -> year 2026, month 1, day 15
- `parse_named_day_first_full` — `"15-January-2026"` -> same
- `parse_named_day_first_space` — `"15 Jan 2026"` -> same
- `parse_named_month_first` — `"Jan-15-2026"` -> same (order-free)
- `parse_named_year_first` — `"2026-Jan-15"` -> same (order-free)
- `parse_named_comma` — `"January 15, 2026"` -> same
- `parse_month_year_dash` — `"Jan-2026"` -> year 2026, month 1, day None
- `parse_month_year_space` — `"January 2026"` -> same
- `parse_no_year` — `"15-Jan"` -> year None, month 1, day 15
- `parse_extracts_time` — `"2026-01-15 14:30"` -> date parts + `time: Some((14, 30, 0.0))`
- `parse_extracts_time_seconds` — `"2026-01-15 14:30:15.5"` -> `time: Some((14, 30, 15.5))`

**`parse_date_text` rejections (return None):**
- `parse_rejects_us_numeric` — `"1/15/2026"` -> None
- `parse_rejects_eu_numeric` — `"15/01/2026"` -> None
- `parse_iso_mixed_separators` — `"2026-01/15"` -> year 2026, month 1, day 15 (Excel-verified acceptance)
- `parse_rejects_two_digit_year_numeric` — `"1/15/26"` -> None
- `parse_rejects_two_digit_year_named` — `"15-Jan-26"` -> None
- `parse_rejects_dots` — `"2026.01.15"` -> None
- `parse_rejects_t_separator` — `"2026-01-15T14:30"` -> None
- `parse_rejects_month_alone` — `"Jan"` -> None
- `parse_rejects_month_first_no_year` — `"Jan 15"`, `"January 15"` -> None (Excel reads the number as year 2015)
- `parse_rejects_invalid_time` — `"2026-01-15 25:00"` -> None
- `parse_rejects_empty` — `""` -> None
- `parse_rejects_plain_text` — `"hello"` -> None
- `parse_rejects_number_text` — `"12345"` -> None

**`month_from_name`:**
- All 12 months, short + full, case variations

### Unit tests for `builtin_datevalue` (in `xlstream-eval/src/builtins/date.rs`)

**Happy path:**
- `datevalue_iso` — `DATEVALUE("2026-01-15")` -> 46037.0
- `datevalue_named_month` — `DATEVALUE("15-Jan-2026")` -> 46037.0
- `datevalue_named_month_first` — `DATEVALUE("Jan-15-2026")` -> 46037.0
- `datevalue_comma_format` — `DATEVALUE("January 15, 2026")` -> 46037.0
- `datevalue_month_year` — `DATEVALUE("Jan-2026")` -> 46023.0
- `datevalue_jan_1_1900` — `DATEVALUE("1900-01-01")` -> 1.0
- `datevalue_dec_31_9999` — `DATEVALUE("9999-12-31")` -> 2958465.0

**Edge cases:**
- `datevalue_strips_time` — `DATEVALUE("2026-01-15 14:30")` -> 46037.0
- `datevalue_whitespace_trimmed` — `DATEVALUE("  2026-01-15  ")` -> 46037.0
- `datevalue_feb_29_leap` — `DATEVALUE("2024-02-29")` -> 45351.0
- `datevalue_feb_29_1900_lotus` — `DATEVALUE("1900-02-29")` -> 60.0
- `datevalue_feb_29_non_leap` — `DATEVALUE("2025-02-29")` -> `#VALUE!`
- `datevalue_day_exceeds_month` — `DATEVALUE("2026-04-31")` -> `#VALUE!`
- `datevalue_month_0` — `DATEVALUE("2026-00-15")` -> `#VALUE!`
- `datevalue_month_13` — `DATEVALUE("2026-13-01")` -> `#VALUE!`
- `datevalue_no_year_uses_volatile_year` — `DATEVALUE("15-Jan")` with injected TODAY -> that year's serial
- `datevalue_no_year_without_volatile_errors` — `DATEVALUE("15-Jan")`, no volatile data -> `#VALUE!`

**Error propagation:**
- `datevalue_propagates_na` — arg is `#N/A` -> `#N/A`
- `datevalue_wrong_arity_zero` — `DATEVALUE()` -> `#VALUE!`
- `datevalue_wrong_arity_two` — `DATEVALUE("a", "b")` -> `#VALUE!`

**Type rejection:**
- `datevalue_rejects_number` — `DATEVALUE(44927)` -> `#VALUE!`
- `datevalue_rejects_bool` — `DATEVALUE(TRUE)` -> `#VALUE!`
- `datevalue_rejects_date` — `DATEVALUE(DATE(2026,1,1))` -> `#VALUE!`
- `datevalue_rejects_plain_text` — `DATEVALUE("hello")` -> `#VALUE!`
- `datevalue_rejects_us_numeric` — `DATEVALUE("01/15/2026")` -> `#VALUE!` (deliberate divergence from US-locale Excel — unit-tested here, kept out of the fixture)
- `datevalue_rejects_two_digit_year` — `DATEVALUE("15-Jan-26")` -> `#VALUE!` (deliberate divergence — Excel's century window)
- `datevalue_rejects_month_first_no_year` — `DATEVALUE("January 15")` -> `#VALUE!` (Excel reads year 2015; day-reading would silently differ)
- `datevalue_accepts_year_first_named` — `DATEVALUE("2026-Jan-15")` -> 46037.0 (deliberate divergence — EU Excel rejects it, verified; unambiguous so we accept)
- `datevalue_time_only_returns_zero` — `DATEVALUE("14:30:00")` -> 0.0 (matches Excel, verified)

**Month name case insensitivity:**
- `datevalue_uppercase_month` — `DATEVALUE("15-JAN-2026")` -> 46037.0
- `datevalue_lowercase_month` — `DATEVALUE("15-jan-2026")` -> 46037.0

### Conformance fixture

`crates/xlstream-eval/tests/fixtures/date/datevalue.xlsx`

The fixture contains only inputs where we expect Excel (US locale) to agree.
Deliberate divergences (`01/15/2026`, `15-Jan-26`) are unit-tested, never
fixture rows — no hand-editing of cached values, Excel stays the oracle.
`2026-Jan-15` is already verified as an EU-Excel rejection
(`datevalue_formats_test.xlsx` row 62) — we accept it (unambiguous); it lives
in unit tests as a documented divergence, not here. Text inputs must be
written via openpyxl: typing them into Excel converts the cell itself to a
date (`Jan-15` typed into a cell became the date Jan-2015 in the formats
fixture).

| Row | A (input) | B (formula) | Expected |
|---|---|---|---|
| 1 | `2026-01-15` | `=DATEVALUE(A1)` | 46037 |
| 2 | `2026/01/15` | `=DATEVALUE(A2)` | 46037 |
| 3 | `15-Jan-2026` | `=DATEVALUE(A3)` | 46037 |
| 4 | `15-January-2026` | `=DATEVALUE(A4)` | 46037 |
| 5 | `15 Jan 2026` | `=DATEVALUE(A5)` | 46037 |
| 6 | `15 January 2026` | `=DATEVALUE(A6)` | 46037 |
| 7 | `15/Jan/2026` | `=DATEVALUE(A7)` | 46037 |
| 8 | `Jan-15-2026` | `=DATEVALUE(A8)` | 46037 |
| 9 | `January 15, 2026` | `=DATEVALUE(A9)` | 46037 |
| 10 | `Jan-2026` | `=DATEVALUE(A10)` | 46023 |
| 11 | `January 2026` | `=DATEVALUE(A11)` | 46023 |
| 12 | `1900-01-01` | `=DATEVALUE(A12)` | 1 |
| 13 | `9999-12-31` | `=DATEVALUE(A13)` | 2958465 |
| 14 | `2024-02-29` | `=DATEVALUE(A14)` | 45351 |
| 15 | `1900-02-29` | `=DATEVALUE(A15)` | 60 |
| 16 | `  2026-01-15  ` | `=DATEVALUE(A16)` | 46037 |
| 17 | `2026-01-15 14:30` | `=DATEVALUE(A17)` | 46037 |
| 18 | `15-JAN-2026` | `=DATEVALUE(A18)` | 46037 |
| 19 | (empty) | `=DATEVALUE(A19)` | `#VALUE!` |
| 20 | `not a date` | `=DATEVALUE(A20)` | `#VALUE!` |
| 21 | `2025-02-29` | `=DATEVALUE(A21)` | `#VALUE!` |
| 22 | | `=DATEVALUE(123)` | `#VALUE!` |
| 23 | | `=DATEVALUE(TRUE)` | `#VALUE!` |
| 24 | `14:30:00` | `=DATEVALUE(A24)` | 0 |

`15-Jan` (no-year) stays out of the fixture — its value depends on the year
at recalc time, so it can never be a stable cached value. Unit-tested with an
injected TODAY.

**Fixture workflow:**
1. Generate with openpyxl
2. Open in Excel (US locale), save
3. Diff cached values against the table. Any mismatch: decide match-Excel vs documented divergence, update spec and fixture, regenerate.
4. Add `#[test] fn datevalue()` in conformance/date.rs

## Future reuse map

| Future function | Reuses from this PR |
|---|---|
| TIMEVALUE | `parse_time_text` directly; `parse_date_text` for date+time strings via `ParsedDate::time` |
| HOUR, MINUTE, SECOND | `serial_to_hms` (not in this PR — add in their PR) |
| TIME | `serial_from_hms` (not in this PR) |
| DAYS, DAYS360, etc. | Nothing from parser — they take serial input |
| Implicit date coercion | `parse_date_text` in `coerce::to_number` (future, not this PR) |

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | `[Unreleased]` -> `### Added`: `xlstream-core: add date text parser (parse_date_text, month_from_name)` and `xlstream-eval: add DATEVALUE` |
| `docs/functions.md:284` | Change DATEVALUE from `.` to `x` |
| `docs/roadmap/v0.4/README.md` | Tick the DATEVALUE checkbox |
| `crates/xlstream-eval/src/registry.rs:2380` | Bump entry count 208 -> 209 |

## Streaming invariant

**No violation.** DATEVALUE is RowLocal. The only prelude access is `volatile_today()` for default year on no-year formats — this is a read of a prelude-computed scalar, same pattern as `TODAY()`. No cross-row reads, no range references.
