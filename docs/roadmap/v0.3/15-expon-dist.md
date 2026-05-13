# Feature: EXPON.DIST

**Branch:** `feat/expon-dist`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

`EXPON.DIST(x, lambda, cumulative)` returns the exponential distribution PDF or CDF.

```
=EXPON.DIST(1, 1.5, TRUE)   → 0.77687  (CDF: P(X ≤ 1))
=EXPON.DIST(1, 1.5, FALSE)  → 0.33470  (PDF: density at x=1)
=EXPON.DIST(0, 1.5, TRUE)   → 0        (CDF at 0)
=EXPON.DIST(0, 1.5, FALSE)  → 1.5      (PDF at 0 = lambda)
```

Currently returns `#VALUE!` (unknown function).

## What already exists

- `statistical.rs` holds all statistical builtins
- `eval_args` scalar dispatch pattern in `mod.rs`
- `finite_or_num` helper for overflow guard
- `f64::exp()` in std — no special math needed

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — add implementation here
- `crates/xlstream-eval/src/builtins/mod.rs:87-248` — dispatch table
- `docs/functions.md:153` — mark as implemented

## Resolution / Evaluation behavior

Pure scalar function. Eagerly evaluated via `eval_args`. No range expansion, no prelude.

```
"EXPON.DIST" => Some(statistical::builtin_expon_dist(&eval_args(args, interp, scope))),
```

### Mathematical foundations

**CDF:** `F(x) = 1 − e^(−λx)` for x ≥ 0

**PDF:** `f(x) = λ * e^(−λx)` for x ≥ 0

Both are trivial — std `f64::exp()` is the only operation needed. No special functions.

**Arg validation:** `x` ≥ 0, `lambda` > 0. Excel returns `#NUM!` for negative x or non-positive lambda.

## Tests

### Unit tests

**Happy path:**
- `expon_cdf_typical` — EXPON.DIST(1, 1.5, TRUE) → 0.77687
- `expon_pdf_typical` — EXPON.DIST(1, 1.5, FALSE) → 0.33470
- `expon_cdf_at_zero` — EXPON.DIST(0, λ, TRUE) → 0
- `expon_pdf_at_zero` — EXPON.DIST(0, λ, FALSE) → λ
- `expon_cdf_large_x` — EXPON.DIST(10, 1, TRUE) → ~1.0

**Edge cases:**
- `expon_x_negative_returns_num` — x<0 → #NUM!
- `expon_lambda_zero_returns_num` — λ=0 → #NUM!
- `expon_lambda_negative_returns_num` — λ<0 → #NUM!
- `expon_very_large_lambda` — λ=1000, x=1 → CDF ≈ 1.0
- `expon_very_small_lambda` — λ=0.001, x=1 → CDF ≈ 0.001

**Error propagation:**
- `expon_propagates_error`

**Arg count:**
- `expon_wrong_arg_count` — !=3 → #VALUE!

### Conformance (expon_dist.xlsx)

**Data layout:** Column A has x values (0, 0.5, 1, 2, 3, 5, 10). lambda=1.5 in B1.

**Formulas (18+):**

CDF:
1. `=EXPON.DIST(0, 1.5, TRUE)` → 0
2. `=EXPON.DIST(0.5, 1.5, TRUE)` → 0.527633
3. `=EXPON.DIST(1, 1.5, TRUE)` → 0.776870
4. `=EXPON.DIST(2, 1.5, TRUE)` → 0.950213
5. `=EXPON.DIST(5, 1.5, TRUE)` → 0.999453

PDF:
6. `=EXPON.DIST(0, 1.5, FALSE)` → 1.5
7. `=EXPON.DIST(0.5, 1.5, FALSE)` → 0.708551
8. `=EXPON.DIST(1, 1.5, FALSE)` → 0.334695
9. `=EXPON.DIST(2, 1.5, FALSE)` → 0.074682

Different lambda:
10. `=EXPON.DIST(1, 1, TRUE)` → 0.632121
11. `=EXPON.DIST(1, 1, FALSE)` → 0.367879
12. `=EXPON.DIST(1, 0.5, TRUE)` → 0.393469
13. `=EXPON.DIST(1, 3, TRUE)` → 0.950213

Large x:
14. `=EXPON.DIST(10, 1, TRUE)` → 0.999955
15. `=EXPON.DIST(100, 1, TRUE)` → ~1.0

Nested:
16. `=ROUND(EXPON.DIST(1, 1.5, TRUE), 4)` → 0.7769
17. `=IF(EXPON.DIST(2, 1, TRUE)>0.8, "likely", "unlikely")` → "likely"

Error:
18. `=EXPON.DIST(-1, 1.5, TRUE)` → #NUM!
19. `=EXPON.DIST(1, 0, TRUE)` → #NUM!
20. `=EXPON.DIST(1, -1, TRUE)` → #NUM!

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add EXPON.DIST |
| `docs/functions.md` | Mark EXPON.DIST as `x` |
| `docs/roadmap/v0.3/README.md` | Tick the checkbox |

## Streaming invariant

No violation. Pure scalar function. No cell reads beyond current row.
