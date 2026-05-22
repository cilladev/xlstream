# Feature: COT / CSC / SEC / COTH / CSCH / SECH

**Branch:** `feat/reciprocal-trig`
**Effort:** ~2.5 hours
**Crates:** xlstream-eval

## What

Reciprocal trigonometric and reciprocal hyperbolic functions. Each takes a single numeric argument and returns the reciprocal of the corresponding trig/hyperbolic function.

- `COT(x)` — cotangent: `1/tan(x)`. x = 0 is a pole.
- `CSC(x)` — cosecant: `1/sin(x)`. sin(x) = 0 is a pole.
- `SEC(x)` — secant: `1/cos(x)`. cos(x) = 0 is a pole.
- `COTH(x)` — hyperbolic cotangent: `cosh(x)/sinh(x)`. x = 0 is a pole.
- `CSCH(x)` — hyperbolic cosecant: `1/sinh(x)`. x = 0 is a pole.
- `SECH(x)` — hyperbolic secant: `1/cosh(x)`. Always defined (cosh >= 1).

```
=COT(1)          -> 0.642092616...
=CSC(1)          -> 1.188395106...
=SEC(0)          -> 1
=COTH(2)         -> 1.037314720...
=CSCH(1)         -> 0.850918128...
=SECH(0)         -> 1
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:396-428` — SIN/COS/TAN — the underlying functions used to compute reciprocals
- `crates/xlstream-eval/src/builtins/math.rs:154-170` — MOD with division-by-zero check (pattern for `#DIV/0!` handling)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists all as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:396-428` — SIN/COS/TAN (1-arg pattern)
- `crates/xlstream-eval/src/builtins/math.rs:154-170` — MOD `#DIV/0!` pattern
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

