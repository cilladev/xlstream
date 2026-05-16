# Feature: COSH / SINH / TANH

**Branch:** `feat/hyperbolic`
**Effort:** ~1.5 hours
**Crates:** xlstream-eval

## What

Hyperbolic trigonometric functions. Each takes a single numeric argument and returns the corresponding hyperbolic value.

- `COSH(x)` — hyperbolic cosine. No domain restriction but overflows for large |x|.
- `SINH(x)` — hyperbolic sine. Same overflow behavior.
- `TANH(x)` — hyperbolic tangent. Always in (-1, 1), no overflow.

```
=COSH(0)         -> 1
=COSH(1)         -> 1.543080635...
=SINH(0)         -> 0
=SINH(1)         -> 1.175201194...
=TANH(0)         -> 0
=TANH(0.5)       -> 0.462117157...
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:396-428` — `builtin_sin`, `builtin_cos`, `builtin_tan` — 1-arg trig with no domain check (TANH follows this pattern)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists all as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:396-428` — SIN/COS/TAN pattern (1-arg, no domain check) — TANH follows this
- `crates/xlstream-eval/src/builtins/math.rs:432-460` — ASIN/ACOS pattern (1-arg with NaN check) — useful reference for `finite_or_num` usage on COSH/SINH
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

All three are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**COSH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `x.cosh()`. For large |x| (roughly |x| > 710), result overflows to Infinity -> guard with `finite_or_num`, return `#NUM!`.
4. Return `Value::Number(result)`.

**SINH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `x.sinh()`. Same overflow concern as COSH -> guard with `finite_or_num`, return `#NUM!`.
4. Return `Value::Number(result)`.

**TANH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `x.tanh()`. Always in (-1, 1) for finite input — no overflow, no domain check needed.
4. Return `Value::Number(result)`.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- COSH/SINH: overflow for very large |x| -> `#NUM!`
- Error in arg: propagate

## Tests

### Unit tests (in `math.rs`)

**COSH happy path:**
- `cosh(0)` -> 1.0
- `cosh(1)` -> ~1.5430806348
- `cosh(-1)` -> ~1.5430806348 (symmetric)
- `cosh(5)` -> ~74.2099485248

**COSH edge/overflow:**
- `cosh(710)` -> very large finite number
- `cosh(711)` -> `#NUM!` (overflow to Infinity)
- `cosh(-711)` -> `#NUM!`

**SINH happy path:**
- `sinh(0)` -> 0.0
- `sinh(1)` -> ~1.1752011936
- `sinh(-1)` -> ~-1.1752011936

**SINH edge/overflow:**
- `sinh(710)` -> very large finite number
- `sinh(711)` -> `#NUM!` (overflow to Infinity)

**TANH happy path:**
- `tanh(0)` -> 0.0
- `tanh(0.5)` -> ~0.4621171573
- `tanh(-0.5)` -> ~-0.4621171573
- `tanh(100)` -> 1.0 (saturates)
- `tanh(-100)` -> -1.0

**Error propagation:**
- `cosh(#N/A)` -> `#N/A`
- `sinh(#N/A)` -> `#N/A`
- `tanh(#N/A)` -> `#N/A`

**Arity errors:**
- `cosh()` -> `#VALUE!`
- `cosh(1, 2)` -> `#VALUE!`

**Coercion:**
- `cosh(TRUE)` -> ~1.543 (TRUE = 1.0)
- `sinh("0")` -> 0.0

**Type mismatch:**
- `cosh("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/hyperbolic.xlsx`.

**Sheet1 data:**
- A: "x" header, rows 2-10: `0, 1, -1, 5, 0.5, -0.5, 100, 710, 711`
- B: "Error" header, row 2: `=NA()`
- C: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `5`

**Formulas (column D, starting row 2) — 25 formulas:**

COSH happy path (4):
1. `=COSH(A2)` -> 1 (cosh(0))
2. `=COSH(A3)` -> ~1.543 (cosh(1))
3. `=COSH(A4)` -> ~1.543 (cosh(-1), symmetric)
4. `=COSH(A5)` -> ~74.210 (cosh(5))

SINH happy path (3):
5. `=SINH(A2)` -> 0 (sinh(0))
6. `=SINH(A3)` -> ~1.175 (sinh(1))
7. `=SINH(A4)` -> ~-1.175 (sinh(-1))

TANH happy path (4):
8. `=TANH(A2)` -> 0 (tanh(0))
9. `=TANH(A6)` -> ~0.462 (tanh(0.5))
10. `=TANH(A7)` -> ~-0.462 (tanh(-0.5))
11. `=TANH(A8)` -> 1 (tanh(100), saturated)

COSH overflow (2):
12. `=COSH(A9)` -> very large number (cosh(710))
13. `=COSH(A10)` -> `#NUM!` (cosh(711), overflow)

SINH overflow (2):
14. `=SINH(A9)` -> very large number (sinh(710))
15. `=SINH(A10)` -> `#NUM!` (sinh(711), overflow)

TANH saturation (1):
16. `=TANH(-100)` -> -1 (saturated negative)

Error propagation (2):
17. `=COSH(B2)` -> `#N/A`
18. `=TANH(B2)` -> `#N/A`

Type error (1):
19. `=COSH(C2)` -> `#VALUE!`

Coercion (2):
20. `=COSH(TRUE)` -> ~1.543 (TRUE = 1)
21. `=SINH("0")` -> 0

Nested (2):
22. `=IF(TANH(1)>0.5, "big", "small")` -> "big"
23. `=IFERROR(COSH(711), "overflow")` -> "overflow"

Combined (1):
24. `=COSH(0)^2 - SINH(0)^2` -> 1 (identity: cosh^2 - sinh^2 = 1)

Cross-sheet (1):
25. `=COSH(Sheet2!A2)` -> ~74.210 (cosh(5))

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn hyperbolic()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "COSH, SINH, TANH hyperbolic functions" under `[Unreleased]` |
| `docs/functions.md` | Change COSH, SINH, TANH from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the COSH / SINH / TANH checkbox |

## Streaming invariant

Does not violate. All three are pure scalar functions of a single argument — no cross-row reads, no prelude dependency.
