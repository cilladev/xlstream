# Feature: BINOM.DIST / BINOM.INV

**Branch:** `feat/binom-dist`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

`BINOM.DIST(number_s, trials, probability_s, cumulative)` returns the binomial distribution PMF or CDF. `BINOM.INV(trials, probability_s, alpha)` returns the smallest k where CDF ≥ alpha (inverse CDF / critical value).

```
=BINOM.DIST(3, 10, 0.5, FALSE)  → 0.11719  (PMF: P(X = 3))
=BINOM.DIST(3, 10, 0.5, TRUE)   → 0.17188  (CDF: P(X ≤ 3))
=BINOM.INV(10, 0.5, 0.17188)    → 3        (smallest k where CDF ≥ alpha)
```

Currently these return `#VALUE!` (unknown function).

## What already exists

- `statistical.rs` holds all statistical builtins
- `eval_args` scalar dispatch pattern in `mod.rs`
- `finite_or_num` helper for overflow guard
- `coerce::to_number` for arg coercion

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — add implementation here
- `crates/xlstream-eval/src/builtins/mod.rs:87-248` — dispatch table
- `docs/functions.md:134-136` — mark as implemented

## Resolution / Evaluation behavior

Pure scalar functions. Eagerly evaluated via `eval_args`. No range expansion, no prelude.

```
"BINOM.DIST" => Some(statistical::builtin_binom_dist(&eval_args(args, interp, scope))),
"BINOM.INV" => Some(statistical::builtin_binom_inv(&eval_args(args, interp, scope))),
```

### Mathematical foundations

**BINOM.DIST PMF:** `P(X=k) = C(n,k) * p^k * (1−p)^(n−k)`

Use log-space to avoid overflow: `ln(PMF) = ln_gamma(n+1) − ln_gamma(k+1) − ln_gamma(n−k+1) + k*ln(p) + (n−k)*ln(1−p)`

**BINOM.DIST CDF:** Sum of PMF from 0 to k. Can also use the regularized incomplete beta function: `CDF(k, n, p) = I_{1−p}(n−k, k+1)`. If the incomplete beta is already implemented for T.DIST (spec 12), reuse it.

**BINOM.INV:** Linear search from k=0 upward, accumulating CDF until CDF ≥ alpha. For large n, binary search is faster but linear is simpler and n is typically small in practice.

**Arg validation:** `number_s` and `trials` are truncated to integers. `trials` ≥ 0, `number_s` in [0, trials], `probability_s` in [0, 1].

## Tests

### Unit tests

**Happy path:**
- `binom_dist_pmf_fair_coin` — BINOM.DIST(5, 10, 0.5, FALSE) → 0.24609
- `binom_dist_cdf_fair_coin` — BINOM.DIST(5, 10, 0.5, TRUE) → 0.62305
- `binom_dist_pmf_zero_successes` — BINOM.DIST(0, 10, 0.5, FALSE) → 0.000977
- `binom_dist_pmf_all_successes` — BINOM.DIST(10, 10, 0.5, FALSE) → 0.000977
- `binom_inv_fair_coin` — BINOM.INV(10, 0.5, 0.5) → 5
- `binom_inv_alpha_near_zero` — BINOM.INV(10, 0.5, 0.001) → 0

**Edge cases:**
- `binom_dist_p_zero` — p=0: PMF(0)=1, PMF(k>0)=0
- `binom_dist_p_one` — p=1: PMF(n)=1, PMF(k<n)=0
- `binom_dist_trials_zero` — n=0, k=0 → PMF=1
- `binom_dist_non_integer_truncates` — number_s=3.7 treated as 3
- `binom_dist_k_negative_returns_num` — k<0 → #NUM!
- `binom_dist_k_exceeds_n_returns_num` — k>n → #NUM!
- `binom_dist_p_negative_returns_num` — p<0 → #NUM!
- `binom_dist_p_above_one_returns_num` — p>1 → #NUM!
- `binom_dist_n_negative_returns_num` — n<0 → #NUM!
- `binom_inv_alpha_zero_returns_num` — alpha=0 → #NUM! (or returns 0, verify against Excel)
- `binom_inv_alpha_above_one_returns_num` — alpha>1 → #NUM!

**Error propagation:**
- `binom_dist_propagates_error`
- `binom_inv_propagates_error`

**Arg count:**
- `binom_dist_wrong_arg_count` — !=4 → #VALUE!
- `binom_inv_wrong_arg_count` — !=3 → #VALUE!

### Conformance (binom_dist.xlsx)

**Data layout:** trials=10 in B1, probability=0.5 in C1. Column A has k values (0, 1, 2, 3, 5, 7, 10).

**Formulas (22+):**

PMF:
1. `=BINOM.DIST(0, 10, 0.5, FALSE)` → 0.000977
2. `=BINOM.DIST(1, 10, 0.5, FALSE)` → 0.009766
3. `=BINOM.DIST(5, 10, 0.5, FALSE)` → 0.246094
4. `=BINOM.DIST(10, 10, 0.5, FALSE)` → 0.000977
5. `=BINOM.DIST(3, 10, 0.3, FALSE)` → 0.266828

CDF:
6. `=BINOM.DIST(5, 10, 0.5, TRUE)` → 0.623047
7. `=BINOM.DIST(0, 10, 0.5, TRUE)` → 0.000977
8. `=BINOM.DIST(10, 10, 0.5, TRUE)` → 1.0
9. `=BINOM.DIST(3, 10, 0.3, TRUE)` → 0.649613
10. `=BINOM.DIST(7, 20, 0.3, TRUE)` → 0.772019

Inverse:
11. `=BINOM.INV(10, 0.5, 0.5)` → 5
12. `=BINOM.INV(10, 0.5, 0.623)` → 5
13. `=BINOM.INV(10, 0.5, 0.624)` → 6
14. `=BINOM.INV(10, 0.5, 0.001)` → 0
15. `=BINOM.INV(10, 0.3, 0.5)` → 3

Boundary:
16. `=BINOM.DIST(0, 10, 0, FALSE)` → 1.0
17. `=BINOM.DIST(0, 10, 1, FALSE)` → 0.0
18. `=BINOM.DIST(10, 10, 1, FALSE)` → 1.0
19. `=BINOM.DIST(0, 0, 0.5, FALSE)` → 1.0

Nested:
20. `=ROUND(BINOM.DIST(5, 10, 0.5, FALSE), 4)` → 0.2461
21. `=IF(BINOM.DIST(8, 10, 0.5, TRUE)>0.95, "unlikely", "plausible")` → "plausible"

Error:
22. `=BINOM.DIST(-1, 10, 0.5, FALSE)` → #NUM!
23. `=BINOM.DIST(11, 10, 0.5, FALSE)` → #NUM!

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add BINOM.DIST, BINOM.INV |
| `docs/functions.md` | Mark BINOM.DIST, BINOM.INV as `x` |
| `docs/roadmap/v0.3/README.md` | Tick the checkbox |

## Streaming invariant

No violation. Pure scalar functions. No cell reads beyond current row.
