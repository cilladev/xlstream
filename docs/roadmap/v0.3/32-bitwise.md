# Feature: BITAND / BITOR / BITXOR / BITLSHIFT / BITRSHIFT

**Branch:** `feat/bitwise`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Bitwise operations on non-negative integers, up to 2^48 - 1.

- `BITAND(number1, number2)` — bitwise AND
- `BITOR(number1, number2)` — bitwise OR
- `BITXOR(number1, number2)` — bitwise XOR
- `BITLSHIFT(number, shift_amount)` — bitwise left shift
- `BITRSHIFT(number, shift_amount)` — bitwise right shift

```
=BITAND(13, 25)        → 9         (1101 & 11001 = 01001)
=BITOR(13, 25)         → 29        (1101 | 11001 = 11101)
=BITXOR(13, 25)        → 20        (1101 ^ 11001 = 10100)
=BITLSHIFT(4, 2)       → 16        (100 << 2 = 10000)
=BITRSHIFT(16, 2)      → 4         (10000 >> 2 = 100)
=BITLSHIFT(4, -2)      → 1         (negative shift = right shift)
```

**Domain:**
- AND/OR/XOR: both args must be non-negative integers in [0, 2^48 - 1] (= 281474976710655). Negative → `#NUM!`. Non-integer (fractional part) → `#NUM!`.
- LSHIFT/RSHIFT: `number` must be non-negative integer in [0, 2^48 - 1]. `shift_amount` is an integer in [-53, 53] (can be negative — BITLSHIFT with negative shift = right shift and vice versa).
- Result must also be in [0, 2^48 - 1]; overflow → `#NUM!`.
- All return numbers.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — empty module (module doc only, lines 1-5). Bitwise functions will land here.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). No dispatch arms yet for engineering functions.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- Not in dispatch
- `docs/functions.md` lists all five as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (currently empty)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:159-175` — math builtins dispatch pattern (pure, eager eval)
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

All five functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. All args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Same pattern as math builtins.

**Shared validation helper (suggested):** A `validate_bit_arg(f64) -> Result<u64, CellError>` helper:
1. If value is negative → `#NUM!`
2. If value has a fractional part (i.e., `value != value.trunc()`) → `#NUM!`
3. If value > 2^48 - 1 → `#NUM!`
4. Cast to `u64` and return

**BITAND(number1, number2):**
1. Arity check: exactly 2 args. Otherwise `#VALUE!`
2. Extract both args as f64 via `num_arg_ce`. Error → propagate.
3. Validate both via `validate_bit_arg`. Error → `#NUM!`.
4. Compute `n1 & n2` (Rust bitwise AND on `u64`).
5. Return `Value::Number(result as f64)`.

**BITOR(number1, number2):**
1. Same as BITAND but use `n1 | n2`.

**BITXOR(number1, number2):**
1. Same as BITAND but use `n1 ^ n2`.

**BITLSHIFT(number, shift_amount):**
1. Arity check: exactly 2 args. Otherwise `#VALUE!`
2. Extract arg 0 as f64, validate via `validate_bit_arg`.
3. Extract arg 1 as f64 via `num_arg_ce`. Must be integer (fractional → `#NUM!`). Range: [-53, 53]. Out of range → `#NUM!`.
4. If shift_amount >= 0: result = `number << shift_amount`. If shift_amount < 0: result = `number >> (-shift_amount)`.
5. Validate result in [0, 2^48 - 1]. Overflow → `#NUM!`.
6. Return `Value::Number(result as f64)`.

**BITRSHIFT(number, shift_amount):**
1. Same as BITLSHIFT but reversed: positive shift = right shift, negative shift = left shift.
2. Equivalently, BITRSHIFT(n, s) = BITLSHIFT(n, -s).

**Error conditions:**
- Wrong arity: `#VALUE!` (AND/OR/XOR require exactly 2; LSHIFT/RSHIFT require exactly 2)
- Negative number (for any function): `#NUM!`
- Non-integer number (fractional part): `#NUM!`
- Number > 2^48 - 1: `#NUM!`
- Shift amount not integer: `#NUM!`
- Shift amount outside [-53, 53]: `#NUM!`
- Result > 2^48 - 1 (overflow from left shift): `#NUM!`
- Non-numeric args: `#VALUE!` (via `coerce::to_number` failure)
- Error in any arg: propagate

