# Feature: AVEDEV

**Branch:** `feat/avedev`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Average of absolute deviations from the mean.

`AVEDEV(range)` = `(1/n) * Σ|xi - mean|`

```
=AVEDEV(2, 4, 8, 16)  → 4.5   (mean=7.5, deviations: 5.5+3.5+0.5+8.5 = 18, 18/4 = 4.5)
=AVEDEV(5)             → 0     (single value, deviation from itself is 0)
=AVEDEV(3, 3, 3)       → 0     (all same, no deviation)
```

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback in `interp.rs`.

## What already exists

- `crates/xlstream-eval/src/builtins/statistical.rs` — empty module (doc comment only), already declared in `builtins/mod.rs`
- `crates/xlstream-eval/src/builtins/aggregate.rs` — `average()` follows the same `fn(values: &[Value]) -> Result<Value, CellError>` pattern: iterate values, skip text/bool/empty, propagate errors, accumulate numerics
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with match arms; `eval_args()` for eager evaluation
- The STDEV/VAR spec (`02-stdev-var.md`) proposes a shared helper to collect numeric values — AVEDEV should reuse it
- Not in `UNSUPPORTED_FUNCTIONS` — classifies as RowLocal by default
- `docs/functions.md` lists it as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/statistical.rs` — implementation home (alongside STDEV/VAR if implemented first)
- `crates/xlstream-eval/src/builtins/mod.rs:87-209` — `dispatch()` — add one match arm
- `crates/xlstream-eval/src/builtins/aggregate.rs:141-168` — `average()` — reference pattern for mean computation, value filtering, error propagation
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test infrastructure
- `crates/xlstream-eval/tests/fixtures/` — fixture directory structure

## Resolution / Evaluation behavior

Pure function of its arguments — row-local, no prelude, no lookups, no streaming concerns.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager dispatch — `dispatch()` calls `eval_args()` to evaluate all arguments, passes the `&[Value]` slice to `builtin_avedev`.

**Implementation:** Two-pass over the collected numeric values:
1. First pass: compute mean (sum / count)
2. Second pass: compute sum of absolute deviations from mean

If STDEV/VAR lands first and introduces a `collect_numbers()` helper, reuse it. Otherwise, extract the same logic from `average()`.

**Value handling (must match Excel):**
- `Value::Number(n)` — include
- `Value::Integer(i)` — include (cast to f64)
- `Value::Date(d)` — include (use serial)
- `Value::Error(e)` — propagate immediately
- `Value::Text(_)` — skip
- `Value::Bool(_)` — skip
- `Value::Empty` — skip

**Minimum n:** n >= 1 required (divides by n). n == 0 → `#DIV/0!`.

## Tests

### Unit tests (in `statistical.rs`)

**Happy path:**
- `avedev([2, 4, 8, 16])` → 4.5
- `avedev([1, 2, 3, 4, 5])` → 1.2
- `avedev([-2, -1, 0, 1, 2])` → 1.2

**Edge cases:**
- Single value: `avedev([5])` → 0.0
- Two values: `avedev([0, 10])` → 5.0
- All same: `avedev([7, 7, 7])` → 0.0
- Empty input: `avedev([])` → `#DIV/0!`
- All text: `avedev(["a", "b"])` → `#DIV/0!`
- Text skipped: `avedev([1, "text", 3])` → same as `avedev([1, 3])` = 1.0
- Bool skipped: `avedev([1, TRUE, 3])` → same as `avedev([1, 3])` = 1.0
- Error propagation: `avedev([1, #N/A, 3])` → `#N/A`
- Large numbers: `avedev([1e10, 1e10+2])` → 1.0

**Regression guards:**
- Existing aggregate and statistical tests must pass unchanged
- Existing conformance suite unaffected

### Conformance fixture

Create `tests/fixtures/statistical/avedev.xlsx` with a Sheet1 data layout and a Sheet2 for cross-sheet tests.

**Sheet1 data (columns A-F):**
- A: "Value" header, rows 2-11: `2, 4, 8, 16, -3, -1, 0, 1, 5, 10`
- B: "Text" header, rows 2-5: `"a", "b", "c", "d"`
- C: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- D: "Same" header, rows 2-5: `7, 7, 7, 7`
- E: "Error" header, rows 2-4: `1, =NA(), 3`
- F: "Single" header, row 2: `42`

**Sheet2 data:**
- A: "XS" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (place in column G, starting row 2):**

Happy path (4):
1. `=AVEDEV(A2:A11)` — 10 mixed values
2. `=AVEDEV(A2:A5)` — 4 positive values (2, 4, 8, 16)
3. `=AVEDEV(A6:A9)` — negative and small values (-3, -1, 0, 1)
4. `=AVEDEV(1, 2, 3, 4, 5)` — literal args

Single value / boundary (3):
5. `=AVEDEV(F2)` → 0 (single value)
6. `=AVEDEV(A2:A3)` → 1.0 (two values: 2, 4; mean=3, |2-3|+|4-3|=2, 2/2=1)
7. `=AVEDEV(D2:D5)` → 0 (all same values)

Type coercion (3):
8. `=AVEDEV(C2:C6)` — mixed with text and bool (only 1, 3, 5 counted)
9. `=AVEDEV(B2:B5)` → `#DIV/0!` (all text, 0 numeric values)
10. `=AVEDEV(TRUE, FALSE, 5)` — booleans skipped, only 5 counted → 0

Error propagation (2):
11. `=AVEDEV(E2:E4)` → `#N/A` (contains NA())
12. `=AVEDEV(1, 2, 1/0)` → `#DIV/0!` (inline error)

Nested usage (2):
13. `=IF(AVEDEV(A2:A11)>3, "high", "low")` — nested in IF
14. `=IFERROR(AVEDEV(B2:B5), "n/a")` — IFERROR catching #DIV/0!

Cross-sheet (1):
15. `=AVEDEV(Sheet2!A2:A6)` — cross-sheet reference (10,20,30,40,50; mean=30, deviations=20+10+0+10+20=60, 60/5=12)

Combined with other functions (2):
16. `=AVEDEV(A2:A11) / AVERAGE(A2:A11)` — ratio of deviation to mean
17. `=AVEDEV(A2:A11) + AVEDEV(A6:A9)` — sum of two AVEDEVs

AVEDEV does NOT need `_xlfn.` prefix in LibreOffice.

**Fixture workflow:**
1. Generate with openpyxl (formulas + data, no cached values)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn avedev()` in `conformance/statistical.rs` calling `run_conformance("statistical/avedev.xlsx")`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "AVEDEV average absolute deviation" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change AVEDEV from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the AVEDEV checkbox |

## Streaming invariant

Does not violate. AVEDEV is a pure function of its cell-reference arguments — no cross-row reads, no prelude dependency, no lookup sheets.
