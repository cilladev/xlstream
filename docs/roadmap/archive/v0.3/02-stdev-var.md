# Feature: STDEV.S / STDEV.P / VAR.S / VAR.P

**Branch:** `feat/stdev-var`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

Sample and population variance and standard deviation.

- `VAR.S(range)` — sample variance (divides by n-1)
- `VAR.P(range)` — population variance (divides by n)
- `STDEV.S(range)` — sample standard deviation = sqrt(VAR.S)
- `STDEV.P(range)` — population standard deviation = sqrt(VAR.P)

```
=VAR.S(2, 4, 6, 8)    → 6.666...
=VAR.P(2, 4, 6, 8)    → 5.0
=STDEV.S(2, 4, 6, 8)  → 2.581...
=STDEV.P(2, 4, 6, 8)  → 2.236...
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — empty module (4 lines, doc comment only), already declared as `mod statistical` in `builtins/mod.rs:18`
- `crates/xlstream-eval/src/builtins/aggregate.rs` — `average()`, `sum()`, `count()` follow the same `fn(values: &[Value]) -> Result<Value, CellError>` pattern: iterate values, skip text/bool/empty, propagate errors, accumulate numerics
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` function with match arms for all builtins; `eval_args()` helper for eager argument evaluation
- Not in `UNSUPPORTED_FUNCTIONS` — will classify as RowLocal by default
- `docs/functions.md` lists all four as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home
- `crates/xlstream-eval/src/builtins/mod.rs:87-209` — `dispatch()` — add four match arms
- `crates/xlstream-eval/src/builtins/aggregate.rs:141-168` — `average()` — reference pattern for iterating values, skipping text/bool/empty, propagating errors
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test infrastructure
- `crates/xlstream-eval/tests/conformance/aggregate.rs` — example conformance test calling `run_conformance()`
- `crates/xlstream-eval/tests/fixtures/aggregate/` — example fixture directory

## Resolution / Evaluation behavior

All four are pure functions of their arguments — row-local, no prelude, no lookups, no streaming concerns.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager dispatch — `dispatch()` calls `eval_args()` to evaluate all arguments, passes the `&[Value]` slice to the builtin function.

**Implementation approach:** Implement `var_s` and `var_p` first. Both do a two-pass computation:
1. First pass: compute mean (same logic as `average()` — skip text/bool/empty, propagate errors, count numerics)
2. Second pass: compute sum of squared deviations from mean

Then `stdev_s` = `var_s().map(|v| v.sqrt())` and `stdev_p` = `var_p().map(|v| v.sqrt())`.

Shared helper: extract the "collect numeric values, propagate errors" logic into a helper function since it's used by average, var, stdev, and will be reused by skew, kurt, avedev.

**Value handling (must match Excel):**
- `Value::Number(n)` — include
- `Value::Integer(i)` — include (cast to f64)
- `Value::Date(d)` — include (use serial)
- `Value::Error(e)` — propagate immediately
- `Value::Text(_)` — skip
- `Value::Bool(_)` — skip
- `Value::Empty` — skip

**Minimum n:**
- VAR.S / STDEV.S: n >= 2 required (divides by n-1). n < 2 → `#DIV/0!`
- VAR.P / STDEV.P: n >= 1 required (divides by n). n == 0 → `#DIV/0!`

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `var_s([2, 4, 6, 8])` → 6.666...
- `var_p([2, 4, 6, 8])` → 5.0
- `stdev_s([2, 4, 6, 8])` → 2.581...
- `stdev_p([2, 4, 6, 8])` → 2.236...

**Edge cases:**
- Single value: `var_s([5])` → `#DIV/0!`, `var_p([5])` → 0.0
- Single value: `stdev_s([5])` → `#DIV/0!`, `stdev_p([5])` → 0.0
- Two values: `var_s([3, 7])` → 8.0, `stdev_s([3, 7])` → 2.828...
- All same values: `var_s([5, 5, 5])` → 0.0, `stdev_s([5, 5, 5])` → 0.0
- Empty input: all four → `#DIV/0!`
- All text: `var_s(["a", "b"])` → `#DIV/0!`
- Text skipped: `var_s([1, "text", 3])` → same as `var_s([1, 3])`
- Bool skipped: `var_s([1, TRUE, 3])` → same as `var_s([1, 3])`
- Error propagation: `var_s([1, #N/A, 3])` → `#N/A`
- Negative numbers: `var_s([-2, -4, -6])` — correct result
- Large numbers: `var_p([1e10, 1e10+1])` — numerical stability

