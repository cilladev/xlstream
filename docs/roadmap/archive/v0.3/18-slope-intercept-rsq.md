# Feature: SLOPE / INTERCEPT / RSQ

**Branch:** `feat/slope-intercept-rsq`
**Effort:** ~0.5 day
**Crates:** xlstream-eval, xlstream-parse
**Depends on:** CORREL (spec 16) for `collect_paired_numerics`

## What

Linear regression statistics from paired (x, y) data.

- `SLOPE(known_ys, known_xs)` — slope of the least-squares regression line
- `INTERCEPT(known_ys, known_xs)` — y-intercept of the regression line
- `RSQ(known_ys, known_xs)` — coefficient of determination (R-squared = CORREL²)

Note: Excel's argument order is `(known_ys, known_xs)` — Y first, X second.

```
=SLOPE({2,4,6,8,10}, {1,2,3,4,5})     → 2.0
=INTERCEPT({2,4,6,8,10}, {1,2,3,4,5}) → 0.0
=RSQ({2,4,6,8,10}, {1,2,3,4,5})       → 1.0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — `collect_numerics`, `mean_and_variance`, `finite_or_num`, and (if CORREL lands first) `collect_paired_numerics`
- `crates/xlstream-eval/src/builtins/mod.rs` — 2-arg range-expanding dispatch pattern
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists all three as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:238-250` — dispatch pattern
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`

## Resolution / Evaluation behavior

All three are pure functions of two same-length arrays — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** Two-arg range-expanding dispatch. Same pattern as CORREL. Note the argument order: `args[0]` = known_ys, `args[1]` = known_xs (Y first).

**Implementation approach:**

All three share the same intermediate computations. Reuse `collect_paired_numerics`, then compute:
- `mean_x`, `mean_y`
- `Sxy = Σ((xi - mean_x)(yi - mean_y))`
- `Sxx = Σ((xi - mean_x)²)`
- `Syy = Σ((yi - mean_y)²)`

Then:
- SLOPE = `Sxy / Sxx`
- INTERCEPT = `mean_y - SLOPE * mean_x`
- RSQ = `(Sxy)² / (Sxx * Syy)` (equivalently, CORREL²)

A shared `linear_regression_stats` function could return `(slope, intercept, rsq)` or just `(sxy, sxx, syy, mean_x, mean_y)` for the three callers to pick what they need.

**Value handling:** Same as CORREL.

**Error conditions:**
- Zero variance in X (`Sxx = 0`) → `#DIV/0!` for SLOPE and INTERCEPT
- Zero variance in both (`Sxx = 0` and `Syy = 0`) → `#DIV/0!` for RSQ
- n < 1 → `#DIV/0!`
- Arrays of different length → `#N/A`
- Error in either array → propagate
- Wrong arity → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**SLOPE happy path:**
- `slope([2,4,6,8,10], [1,2,3,4,5])` → 2.0
- `slope([3,5,4,6,8], [1,2,3,4,5])` → ~1.1 (imperfect fit)
- `slope([10,8,6,4,2], [1,2,3,4,5])` → -2.0 (negative slope)

**INTERCEPT happy path:**
- `intercept([2,4,6,8,10], [1,2,3,4,5])` → 0.0
- `intercept([3,5,7,9,11], [1,2,3,4,5])` → 1.0

**RSQ happy path:**
- `rsq([2,4,6,8,10], [1,2,3,4,5])` → 1.0
- `rsq([3,5,4,6,8], [1,2,3,4,5])` → between 0 and 1
- `rsq([10,8,6,4,2], [1,2,3,4,5])` → 1.0 (perfect fit regardless of direction)

**Edge cases (shared):**
- Different length: → `#N/A`
- Constant X: `slope([1,2,3], [5,5,5])` → `#DIV/0!`
- Constant Y: `slope([5,5,5], [1,2,3])` → 0.0 (zero slope is valid)
- Two pairs: `slope([1,3], [1,2])` → 2.0
- Empty: → `#DIV/0!`

**Type handling:**
- Text skipped pairwise
- Error propagation

### Conformance fixture

Create `tests/fixtures/statistical/slope_intercept_rsq.xlsx`.

**Sheet1 data:**
- A: "X" rows 2-11: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`
- B: "Y_perfect" rows 2-11: `2, 4, 6, 8, 10, 12, 14, 16, 18, 20`
- C: "Y_noisy" rows 2-11: `3, 5, 4, 8, 9, 11, 15, 14, 18, 21`
- D: "Y_neg" rows 2-11: `20, 18, 16, 14, 12, 10, 8, 6, 4, 2`
- E: "Const" rows 2-6: `5, 5, 5, 5, 5`
- F: "Error" rows 2-4: `1, =NA(), 3`

**Sheet2:**
- A/B: paired data

**Formulas (column G) — 22 formulas:**

SLOPE (5):
1. `=SLOPE(B2:B11, A2:A11)` → 2.0
2. `=SLOPE(C2:C11, A2:A11)` → ~1.939...
3. `=SLOPE(D2:D11, A2:A11)` → -2.0
4. `=SLOPE(B2:B6, A2:A6)` → 2.0
5. `=SLOPE(B2:B3, A2:A3)` → 2.0

INTERCEPT (4):
6. `=INTERCEPT(B2:B11, A2:A11)` → 0.0
7. `=INTERCEPT(C2:C11, A2:A11)` → ~0.666...
8. `=INTERCEPT(D2:D11, A2:A11)` → 22.0
9. `=INTERCEPT(B2:B6, A2:A6)` → 0.0

RSQ (3):
10. `=RSQ(B2:B11, A2:A11)` → 1.0
11. `=RSQ(C2:C11, A2:A11)` → ~0.959...
12. `=RSQ(D2:D11, A2:A11)` → 1.0

Boundary (3):
13. `=SLOPE(A2:A6, E2:E6)` → `#DIV/0!` (constant X)
14. `=SLOPE(E2:E6, A2:A6)` → 0.0 (constant Y, slope=0)
15. `=RSQ(A2:A6, E2:E6)` → `#DIV/0!`

Error propagation (1):
16. `=SLOPE(A2:A4, F2:F4)` → `#N/A`

Nested (2):
17. `=IF(RSQ(C2:C11, A2:A11)>0.9, "good fit", "poor fit")` → "good fit"
18. `=IFERROR(SLOPE(A2:A6, E2:E6), "n/a")` → "n/a"

Cross-sheet (1):
19. `=SLOPE(Sheet2!B2:B6, Sheet2!A2:A6)`

Combined (3):
20. `=SLOPE(B2:B11, A2:A11) * 5 + INTERCEPT(B2:B11, A2:A11)` → 10.0 (predict Y at X=5)
21. `=RSQ(B2:B11, A2:A11) - CORREL(B2:B11, A2:A11)^2` → 0.0 (RSQ = r²)
22. `=INTERCEPT(B2:B11, A2:A11) + SLOPE(B2:B11, A2:A11) * AVERAGE(A2:A11)` → AVERAGE(B2:B11)

SLOPE, INTERCEPT, RSQ do NOT need `_xlfn.` prefix.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "SLOPE, INTERCEPT, RSQ linear regression" under `[Unreleased]` |
| `docs/functions.md` | Change SLOPE, INTERCEPT, RSQ from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the SLOPE / INTERCEPT / RSQ checkbox |

## Streaming invariant

Does not violate. All three are pure functions of expanded range arguments.
