# Feature: NORM.S.DIST / NORM.S.INV

**Branch:** `feat/norm-s-dist`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

`NORM.S.DIST(z, cumulative)` returns the standard normal distribution (mean=0, stdev=1) PDF or CDF. `NORM.S.INV(probability)` returns the inverse CDF.

These are thin wrappers over NORM.DIST / NORM.INV with mean=0 and stdev=1.

```
=NORM.S.DIST(1.96, TRUE)   → 0.975002  (CDF)
=NORM.S.DIST(0, FALSE)     → 0.398942  (PDF at z=0)
=NORM.S.INV(0.975)         → 1.95996   (inverse CDF)
```

Currently these return `#VALUE!` (unknown function).

## What already exists

- Everything from spec 10-norm-dist — NORM.DIST / NORM.INV must be implemented first (or concurrently). NORM.S.DIST/INV delegate to the same underlying math.
- `statistical.rs` holds all statistical builtins
- `eval_args` dispatch pattern in `mod.rs`

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — add implementation here, reuse the erf/inverse-normal internals from NORM.DIST
- `crates/xlstream-eval/src/builtins/mod.rs:87-248` — dispatch table
- `docs/functions.md:192-193` — mark as implemented

## Resolution / Evaluation behavior

Pure scalar functions. Eagerly evaluated via `eval_args`. No range expansion, no prelude.

```
"NORM.S.DIST" => Some(statistical::builtin_norm_s_dist(&eval_args(args, interp, scope))),
"NORM.S.INV" => Some(statistical::builtin_norm_s_inv(&eval_args(args, interp, scope))),
```

Implementation: NORM.S.DIST(z, cum) = NORM.DIST(z, 0, 1, cum). NORM.S.INV(p) = NORM.INV(p, 0, 1). Delegate to the internal helpers, not the builtin wrappers.

## Tests

### Unit tests

**Happy path:**
- `norm_s_dist_cdf_zero` — z=0 returns 0.5
- `norm_s_dist_cdf_positive` — z=1.96 returns ~0.975
- `norm_s_dist_cdf_negative` — z=−1.96 returns ~0.025
- `norm_s_dist_pdf_zero` — z=0 returns ~0.39894
- `norm_s_inv_median` — p=0.5 returns 0
- `norm_s_inv_upper_tail` — p=0.975 returns ~1.96

**Edge cases:**
- `norm_s_inv_p_zero_returns_num` — p=0 → #NUM!
- `norm_s_inv_p_one_returns_num` — p=1 → #NUM!
- `norm_s_inv_p_negative_returns_num` — p<0 → #NUM!
- `norm_s_dist_large_z` — z=6 → CDF ≈ 1.0
- `norm_s_dist_large_negative_z` — z=−6 → CDF ≈ 0.0

**Error propagation:**
- `norm_s_dist_propagates_error`
- `norm_s_inv_propagates_error`

**Arg count:**
- `norm_s_dist_wrong_arg_count` — !=2 args → #VALUE!
- `norm_s_inv_wrong_arg_count` — !=1 arg → #VALUE!

### Conformance (norm_s_dist.xlsx)

**Data layout:** Column A has z values (−3, −2, −1, 0, 1, 2, 3).

**Formulas (18+):**

CDF:
1. `=NORM.S.DIST(0, TRUE)` → 0.5
2. `=NORM.S.DIST(1, TRUE)` → 0.841345
3. `=NORM.S.DIST(-1, TRUE)` → 0.158655
4. `=NORM.S.DIST(1.96, TRUE)` → 0.975002
5. `=NORM.S.DIST(-1.96, TRUE)` → 0.024998
6. `=NORM.S.DIST(3, TRUE)` → 0.998650

PDF:
7. `=NORM.S.DIST(0, FALSE)` → 0.398942
8. `=NORM.S.DIST(1, FALSE)` → 0.241971
9. `=NORM.S.DIST(-1, FALSE)` → 0.241971

Inverse:
10. `=NORM.S.INV(0.5)` → 0
11. `=NORM.S.INV(0.975)` → 1.95996
12. `=NORM.S.INV(0.025)` → −1.95996
13. `=NORM.S.INV(0.841345)` → ~1.0
14. `=NORM.S.INV(0.001)` → −3.09023

Round-trip:
15. `=NORM.S.INV(NORM.S.DIST(1.5, TRUE))` → 1.5

Nested:
16. `=ROUND(NORM.S.DIST(0, TRUE), 1)` → 0.5
17. `=IF(NORM.S.DIST(2,TRUE)>0.95, "sig", "ns")` → "sig"

Error:
18. `=NORM.S.INV(0)` → #NUM!
19. `=NORM.S.INV(1)` → #NUM!

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add NORM.S.DIST, NORM.S.INV |
| `docs/functions.md` | Mark NORM.S.DIST, NORM.S.INV as `x` |
| `docs/roadmap/v0.3/README.md` | Tick the checkbox |

## Streaming invariant

No violation. Pure scalar function. No cell reads beyond current row.
