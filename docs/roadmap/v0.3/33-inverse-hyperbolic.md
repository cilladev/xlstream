# Feature: ACOSH / ASINH / ATANH

**Branch:** `feat/inverse-hyperbolic`
**Effort:** ~1.5 hours
**Crates:** xlstream-eval

## What

Inverse hyperbolic trigonometric functions. Each takes a single numeric argument and returns the corresponding inverse hyperbolic value.

- `ACOSH(x)` — inverse hyperbolic cosine. Domain: x >= 1.
- `ASINH(x)` — inverse hyperbolic sine. No domain restriction.
- `ATANH(x)` — inverse hyperbolic arctangent. Domain: -1 < x < 1 (exclusive).

```
=ACOSH(1)        -> 0
=ACOSH(10)       -> 2.993222846...
=ASINH(0)        -> 0
=ASINH(-2.5)     -> -1.647231146...
=ATANH(0.5)      -> 0.549306144...
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:432-460` — `builtin_asin`, `builtin_acos` — 1-arg with NaN domain check (exact pattern to follow)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists all as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:432-460` — ASIN/ACOS pattern (1-arg, NaN domain check)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

All three are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**ACOSH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `x.acosh()`. If x < 1, `acosh` returns NaN -> detect and return `#NUM!`.
4. Return `Value::Number(result)`.

**ASINH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `x.asinh()`. Always finite for finite input — no domain check needed.
4. Return `Value::Number(result)`.

**ATANH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `x.atanh()`. If |x| >= 1, result is +/-Infinity or NaN -> detect with `finite_or_num` and return `#NUM!`.
4. Return `Value::Number(result)`.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- ACOSH: x < 1 -> `#NUM!`
- ATANH: x <= -1 or x >= 1 -> `#NUM!`
- Error in arg: propagate

## Tests

### Unit tests (in `math.rs`)

**ACOSH happy path:**
- `acosh(1)` -> 0.0
- `acosh(2)` -> ~1.3169578969
- `acosh(10)` -> ~2.9932228461

**ACOSH edge/domain:**
- `acosh(0.99)` -> `#NUM!` (x < 1)
- `acosh(0)` -> `#NUM!`
- `acosh(-1)` -> `#NUM!`

**ASINH happy path:**
- `asinh(0)` -> 0.0
- `asinh(1)` -> ~0.8813736198
- `asinh(-2.5)` -> ~-1.6472311464

**ASINH edge:**
- `asinh(-1000)` — large negative, still finite
- `asinh(0)` -> 0.0

**ATANH happy path:**
- `atanh(0)` -> 0.0
- `atanh(0.5)` -> ~0.5493061443
- `atanh(-0.5)` -> ~-0.5493061443
- `atanh(0.99)` -> ~2.6466524124

**ATANH edge/domain:**
- `atanh(1)` -> `#NUM!` (boundary, Infinity)
- `atanh(-1)` -> `#NUM!` (boundary, -Infinity)
- `atanh(2)` -> `#NUM!` (NaN)

**Error propagation:**
- `acosh(#N/A)` -> `#N/A`
- `asinh(#N/A)` -> `#N/A`
- `atanh(#N/A)` -> `#N/A`

**Arity errors:**
- `acosh()` -> `#VALUE!`
- `acosh(1, 2)` -> `#VALUE!`

**Coercion:**
- `acosh(TRUE)` -> 0.0 (TRUE = 1.0)
- `asinh("0")` -> 0.0

**Type mismatch:**
- `acosh("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/inverse_hyperbolic.xlsx`.

**Sheet1 data:**
- A: "x" header, rows 2-10: `1, 2, 10, 0, 0.5, -0.5, 0.99, -2.5, -1`
- B: "Error" header, row 2: `=NA()`
- C: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `10`

**Formulas (column D, starting row 2) — 25 formulas:**

ACOSH happy path (3):
1. `=ACOSH(A2)` -> 0 (acosh(1))
2. `=ACOSH(A3)` -> ~1.317 (acosh(2))
3. `=ACOSH(A4)` -> ~2.993 (acosh(10))

ASINH happy path (3):
4. `=ASINH(A5)` -> 0 (asinh(0))
5. `=ASINH(A6)` -> ~0.481 (asinh(0.5))
6. `=ASINH(A9)` -> ~-1.647 (asinh(-2.5))

ATANH happy path (3):
7. `=ATANH(A5)` -> 0 (atanh(0))
8. `=ATANH(A6)` -> ~0.549 (atanh(0.5))
9. `=ATANH(A7)` -> ~-0.549 (atanh(-0.5))

ACOSH domain errors (3):
10. `=ACOSH(A5)` -> `#NUM!` (acosh(0))
11. `=ACOSH(A6)` -> `#NUM!` (acosh(0.5))
12. `=ACOSH(A10)` -> `#NUM!` (acosh(-1))

ATANH domain errors (2):
13. `=ATANH(A2)` -> `#NUM!` (atanh(1))
14. `=ATANH(A10)` -> `#NUM!` (atanh(-1))

ATANH boundary (1):
15. `=ATANH(A8)` -> ~2.647 (atanh(0.99))

Error propagation (2):
16. `=ACOSH(B2)` -> `#N/A`
17. `=ATANH(B2)` -> `#N/A`

Type error (1):
18. `=ACOSH(C2)` -> `#VALUE!`

Coercion (2):
19. `=ACOSH(TRUE)` -> 0 (TRUE = 1)
20. `=ASINH("0")` -> 0

Nested (2):
21. `=IF(ACOSH(2)>1, "big", "small")` -> "big"
22. `=IFERROR(ACOSH(0), "invalid")` -> "invalid"

Combined (2):
23. `=ASINH(0) + ACOSH(1)` -> 0
24. `=ATANH(0.5) + ATANH(-0.5)` -> 0

Cross-sheet (1):
25. `=ACOSH(Sheet2!A2)` -> ~2.993 (acosh(10))

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn inverse_hyperbolic()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "ACOSH, ASINH, ATANH inverse hyperbolic functions" under `[Unreleased]` |
| `docs/functions.md` | Change ACOSH, ASINH, ATANH from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the ACOSH / ASINH / ATANH checkbox |

## Streaming invariant

Does not violate. All three are pure scalar functions of a single argument — no cross-row reads, no prelude dependency.