**Regression guards:**
- Existing aggregate tests must pass unchanged
- Existing conformance suite unaffected

### Conformance fixture

Create `tests/fixtures/statistical/stdev_var.xlsx` with a Sheet1 data layout and a Sheet2 for cross-sheet tests.

**Sheet1 data (columns A-F):**
- A: "Value" header, rows 2-11: `2, 4, 6, 8, 10, -3, -1, 0, 100, 0.5`
- B: "Text" header, rows 2-6: `"a", "b", "c", "d", "e"`
- C: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- D: "Same" header, rows 2-6: `7, 7, 7, 7, 7`
- E: "Error" header, rows 2-4: `1, =NA(), 3`
- F: "Single" header, row 2: `42`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (place in columns G-H, starting row 2):**

Happy path (8):
1. `=_xlfn.VAR.S(A2:A11)` — sample variance of 10 mixed numbers
2. `=_xlfn.VAR.P(A2:A11)` — population variance of 10 mixed numbers
3. `=_xlfn.STDEV.S(A2:A11)` — sample stdev of 10 mixed numbers
4. `=_xlfn.STDEV.P(A2:A11)` — population stdev of 10 mixed numbers
5. `=_xlfn.VAR.S(A2:A5)` — 4 positive values
6. `=_xlfn.VAR.P(A2:A5)` — 4 positive values
7. `=_xlfn.STDEV.S(A2:A5)` — 4 positive values
8. `=_xlfn.STDEV.P(A2:A5)` — 4 positive values

Single value / boundary (4):
9. `=_xlfn.VAR.S(F2)` → `#DIV/0!` (n=1, n-1=0)
10. `=_xlfn.VAR.P(F2)` → 0
11. `=_xlfn.STDEV.S(F2)` → `#DIV/0!`
12. `=_xlfn.STDEV.P(F2)` → 0

All same values (2):
13. `=_xlfn.VAR.S(D2:D6)` → 0
14. `=_xlfn.STDEV.P(D2:D6)` → 0

Two values — minimum for .S (2):
15. `=_xlfn.VAR.S(A2:A3)` — two values (n-1=1)
16. `=_xlfn.STDEV.S(A2:A3)` — two values

Type coercion — text and bool skipped (3):
17. `=_xlfn.VAR.S(C2:C6)` — mixed with text and bool (only 1, 3, 5 counted)
18. `=_xlfn.STDEV.P(C2:C6)` — same range, population
19. `=_xlfn.VAR.S(B2:B6)` → `#DIV/0!` (all text, 0 numeric values)

Error propagation (2):
20. `=_xlfn.VAR.S(E2:E4)` → `#N/A` (contains NA())
21. `=_xlfn.STDEV.P(E2:E4)` → `#N/A`

Nested usage (2):
22. `=IF(_xlfn.STDEV.S(A2:A11)>10,"high","low")` — nested in IF
23. `=IFERROR(_xlfn.VAR.S(F2),"n/a")` — IFERROR catching #DIV/0!

Cross-sheet (2):
24. `=_xlfn.STDEV.S(Sheet2!A2:A6)` — cross-sheet reference
25. `=_xlfn.VAR.P(Sheet2!A2:A6)` — cross-sheet reference

Combined (1):
26. `=_xlfn.STDEV.S(A2:A11)+_xlfn.VAR.P(A2:A11)` — arithmetic combination

Negative numbers (1):
27. `=_xlfn.VAR.S(A7:A9)` — negative and zero values (-3, -1, 0)

**Fixture workflow:**
1. Generate with openpyxl (formulas use `_xlfn.` prefix + data, no cached values)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn stdev_var()` in `conformance/statistical.rs` calling `run_conformance("statistical/stdev_var.xlsx")`
4. Create `conformance/statistical.rs` module if it doesn't exist, add `mod statistical;` to `conformance/mod.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "STDEV.S, STDEV.P, VAR.S, VAR.P statistical functions" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change STDEV.S, STDEV.P, VAR.S, VAR.P from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the STDEV/VAR checkbox |

## Streaming invariant

Does not violate. All four are pure functions of their cell-reference arguments — no cross-row reads, no prelude dependency, no lookup sheets.