All six are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**COT(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. If x == 0.0 -> return `#DIV/0!` (Excel behavior: COT(0) is division by zero).
4. Compute `x.cos() / x.sin()`. Guard result with `finite_or_num` -> `#NUM!` on overflow.
5. Return `Value::Number(result)`.

**CSC(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `s = x.sin()`. If s == 0.0 -> return `#DIV/0!`.
4. Compute `1.0 / s`. Guard with `finite_or_num`.
5. Return `Value::Number(result)`.

**SEC(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `c = x.cos()`. If c == 0.0 -> return `#DIV/0!`.
4. Compute `1.0 / c`. Guard with `finite_or_num`.
5. Return `Value::Number(result)`.

**COTH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. If x == 0.0 -> return `#DIV/0!`.
4. Compute `x.cosh() / x.sinh()`. Guard with `finite_or_num`.
5. Return `Value::Number(result)`.

**CSCH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. If x == 0.0 -> return `#DIV/0!`.
4. Compute `1.0 / x.sinh()`. Guard with `finite_or_num`.
5. Return `Value::Number(result)`.

**SECH(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `1.0 / x.cosh()`. cosh(x) >= 1 for all real x, so no division-by-zero possible. For very large |x|, cosh overflows to Infinity and 1/Inf = 0, which is fine.
4. Guard with `finite_or_num` defensively.
5. Return `Value::Number(result)`.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- COT(0): `#DIV/0!`
- CSC(n*PI): `#DIV/0!` (where sin = 0; in practice, floating-point sin(PI) is not exactly 0, so only CSC(0) reliably triggers this — match Excel)
- SEC(PI/2): theoretically `#DIV/0!` but floating-point cos(PI/2) is not exactly 0 — match Excel behavior
- COTH(0): `#DIV/0!`
- CSCH(0): `#DIV/0!`
- SECH: no division-by-zero possible
- Overflow from `finite_or_num`: `#NUM!`
- Error in arg: propagate

**Note on floating-point near-zero:** For CSC and SEC at theoretical poles (multiples of PI), floating-point imprecision means `sin(PI)` and `cos(PI/2)` are not exactly zero — they return very large but finite values. This matches Excel's behavior. Only exact zero triggers `#DIV/0!`.

## Tests

### Unit tests (in `math.rs`)

**COT happy path:**
- `cot(1)` -> ~0.6420926160
- `cot(PI/4)` -> ~1.0

**COT division by zero:**
- `cot(0)` -> `#DIV/0!`

**CSC happy path:**
- `csc(1)` -> ~1.1883951058
- `csc(PI/2)` -> ~1.0
- `csc(PI/6)` -> ~2.0

**CSC division by zero:**
- `csc(0)` -> `#DIV/0!`

**SEC happy path:**
- `sec(0)` -> 1.0
- `sec(PI/3)` -> ~2.0
- `sec(1)` -> ~1.8508157177

**COTH happy path:**
- `coth(1)` -> ~1.3130352855
- `coth(2)` -> ~1.0373147207

**COTH division by zero:**
- `coth(0)` -> `#DIV/0!`

**CSCH happy path:**
- `csch(1)` -> ~0.8509181282
- `csch(-1)` -> ~-0.8509181282

**CSCH division by zero:**
- `csch(0)` -> `#DIV/0!`

**SECH happy path:**
- `sech(0)` -> 1.0
- `sech(1)` -> ~0.6480542737
- `sech(100)` -> ~0.0 (very small)

**Error propagation:**
- `cot(#N/A)` -> `#N/A`
- `csc(#N/A)` -> `#N/A`
- `sech(#N/A)` -> `#N/A`

**Arity errors:**
- `cot()` -> `#VALUE!`
- `csc(1, 2)` -> `#VALUE!`

**Coercion:**
- `cot(TRUE)` -> ~0.642 (TRUE = 1.0)
- `csc("1")` -> ~1.188

**Type mismatch:**
- `cot("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/reciprocal_trig.xlsx`.

**Sheet1 data:**
- A: "x" header, rows 2-12: `1, 0, 0.5, -1, 2, -0.5, 100, 0.523598776 (PI/6), 1.047197551 (PI/3), 1.570796327 (PI/2), 0.785398163 (PI/4)`
- B: "Error" header, row 2: `=NA()`
- C: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `1`

**Formulas (column D, starting row 2) — 30 formulas:**

COT happy path (2):
1. `=COT(A2)` -> ~0.642 (cot(1))
2. `=COT(A12)` -> ~1.0 (cot(PI/4))

COT error (1):
3. `=COT(A3)` -> `#DIV/0!` (cot(0))

CSC happy path (2):
4. `=CSC(A2)` -> ~1.188 (csc(1))
5. `=CSC(A9)` -> ~2.0 (csc(PI/6))

CSC error (1):
6. `=CSC(A3)` -> `#DIV/0!` (csc(0))

SEC happy path (2):
7. `=SEC(A3)` -> 1 (sec(0))
8. `=SEC(A10)` -> ~2.0 (sec(PI/3))

COTH happy path (2):
9. `=COTH(A2)` -> ~1.313 (coth(1))
10. `=COTH(A6)` -> ~1.037 (coth(2))

COTH error (1):
11. `=COTH(A3)` -> `#DIV/0!` (coth(0))

CSCH happy path (2):
12. `=CSCH(A2)` -> ~0.851 (csch(1))
13. `=CSCH(A5)` -> ~-0.851 (csch(-1))

CSCH error (1):
14. `=CSCH(A3)` -> `#DIV/0!` (csch(0))

SECH happy path (3):
15. `=SECH(A3)` -> 1 (sech(0))
16. `=SECH(A2)` -> ~0.648 (sech(1))
17. `=SECH(A8)` -> ~0.0 (sech(100))

Error propagation (3):
18. `=COT(B2)` -> `#N/A`
19. `=CSC(B2)` -> `#N/A`
20. `=SECH(B2)` -> `#N/A`

Type error (1):
21. `=COT(C2)` -> `#VALUE!`

Coercion (2):
22. `=COT(TRUE)` -> ~0.642 (TRUE = 1)
23. `=CSC("1")` -> ~1.188

Nested (2):
24. `=IF(SECH(0)=1, "yes", "no")` -> "yes"
25. `=IFERROR(COT(0), "div0")` -> "div0"

Combined (2):
26. `=SIN(1)*CSC(1)` -> 1 (sin * 1/sin = 1)
27. `=COS(1)*SEC(1)` -> 1 (cos * 1/cos = 1)

Cross-sheet (1):
28. `=COT(Sheet2!A2)` -> ~0.642 (cot(1))

Additional edge (2):
29. `=CSC(A11)` -> ~1.0 (csc(PI/2))
30. `=CSCH(-1)` -> ~-0.851

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn reciprocal_trig()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "COT, CSC, SEC, COTH, CSCH, SECH reciprocal trig/hyperbolic functions" under `[Unreleased]` |
| `docs/functions.md` | Change COT, CSC, SEC, COTH, CSCH, SECH from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the COT / CSC / SEC / COTH / CSCH / SECH checkbox |

## Streaming invariant

Does not violate. All six are pure scalar functions of a single argument — no cross-row reads, no prelude dependency.
