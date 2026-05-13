# Feature: MODE.SNGL

**Branch:** `feat/mode-sngl`
**Effort:** ~2 hours
**Crates:** xlstream-eval, xlstream-parse

## What

Returns the most frequently occurring value in a data set. If multiple values share the highest frequency, returns the one that appears first in the data.

- `MODE.SNGL(range, ...)` — single mode. Accepts one or more ranges/values.

```
=MODE.SNGL(1, 2, 3, 3, 4)    → 3     (3 appears twice)
=MODE.SNGL(1, 2, 2, 3, 3, 4) → 2     (tie: 2 and 3 both appear twice, 2 comes first)
=MODE.SNGL(1, 2, 3, 4)       → #N/A  (no repeats — all unique)
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — module with `collect_numerics`, `finite_or_num`
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with range-expanding wrapper pattern (empty-args guard, `expand_range`, `map_or_else`)
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS` phf set
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists MODE.SNGL as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:269-276` — `builtin_var_s` wrapper — reference pattern for single-range stat functions
- `crates/xlstream-eval/src/builtins/mod.rs:238-248` — dispatch match arms
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS`
- `crates/xlstream-eval/tests/conformance/statistical.rs` — conformance test module

## Resolution / Evaluation behavior

Pure function — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** Same dispatch pattern as VAR.S/STDEV.S — all args are range-expanded, concatenated into a single `Vec<Value>`, passed to the pure function. The wrapper follows the standard pattern:
1. Empty-args guard → `#VALUE!`
2. Expand all args via `expand_range`, flat_map into `Vec<Value>`
3. Call `mode_sngl(&values)` → `Result<f64, CellError>`
4. `map_or_else(Value::Error, Value::Number)`

**Implementation approach:**

1. Collect numerics via `collect_numerics`
2. If empty, return `#N/A`
3. Count frequency of each value. Use a `HashMap<u64, (usize, usize)>` where key is `f64::to_bits()` (exact bit comparison), value is `(count, first_position)`.
4. Find the maximum count. If max count == 1 (no repeats), return `#N/A`.
5. Among values with max count, return the one with the smallest first_position (first occurrence in data).

Note on float equality: Excel uses exact bit-level comparison for MODE. `0.1 + 0.2` and `0.3` are different values. Using `f64::to_bits()` matches this behavior. Special case: +0.0 and -0.0 have different bits but should be treated as equal. Normalize by mapping -0.0 to +0.0 before hashing.

**Value handling:** Uses `collect_numerics` for the range. Same skip/propagate rules as all stat functions.

**Error conditions:**
- No repeating value → `#N/A`
- No numeric values → `#N/A`
- Error in data range → propagate
- No arguments → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `mode_sngl([1, 2, 3, 3, 4])` → 3.0
- `mode_sngl([1, 2, 2, 3, 3, 4])` → 2.0 (tie broken by first occurrence)
- `mode_sngl([5, 5, 5])` → 5.0
- `mode_sngl([1.5, 1.5, 2.5, 2.5, 2.5])` → 2.5

**Edge cases:**
- All unique: `mode_sngl([1, 2, 3, 4])` → `#N/A`
- Single value: `mode_sngl([5])` → `#N/A` (no repeat)
- Two identical: `mode_sngl([5, 5])` → 5.0
- Empty: `mode_sngl([])` → `#N/A`
- All text: `mode_sngl(["a", "b"])` → `#N/A`
- Negative values: `mode_sngl([-3, -3, -1, -1, -1])` → -1.0
- Zero: `mode_sngl([0, 0, 1])` → 0.0
- Large count: `mode_sngl([1, 1, 1, 2, 2, 3])` → 1.0

**Type handling:**
- Text skipped: `mode_sngl([1, "text", 1, 3])` → 1.0
- Bool skipped: `mode_sngl([1, TRUE, 1, 3])` → 1.0
- Error propagation: `mode_sngl([1, #N/A, 1, 3])` → `#N/A`

**Float edge cases:**
- Positive and negative zero: `mode_sngl([0.0, -0.0, 1.0])` → 0.0 (both zeros are the same value)

### Conformance fixture

Create `tests/fixtures/statistical/mode_sngl.xlsx`.

**Sheet1 data (columns A-F):**
- A: "Repeats" header, rows 2-11: `10, 20, 30, 30, 40, 40, 40, 50, 60, 70`
- B: "Unique" header, rows 2-6: `1, 2, 3, 4, 5`
- C: "TieFirst" header, rows 2-7: `1, 1, 2, 2, 3, 3`
- D: "Mixed" header, rows 2-6: `1, "text", 1, TRUE, 3`
- E: "Error" header, rows 2-4: `1, =NA(), 1`
- F: "Single" header, row 2: `42`

**Sheet2 data:**
- A: "XS" header, rows 2-7: `10, 10, 20, 20, 20, 30`

**Formulas (column G, starting row 2):**

Happy path (3):
1. `=_xlfn.MODE.SNGL(A2:A11)` → 40 (appears 3 times)
2. `=_xlfn.MODE.SNGL(C2:C7)` → 1 (tie: 1, 2, 3 all appear twice; 1 is first)
3. `=_xlfn.MODE.SNGL(A2:A5)` → 30 (appears twice in 10, 20, 30, 30)

All unique (1):
4. `=_xlfn.MODE.SNGL(B2:B6)` → `#N/A`

Single value (1):
5. `=_xlfn.MODE.SNGL(F2)` → `#N/A`

Type coercion (2):
6. `=_xlfn.MODE.SNGL(D2:D6)` → 1 (only 1, 1, 3 counted — 1 repeats)
7. `=_xlfn.MODE.SNGL(1, "text", 1, 3)` → 1 (literal args, text skipped)

Error propagation (1):
8. `=_xlfn.MODE.SNGL(E2:E4)` → `#N/A`

Nested (2):
9. `=IF(_xlfn.MODE.SNGL(A2:A11)>30, "above", "below")` → "above"
10. `=IFERROR(_xlfn.MODE.SNGL(B2:B6), "none")` → "none"

Cross-sheet (1):
11. `=_xlfn.MODE.SNGL(Sheet2!A2:A7)` → 20 (appears 3 times)

Combined (1):
12. `=_xlfn.MODE.SNGL(A2:A11) + 10` → 50

Multiple ranges (1):
13. `=_xlfn.MODE.SNGL(B2:B4, B2:B4)` → 1 (each value appears twice now; 1 is first)

Negative / zero (2):
14. `=_xlfn.MODE.SNGL(-5, -5, -3, -3, -3)` → -3
15. `=_xlfn.MODE.SNGL(0, 0, 1, 2)` → 0

**Fixture workflow:**
1. Generate with openpyxl (formulas use `_xlfn.MODE.SNGL` prefix)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn mode_sngl()` in `conformance/statistical.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "MODE.SNGL most frequent value" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change MODE.SNGL from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the MODE.SNGL checkbox |

## Streaming invariant

Does not violate. MODE.SNGL is a pure function of its expanded range arguments — no cross-row reads, no prelude dependency.
