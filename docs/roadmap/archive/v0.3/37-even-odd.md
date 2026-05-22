# Feature: EVEN / ODD

**Branch:** `feat/even-odd`
**Effort:** ~1.5 hours
**Crates:** xlstream-eval

## What

Rounding functions that round to the next even or odd integer, always away from zero.

- `EVEN(x)` — round away from zero to the next even integer.
- `ODD(x)` — round away from zero to the next odd integer. `ODD(0)` returns 1 (Excel convention).

```
=EVEN(1.5)       -> 2
=EVEN(3)         -> 4
=EVEN(-1)        -> -2
=EVEN(0)         -> 0
=ODD(1.5)        -> 3
=ODD(3)          -> 3
=ODD(2)          -> 3
=ODD(-1)         -> -1
=ODD(0)          -> 1
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:256-297` — `builtin_ceiling`, `builtin_floor` — rounding functions (nearby pattern)
- `crates/xlstream-eval/src/builtins/math.rs:140-152` — `builtin_int` (1-arg rounding pattern)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:140-152` — INT (1-arg, simple rounding)
- `crates/xlstream-eval/src/builtins/math.rs:256-297` — CEILING/FLOOR (rounding with significance)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**EVEN(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. If x == 0.0 -> return 0.
4. Round away from zero to next even integer:
   - If x > 0: `ceil(x)`, then if result is odd, add 1.
   - If x < 0: `floor(x)`, then if result is odd (away from zero), subtract 1.
   - More precisely: `sign(x) * ceil(abs(x) / 2.0) * 2.0`
5. Return `Value::Number(result)`.

**ODD(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. If x == 0.0 -> return 1 (special Excel behavior).
4. Round away from zero to next odd integer:
   - Compute `n = ceil(abs(x))`. If n is even, add 1. Apply sign.
   - More precisely: `sign(x) * (ceil((abs(x) - 1.0) / 2.0) * 2.0 + 1.0)`, handling x=0 specially.
5. Return `Value::Number(result)`.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- Error in arg: propagate
- No domain restrictions. No `#NUM!` cases.

## Tests

### Unit tests (in `math.rs`)

**EVEN happy path:**
- `even(1.5)` -> 2.0
- `even(3)` -> 4.0
- `even(2)` -> 2.0
- `even(4)` -> 4.0
- `even(0.5)` -> 2.0

**EVEN negative:**
- `even(-1)` -> -2.0
- `even(-1.5)` -> -2.0
- `even(-2)` -> -2.0
- `even(-3)` -> -4.0

**EVEN zero:**
- `even(0)` -> 0.0

**ODD happy path:**
- `odd(1.5)` -> 3.0
- `odd(3)` -> 3.0
- `odd(2)` -> 3.0
- `odd(1)` -> 1.0
- `odd(4)` -> 5.0
- `odd(0.3)` -> 1.0

**ODD negative:**
- `odd(-1)` -> -1.0
- `odd(-1.5)` -> -3.0
- `odd(-2)` -> -3.0
- `odd(-3)` -> -3.0

**ODD zero:**
- `odd(0)` -> 1.0 (special Excel behavior)

**Error propagation:**
- `even(#N/A)` -> `#N/A`
- `odd(#N/A)` -> `#N/A`

**Arity errors:**
- `even()` -> `#VALUE!`
- `even(1, 2)` -> `#VALUE!`
- `odd()` -> `#VALUE!`

**Coercion:**
- `even(TRUE)` -> 2.0 (TRUE = 1.0)
- `odd("3")` -> 3.0

**Type mismatch:**
- `even("abc")` -> `#VALUE!`
- `odd("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/even_odd.xlsx`.

**Sheet1 data:**
- A: "x" header, rows 2-12: `1.5, 3, 2, -1, -1.5, 0, 0.5, 4, -3, -2, 0.3`
- B: "Error" header, row 2: `=NA()`
- C: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `3`

**Formulas (column D, starting row 2) — 25 formulas:**

EVEN happy path (5):
1. `=EVEN(A2)` -> 2 (1.5)
2. `=EVEN(A3)` -> 4 (3)
3. `=EVEN(A4)` -> 2 (2, already even)
4. `=EVEN(A7)` -> 0 (0)
5. `=EVEN(A8)` -> 2 (0.5)

EVEN negative (3):
6. `=EVEN(A5)` -> -2 (-1)
7. `=EVEN(A6)` -> -2 (-1.5)
8. `=EVEN(A10)` -> -4 (-3)

ODD happy path (5):
9. `=ODD(A2)` -> 3 (1.5)
10. `=ODD(A3)` -> 3 (3, already odd)
11. `=ODD(A4)` -> 3 (2)
12. `=ODD(A9)` -> 5 (4)
13. `=ODD(A12)` -> 1 (0.3)

ODD negative (3):
14. `=ODD(A5)` -> -1 (-1, already odd)
15. `=ODD(A6)` -> -3 (-1.5)
16. `=ODD(A11)` -> -3 (-2)

ODD zero (1):
17. `=ODD(A7)` -> 1 (special: ODD(0)=1)

Error propagation (2):
18. `=EVEN(B2)` -> `#N/A`
19. `=ODD(B2)` -> `#N/A`

Type error (1):
20. `=EVEN(C2)` -> `#VALUE!`

Coercion (2):
21. `=EVEN(TRUE)` -> 2 (TRUE = 1)
22. `=ODD("3")` -> 3

Nested (2):
23. `=IF(EVEN(3)=4, "yes", "no")` -> "yes"
24. `=IFERROR(ODD("abc"), "bad")` -> "bad"

Cross-sheet (1):
25. `=EVEN(Sheet2!A2)` -> 4 (EVEN(3))

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn even_odd()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "EVEN, ODD rounding functions" under `[Unreleased]` |
| `docs/functions.md` | Change EVEN, ODD from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the EVEN / ODD checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of a single argument — no cross-row reads, no prelude dependency.
