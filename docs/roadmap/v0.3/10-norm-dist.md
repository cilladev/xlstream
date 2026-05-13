# Feature: NORM.DIST / NORM.INV

**Branch:** `feat/norm-dist`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

`NORM.DIST(x, mean, standard_dev, cumulative)` returns the normal distribution PDF or CDF. `NORM.INV(probability, mean, standard_dev)` returns the inverse CDF (quantile function).

```
=NORM.DIST(42, 40, 1.5, TRUE)   → 0.9087888   (CDF: P(X ≤ 42))
=NORM.DIST(42, 40, 1.5, FALSE)  → 0.1094687   (PDF: density at x=42)
=NORM.INV(0.9087888, 40, 1.5)   → 42.0         (inverse CDF)
```

Currently these return `#VALUE!` (unknown function).

## What already exists

- `statistical.rs` holds all statistical builtins (`var_s`, `stdev_s`, `skew`, `kurt`, etc.)
- `collect_numerics` and `finite_or_num` helpers in `statistical.rs`
- Math builtins in `math.rs` use `num_arg` helper + `eval_args` dispatch pattern — distribution functions should follow the same scalar pattern
- `coerce::to_number` in xlstream-core for arg coercion
- Conformance test harness in `tests/conformance/statistical.rs`

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — add implementation here
- `crates/xlstream-eval/src/builtins/mod.rs:87-248` — dispatch table, add entries using `eval_args` pattern
- `crates/xlstream-eval/src/builtins/math.rs:18-26` — `num_arg` helper pattern for scalar builtins
- `crates/xlstream-eval/tests/conformance/statistical.rs` — add conformance test
- `docs/functions.md:190-191` — mark as implemented

## Resolution / Evaluation behavior

Pure scalar functions. All args evaluated eagerly via `eval_args`. No range expansion, no prelude interaction. Classified as row-local.

Dispatch pattern (same as ROUND, SQRT, etc.):
```
"NORM.DIST" => Some(statistical::builtin_norm_dist(&eval_args(args, interp, scope))),
"NORM.INV" => Some(statistical::builtin_norm_inv(&eval_args(args, interp, scope))),
```

### Mathematical foundations

**NORM.DIST CDF:** `Φ((x - mean) / stdev)` where `Φ(z) = 0.5 * (1 + erf(z / √2))`

**NORM.DIST PDF:** `(1 / (stdev * √(2π))) * exp(-0.5 * ((x - mean) / stdev)²)`

**NORM.INV:** Inverse of the CDF. Common algorithms: rational approximation (Abramowitz & Stegun 26.2.23) or Peter Acklam's algorithm. Must handle tails accurately (probability near 0 or 1).

The agent must decide whether to implement erf/erfc from scratch (rational approximation, ~30 lines) or evaluate pulling in a dependency. Document the choice in the PR description.

## Tests

### Unit tests

**Happy path:**
- `norm_dist_cdf_positive_z` — x above mean, cumulative=true
- `norm_dist_cdf_negative_z` — x below mean, cumulative=true
- `norm_dist_cdf_at_mean` — x=mean returns 0.5 (cumulative=true)
- `norm_dist_pdf_at_mean` — peak density at x=mean
- `norm_dist_pdf_symmetry` — PDF(mean-d) == PDF(mean+d)
- `norm_inv_median` — probability=0.5 returns mean
- `norm_inv_round_trip` — NORM.INV(NORM.DIST(x, m, s, TRUE), m, s) ≈ x

**Edge cases:**
- `norm_dist_stdev_zero_returns_num` — standard_dev=0 → #NUM!
- `norm_dist_stdev_negative_returns_num` — standard_dev<0 → #NUM!
- `norm_inv_p_zero_returns_num` — probability=0 → #NUM!
- `norm_inv_p_one_returns_num` — probability=1 → #NUM!
- `norm_inv_p_negative_returns_num` — probability<0 → #NUM!
- `norm_inv_p_above_one_returns_num` — probability>1 → #NUM!
- `norm_dist_large_z_cdf_near_one` — very large x, CDF ≈ 1.0
- `norm_dist_large_negative_z_cdf_near_zero` — very negative x, CDF ≈ 0.0

**Error propagation:**
- `norm_dist_propagates_error` — #N/A in any arg propagates
- `norm_inv_propagates_error` — #N/A in any arg propagates

**Arg count:**
- `norm_dist_wrong_arg_count_returns_value` — !=4 args → #VALUE!
- `norm_inv_wrong_arg_count_returns_value` — !=3 args → #VALUE!

**Boolean cumulative arg:**
- `norm_dist_cumulative_true_gives_cdf` — TRUE → CDF
- `norm_dist_cumulative_false_gives_pdf` — FALSE → PDF

### Conformance (norm_dist.xlsx)

**Data layout:** Column A has x values (−3, −1, 0, 1, 2, 3, 5), mean=0 in B1, stdev=1 in C1.

**Formulas (20+):**

Happy path CDF:
1. `=NORM.DIST(0, 0, 1, TRUE)` → 0.5
2. `=NORM.DIST(1, 0, 1, TRUE)` → 0.841345
3. `=NORM.DIST(-1, 0, 1, TRUE)` → 0.158655
4. `=NORM.DIST(2, 0, 1, TRUE)` → 0.977250
5. `=NORM.DIST(42, 40, 1.5, TRUE)` → 0.908789

Happy path PDF:
6. `=NORM.DIST(0, 0, 1, FALSE)` → 0.398942
7. `=NORM.DIST(1, 0, 1, FALSE)` → 0.241971
8. `=NORM.DIST(42, 40, 1.5, FALSE)` → 0.109469

Non-standard mean/stdev:
9. `=NORM.DIST(50, 50, 10, TRUE)` → 0.5
10. `=NORM.DIST(60, 50, 10, TRUE)` → 0.841345

Inverse:
11. `=NORM.INV(0.5, 0, 1)` → 0
12. `=NORM.INV(0.841345, 0, 1)` → ~1.0
13. `=NORM.INV(0.025, 0, 1)` → −1.95996
14. `=NORM.INV(0.975, 0, 1)` → 1.95996
15. `=NORM.INV(0.5, 100, 15)` → 100

Boundary:
16. `=NORM.DIST(-5, 0, 1, TRUE)` → ~0.0000003
17. `=NORM.DIST(5, 0, 1, TRUE)` → ~0.9999997

Nested:
18. `=IF(NORM.DIST(1,0,1,TRUE)>0.8, "high", "low")` → "high"
19. `=ROUND(NORM.INV(0.5,0,1), 2)` → 0

Error:
20. `=NORM.DIST(1, 0, 0, TRUE)` → #NUM!
21. `=NORM.INV(1.5, 0, 1)` → #NUM!

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add NORM.DIST, NORM.INV |
| `docs/functions.md` | Mark NORM.DIST, NORM.INV as `x` |
| `docs/roadmap/v0.3/README.md` | Tick the checkbox |

## Streaming invariant

No violation. Pure scalar function of its arguments. No cell reads beyond current row.
