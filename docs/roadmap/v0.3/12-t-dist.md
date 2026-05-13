# Feature: T.DIST / T.INV / T.DIST.2T / T.DIST.RT

**Branch:** `feat/t-dist`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

Student's t-distribution functions:

| Function | Signature | Returns |
|---|---|---|
| `T.DIST(x, deg_freedom, cumulative)` | left-tail CDF or PDF | CDF: P(T ≤ x); PDF: density |
| `T.DIST.RT(x, deg_freedom)` | right-tail CDF | P(T ≥ x) = 1 − T.DIST(x, df, TRUE) |
| `T.DIST.2T(x, deg_freedom)` | two-tail CDF | P(\|T\| ≥ x) — x must be ≥ 0 |
| `T.INV(probability, deg_freedom)` | left-tail inverse | quantile for left-tail p |
| `T.INV.2T(probability, deg_freedom)` | two-tail inverse | quantile for two-tail p |

```
=T.DIST(1.0, 10, TRUE)    → 0.82955   (left-tail CDF, 10 df)
=T.DIST(1.0, 10, FALSE)   → 0.23036   (PDF)
=T.DIST.RT(1.0, 10)       → 0.17045   (right-tail)
=T.DIST.2T(1.0, 10)       → 0.34090   (two-tail)
=T.INV(0.95, 10)           → 1.81246   (left-tail inverse)
=T.INV.2T(0.05, 10)        → 2.22814   (two-tail inverse)
```

Currently all return `#VALUE!` (unknown function).

## What already exists

- `statistical.rs` holds all statistical builtins
- `eval_args` + `num_arg`-style scalar dispatch pattern
- NORM.DIST / NORM.INV (spec 10) will provide erf/erfc infrastructure that the t-distribution can build on — the t-distribution CDF uses the regularized incomplete beta function, not erf directly, but the inverse can use the normal inverse as an initial guess.

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — add implementation here
- `crates/xlstream-eval/src/builtins/mod.rs:87-248` — dispatch table
- `docs/functions.md:219-223` — mark as implemented

## Resolution / Evaluation behavior

Pure scalar functions. Eagerly evaluated via `eval_args`. No range expansion, no prelude.

```
"T.DIST" => Some(statistical::builtin_t_dist(&eval_args(args, interp, scope))),
"T.DIST.RT" => Some(statistical::builtin_t_dist_rt(&eval_args(args, interp, scope))),
"T.DIST.2T" => Some(statistical::builtin_t_dist_2t(&eval_args(args, interp, scope))),
"T.INV" => Some(statistical::builtin_t_inv(&eval_args(args, interp, scope))),
"T.INV.2T" => Some(statistical::builtin_t_inv_2t(&eval_args(args, interp, scope))),
```

### Mathematical foundations

**T.DIST CDF:** Uses the regularized incomplete beta function I_x(a, b):
`CDF(t, ν) = 1 − 0.5 * I_{ν/(ν+t²)}(ν/2, 0.5)` for t ≥ 0, with symmetry for t < 0.

**T.DIST PDF:** `f(t, ν) = Γ((ν+1)/2) / (√(νπ) * Γ(ν/2)) * (1 + t²/ν)^(−(ν+1)/2)`

**T.INV:** Newton-Raphson iteration using T.DIST CDF and PDF, with initial guess from NORM.INV.

