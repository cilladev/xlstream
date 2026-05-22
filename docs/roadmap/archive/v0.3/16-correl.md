# Feature: CORREL

**Branch:** `feat/correl`
**Effort:** ~2 hours
**Crates:** xlstream-eval, xlstream-parse

## What

Pearson product-moment correlation coefficient between two data sets.

- `CORREL(array1, array2)` — returns a value in [-1, 1] measuring linear correlation.

```
=CORREL({1,2,3,4,5}, {2,4,6,8,10})  → 1.0    (perfect positive correlation)
=CORREL({1,2,3,4,5}, {10,8,6,4,2})  → -1.0   (perfect negative correlation)
=CORREL({1,2,3,4,5}, {5,5,5,5,5})   → #DIV/0! (zero stdev in array2)
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — module with `collect_numerics`, `mean_and_variance`, `finite_or_num`
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with 2-arg range-expanding pattern (LARGE/SMALL, PERCENTILE, RANK)
- `crates/xlstream-eval/src/builtins/mod.rs:40-81` — `expand_range` helper
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS` phf set
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists CORREL as `.` (planned) for v0.3 with note "Prelude: two-column"

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/statistical.rs:42-51` — `collect_numerics`
- `crates/xlstream-eval/src/builtins/statistical.rs:61-90` — `mean_and_variance`
- `crates/xlstream-eval/src/builtins/mod.rs:238-250` — dispatch match arms
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`
- `crates/xlstream-eval/tests/conformance/statistical.rs` — conformance test module

## Resolution / Evaluation behavior

Pure function of two same-length arrays — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal (default).

**Prelude:** Nothing needed. The "Prelude: two-column" note in functions.md was aspirational — inline computation is fine for bounded ranges. Remove the note when shipping.

**Row eval:** Two-arg range-expanding dispatch. Both `args[0]` and `args[1]` are expanded via `expand_range`. The pure function receives two `&[Value]` slices.

**Implementation approach:**

CORREL uses paired numeric values from two arrays. The implementation must:
1. Iterate both arrays in parallel, collecting pairs where BOTH values are numeric (skip pairs where either is text/bool/empty)
2. Propagate any error from either array immediately
3. Require at least 1 pair (else `#N/A`); require arrays of same length (else `#N/A`)
4. Compute: `r = Σ((xi - mean_x)(yi - mean_y)) / sqrt(Σ(xi - mean_x)² * Σ(yi - mean_y)²)`
5. If either stdev is zero, return `#DIV/0!`

The paired-collection pattern is new — existing stat functions collect from a single array. A shared `collect_paired_numerics` helper would serve CORREL, COVARIANCE, SLOPE, INTERCEPT, RSQ, and FORECAST.LINEAR.

**Value handling (per element in each array):**
- `Value::Number(n)` — include if the paired element is also numeric
- `Value::Integer(i)` — include (cast to f64)
- `Value::Date(d)` — include (use serial)
- `Value::Error(e)` — propagate immediately
- `Value::Text(_)` / `Value::Bool(_)` / `Value::Empty` — skip the pair

**Error conditions:**
- Arrays of different length → `#N/A`
- Fewer than 1 paired numeric value → `#N/A` (though in practice need ≥ 2 for meaningful correlation)
- Zero stdev in either array → `#DIV/0!`
- Error in either array → propagate
- Wrong arity (not exactly 2 args) → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `correl([1,2,3,4,5], [2,4,6,8,10])` → 1.0 (perfect positive)
- `correl([1,2,3,4,5], [10,8,6,4,2])` → -1.0 (perfect negative)
- `correl([1,2,3], [1,3,2])` → 0.5 (partial correlation)
- `correl([1,2,3,4,5], [3,5,4,6,8])` → positive, between 0 and 1

