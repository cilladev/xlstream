# Feature: POISSON.DIST

**Branch:** `feat/poisson-dist`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

`POISSON.DIST(x, mean, cumulative)` returns the Poisson distribution PMF or CDF.

```
=POISSON.DIST(3, 5, FALSE)  → 0.14037  (PMF: P(X = 3))
=POISSON.DIST(3, 5, TRUE)   → 0.26503  (CDF: P(X ≤ 3))
```

Currently returns `#VALUE!` (unknown function).

## What already exists

- `statistical.rs` holds all statistical builtins
- `eval_args` scalar dispatch pattern in `mod.rs`
- `finite_or_num` helper for overflow guard
- If BINOM.DIST (spec 13) lands first, its log-gamma machinery can be reused

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — add implementation here
- `crates/xlstream-eval/src/builtins/mod.rs:87-248` — dispatch table
- `docs/functions.md:202` — mark as implemented

## Resolution / Evaluation behavior

Pure scalar function. Eagerly evaluated via `eval_args`. No range expansion, no prelude.

```
"POISSON.DIST" => Some(statistical::builtin_poisson_dist(&eval_args(args, interp, scope))),
```

### Mathematical foundations

**PMF:** `P(X=k) = e^(−λ) * λ^k / k!`

Use log-space to avoid overflow: `ln(PMF) = −λ + k*ln(λ) − ln_gamma(k+1)`

**CDF:** Sum PMF from 0 to k. Alternatively, use the regularized incomplete gamma function: `CDF(k, λ) = 1 − P(k+1, λ) / Γ(k+1)`. The direct summation is simpler and sufficient for typical k values (< 1000).

**Arg validation:** `x` is truncated to integer. `x` ≥ 0, `mean` > 0 (Excel allows mean=0 only when x=0).

## Tests

### Unit tests

**Happy path:**
- `poisson_pmf_typical` — POISSON.DIST(3, 5, FALSE)
- `poisson_cdf_typical` — POISSON.DIST(3, 5, TRUE)
- `poisson_pmf_zero` — POISSON.DIST(0, 5, FALSE) → e^(−5) ≈ 0.00674
- `poisson_cdf_zero` — POISSON.DIST(0, 5, TRUE) → e^(−5)
- `poisson_pmf_equals_mean` — POISSON.DIST(5, 5, FALSE) → peak region

**Edge cases:**
- `poisson_mean_zero_x_zero` — POISSON.DIST(0, 0, FALSE) → 1.0 (degenerate)
- `poisson_mean_zero_x_positive_returns_zero` — POISSON.DIST(1, 0, FALSE) → 0.0 (or verify Excel behavior)
- `poisson_x_negative_returns_num` — x<0 → #NUM!
- `poisson_mean_negative_returns_num` — mean<0 → #NUM!
- `poisson_non_integer_x_truncates` — x=3.7 treated as 3
- `poisson_large_mean` — mean=100, x=100, verify no overflow
- `poisson_large_x` — x=200, mean=100, verify CDF near 1.0

**Error propagation:**
- `poisson_propagates_error`

**Arg count:**
- `poisson_wrong_arg_count` — !=3 → #VALUE!

### Conformance (poisson_dist.xlsx)

**Data layout:** Column A has x values (0, 1, 2, 3, 5, 10, 20). mean=5 in B1.

**Formulas (20+):**

PMF:
1. `=POISSON.DIST(0, 5, FALSE)` → 0.006738
2. `=POISSON.DIST(1, 5, FALSE)` → 0.033690
3. `=POISSON.DIST(3, 5, FALSE)` → 0.140374
4. `=POISSON.DIST(5, 5, FALSE)` → 0.175467
5. `=POISSON.DIST(10, 5, FALSE)` → 0.018133

CDF:
6. `=POISSON.DIST(0, 5, TRUE)` → 0.006738
7. `=POISSON.DIST(3, 5, TRUE)` → 0.265026
8. `=POISSON.DIST(5, 5, TRUE)` → 0.615961
9. `=POISSON.DIST(10, 5, TRUE)` → 0.986305
10. `=POISSON.DIST(20, 5, TRUE)` → ~1.0

Different mean:
11. `=POISSON.DIST(0, 1, FALSE)` → 0.367879
12. `=POISSON.DIST(1, 1, TRUE)` → 0.735759
13. `=POISSON.DIST(3, 10, FALSE)` → 0.007567
14. `=POISSON.DIST(10, 10, TRUE)` → 0.583040

Boundary:
15. `=POISSON.DIST(0, 0, FALSE)` → 1.0
16. `=POISSON.DIST(0, 0, TRUE)` → 1.0

Large mean:
17. `=POISSON.DIST(100, 100, TRUE)` → ~0.527
18. `=POISSON.DIST(50, 100, TRUE)` → ~0.0 (very small)

Nested:
19. `=ROUND(POISSON.DIST(3, 5, FALSE), 4)` → 0.1404
20. `=IF(POISSON.DIST(10, 5, TRUE)>0.95, "rare", "common")` → "rare"

Error:
21. `=POISSON.DIST(-1, 5, FALSE)` → #NUM!
22. `=POISSON.DIST(3, -1, FALSE)` → #NUM!

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add POISSON.DIST |
| `docs/functions.md` | Mark POISSON.DIST as `x` |
| `docs/roadmap/v0.3/README.md` | Tick the checkbox |

## Streaming invariant

No violation. Pure scalar function. No cell reads beyond current row.