The regularized incomplete beta function is the key building block. Common approach: continued fraction expansion (Lentz's algorithm). The agent must decide whether to implement this from scratch (~50-80 lines) or evaluate a dependency. Document the choice.

**Degrees of freedom:** Excel accepts non-integer df and truncates to integer. The agent should verify this behavior.

## Tests

### Unit tests

**Happy path:**
- `t_dist_cdf_positive` — T.DIST(1, 10, TRUE)
- `t_dist_cdf_at_zero` — T.DIST(0, df, TRUE) = 0.5 (symmetric)
- `t_dist_pdf_at_zero` — peak density
- `t_dist_rt_positive` — T.DIST.RT(1, 10) = 1 − T.DIST(1, 10, TRUE)
- `t_dist_2t_positive` — T.DIST.2T(1, 10) = 2 * T.DIST.RT(1, 10)
- `t_inv_median` — T.INV(0.5, df) = 0
- `t_inv_2t_symmetric` — T.INV.2T(p, df) = −T.INV(p/2, df)

**Edge cases:**
- `t_dist_df_one_is_cauchy` — df=1 gives Cauchy distribution
- `t_dist_large_df_approaches_normal` — df=1000, CDF ≈ NORM.S.DIST
- `t_dist_2t_x_negative_returns_num` — x < 0 → #NUM!
- `t_dist_df_zero_returns_num` — df ≤ 0 → #NUM!
- `t_inv_p_zero_returns_num` — p=0 → #NUM!
- `t_inv_p_one_returns_num` — p=1 → #NUM!
- `t_inv_2t_p_zero_returns_num` — p=0 → #NUM!
- `t_inv_2t_p_above_one_returns_num` — p>1 → #NUM!
- `t_dist_non_integer_df_truncates` — df=10.7 treated as df=10

**Error propagation:**
- `t_dist_propagates_error` — #N/A in any arg propagates
- `t_inv_propagates_error`

**Arg count:**
- `t_dist_wrong_arg_count` — !=3 → #VALUE!
- `t_dist_rt_wrong_arg_count` — !=2 → #VALUE!
- `t_dist_2t_wrong_arg_count` — !=2 → #VALUE!
- `t_inv_wrong_arg_count` — !=2 → #VALUE!
- `t_inv_2t_wrong_arg_count` — !=2 → #VALUE!

### Conformance (t_dist.xlsx)

**Data layout:** Column A has t values (−3, −1, 0, 1, 2, 3). Column B has df values (1, 5, 10, 30).

**Formulas (25+):**

T.DIST CDF:
1. `=T.DIST(0, 10, TRUE)` → 0.5
2. `=T.DIST(1, 10, TRUE)` → 0.82955
3. `=T.DIST(-1, 10, TRUE)` → 0.17045
4. `=T.DIST(2, 10, TRUE)` → 0.96317
5. `=T.DIST(1, 1, TRUE)` → 0.75 (Cauchy)
6. `=T.DIST(1, 30, TRUE)` → 0.83727

T.DIST PDF:
7. `=T.DIST(0, 10, FALSE)` → 0.38909
8. `=T.DIST(1, 10, FALSE)` → 0.23036

T.DIST.RT:
9. `=T.DIST.RT(1, 10)` → 0.17045
10. `=T.DIST.RT(2, 10)` → 0.03683
11. `=T.DIST.RT(0, 10)` → 0.5

T.DIST.2T:
12. `=T.DIST.2T(1, 10)` → 0.34090
13. `=T.DIST.2T(2, 10)` → 0.07366
14. `=T.DIST.2T(0, 10)` → 1.0

T.INV:
15. `=T.INV(0.5, 10)` → 0
16. `=T.INV(0.95, 10)` → 1.81246
17. `=T.INV(0.025, 10)` → −2.22814
18. `=T.INV(0.975, 10)` → 2.22814

T.INV.2T:
19. `=T.INV.2T(0.05, 10)` → 2.22814
20. `=T.INV.2T(0.10, 10)` → 1.81246
21. `=T.INV.2T(1.0, 10)` → 0

Large df (approaches normal):
22. `=T.DIST(1.96, 1000, TRUE)` → ~0.975
23. `=T.INV(0.975, 1000)` → ~1.962

Nested:
24. `=ROUND(T.DIST(0, 10, TRUE), 1)` → 0.5
25. `=IF(T.DIST.2T(2.5, 10)<0.05, "sig", "ns")` → "sig"

Error:
26. `=T.DIST.2T(-1, 10)` → #NUM!
27. `=T.INV(0, 10)` → #NUM!

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add T.DIST, T.DIST.RT, T.DIST.2T, T.INV, T.INV.2T |
| `docs/functions.md` | Mark all 5 as `x` |
| `docs/roadmap/v0.3/README.md` | Tick the checkbox |

## Streaming invariant

No violation. Pure scalar functions. No cell reads beyond current row.
