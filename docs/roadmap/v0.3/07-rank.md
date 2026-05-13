# Feature: RANK.EQ / RANK.AVG

**Branch:** `feat/rank`
**Effort:** ~0.5 day
**Crates:** xlstream-eval, xlstream-parse

## What

Rank functions that return the rank of a number within a list.

- `RANK.EQ(number, ref, [order])` — rank of `number` in `ref`. If duplicates exist, returns the smallest rank (top rank). `order` 0 or omitted = descending (largest is rank 1); `order` nonzero = ascending (smallest is rank 1).
- `RANK.AVG(number, ref, [order])` — same as RANK.EQ but duplicates get the average of their ranks.

```
=RANK.EQ(3, {5,3,3,1}, 0)   → 2   (descending: 5=1, 3=2, 3=2, 1=4)
=RANK.AVG(3, {5,3,3,1}, 0)  → 2.5 (descending: 5=1, 3=avg(2,3)=2.5, 1=4)
=RANK.EQ(3, {1,3,3,5}, 1)   → 2   (ascending: 1=1, 3=2, 3=2, 5=4)
=RANK.AVG(3, {1,3,3,5}, 1)  → 2.5 (ascending: 1=1, 3=avg(2,3)=2.5, 5=4)
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — module with `collect_numerics`, `finite_or_num`
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with range-expanding wrapper pattern
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS` phf set
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:238-248` — dispatch pattern
- `crates/xlstream-eval/src/builtins/mod.rs:40-81` — `expand_range` helper
- `crates/xlstream-parse/src/sets.rs:138-143` — `RANGE_EXPANDING_FUNCTIONS`
- `crates/xlstream-eval/tests/conformance/statistical.rs` — conformance test module

## Resolution / Evaluation behavior

Both are pure functions — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** Three arguments: `args[0]` is the number to rank (eval as scalar), `args[1]` is the data range (expand via `expand_range`), `args[2]` is optional order (eval as scalar, default 0). The wrapper must:
1. Check arity (2 or 3 args)
2. Evaluate `args[0]` as scalar, coerce to f64 — this is the value to rank
3. Expand `args[1]` into `Vec<Value>` via `expand_range`
4. Optionally evaluate `args[2]` as scalar, coerce to f64 (default 0)
5. Call the pure function

**Implementation approach:**

RANK.EQ:
1. Collect numerics from range via `collect_numerics`
2. If `number` is not found in the list, return `#N/A`
3. Count how many values are greater (descending) or less (ascending) than `number`
4. Rank = count + 1

RANK.AVG:
1. Same as RANK.EQ, but also count duplicates
2. Rank = (first_rank + last_rank) / 2, where first_rank is the EQ rank and last_rank = first_rank + dup_count - 1

**Value handling:** Uses `collect_numerics` for the range. The `number` argument is coerced to f64 from the evaluated Value.

**Error conditions:**
- Number not found in ref → `#N/A`
- No numeric values in ref → `#N/A`
- Error in data range → propagate
- Error in number → propagate
- Wrong arity → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `rank_eq(3, [5,3,1], 0)` → 2 (descending)
- `rank_eq(3, [1,3,5], 1)` → 2 (ascending)
- `rank_eq(5, [5,3,1], 0)` → 1 (highest, descending)
- `rank_eq(1, [5,3,1], 0)` → 3 (lowest, descending)
- `rank_avg(3, [5,3,3,1], 0)` → 2.5 (duplicates averaged)
- `rank_avg(3, [1,3,3,5], 1)` → 2.5 (ascending, duplicates averaged)

**Edge cases:**
- Not found: `rank_eq(4, [1,2,3,5], 0)` → `#N/A`
- Single value found: `rank_eq(5, [5], 0)` → 1
- Single value not found: `rank_eq(3, [5], 0)` → `#N/A`
- All same: `rank_eq(5, [5,5,5], 0)` → 1
- All same AVG: `rank_avg(5, [5,5,5], 0)` → 2.0 (average of 1,2,3)
- Empty range: `rank_eq(1, [], 0)` → `#N/A`
- Default order (omitted): `rank_eq(3, [5,3,1])` → 2 (descending by default)
- Negative values: `rank_eq(-1, [-3,-1,0,2], 1)` → 2

