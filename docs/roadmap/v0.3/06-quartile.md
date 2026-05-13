# Feature: QUARTILE.INC / QUARTILE.EXC

**Branch:** `feat/quartile`
**Effort:** ~2 hours
**Crates:** xlstream-eval, xlstream-parse
**Depends on:** PERCENTILE.INC / PERCENTILE.EXC (spec 05)

## What

Quartile functions — thin wrappers over the PERCENTILE functions.

- `QUARTILE.INC(array, quart)` — inclusive quartile. `quart` is 0-4. Equivalent to `PERCENTILE.INC(array, quart/4)`.
- `QUARTILE.EXC(array, quart)` — exclusive quartile. `quart` is 1-3. Equivalent to `PERCENTILE.EXC(array, quart/4)`.

```
=QUARTILE.INC({1,2,3,4,5}, 1)  → 2.0   (= PERCENTILE.INC(..., 0.25))
=QUARTILE.INC({1,2,3,4,5}, 2)  → 3.0   (= PERCENTILE.INC(..., 0.5))
=QUARTILE.INC({1,2,3,4,5}, 3)  → 4.0   (= PERCENTILE.INC(..., 0.75))
=QUARTILE.EXC({1,2,3,4,5}, 1)  → 1.5   (= PERCENTILE.EXC(..., 0.25))
=QUARTILE.EXC({1,2,3,4,5}, 2)  → 3.0   (= PERCENTILE.EXC(..., 0.5))
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — module with `collect_numerics`, `finite_or_num`, existing stat functions. PERCENTILE.INC/EXC will be implemented here first (spec 05).
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with range-expanding wrapper pattern
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS` phf set
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home. QUARTILE delegates to `percentile_inc` / `percentile_exc` (from spec 05).
- `crates/xlstream-eval/src/builtins/mod.rs:238-248` — dispatch pattern
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS`
- `crates/xlstream-eval/tests/conformance/statistical.rs` — conformance test module

## Resolution / Evaluation behavior

Both are pure functions — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** Same dispatch pattern as PERCENTILE — `args[0]` is data range (expand), `args[1]` is quart value (evaluate as scalar). The pure function validates `quart` is an integer in the allowed range, converts to k = quart/4, and delegates to the corresponding percentile function.

**Implementation approach:**

QUARTILE.INC:
1. Validate `quart` is integer in [0, 4]. Non-integer or out of range → `#NUM!`
2. Compute `k = quart / 4.0`
3. Delegate to `percentile_inc(values, k)`

QUARTILE.EXC:
1. Validate `quart` is integer in [1, 3]. Out of range → `#NUM!`
2. Compute `k = quart / 4.0`
3. Delegate to `percentile_exc(values, k)`

**Value handling:** Same as PERCENTILE — uses `collect_numerics`.

**Error conditions:**
- QUARTILE.INC: quart not in {0, 1, 2, 3, 4} → `#NUM!`. No numeric values → `#NUM!`.
- QUARTILE.EXC: quart not in {1, 2, 3} → `#NUM!`. Insufficient data for exclusive range → `#NUM!`.
- Wrong number of arguments → `#VALUE!`
- Error in data range → propagate

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `quartile_inc([1,2,3,4,5], 0)` → 1.0
- `quartile_inc([1,2,3,4,5], 1)` → 2.0
- `quartile_inc([1,2,3,4,5], 2)` → 3.0
- `quartile_inc([1,2,3,4,5], 3)` → 4.0
- `quartile_inc([1,2,3,4,5], 4)` → 5.0
- `quartile_exc([1,2,3,4,5], 1)` → 1.5
- `quartile_exc([1,2,3,4,5], 2)` → 3.0
- `quartile_exc([1,2,3,4,5], 3)` → 4.5

