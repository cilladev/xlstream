# Feature: TRUNC

**Branch:** `feat/trunc`
**Effort:** ~1 hour
**Crates:** xlstream-eval

## What

Truncation function that removes the fractional part of a number, truncating toward zero. Optionally specifies the number of decimal digits to keep.

- `TRUNC(number, [num_digits])` — truncate toward zero. Default num_digits = 0.

Different from `INT`: `INT` rounds toward negative infinity, while `TRUNC` rounds toward zero. `INT(-2.5) = -3` but `TRUNC(-2.5) = -2`.

```
=TRUNC(8.9)          -> 8
=TRUNC(-8.9)         -> -8
=TRUNC(2.345, 2)     -> 2.34
=TRUNC(-2.345, 2)    -> -2.34
=TRUNC(1234, -2)     -> 1200
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:72-88` — `builtin_round` — 2-arg with Decimal-based rounding (exact pattern to follow for digit-precision truncation)
- `crates/xlstream-eval/src/builtins/math.rs:45-50` — `to_decimal` helper for Decimal conversion
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists TRUNC as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:72-88` — ROUND (2-arg Decimal rounding — same num_digits pattern)
- `crates/xlstream-eval/src/builtins/math.rs:90-130` — ROUNDUP/ROUNDDOWN (truncation-direction variants)
- `crates/xlstream-eval/src/builtins/math.rs:45-50` — `to_decimal` helper
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

Pure scalar function — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**TRUNC(number, [num_digits]):**
1. Arity check: 1 or 2 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate.
3. Extract num_digits as f64 via `num_arg` (default 0 if omitted). Error -> propagate. Truncate num_digits to integer.
4. Use Decimal-based truncation toward zero:
   - Convert to Decimal via `to_decimal`.
   - Apply `trunc` at the specified digit position.
   - For negative num_digits: truncate to powers of 10 (e.g., num_digits=-2 truncates to hundreds).
5. Return `Value::Number(result)`.

**Implementation note:** Use the same Decimal infrastructure as `builtin_round`. The difference is rounding strategy: ROUND uses half-away-from-zero, TRUNC always truncates toward zero (equivalent to `RoundingStrategy::ToZero` or `Decimal::trunc()`).

**Error conditions:**
- Wrong arity (0 or 3+ args): `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- Error in arg: propagate
- No `#NUM!` cases — TRUNC is defined for all finite inputs.

## Tests

### Unit tests (in `math.rs`)

**TRUNC happy path (default num_digits=0):**
- `trunc(8.9)` -> 8.0
- `trunc(-8.9)` -> -8.0
- `trunc(0.5)` -> 0.0
- `trunc(-0.5)` -> 0.0

**TRUNC with positive num_digits:**
- `trunc(2.345, 2)` -> 2.34
- `trunc(-2.345, 2)` -> -2.34
- `trunc(1.999, 1)` -> 1.9

**TRUNC with negative num_digits:**
- `trunc(1234, -2)` -> 1200.0
- `trunc(1234, -3)` -> 1000.0
- `trunc(-1234, -2)` -> -1200.0

**TRUNC vs INT distinction:**
- `trunc(-2.5)` -> -2.0 (toward zero)
- `int(-2.5)` -> -3.0 (toward negative infinity)

**TRUNC edge cases:**
- `trunc(0)` -> 0.0
- `trunc(5, 0)` -> 5.0 (integer unchanged)
- `trunc(5.5, 10)` -> 5.5 (more digits than needed)

**Error propagation:**
- `trunc(#N/A)` -> `#N/A`
- `trunc(1, #N/A)` -> `#N/A`

**Arity errors:**
- `trunc()` -> `#VALUE!`
- `trunc(1, 2, 3)` -> `#VALUE!`

**Coercion:**
- `trunc(TRUE)` -> 1.0 (TRUE = 1.0)
- `trunc("8.9")` -> 8.0

**Type mismatch:**
- `trunc("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/trunc.xlsx`.

**Sheet1 data:**
- A: "Number" header, rows 2-9: `8.9, -8.9, 2.345, -2.345, 1234, 0, 0.5, -2.5`
- B: "Digits" header, rows 2-7: `0, 2, 2, -2, -3, 1`
- C: "Error" header, row 2: `=NA()`
- D: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `8.9`

**Formulas (column E, starting row 2) — 22 formulas:**

TRUNC default (3):
1. `=TRUNC(A2)` -> 8 (8.9, default digits=0)
2. `=TRUNC(A3)` -> -8 (-8.9, toward zero)
3. `=TRUNC(A7)` -> 0 (0)

TRUNC positive digits (3):
4. `=TRUNC(A4, 2)` -> 2.34
5. `=TRUNC(A5, 2)` -> -2.34
6. `=TRUNC(A2, B7)` -> 8.9 (digits=1)

TRUNC negative digits (2):
7. `=TRUNC(A6, B5)` -> 1200 (digits=-2)
8. `=TRUNC(A6, B6)` -> 1000 (digits=-3)

TRUNC vs INT (2):
9. `=TRUNC(A9)` -> -2 (TRUNC toward zero)
10. `=INT(A9)` -> -3 (INT toward negative infinity)

Edge cases (2):
11. `=TRUNC(A7)` -> 0
12. `=TRUNC(A8)` -> 0 (0.5 truncated)

Error propagation (2):
13. `=TRUNC(C2)` -> `#N/A`
14. `=TRUNC(1, C2)` -> `#N/A`

Type error (1):
15. `=TRUNC(D2)` -> `#VALUE!`

Coercion (2):
16. `=TRUNC(TRUE)` -> 1
17. `=TRUNC("8.9")` -> 8

Nested (2):
18. `=IF(TRUNC(8.9)=8, "yes", "no")` -> "yes"
19. `=IFERROR(TRUNC("abc"), "bad")` -> "bad"

Combined (1):
20. `=TRUNC(2.345, 2) + TRUNC(-2.345, 2)` -> 0

Cross-sheet (1):
21. `=TRUNC(Sheet2!A2)` -> 8

Negative trunc (1):
22. `=TRUNC(-1234, -2)` -> -1200

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn trunc()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "TRUNC truncation function" under `[Unreleased]` |
| `docs/functions.md` | Change TRUNC from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the TRUNC checkbox |

## Streaming invariant

Does not violate. Pure scalar function of its arguments — no cross-row reads, no prelude dependency.