**Edge cases:**
- Different length arrays: `correl([1,2,3], [1,2])` → `#N/A`
- Single pair: `correl([1], [2])` → `#DIV/0!` (zero stdev with n=1)
- Two pairs: `correl([1,2], [3,4])` → 1.0
- Zero stdev in one array: `correl([1,2,3], [5,5,5])` → `#DIV/0!`
- Empty arrays: `correl([], [])` → `#N/A`
- All text in one array: `correl([1,2,3], ["a","b","c"])` → `#N/A` (no pairs)
- Mixed types — text skipped pairwise: `correl([1,"x",3], [2,"y",4])` → same as `correl([1,3], [2,4])`

**Type handling:**
- Text pairs skipped: `correl([1, "text", 3], [2, "text", 4])` → `correl([1,3], [2,4])`
- Bool skipped in range: `correl([1, TRUE, 3], [2, FALSE, 4])` → `correl([1,3], [2,4])`
- Error propagation from array1: `correl([1, #N/A, 3], [2, 4, 6])` → `#N/A`
- Error propagation from array2: `correl([1, 2, 3], [2, #N/A, 6])` → `#N/A`

**Numerical stability:**
- Large values: `correl([1e10, 1e10+1, 1e10+2], [1e10, 1e10+1, 1e10+2])` → 1.0

### Conformance fixture

Create `tests/fixtures/statistical/correl.xlsx`.

**Sheet1 data:**
- A: "X" header, rows 2-11: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`
- B: "Y_pos" header, rows 2-11: `2, 4, 6, 8, 10, 12, 14, 16, 18, 20`
- C: "Y_neg" header, rows 2-11: `20, 18, 16, 14, 12, 10, 8, 6, 4, 2`
- D: "Y_partial" header, rows 2-11: `3, 5, 4, 6, 8, 7, 10, 9, 12, 11`
- E: "Const" header, rows 2-11: `5, 5, 5, 5, 5, 5, 5, 5, 5, 5`
- F: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- G: "Error" header, rows 2-4: `1, =NA(), 3`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`
- B: "YS" header, rows 2-6: `15, 25, 35, 45, 55`

**Formulas (column H, starting row 2):**

Happy path (4):
1. `=CORREL(A2:A11, B2:B11)` → 1.0
2. `=CORREL(A2:A11, C2:C11)` → -1.0
3. `=CORREL(A2:A11, D2:D11)` → positive partial
4. `=CORREL(A2:A6, B2:B6)` → 1.0 (subset)

Boundary (3):
5. `=CORREL(A2:A3, B2:B3)` → 1.0 (minimum 2 pairs)
6. `=CORREL(A2:A11, E2:E11)` → `#DIV/0!` (constant Y)
7. `=CORREL(E2:E11, A2:A11)` → `#DIV/0!` (constant X)

Type coercion (2):
8. `=CORREL(A2:A6, F2:F6)` — mixed with text and bool
9. `=CORREL(F2:F6, A2:A6)` — reversed

Error propagation (2):
10. `=CORREL(A2:A4, G2:G4)` → `#N/A`
11. `=CORREL(G2:G4, A2:A4)` → `#N/A`

Nested (2):
12. `=IF(CORREL(A2:A11, B2:B11)>0.9, "strong", "weak")` → "strong"
13. `=IFERROR(CORREL(A2:A11, E2:E11), "n/a")` → "n/a"

Cross-sheet (1):
14. `=CORREL(Sheet2!A2:A6, Sheet2!B2:B6)` → 1.0

Combined (1):
15. `=CORREL(A2:A11, B2:B11) + CORREL(A2:A11, C2:C11)` → 0.0

CORREL does NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn correl()` in `conformance/statistical.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "CORREL Pearson correlation coefficient" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change CORREL from `.` to `x`. Remove "Prelude: two-column" note. |
| `docs/roadmap/v0.3/README.md` | Tick the CORREL checkbox |

## Streaming invariant

Does not violate. CORREL is a pure function of two expanded range arguments — no cross-row reads, no prelude dependency.
