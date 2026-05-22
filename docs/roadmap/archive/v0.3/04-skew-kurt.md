# Feature: SKEW / SKEW.P / KURT

**Branch:** `feat/skew-kurt`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Higher-order moment statistics: skewness (3rd moment) and excess kurtosis (4th moment).

- `SKEW(range)` — sample skewness (adjusted): `[n / ((n-1)(n-2))] * Σ[(xi - mean) / stdev_s]³`
- `SKEW.P(range)` — population skewness: `(1/n) * Σ[(xi - mean) / stdev_p]³`
- `KURT(range)` — excess kurtosis (sample-adjusted): `[(n(n+1)) / ((n-1)(n-2)(n-3))] * Σ[(xi - mean) / stdev_s]⁴ - [3(n-1)² / ((n-2)(n-3))]`

```
=SKEW(1, 2, 3, 4, 100)   → positive (right-skewed)
=SKEW(1, 97, 98, 99, 100) → negative (left-skewed)
=SKEW(1, 2, 3, 4, 5)      → 0 (symmetric)
=KURT(1, 2, 3, 4, 5)      → -1.2 (platykurtic — lighter tails than normal)
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — empty module (doc comment only), already declared in `builtins/mod.rs`
- `crates/xlstream-eval/src/builtins/aggregate.rs` — `average()` pattern for iterating values, skipping text/bool/empty, propagating errors
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with match arms; `eval_args()` for eager evaluation
- The STDEV/VAR spec (`02-stdev-var.md`) proposes `var_s`, `var_p`, `stdev_s`, `stdev_p` and a shared `collect_numbers()` helper — SKEW and KURT build on these
- Not in `UNSUPPORTED_FUNCTIONS` — classifies as RowLocal by default
- `docs/functions.md` lists SKEW, SKEW.P, KURT as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home (alongside STDEV/VAR)
- `crates/xlstream-eval/src/builtins/mod.rs:87-209` — `dispatch()` — add three match arms
- `crates/xlstream-eval/src/builtins/aggregate.rs:141-168` — `average()` — reference pattern
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance infrastructure
- `crates/xlstream-eval/tests/fixtures/` — fixture directory

## Resolution / Evaluation behavior

All three are pure functions of their arguments — row-local, no prelude, no lookups, no streaming concerns.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager dispatch — `dispatch()` calls `eval_args()`, passes `&[Value]` to the builtin.

**Implementation:** All three depend on mean and standard deviation (sample or population). Reuse the `collect_numbers()` helper from STDEV/VAR.

1. Collect numeric values (skip text/bool/empty, propagate errors)
2. Compute mean
3. Compute stdev (sample for SKEW/KURT, population for SKEW.P)
4. Compute the sum of standardized moments (cubed for skew, fourth power for kurt)
5. Apply the adjustment factor

If stdev = 0 (all values identical), return `#DIV/0!` — division by zero in the standardized moment.

**Value handling (must match Excel):**
- `Value::Number(n)` — include
- `Value::Integer(i)` — include (cast to f64)
- `Value::Date(d)` — include (use serial)
- `Value::Error(e)` — propagate immediately
- `Value::Text(_)` — skip
- `Value::Bool(_)` — skip
- `Value::Empty` — skip

**Minimum n:**
- SKEW: n >= 3 required (denominator has (n-1)(n-2)). n < 3 → `#DIV/0!`
- SKEW.P: n >= 1 required (divides by n). n == 0 → `#DIV/0!`. stdev_p == 0 → `#DIV/0!`
- KURT: n >= 4 required (denominator has (n-1)(n-2)(n-3)). n < 4 → `#DIV/0!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `skew([1, 2, 3, 4, 100])` → positive value (right-skewed)
- `skew([1, 97, 98, 99, 100])` → negative value (left-skewed)
- `skew([1, 2, 3, 4, 5])` → 0.0 (symmetric)
- `skew_p([1, 2, 3, 4, 5])` → 0.0 (symmetric)
- `kurt([1, 2, 3, 4, 5])` → -1.2 (uniform-like, platykurtic)

**Edge cases:**
- Minimum n for SKEW: `skew([1, 2, 3])` → valid result (n=3)
- Below minimum SKEW: `skew([1, 2])` → `#DIV/0!`
- Below minimum SKEW: `skew([1])` → `#DIV/0!`
- Minimum n for SKEW.P: `skew_p([5])` → `#DIV/0!` (n=1 but stdev_p=0)
- SKEW.P with 2 values: `skew_p([1, 3])` → valid
- Minimum n for KURT: `kurt([1, 2, 3, 4])` → valid (n=4)
- Below minimum KURT: `kurt([1, 2, 3])` → `#DIV/0!`
- All same values: `skew([5, 5, 5])` → `#DIV/0!` (stdev=0)
- All same values: `kurt([5, 5, 5, 5])` → `#DIV/0!` (stdev=0)
- Empty input: all three → `#DIV/0!`
- Text skipped: `skew([1, "text", 3, 5])` → same as `skew([1, 3, 5])`
- Bool skipped: `kurt([1, TRUE, 3, 5, 7])` → same as `kurt([1, 3, 5, 7])`
- Error propagation: `skew([1, #N/A, 3, 5])` → `#N/A`