**Edge cases:**
- Invalid quart INC: `quartile_inc([1,2,3], 5)` → `#NUM!`
- Invalid quart INC: `quartile_inc([1,2,3], -1)` → `#NUM!`
- Non-integer quart: `quartile_inc([1,2,3], 1.5)` → `#NUM!`
- Invalid quart EXC: `quartile_exc([1,2,3], 0)` → `#NUM!`
- Invalid quart EXC: `quartile_exc([1,2,3], 4)` → `#NUM!`
- Single value INC: `quartile_inc([5], 2)` → 5.0
- Single value EXC: `quartile_exc([5], 1)` → `#NUM!` (delegates to percentile_exc which requires n >= 2 for exclusive)
- Empty: `quartile_inc([], 1)` → `#NUM!`
- All same: `quartile_inc([7,7,7], 3)` → 7.0

**Type handling:**
- Text skipped: `quartile_inc([1, "x", 3], 1)` → same as `quartile_inc([1, 3], 1)`
- Error propagation: `quartile_inc([1, #N/A, 3], 1)` → `#N/A`

### Conformance fixture

Create `tests/fixtures/statistical/quartile.xlsx`.

**Sheet1 data (columns A-E):**
- A: "Values" header, rows 2-11: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`
- B: "Small" header, rows 2-4: `10, 20, 30`
- C: "Same" header, rows 2-5: `7, 7, 7, 7`
- D: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- E: "Error" header, rows 2-4: `1, =NA(), 3`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (column F, starting row 2):**

Happy path — QUARTILE.INC (5):
1. `=_xlfn.QUARTILE.INC(A2:A11, 0)` → 1
2. `=_xlfn.QUARTILE.INC(A2:A11, 1)` → 3.25
3. `=_xlfn.QUARTILE.INC(A2:A11, 2)` → 5.5
4. `=_xlfn.QUARTILE.INC(A2:A11, 3)` → 7.75
5. `=_xlfn.QUARTILE.INC(A2:A11, 4)` → 10

Happy path — QUARTILE.EXC (3):
6. `=_xlfn.QUARTILE.EXC(A2:A11, 1)` → 2.75
7. `=_xlfn.QUARTILE.EXC(A2:A11, 2)` → 5.5
8. `=_xlfn.QUARTILE.EXC(A2:A11, 3)` → 8.25

Invalid quart (4):
9. `=_xlfn.QUARTILE.INC(A2:A11, 5)` → `#NUM!`
10. `=_xlfn.QUARTILE.INC(A2:A11, -1)` → `#NUM!`
11. `=_xlfn.QUARTILE.EXC(A2:A11, 0)` → `#NUM!`
12. `=_xlfn.QUARTILE.EXC(A2:A11, 4)` → `#NUM!`

All same (1):
13. `=_xlfn.QUARTILE.INC(C2:C5, 2)` → 7

Type coercion (2):
14. `=_xlfn.QUARTILE.INC(D2:D6, 2)` → 3 (only 1, 3, 5)
15. `=_xlfn.QUARTILE.EXC(D2:D6, 2)` → 3

Error propagation (1):
16. `=_xlfn.QUARTILE.INC(E2:E4, 1)` → `#N/A`

Nested (2):
17. `=IF(_xlfn.QUARTILE.INC(A2:A11, 3)>7, "high", "low")` → "high"
18. `=IFERROR(_xlfn.QUARTILE.EXC(A2:A11, 0), "n/a")` → "n/a"

Cross-sheet (1):
19. `=_xlfn.QUARTILE.INC(Sheet2!A2:A6, 2)` → 30

IQR via QUARTILE (1):
20. `=_xlfn.QUARTILE.INC(A2:A11, 3) - _xlfn.QUARTILE.INC(A2:A11, 1)` → 4.5

**Fixture workflow:**
1. Generate with openpyxl (formulas use `_xlfn.QUARTILE.INC` / `_xlfn.QUARTILE.EXC` prefix)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn quartile()` in `conformance/statistical.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "QUARTILE.INC, QUARTILE.EXC quartile functions" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change QUARTILE.INC, QUARTILE.EXC from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the QUARTILE checkbox |

## Streaming invariant

Does not violate. Delegates to PERCENTILE which operates on expanded range values within a single evaluation — no cross-row reads, no prelude dependency.
