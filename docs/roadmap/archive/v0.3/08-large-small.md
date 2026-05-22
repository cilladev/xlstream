# Feature: LARGE / SMALL

**Branch:** `feat/large-small`
**Effort:** ~2 hours
**Crates:** xlstream-eval, xlstream-parse

## What

Return the k-th largest or smallest value from a data set.

- `LARGE(array, k)` — returns the k-th largest value. k=1 is the maximum.
- `SMALL(array, k)` — returns the k-th smallest value. k=1 is the minimum.

```
=LARGE({3,1,4,1,5,9}, 1)  → 9   (largest)
=LARGE({3,1,4,1,5,9}, 2)  → 5   (2nd largest)
=LARGE({3,1,4,1,5,9}, 3)  → 4   (3rd largest)
=SMALL({3,1,4,1,5,9}, 1)  → 1   (smallest)
=SMALL({3,1,4,1,5,9}, 2)  → 1   (2nd smallest — duplicate 1)
=SMALL({3,1,4,1,5,9}, 3)  → 3   (3rd smallest)
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — module with `collect_numerics`, `finite_or_num`
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with range-expanding wrapper pattern
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS` phf set
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists LARGE as `.` with note "Prelude: sorted"; SMALL as `.` for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:238-248` — dispatch pattern
- `crates/xlstream-eval/src/builtins/mod.rs:40-81` — `expand_range` helper
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS`
- `crates/xlstream-eval/tests/conformance/statistical.rs` — conformance test module

## Resolution / Evaluation behavior

Both are pure functions — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal.

**Prelude:** Nothing needed. The "Prelude: sorted" note in functions.md was aspirational — for bounded range args in the row stream, we sort inline. No prelude required.

**Row eval:** Two arguments: `args[0]` is the data range (expand via `expand_range`), `args[1]` is k (evaluate as scalar). The wrapper must:
1. Check arity (exactly 2 args)
2. Expand `args[0]` into `Vec<Value>` via `expand_range`
3. Evaluate `args[1]` as scalar, coerce to f64, truncate to integer
4. Call the pure function

**Implementation approach:**

LARGE:
1. Collect numerics via `collect_numerics`, sort descending
2. Validate k: must be integer >= 1 and <= n. Otherwise `#NUM!`
3. Return `sorted[k-1]`

SMALL:
1. Collect numerics via `collect_numerics`, sort ascending
2. Validate k: same as LARGE
3. Return `sorted[k-1]`

Both can share a `kth_value(values, k, descending)` helper.

**Value handling:** Uses `collect_numerics` for the range. k is coerced from the scalar arg.

**Error conditions:**
- k < 1 or k > n → `#NUM!`
- k is not a positive integer → `#NUM!`
- No numeric values → `#NUM!`
- Error in data range → propagate
- Wrong arity → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `large([3,1,4,1,5,9], 1)` → 9
- `large([3,1,4,1,5,9], 2)` → 5
- `large([3,1,4,1,5,9], 6)` → 1 (last = smallest)
- `small([3,1,4,1,5,9], 1)` → 1
- `small([3,1,4,1,5,9], 2)` → 1 (duplicate)
- `small([3,1,4,1,5,9], 3)` → 3

**Edge cases:**
- k=1 single value: `large([5], 1)` → 5
- k=1 single value: `small([5], 1)` → 5
- k > n: `large([1,2,3], 4)` → `#NUM!`
- k = 0: `large([1,2,3], 0)` → `#NUM!`
- k negative: `large([1,2,3], -1)` → `#NUM!`
- k fractional: `large([1,2,3], 1.9)` → truncates to 1, returns 3 (largest)
- Empty: `large([], 1)` → `#NUM!`
- All same: `large([5,5,5], 2)` → 5
- All text: `large(["a","b"], 1)` → `#NUM!`
- Negative values: `small([-5, -3, -1], 1)` → -5

**Type handling:**
- Text skipped: `large([1, "text", 3], 1)` → 3
- Bool skipped: `large([1, TRUE, 3], 1)` → 3
- Error propagation: `large([1, #N/A, 3], 1)` → `#N/A`

### Conformance fixture

Create `tests/fixtures/statistical/large_small.xlsx`.

**Sheet1 data (columns A-F):**
- A: "Values" header, rows 2-11: `10, 20, 30, 30, 40, 50, 60, 70, 80, 90`
- B: "Small" header, rows 2-4: `5, 15, 25`
- C: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- D: "Same" header, rows 2-5: `7, 7, 7, 7`
- E: "Error" header, rows 2-4: `1, =NA(), 3`
- F: "Single" header, row 2: `42`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (column G, starting row 2):**

Happy path — LARGE (4):
1. `=LARGE(A2:A11, 1)` → 90
2. `=LARGE(A2:A11, 2)` → 80
3. `=LARGE(A2:A11, 5)` → 40
4. `=LARGE(A2:A11, 10)` → 10

Happy path — SMALL (4):
5. `=SMALL(A2:A11, 1)` → 10
6. `=SMALL(A2:A11, 2)` → 20
7. `=SMALL(A2:A11, 5)` → 40
8. `=SMALL(A2:A11, 10)` → 90

k out of range (3):
9. `=LARGE(B2:B4, 4)` → `#NUM!`
10. `=LARGE(B2:B4, 0)` → `#NUM!`
11. `=SMALL(B2:B4, 4)` → `#NUM!`

Duplicates (2):
12. `=LARGE(A2:A11, 4)` → 30 (second of two 30s)
13. `=SMALL(A2:A11, 3)` → 30 (first of two 30s)

Single value (1):
14. `=LARGE(F2, 1)` → 42

All same (1):
15. `=LARGE(D2:D5, 3)` → 7

Type coercion (2):
16. `=LARGE(C2:C6, 1)` → 5 (only 1, 3, 5 counted)
17. `=SMALL(C2:C6, 1)` → 1

Error propagation (1):
18. `=LARGE(E2:E4, 1)` → `#N/A`

Nested (2):
19. `=IF(LARGE(A2:A11, 1)>80, "high", "low")` → "high"
20. `=IFERROR(LARGE(B2:B4, 5), "n/a")` → "n/a"

Cross-sheet (1):
21. `=SMALL(Sheet2!A2:A6, 2)` → 20

Combined (1):
22. `=LARGE(A2:A11, 1) - SMALL(A2:A11, 1)` → 80 (range)

**Fixture workflow:**
1. Generate with openpyxl (LARGE and SMALL do not need `_xlfn.` prefix)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn large_small()` in `conformance/statistical.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "LARGE, SMALL k-th largest/smallest" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change LARGE, SMALL from `.` to `x`. Remove "Prelude: sorted" note from LARGE. |
| `docs/roadmap/v0.3/README.md` | Tick the LARGE / SMALL checkbox |

## Streaming invariant

Does not violate. Both sort the expanded range values inline within a single evaluation — no cross-row reads, no prelude dependency.
