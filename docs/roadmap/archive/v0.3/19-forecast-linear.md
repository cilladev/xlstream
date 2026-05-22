# Feature: FORECAST.LINEAR

**Branch:** `feat/forecast-linear`
**Effort:** ~2 hours
**Crates:** xlstream-eval, xlstream-parse
**Depends on:** SLOPE/INTERCEPT (spec 18) for shared regression internals

## What

Predicts a Y value for a given X using linear regression on known data.

- `FORECAST.LINEAR(x, known_ys, known_xs)` — returns `INTERCEPT + SLOPE * x`

```
=FORECAST.LINEAR(6, {2,4,6,8,10}, {1,2,3,4,5})  → 12.0
=FORECAST.LINEAR(0, {2,4,6,8,10}, {1,2,3,4,5})  → 0.0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — if SLOPE/INTERCEPT are implemented first, the regression internals (collect paired, compute slope/intercept) will be available
- `crates/xlstream-eval/src/builtins/mod.rs` — dispatch patterns
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists FORECAST.LINEAR as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:238-250` — dispatch pattern
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`

## Resolution / Evaluation behavior

FORECAST.LINEAR takes three arguments: a scalar x, and two arrays (known_ys, known_xs). It computes slope and intercept from the arrays, then returns `intercept + slope * x`.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** Three-arg dispatch: `args[0]` is scalar x (evaluate via `interp.eval`), `args[1]` and `args[2]` are data ranges (expand via `expand_range`). This is a new 3-arg pattern — scalar + range + range. The wrapper must:
1. Check arity (exactly 3 args)
2. Evaluate `args[0]` as scalar x
3. Expand `args[1]` (known_ys) and `args[2]` (known_xs) via `expand_range`
4. Call the pure function

**Implementation approach:**

Reuse the regression internals from SLOPE/INTERCEPT. Then: `result = intercept + slope * x`. Apply `finite_or_num` on the result.

Both FORECAST.LINEAR and the `_xlfn.FORECAST.LINEAR` prefix should be recognized. Additionally, the old `FORECAST` function (without `.LINEAR`) is a compatibility alias planned for v0.5. Do NOT add the alias now.

**Value handling:** Same as CORREL/SLOPE for the array arguments. The scalar x uses `coerce::to_number`.

**Error conditions:**
- Zero variance in known_xs (`Sxx = 0`) → `#DIV/0!`
- n < 1 → `#N/A`
- Arrays of different length → `#N/A`
- Error in x, known_ys, or known_xs → propagate
- Wrong arity → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `forecast_linear(6, [2,4,6,8,10], [1,2,3,4,5])` → 12.0
- `forecast_linear(0, [2,4,6,8,10], [1,2,3,4,5])` → 0.0
- `forecast_linear(3, [3,5,4,6,8], [1,2,3,4,5])` → predicted value (verify against Excel)
- `forecast_linear(10, [10,8,6,4,2], [1,2,3,4,5])` → -8.0

**Edge cases:**
- Extrapolation beyond data: `forecast_linear(100, [2,4,6,8,10], [1,2,3,4,5])` → 200.0
- Negative x: `forecast_linear(-1, [2,4,6,8,10], [1,2,3,4,5])` → -2.0
- Constant X: `forecast_linear(5, [1,2,3], [5,5,5])` → `#DIV/0!`
- Two pairs: `forecast_linear(3, [1,3], [1,2])` → 5.0

**Type handling:**
- Text skipped pairwise in arrays
- Error propagation from x: `forecast_linear(#N/A, [1,2,3], [1,2,3])` → `#N/A`
- Error propagation from arrays

### Conformance fixture

Create `tests/fixtures/statistical/forecast_linear.xlsx`.

**Sheet1 data:**
- A: "X" rows 2-6: `1, 2, 3, 4, 5`
- B: "Y" rows 2-6: `2, 4, 6, 8, 10`
- C: "Y_noisy" rows 2-6: `3, 5, 4, 8, 9`
- D: "Const" rows 2-6: `5, 5, 5, 5, 5`
- E: "Error" rows 2-4: `1, =NA(), 3`
- F: "Pred_X" rows 2-6: `0, 3, 6, 10, -1`

**Sheet2:**
- A/B: paired data

**Formulas (column G) — 18 formulas:**

Happy path (4):
1. `=_xlfn.FORECAST.LINEAR(6, B2:B6, A2:A6)` → 12.0
2. `=_xlfn.FORECAST.LINEAR(0, B2:B6, A2:A6)` → 0.0
3. `=_xlfn.FORECAST.LINEAR(3, C2:C6, A2:A6)` → predicted
4. `=_xlfn.FORECAST.LINEAR(-1, B2:B6, A2:A6)` → -2.0

Extrapolation (2):
5. `=_xlfn.FORECAST.LINEAR(100, B2:B6, A2:A6)` → 200.0
6. `=_xlfn.FORECAST.LINEAR(10, B2:B6, A2:A6)` → 20.0

Boundary (2):
7. `=_xlfn.FORECAST.LINEAR(3, B2:B3, A2:A3)` → 6.0
8. `=_xlfn.FORECAST.LINEAR(3, B2:B6, D2:D6)` → `#DIV/0!`

Error propagation (2):
9. `=_xlfn.FORECAST.LINEAR(3, A2:A4, E2:E4)` → `#N/A`
10. `=_xlfn.FORECAST.LINEAR(F2, B2:B6, A2:A6)` → predict at X=0

Nested (2):
11. `=IF(_xlfn.FORECAST.LINEAR(6, B2:B6, A2:A6)>10, "above", "below")` → "above"
12. `=IFERROR(_xlfn.FORECAST.LINEAR(3, B2:B6, D2:D6), "n/a")` → "n/a"

Cross-sheet (1):
13. `=_xlfn.FORECAST.LINEAR(3, Sheet2!B2:B6, Sheet2!A2:A6)`

Combined (1):
14. `=_xlfn.FORECAST.LINEAR(3, B2:B6, A2:A6) - (SLOPE(B2:B6, A2:A6)*3 + INTERCEPT(B2:B6, A2:A6))` → 0.0

Multiple predictions (2):
15. `=_xlfn.FORECAST.LINEAR(F2, B2:B6, A2:A6)` → predict at X=0
16. `=_xlfn.FORECAST.LINEAR(F4, B2:B6, A2:A6)` → predict at X=6

FORECAST.LINEAR needs `_xlfn.` prefix for LibreOffice.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "FORECAST.LINEAR linear prediction" under `[Unreleased]` |
| `docs/functions.md` | Change FORECAST.LINEAR from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the FORECAST.LINEAR checkbox |

## Streaming invariant

Does not violate. Pure function of a scalar + two expanded range arguments.
