# Feature: COVARIANCE.P / COVARIANCE.S

**Branch:** `feat/covariance`
**Effort:** ~2 hours
**Crates:** xlstream-eval, xlstream-parse
**Depends on:** CORREL (spec 16) for the `collect_paired_numerics` helper

## What

Population and sample covariance between two data sets.

- `COVARIANCE.P(array1, array2)` — population covariance (divides by n)
- `COVARIANCE.S(array1, array2)` — sample covariance (divides by n-1)

```
=COVARIANCE.P({1,2,3,4,5}, {2,4,6,8,10})  → 4.0
=COVARIANCE.S({1,2,3,4,5}, {2,4,6,8,10})  → 5.0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — `collect_numerics`, `mean_and_variance`, `finite_or_num`. If CORREL lands first, a `collect_paired_numerics` helper will also be available.
- `crates/xlstream-eval/src/builtins/mod.rs` — 2-arg range-expanding dispatch pattern
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:238-250` — dispatch pattern
- `crates/xlstream-parse/src/sets.rs:138-147` — `RANGE_EXPANDING_FUNCTIONS`

## Resolution / Evaluation behavior

Pure function of two same-length arrays — row-local, no prelude, no streaming concerns.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** Two-arg range-expanding dispatch. Same pattern as CORREL.

**Implementation approach:**

Reuse `collect_paired_numerics` (from CORREL). Then:
- COVARIANCE.P: `Σ((xi - mean_x)(yi - mean_y)) / n`
- COVARIANCE.S: `Σ((xi - mean_x)(yi - mean_y)) / (n - 1)`

**Value handling:** Same as CORREL — paired numeric collection, skip pairs where either value is non-numeric.

**Error conditions:**
- COVARIANCE.P: n < 1 → `#DIV/0!`
- COVARIANCE.S: n < 2 → `#DIV/0!`
- Arrays of different length → `#N/A`
- Error in either array → propagate
- Wrong arity → `#VALUE!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `covariance_p([1,2,3,4,5], [2,4,6,8,10])` → 4.0
- `covariance_s([1,2,3,4,5], [2,4,6,8,10])` → 5.0
- `covariance_p([1,2,3], [3,2,1])` → -0.666... (negative)
- `covariance_s([1,2,3], [3,2,1])` → -1.0

**Edge cases:**
- Different length: `covariance_p([1,2], [1])` → `#N/A`
- Single pair P: `covariance_p([1], [2])` → 0.0
- Single pair S: `covariance_s([1], [2])` → `#DIV/0!`
- Two pairs S: `covariance_s([1,2], [3,4])` → 0.5
- Empty: `covariance_p([], [])` → `#DIV/0!`
- All same: `covariance_p([5,5,5], [5,5,5])` → 0.0

**Type handling:**
- Text skipped pairwise: `covariance_p([1, "x", 3], [2, "y", 4])` → same as `([1,3], [2,4])`
- Error propagation: `covariance_p([1, #N/A, 3], [2, 4, 6])` → `#N/A`

### Conformance fixture

Create `tests/fixtures/statistical/covariance.xlsx`.

**Sheet1 data:**
- A: "X" header, rows 2-11: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`
- B: "Y" header, rows 2-11: `2, 4, 6, 8, 10, 12, 14, 16, 18, 20`
- C: "Y_neg" header, rows 2-11: `20, 18, 16, 14, 12, 10, 8, 6, 4, 2`
- D: "Const" header, rows 2-6: `5, 5, 5, 5, 5`
- E: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- F: "Error" header, rows 2-4: `1, =NA(), 3`

**Sheet2:**
- A/B: paired data for cross-sheet test

**Formulas (column G, starting row 2) — 18 formulas:**

Happy path (4):
1. `=_xlfn.COVARIANCE.P(A2:A11, B2:B11)` → 16.5
2. `=_xlfn.COVARIANCE.S(A2:A11, B2:B11)` → 18.333...
3. `=_xlfn.COVARIANCE.P(A2:A11, C2:C11)` → -16.5
4. `=_xlfn.COVARIANCE.S(A2:A11, C2:C11)` → -18.333...

Boundary (3):
5. `=_xlfn.COVARIANCE.P(A2:A3, B2:B3)` → 1.0
6. `=_xlfn.COVARIANCE.S(A2:A3, B2:B3)` → 2.0
7. `=_xlfn.COVARIANCE.P(A2:A6, D2:D6)` → 0.0

Type coercion (2):
8. `=_xlfn.COVARIANCE.P(A2:A6, E2:E6)` — mixed
9. `=_xlfn.COVARIANCE.S(A2:A6, E2:E6)` — mixed

Error propagation (1):
10. `=_xlfn.COVARIANCE.P(A2:A4, F2:F4)` → `#N/A`

Nested (2):
11. `=IF(_xlfn.COVARIANCE.P(A2:A11, B2:B11)>10, "high", "low")` → "high"
12. `=IFERROR(_xlfn.COVARIANCE.S(A2:A3, D2:D3), "n/a")` — may or may not error

Cross-sheet (1):
13. `=_xlfn.COVARIANCE.P(Sheet2!A2:A6, Sheet2!B2:B6)`

Combined (1):
14. `=_xlfn.COVARIANCE.S(A2:A11, B2:B11) / (_xlfn.COVARIANCE.S(A2:A11, A2:A11))` — regression slope

Self-covariance (2):
15. `=_xlfn.COVARIANCE.P(A2:A11, A2:A11)` → equals VAR.P
16. `=_xlfn.COVARIANCE.S(A2:A11, A2:A11)` → equals VAR.S

COVARIANCE.P/S need `_xlfn.` prefix for LibreOffice.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "COVARIANCE.P, COVARIANCE.S covariance" under `[Unreleased]` |
| `docs/functions.md` | Change COVARIANCE.P, COVARIANCE.S from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the COVARIANCE checkbox |

## Streaming invariant

Does not violate. Pure function of expanded range arguments.