**Type handling:**
- Text skipped in range: `rank_eq(3, [1, "text", 3, 5], 0)` → 2
- Bool skipped in range: `rank_eq(3, [1, TRUE, 3, 5], 0)` → 2
- Error in range: `rank_eq(3, [1, #N/A, 3], 0)` → `#N/A`

**Float equality:**
- Exact match: `rank_eq(0.1+0.2, [0.3, 0.5, 0.7], 0)` — this is tricky due to float precision. Excel uses a tolerance. Test what behavior we want and document the decision.

### Conformance fixture

Create `tests/fixtures/statistical/rank.xlsx`.

**Sheet1 data (columns A-F):**
- A: "Values" header, rows 2-11: `10, 20, 30, 30, 40, 50, 60, 70, 80, 90`
- B: "NoDups" header, rows 2-6: `5, 10, 15, 20, 25`
- C: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- D: "Error" header, rows 2-4: `1, =NA(), 3`
- E: "Single" header, row 2: `42`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (column F, starting row 2):**

Happy path — RANK.EQ (5):
1. `=_xlfn.RANK.EQ(30, A2:A11, 0)` → 7 (descending, first of duplicates)
2. `=_xlfn.RANK.EQ(30, A2:A11, 1)` → 3 (ascending, first of duplicates)
3. `=_xlfn.RANK.EQ(90, A2:A11, 0)` → 1 (highest, descending)
4. `=_xlfn.RANK.EQ(10, A2:A11, 0)` → 10 (lowest, descending)
5. `=_xlfn.RANK.EQ(10, A2:A11, 1)` → 1 (lowest, ascending)

Happy path — RANK.AVG (3):
6. `=_xlfn.RANK.AVG(30, A2:A11, 0)` → 7.5 (descending, avg of 7 and 8)
7. `=_xlfn.RANK.AVG(30, A2:A11, 1)` → 3.5 (ascending, avg of 3 and 4)
8. `=_xlfn.RANK.AVG(90, A2:A11, 0)` → 1 (no duplicates)

Not found (1):
9. `=_xlfn.RANK.EQ(35, A2:A11, 0)` → `#N/A`

Default order (1):
10. `=_xlfn.RANK.EQ(50, A2:A11)` → 5 (default descending)

Single value (1):
11. `=_xlfn.RANK.EQ(42, E2, 0)` → 1

Type coercion (2):
12. `=_xlfn.RANK.EQ(3, C2:C6, 0)` → 2 (only 1, 3, 5 counted)
13. `=_xlfn.RANK.AVG(3, C2:C6, 1)` → 2

Error propagation (1):
14. `=_xlfn.RANK.EQ(1, D2:D4, 0)` → `#N/A`

Nested (2):
15. `=IF(_xlfn.RANK.EQ(50, A2:A11, 0)<=5, "top half", "bottom half")` → "top half"
16. `=IFERROR(_xlfn.RANK.EQ(999, A2:A11, 0), "not found")` → "not found"

Cross-sheet (1):
17. `=_xlfn.RANK.EQ(30, Sheet2!A2:A6, 0)` → 3

Combined (1):
18. `=_xlfn.RANK.AVG(30, A2:A11, 0) / 10` → 0.75

No-dups (1):
19. `=_xlfn.RANK.EQ(15, B2:B6, 1)` → 3

**Fixture workflow:**
1. Generate with openpyxl (`_xlfn.RANK.EQ` / `_xlfn.RANK.AVG` prefix)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn rank()` in `conformance/statistical.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "RANK.EQ, RANK.AVG rank functions" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change RANK.EQ, RANK.AVG from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the RANK checkbox |

## Streaming invariant

Does not violate. Both are pure functions of their expanded range arguments — no cross-row reads, no prelude dependency.
