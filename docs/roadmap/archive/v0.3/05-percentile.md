# Feature: PERCENTILE.INC / PERCENTILE.EXC

**Branch:** `feat/percentile`
**Effort:** ~0.5 day
**Crates:** xlstream-eval, xlstream-parse

## What

Percentile functions that return the k-th percentile of a data set.

- `PERCENTILE.INC(array, k)` — inclusive percentile (k in [0, 1]). Uses linear interpolation between sorted values. Equivalent to the old `PERCENTILE` function.
- `PERCENTILE.EXC(array, k)` — exclusive percentile (k in (0, 1)). Excludes the 0th and 100th percentile.

```
=PERCENTILE.INC({1,2,3,4,5}, 0.25)  → 2.0
=PERCENTILE.INC({1,2,3,4,5}, 0.5)   → 3.0
=PERCENTILE.EXC({1,2,3,4,5}, 0.25)  → 1.5
=PERCENTILE.EXC({1,2,3,4,5}, 0.5)   → 3.0
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — module with `collect_numerics`, `finite_or_num`, and 7 implemented functions (VAR.S/P, STDEV.S/P, SKEW, SKEW.P, KURT)
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with range-expanding wrapper pattern (empty-args guard, `expand_range`, `map_or_else`)
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS` phf set
- Not in `UNSUPPORTED_FUNCTIONS` — classifies as RowLocal by default
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:238-248` — dispatch match arms + wrapper function pattern
- `crates/xlstream-eval/src/builtins/mod.rs:40-81` — `expand_range` helper
- `crates/xlstream-eval/src/builtins/mod.rs:269-276` — `builtin_var_s` wrapper — reference pattern for range-expanding stat functions
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS` — add both functions here
- `crates/xlstream-eval/tests/conformance/statistical.rs` — conformance test module
- `crates/xlstream-eval/tests/fixtures/statistical/` — fixture directory

## Resolution / Evaluation behavior

Both are pure functions of their arguments — row-local, no prelude, no lookups, no streaming concerns.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** The dispatch wrapper must handle two arguments differently: `args[0]` is the data range (expand via `expand_range`), `args[1]` is the k value (evaluate as a scalar via `interp.eval`). This is a new pattern — existing stat functions expand all args as ranges. The wrapper must:
1. Check arity (exactly 2 args required)
2. Expand `args[0]` into a `Vec<Value>` via `expand_range`
3. Evaluate `args[1]` as a scalar via `interp.eval` and coerce to f64
4. Call the pure function with `(&[Value], k)`

**Implementation approach:**