**Edge cases:**
- `BITAND(0, 0)` → 0
- `BITOR(0, 0)` → 0
- `BITXOR(0, 0)` → 0
- `BITAND(281474976710655, 281474976710655)` → 281474976710655 (max & max = max)
- `BITOR(281474976710655, 0)` → 281474976710655
- `BITLSHIFT(1, 47)` → 140737488355328 (2^47, within limit)
- `BITLSHIFT(1, 48)` → `#NUM!` (2^48 exceeds max)
- `BITRSHIFT(1, 1)` → 0 (bit shifted out)
- `BITLSHIFT(4, -2)` → 1 (negative = right shift)
- `BITRSHIFT(4, -2)` → 16 (negative = left shift)
- `BITLSHIFT(0, 53)` → 0 (zero shifted is zero)
- Boolean coercion: `BITAND(TRUE, 1)` → 1 (TRUE coerces to 1.0)
- Truncation NOT allowed: `BITAND(13.5, 25)` → `#NUM!` (Excel rejects non-integers)

## Tests

### Unit tests (in `engineering.rs`)

**BITAND happy path:**
- `bitand(13, 25)` → 9
- `bitand(1, 1)` → 1
- `bitand(0, 0)` → 0
- `bitand(255, 15)` → 15
- `bitand(281474976710655, 281474976710655)` → 281474976710655 (max)

**BITOR happy path:**
- `bitor(13, 25)` → 29
- `bitor(0, 0)` → 0
- `bitor(255, 256)` → 511
- `bitor(281474976710655, 0)` → 281474976710655

**BITXOR happy path:**
- `bitxor(13, 25)` → 20
- `bitxor(0, 0)` → 0
- `bitxor(255, 255)` → 0
- `bitxor(255, 0)` → 255

**BITLSHIFT happy path:**
- `bitlshift(4, 2)` → 16
- `bitlshift(1, 0)` → 1
- `bitlshift(0, 10)` → 0
- `bitlshift(1, 47)` → 140737488355328

**BITLSHIFT negative shift (= right shift):**
- `bitlshift(16, -2)` → 4
- `bitlshift(4, -2)` → 1

**BITRSHIFT happy path:**
- `bitrshift(16, 2)` → 4
- `bitrshift(1, 0)` → 1
- `bitrshift(1, 1)` → 0

**BITRSHIFT negative shift (= left shift):**
- `bitrshift(4, -2)` → 16
- `bitrshift(1, -3)` → 8

