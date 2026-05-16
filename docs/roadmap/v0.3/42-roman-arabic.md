# Feature: ROMAN / ARABIC

**Branch:** `feat/roman-arabic`
**Effort:** ~3 hours
**Crates:** xlstream-eval

## What

Convert between integers and Roman numeral text representations.

- `ROMAN(number, [form])` — convert a positive integer (0-3999) to a Roman numeral string. The `form` parameter (0-4) controls abbreviation level: 0 = classic (default), 4 = most abbreviated.
- `ARABIC(text)` — convert a Roman numeral string to a number. Case insensitive. Returns 0 for empty string.

```
=ROMAN(499)          -> "CDXCIX"
=ROMAN(499, 0)       -> "CDXCIX"    (classic)
=ROMAN(499, 1)       -> "LDVLIV"    (less classic — Excel-specific abbreviations)
=ROMAN(499, 4)       -> "ID"        (most abbreviated)
=ROMAN(0)            -> ""
=ARABIC("MCMXCIX")   -> 1999
=ARABIC("mmvi")      -> 2006        (case insensitive)
=ARABIC("")          -> 0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:72-88` — ROUND (2-arg optional pattern)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch
- String builtins may provide pattern for text-returning functions — check `crates/xlstream-eval/src/builtins/string.rs`

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**ROMAN(number, [form]):**
1. Arity check: 1-2 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate. Truncate to integer.
3. If number < 0 or number > 3999 -> return `#VALUE!`.
4. If number == 0 -> return `Value::Text("")`.
5. Extract form as f64 (default: 0). Error -> propagate. Truncate to integer.
6. If form < 0 or form > 4 -> return `#VALUE!`.
7. Convert number to Roman numeral string using the specified form:
   - Form 0 (classic): standard subtractive notation (I, IV, V, IX, X, XL, L, XC, C, CD, D, CM, M).
   - Forms 1-4: progressively more abbreviated. Excel defines specific abbreviation rules for each form level. Form 4 allows subtractions like ID (499), XM (990), etc.
8. Return `Value::Text(result)`.

**ARABIC(text):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg. If error -> propagate. If not text -> coerce to text.
3. If empty string -> return `Value::Number(0.0)`.
4. Convert to uppercase for case-insensitive matching.
5. Strip leading/trailing whitespace.
6. Handle leading `-` for negative Roman numerals (Excel supports this).
7. Parse left to right. For each character, look up its value (I=1, V=5, X=10, L=50, C=100, D=500, M=1000). If the current char's value is less than the next char's value, subtract it; otherwise add it.
8. If any character is not a valid Roman numeral letter -> return `#VALUE!`.
9. Return `Value::Number(result as f64)`.

**ROMAN form abbreviation rules (form 0-4):**
- Form 0 (classic): Only standard subtractions: IV, IX, XL, XC, CD, CM.
- Form 1: Additionally allows: IL (49), IC (99), XD (490), XM (990).
- Form 2: Additionally allows: VL (45), VC (95), LD (450), LC (950), DM (500 subtracted from M context — not standard).
- Form 3: Additionally allows more aggressive subtractions.
- Form 4: Most abbreviated. All subtractions allowed (any smaller value before any larger value).

**Implementation note:** The form parameter affects encoding (ROMAN) but not decoding (ARABIC). ARABIC always accepts any valid subtractive notation regardless of form level.

**Error conditions:**
- ROMAN: wrong arity -> `#VALUE!`
- ROMAN: number < 0 or > 3999 -> `#VALUE!`
- ROMAN: form < 0 or > 4 -> `#VALUE!`
- ROMAN: non-numeric -> `#VALUE!`
- ARABIC: wrong arity -> `#VALUE!`
- ARABIC: invalid Roman numeral characters -> `#VALUE!`
- Error in arg: propagate

## Tests

### Unit tests (in `math.rs`)

**ROMAN form 0 (classic):**
- `roman(1)` -> "I"
- `roman(4)` -> "IV"
- `roman(9)` -> "IX"
- `roman(14)` -> "XIV"
- `roman(49)` -> "XLIX"
- `roman(99)` -> "XCIX"
- `roman(499)` -> "CDXCIX"
- `roman(999)` -> "CMXCIX"
- `roman(1999)` -> "MCMXCIX"
- `roman(3999)` -> "MMMCMXCIX"

**ROMAN form 4 (most abbreviated):**
- `roman(499, 4)` -> "ID"
- `roman(999, 4)` -> "IM"