PERCENTILE.INC interpolation (Excel's method):
1. Collect numeric values via `collect_numerics`, sort ascending
2. Compute rank: `rank = k * (n - 1)` (0-based)
3. Floor and ceil indices: `lo = floor(rank)`, `hi = ceil(rank)`
4. Fraction: `frac = rank - lo`
5. Result: `sorted[lo] + frac * (sorted[hi] - sorted[lo])`

PERCENTILE.EXC interpolation:
1. Collect numerics, sort ascending
2. Compute rank: `rank = k * (n + 1)` (1-based)
3. If rank < 1 or rank > n, return `#NUM!`
4. Floor: `lo = floor(rank) - 1` (0-based), ceil: `hi = ceil(rank) - 1`
5. Fraction: `frac = rank - floor(rank)`
6. Result: `sorted[lo] + frac * (sorted[hi] - sorted[lo])`

**Value handling (must match Excel):**
- `Value::Number(n)` — include
- `Value::Integer(i)` — include (cast to f64)
- `Value::Date(d)` — include (use serial)
- `Value::Error(e)` — propagate immediately
- `Value::Text(_)` — skip
- `Value::Bool(_)` — skip
- `Value::Empty` — skip

**Error conditions:**
- PERCENTILE.INC: k < 0 or k > 1 → `#NUM!`. No numeric values → `#NUM!`.
- PERCENTILE.EXC: k <= 0 or k >= 1 → `#NUM!`. k < 1/(n+1) or k > n/(n+1) → `#NUM!`. No numeric values → `#NUM!`.
- Wrong number of arguments → `#VALUE!`
- Error in data range → propagate

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `percentile_inc([1,2,3,4,5], 0.25)` → 2.0
- `percentile_inc([1,2,3,4,5], 0.5)` → 3.0
- `percentile_inc([1,2,3,4,5], 0.0)` → 1.0
- `percentile_inc([1,2,3,4,5], 1.0)` → 5.0
- `percentile_exc([1,2,3,4,5], 0.5)` → 3.0
- `percentile_exc([1,2,3,4,5], 0.25)` → 1.5

**Edge cases:**
- Single value: `percentile_inc([5], 0.5)` → 5.0
- Single value: `percentile_exc([5], 0.5)` → `#NUM!` (k must be in (1/(n+1), n/(n+1)) = (0.5, 0.5) — empty interval)
- Two values: `percentile_inc([1, 3], 0.5)` → 2.0
- Two values: `percentile_exc([1, 3], 1/3)` → 1.0 (lower boundary)
- k=0: `percentile_inc([1,2,3], 0)` → 1.0
- k=1: `percentile_inc([1,2,3], 1)` → 3.0
- k=0: `percentile_exc([1,2,3], 0)` → `#NUM!`
- k=1: `percentile_exc([1,2,3], 1)` → `#NUM!`
- k out of range: `percentile_inc([1,2,3], 1.5)` → `#NUM!`
- k negative: `percentile_inc([1,2,3], -0.1)` → `#NUM!`
- Empty: `percentile_inc([], 0.5)` → `#NUM!`
- All text: `percentile_inc(["a","b"], 0.5)` → `#NUM!`
- All same: `percentile_inc([5,5,5], 0.25)` → 5.0

**Type handling:**
- Text skipped: `percentile_inc([1, "text", 3], 0.5)` → same as `percentile_inc([1, 3], 0.5)` = 2.0
- Bool skipped: `percentile_inc([1, TRUE, 3], 0.5)` → 2.0
- Error propagation: `percentile_inc([1, #N/A, 3], 0.5)` → `#N/A`

**Numerical stability:**
- Large numbers: `percentile_inc([1e10, 1e10+1], 0.5)` → 1e10 + 0.5

### Conformance fixture

Create `tests/fixtures/statistical/percentile.xlsx`.

**Sheet1 data (columns A-F):**
- A: "Values" header, rows 2-11: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`
- B: "Small" header, rows 2-4: `10, 20, 30`
- C: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- D: "Same" header, rows 2-5: `7, 7, 7, 7`
- E: "Error" header, rows 2-4: `1, =NA(), 3`
- F: "Single" header, row 2: `42`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (column G, starting row 2):**

Happy path — PERCENTILE.INC (5):
1. `=_xlfn.PERCENTILE.INC(A2:A11, 0.25)` → 3.25
2. `=_xlfn.PERCENTILE.INC(A2:A11, 0.5)` → 5.5
3. `=_xlfn.PERCENTILE.INC(A2:A11, 0.75)` → 7.75
4. `=_xlfn.PERCENTILE.INC(A2:A11, 0)` → 1
5. `=_xlfn.PERCENTILE.INC(A2:A11, 1)` → 10

Happy path — PERCENTILE.EXC (3):
6. `=_xlfn.PERCENTILE.EXC(A2:A11, 0.25)` → 2.75
7. `=_xlfn.PERCENTILE.EXC(A2:A11, 0.5)` → 5.5
8. `=_xlfn.PERCENTILE.EXC(A2:A11, 0.75)` → 8.25

Boundary — k out of range (4):
9. `=_xlfn.PERCENTILE.INC(B2:B4, -0.1)` → `#NUM!`
10. `=_xlfn.PERCENTILE.INC(B2:B4, 1.1)` → `#NUM!`
11. `=_xlfn.PERCENTILE.EXC(B2:B4, 0)` → `#NUM!`
12. `=_xlfn.PERCENTILE.EXC(B2:B4, 1)` → `#NUM!`

Single value (2):
13. `=_xlfn.PERCENTILE.INC(F2, 0.5)` → 42
14. `=_xlfn.PERCENTILE.EXC(F2, 0.5)` → `#NUM!`

All same (1):
15. `=_xlfn.PERCENTILE.INC(D2:D5, 0.75)` → 7

Type coercion (2):
16. `=_xlfn.PERCENTILE.INC(C2:C6, 0.5)` → 3 (only 1, 3, 5 counted)
17. `=_xlfn.PERCENTILE.EXC(C2:C6, 0.5)` → 3

Error propagation (2):
18. `=_xlfn.PERCENTILE.INC(E2:E4, 0.5)` → `#N/A`
19. `=_xlfn.PERCENTILE.EXC(E2:E4, 0.5)` → `#N/A`

Nested (2):
20. `=IF(_xlfn.PERCENTILE.INC(A2:A11, 0.5)>5, "above", "below")` → "above"
21. `=IFERROR(_xlfn.PERCENTILE.EXC(F2, 0.5), "n/a")` → "n/a"

Cross-sheet (2):
22. `=_xlfn.PERCENTILE.INC(Sheet2!A2:A6, 0.5)` → 30
23. `=_xlfn.PERCENTILE.EXC(Sheet2!A2:A6, 0.5)` → 30

Combined (1):
24. `=_xlfn.PERCENTILE.INC(A2:A11, 0.75) - _xlfn.PERCENTILE.INC(A2:A11, 0.25)` → IQR = 4.5

**Fixture workflow:**
1. Generate with openpyxl (formulas use `_xlfn.PERCENTILE.INC` / `_xlfn.PERCENTILE.EXC` prefix + data, no cached values)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn percentile()` in `conformance/statistical.rs` calling `run_conformance("statistical/percentile.xlsx")`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "PERCENTILE.INC, PERCENTILE.EXC percentile functions" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change PERCENTILE.INC, PERCENTILE.EXC from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the PERCENTILE checkbox |

## Streaming invariant

Does not violate. Both are pure functions of their cell-reference arguments — no cross-row reads, no prelude dependency, no lookup sheets. The sort is on the expanded range values within a single evaluation, not across rows.