**Domain errors (#NUM!):**
- `bitand(-1, 0)` → `#NUM!` (negative)
- `bitand(0, -1)` → `#NUM!`
- `bitor(281474976710656, 0)` → `#NUM!` (> 2^48 - 1)
- `bitand(13.5, 25)` → `#NUM!` (non-integer)
- `bitxor(13, 25.1)` → `#NUM!`
- `bitlshift(1, 48)` → `#NUM!` (result = 2^48 > max)
- `bitlshift(1, 54)` → `#NUM!` (shift > 53)
- `bitlshift(1, -54)` → `#NUM!` (shift < -53)
- `bitrshift(-1, 1)` → `#NUM!` (negative number)
- `bitlshift(1.5, 2)` → `#NUM!` (non-integer number)
- `bitlshift(1, 2.5)` → `#NUM!` (non-integer shift)

**Arity errors (#VALUE!):**
- `bitand(1)` → `#VALUE!` (too few)
- `bitand(1, 2, 3)` → `#VALUE!` (too many)
- `bitlshift(1)` → `#VALUE!`

**Type errors (#VALUE!):**
- `bitand("abc", 1)` → `#VALUE!`

**Error propagation:**
- `bitand(#N/A, 1)` → `#N/A`
- `bitlshift(#N/A, 1)` → `#N/A`

**Coercion:**
- `bitand(TRUE, 1)` → 1 (TRUE = 1.0)
- `bitor("13", 25)` → 29 (text coerced)

### Conformance fixture

Create `tests/fixtures/engineering/bitwise.xlsx`.

**Sheet1 data:**
- A: "Num1" header, rows 2-9: `13, 0, 255, 281474976710655, 4, 16, 1, 1`
- B: "Num2" header, rows 2-9: `25, 0, 15, 281474976710655, 2, 2, 47, 0`
- C: "NegShift" header, rows 2-4: `-2, -3, -1`
- D: "Error" header, row 2: `=NA()`
- E: "BadNum" header, rows 2-4: `-1, 13.5, 281474976710656`
- F: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, rows 2-3: `13, 25`

**Formulas (column G, starting row 2) — 30 formulas:**

BITAND happy path (3):
1. `=BITAND(A2, B2)` → 9 (13 & 25)
2. `=BITAND(A3, B3)` → 0 (0 & 0)
3. `=BITAND(A4, B4)` → 15 (255 & 15)

BITOR happy path (3):
4. `=BITOR(A2, B2)` → 29 (13 | 25)
5. `=BITOR(A3, B3)` → 0 (0 | 0)
6. `=BITOR(A5, B5)` → 281474976710655 (max | max)

BITXOR happy path (3):
7. `=BITXOR(A2, B2)` → 20 (13 ^ 25)
8. `=BITXOR(A3, B3)` → 0 (0 ^ 0)
9. `=BITXOR(A4, A4)` → 0 (255 ^ 255)

BITLSHIFT happy path (3):
10. `=BITLSHIFT(A6, B6)` → 16 (4 << 2)
11. `=BITLSHIFT(A8, B8)` → 140737488355328 (1 << 47)
12. `=BITLSHIFT(A3, 10)` → 0 (0 << 10)

BITRSHIFT happy path (2):
13. `=BITRSHIFT(A7, B7)` → 4 (16 >> 2)
14. `=BITRSHIFT(A8, B9)` → 1 (1 >> 0, unchanged)

Negative shift (3):
15. `=BITLSHIFT(A7, C2)` → 4 (16 << -2 = 16 >> 2)
16. `=BITRSHIFT(A6, C2)` → 16 (4 >> -2 = 4 << 2)
17. `=BITRSHIFT(A8, C3)` → 8 (1 >> -3 = 1 << 3)

Max value (1):
18. `=BITAND(A5, A5)` → 281474976710655

Domain errors (5):
19. `=BITAND(E2, 0)` → `#NUM!` (-1 negative)
20. `=BITAND(E3, 25)` → `#NUM!` (13.5 non-integer)
21. `=BITOR(E4, 0)` → `#NUM!` (2^48 exceeds max)
22. `=BITLSHIFT(1, 48)` → `#NUM!` (result 2^48 > max)
23. `=BITLSHIFT(1, 54)` → `#NUM!` (shift > 53)

Error propagation (2):
24. `=BITAND(D2, 1)` → `#N/A`
25. `=BITLSHIFT(D2, 1)` → `#N/A`

Type error (1):
26. `=BITAND(F2, 1)` → `#VALUE!` ("abc" not numeric)

Coercion (1):
27. `=BITAND(TRUE, 1)` → 1

Nested / combined (2):
28. `=BITOR(BITAND(A2, B2), 16)` → 25 (9 | 16)
29. `=IF(BITAND(A4, 1)=1, "odd", "even")` → "odd" (255 & 1 = 1)

Cross-sheet (1):
30. `=BITXOR(Sheet2!A2, Sheet2!A3)` → 20 (13 ^ 25)

BITAND, BITOR, BITXOR, BITLSHIFT, BITRSHIFT do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn bitwise()` in `conformance/engineering.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "BITAND, BITOR, BITXOR, BITLSHIFT, BITRSHIFT bitwise operations" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change BITAND, BITOR, BITXOR, BITLSHIFT, BITRSHIFT from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the BITAND / BITOR / BITXOR / BITLSHIFT / BITRSHIFT checkbox |

## Streaming invariant

Does not violate. All five functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