**ROMAN edge cases:**
- `roman(0)` -> ""
- `roman(0, 0)` -> ""

**ROMAN errors:**
- `roman(-1)` -> `#VALUE!`
- `roman(4000)` -> `#VALUE!`
- `roman(1, 5)` -> `#VALUE!` (form out of range)
- `roman(1, -1)` -> `#VALUE!`

**ARABIC happy path:**
- `arabic("I")` -> 1
- `arabic("IV")` -> 4
- `arabic("IX")` -> 9
- `arabic("MCMXCIX")` -> 1999
- `arabic("MMMCMXCIX")` -> 3999
- `arabic("mmvi")` -> 2006 (case insensitive)

**ARABIC edge cases:**
- `arabic("")` -> 0
- `arabic("-X")` -> -10 (negative)

**ARABIC errors:**
- `arabic("ABC")` -> `#VALUE!` (invalid characters)
- `arabic("123")` -> `#VALUE!`

**Error propagation:**
- `roman(#N/A)` -> `#N/A`
- `arabic(#N/A)` -> `#N/A`

**Arity errors:**
- `roman()` -> `#VALUE!`
- `roman(1, 0, 0)` -> `#VALUE!`
- `arabic()` -> `#VALUE!`
- `arabic("I", "V")` -> `#VALUE!`

**Coercion:**
- `roman(TRUE)` -> "I" (TRUE = 1)
- `roman("5")` -> "V" (text coerced to number)

**Type mismatch:**
- `roman("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/roman_arabic.xlsx`.

**Sheet1 data:**
- A: "Number" header, rows 2-12: `1, 4, 9, 49, 99, 499, 999, 1999, 3999, 0, -1`
- B: "Form" header, rows 2-6: `0, 1, 2, 3, 4`
- C: "Roman" header, rows 2-8: `"I", "IV", "IX", "MCMXCIX", "mmvi", "", "ABC"`
- D: "Error" header, row 2: `=NA()`
- E: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `499`

**Formulas (column F, starting row 2) — 28 formulas:**

ROMAN classic (6):
1. `=ROMAN(A2)` -> "I"
2. `=ROMAN(A3)` -> "IV"
3. `=ROMAN(A4)` -> "IX"
4. `=ROMAN(A7)` -> "CDXCIX"
5. `=ROMAN(A9)` -> "MCMXCIX"
6. `=ROMAN(A10)` -> "MMMCMXCIX"

ROMAN form variations (3):
7. `=ROMAN(A7, B2)` -> "CDXCIX" (form 0, classic)
8. `=ROMAN(A7, B6)` -> "ID" (form 4, most abbreviated)
9. `=ROMAN(A8, B6)` -> "IM" (999, form 4)

ROMAN edge (2):
10. `=ROMAN(A11)` -> "" (0)
11. `=ROMAN(A11, 0)` -> "" (0, explicit form)

ARABIC happy path (5):
12. `=ARABIC(C2)` -> 1
13. `=ARABIC(C3)` -> 4
14. `=ARABIC(C4)` -> 9
15. `=ARABIC(C5)` -> 1999
16. `=ARABIC(C6)` -> 2006 (case insensitive)

ARABIC edge (1):
17. `=ARABIC(C7)` -> 0 (empty string)

ROMAN errors (2):
18. `=ROMAN(A12)` -> `#VALUE!` (-1)
19. `=ROMAN(4000)` -> `#VALUE!`

ARABIC errors (1):
20. `=ARABIC(C8)` -> `#VALUE!` ("ABC" invalid)

Error propagation (2):
21. `=ROMAN(D2)` -> `#N/A`
22. `=ARABIC(D2)` -> `#N/A`

Coercion (2):
23. `=ROMAN(TRUE)` -> "I"
24. `=ROMAN("5")` -> "V"

Nested (2):
25. `=IF(ARABIC("X")=10, "yes", "no")` -> "yes"
26. `=IFERROR(ROMAN(-1), "bad")` -> "bad"

Round-trip (2):
27. `=ARABIC(ROMAN(A7))` -> 499
28. `=ARABIC(ROMAN(A9))` -> 1999

Cross-sheet (1):
29. `=ROMAN(Sheet2!A2)` -> "CDXCIX" (499)

Note: No `_xlfn.` prefix needed for ROMAN/ARABIC.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn roman_arabic()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "ROMAN, ARABIC Roman numeral conversion functions" under `[Unreleased]` |
| `docs/functions.md` | Change ROMAN, ARABIC from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the ROMAN / ARABIC checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