**Regression guards:**
- Existing aggregate and statistical tests must pass unchanged
- Existing conformance suite unaffected

### Conformance fixture

Create `tests/fixtures/statistical/skew_kurt.xlsx` with a Sheet1 data layout and a Sheet2 for cross-sheet tests.

**Sheet1 data (columns A-F):**
- A: "Symmetric" header, rows 2-6: `1, 2, 3, 4, 5`
- B: "RightSkew" header, rows 2-6: `1, 2, 3, 4, 100`
- C: "LeftSkew" header, rows 2-6: `1, 97, 98, 99, 100`
- D: "Same" header, rows 2-6: `5, 5, 5, 5, 5`
- E: "Mixed" header, rows 2-7: `1, "text", 3, TRUE, 5, 7`
- F: "Error" header, rows 2-5: `1, =NA(), 3, 5`
- G: "Small" header, rows 2-4: `10, 20, 30`
- H: "Pair" header, rows 2-3: `1, 9`
- I: "Big" header, rows 2-11: `2, 4, 6, 8, 10, 12, 14, 16, 18, 20`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (place in column J, starting row 2):**

Happy path — SKEW (4):
1. `=SKEW(A2:A6)` → 0.0 (symmetric)
2. `=SKEW(B2:B6)` → positive (right-skewed)
3. `=SKEW(C2:C6)` → negative (left-skewed)
4. `=SKEW(I2:I11)` → 0.0 (symmetric, 10 values)

Happy path — SKEW.P (2):
5. `=_xlfn.SKEW.P(A2:A6)` → 0.0 (symmetric)
6. `=_xlfn.SKEW.P(B2:B6)` → positive (right-skewed, population)

Happy path — KURT (3):
7. `=KURT(A2:A6)` → -1.2 (uniform, platykurtic)
8. `=KURT(B2:B6)` → positive (heavy right tail)
9. `=KURT(I2:I11)` → -1.2 (uniform, 10 values)

Minimum n / boundary (5):
10. `=SKEW(G2:G4)` → valid (n=3, minimum for SKEW)
11. `=SKEW(H2:H3)` → `#DIV/0!` (n=2, below minimum)
12. `=_xlfn.SKEW.P(H2:H3)` → valid (n=2, population)
13. `=KURT(A2:A5)` → valid (n=4, minimum for KURT)
14. `=KURT(G2:G4)` → `#DIV/0!` (n=3, below minimum)

All same values — stdev=0 (2):
15. `=SKEW(D2:D6)` → `#DIV/0!`
16. `=KURT(D2:D6)` → `#DIV/0!`

Type coercion (2):
17. `=SKEW(E2:E7)` — mixed with text and bool (only 1, 3, 5, 7 counted = 4 values)
18. `=KURT(E2:E7)` — same range (4 numeric values, minimum for KURT)

Error propagation (2):
19. `=SKEW(F2:F5)` → `#N/A` (contains NA())
20. `=KURT(F2:F5)` → `#N/A`

Nested usage (2):
21. `=IF(SKEW(B2:B6)>0, "right", "left")` → "right"
22. `=IFERROR(KURT(G2:G4), "n/a")` → "n/a" (catches #DIV/0!)

Cross-sheet (2):
23. `=SKEW(Sheet2!A2:A6)` — cross-sheet reference
24. `=KURT(Sheet2!A2:A6)` — cross-sheet reference (n=5, enough for KURT)

Combined (1):
25. `=SKEW(A2:A6) + KURT(A2:A6)` → arithmetic combination (0 + (-1.2) = -1.2)

**Fixture workflow:**
1. Generate with openpyxl (formulas use `_xlfn.SKEW.P` prefix where needed + data, no cached values)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn skew_kurt()` in `conformance/statistical.rs` calling `run_conformance("statistical/skew_kurt.xlsx")`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "SKEW, SKEW.P, KURT higher-order moment statistics" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change SKEW, SKEW.P, KURT from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the SKEW/SKEW.P and KURT checkboxes |

## Streaming invariant

Does not violate. All three are pure functions of their cell-reference arguments — no cross-row reads, no prelude dependency, no lookup sheets.
