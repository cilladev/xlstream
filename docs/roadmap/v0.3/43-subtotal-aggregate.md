# Feature: SUBTOTAL / AGGREGATE

**Branch:** `feat/subtotal-aggregate`
**Effort:** ~6-8 hours (may defer parts to v0.4)
**Crates:** xlstream-eval

## What

Multi-mode aggregate dispatch functions. Each takes a `function_num` argument that selects which aggregate operation to perform, then applies it to the remaining arguments.

- `SUBTOTAL(function_num, ref1, [ref2], ...)` — applies one of 11 aggregate functions. function_num 1-11 = AVERAGE, COUNT, COUNTA, MAX, MIN, PRODUCT, STDEV.S, STDEV.P, SUM, VAR.S, VAR.P. function_num 101-111 = same but ignores manually hidden rows.
- `AGGREGATE(function_num, options, ref1, [ref2], ...)` — extended SUBTOTAL. function_num 1-19 adds MEDIAN, MODE.SNGL, LARGE, SMALL, PERCENTILE.INC, QUARTILE.INC, PERCENTILE.EXC, QUARTILE.EXC. Options 0-7 control what to ignore (errors, hidden rows, nested SUBTOTALs, combinations).

```
=SUBTOTAL(9, A1:A10)         -> same as SUM(A1:A10)
=SUBTOTAL(1, A1:A10)         -> same as AVERAGE(A1:A10)
=SUBTOTAL(109, A1:A10)       -> SUM ignoring hidden rows
=AGGREGATE(9, 6, A1:A10)     -> SUM ignoring errors and hidden rows
=AGGREGATE(14, 6, A1:A10, 2) -> LARGE ignoring errors, k=2
```

**NOTE: This is the most complex spec in v0.3. Some parts (hidden-row awareness, AGGREGATE options, functions 14-19) may be deferred to v0.4 if they require architectural changes. The v0.3 scope covers SUBTOTAL function_nums 1-11 on column ranges (prelude-computed) and row-local usage.**

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- Existing aggregate implementations already present in the evaluator:
  - SUM, AVERAGE, COUNT, COUNTA, MAX, MIN — prelude-computed for column ranges
  - STDEV.S, STDEV.P, VAR.S, VAR.P — in `crates/xlstream-eval/src/builtins/stats.rs`
  - PRODUCT — if implemented; otherwise needs adding
  - MEDIAN, MODE.SNGL, LARGE, SMALL, PERCENTILE, QUARTILE — in stats.rs (from earlier v0.3 specs)
- `crates/xlstream-eval/src/prelude.rs` — prelude aggregation logic for column-range aggregates
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — possible implementation home (or a new `aggregate.rs` module)
- `crates/xlstream-eval/src/builtins/stats.rs` — existing aggregate functions to dispatch to
- `crates/xlstream-eval/src/prelude.rs` — prelude aggregation (for column-range SUBTOTAL)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — dispatch
- `crates/xlstream-eval/src/classify.rs` — formula classification (may need SUBTOTAL-aware classification)

## Resolution / Evaluation behavior

SUBTOTAL and AGGREGATE are meta-functions: they dispatch to other aggregate functions based on `function_num`. Their streaming behavior depends on what they're dispatching to and what range they operate on.

**Classification:** Depends on arguments.
- SUBTOTAL/AGGREGATE on a column range (e.g., `A:A`, `A1:A10000`): PreludeAggregate — same as calling the underlying function directly on that range.
- SUBTOTAL/AGGREGATE on row-local cells (e.g., `A1, B1, C1`): RowLocal.
- The classifier needs to look at `function_num` (if it's a literal) and the ref arguments to determine classification.

**Prelude:** Same as the underlying function. SUBTOTAL(9, A:A) has the same prelude requirement as SUM(A:A).

**Row eval:**
- For row-local usage: evaluate all ref args, then dispatch to the appropriate aggregate function.
- For prelude-computed column ranges: look up the prelude-computed value (same as the underlying function would).

**SUBTOTAL(function_num, ref1, [ref2], ...):**
1. Arity check: at least 2 args. Otherwise `#VALUE!`.
2. Extract function_num as f64 via `num_arg`. Error -> propagate. Truncate to integer.
3. Validate function_num: must be 1-11 or 101-111. Otherwise `#VALUE!`.
4. Determine the underlying function:
   - 1/101: AVERAGE
   - 2/102: COUNT
   - 3/103: COUNTA
   - 4/104: MAX
   - 5/105: MIN
   - 6/106: PRODUCT
   - 7/107: STDEV.S (STDEV)
   - 8/108: STDEV.P (STDEVP)
   - 9/109: SUM
   - 10/110: VAR.S (VAR)
   - 11/111: VAR.P (VARP)
5. function_num 101-111: ignore hidden rows. **In v0.3, xlstream does not track row visibility, so 101-111 behave identically to 1-11.** Document this limitation.
6. Collect remaining args (ref1, ref2, ...) and pass to the underlying aggregate function.
7. Return the result.

**AGGREGATE(function_num, options, ref1, [ref2], ...):**
1. Arity check: at least 3 args. Otherwise `#VALUE!`.
2. Extract function_num as f64 via `num_arg`. Error -> propagate. Truncate to integer.
3. Validate function_num: must be 1-19. Otherwise `#VALUE!`.
4. Extract options as f64 via `num_arg`. Error -> propagate. Truncate to integer.
5. Validate options: must be 0-7. Otherwise `#VALUE!`.
6. Determine the underlying function (1-11 same as SUBTOTAL, plus):
   - 12: MEDIAN
   - 13: MODE.SNGL
   - 14: LARGE (requires extra k argument)
   - 15: SMALL (requires extra k argument)
   - 16: PERCENTILE.INC
   - 17: QUARTILE.INC
   - 18: PERCENTILE.EXC
   - 19: QUARTILE.EXC
7. Options control what to ignore:
   - 0: nothing (ignore nested SUBTOTAL/AGGREGATE only)
   - 1: ignore hidden rows + nested SUBTOTAL/AGGREGATE
   - 2: ignore errors + nested SUBTOTAL/AGGREGATE
   - 3: ignore hidden rows + errors + nested SUBTOTAL/AGGREGATE
   - 4: nothing
   - 5: ignore hidden rows
   - 6: ignore errors
   - 7: ignore hidden rows + errors
8. **v0.3 scope:** Implement options 0, 2, 4, 6 (non-hidden-row options). Hidden-row awareness (options 1, 3, 5, 7) returns correct results only if no rows are hidden (same as SUBTOTAL 101-111). Nested SUBTOTAL/AGGREGATE ignoring is deferred.
9. For options that ignore errors: filter out `Value::Error` from the collected values before passing to the aggregate function.
10. Return the result.

**v0.3 scope (implement):**
- SUBTOTAL with function_num 1-11 (and 101-111 as aliases)
- AGGREGATE with function_num 1-13, options 0, 2, 4, 6

**May defer to v0.4:**
- AGGREGATE function_num 14-19 (LARGE, SMALL, PERCENTILE, QUARTILE — require extra k/quart argument with different arg layout)
- Hidden-row awareness (SUBTOTAL 101-111 distinct behavior, AGGREGATE options 1, 3, 5, 7)
- Nested SUBTOTAL/AGGREGATE detection and exclusion
- AGGREGATE with array-form arguments

**Error conditions:**
- SUBTOTAL: fewer than 2 args -> `#VALUE!`
- SUBTOTAL: function_num not in 1-11 or 101-111 -> `#VALUE!`
- AGGREGATE: fewer than 3 args -> `#VALUE!`
- AGGREGATE: function_num not in 1-19 -> `#VALUE!`
- AGGREGATE: options not in 0-7 -> `#VALUE!`
- Non-numeric function_num/options -> `#VALUE!`
- Error in function_num/options arg -> propagate
- Underlying function errors propagate (e.g., AVERAGE of empty range -> `#DIV/0!`)

## Tests

### Unit tests (in new `aggregate.rs` or `math.rs`)

**SUBTOTAL function dispatch (11 tests):**
- `subtotal(1, [1,2,3,4,5])` -> 3.0 (AVERAGE)
- `subtotal(2, [1,2,"a",TRUE,5])` -> 3 (COUNT numbers only)
- `subtotal(3, [1,2,"a",TRUE,5])` -> 5 (COUNTA)
- `subtotal(4, [1,2,3,4,5])` -> 5 (MAX)
- `subtotal(5, [1,2,3,4,5])` -> 1 (MIN)
- `subtotal(6, [1,2,3,4,5])` -> 120 (PRODUCT)
- `subtotal(7, [2,4,4,4,5,5,7,9])` -> ~2.138 (STDEV.S)
- `subtotal(8, [2,4,4,4,5,5,7,9])` -> ~2.0 (STDEV.P)
- `subtotal(9, [1,2,3,4,5])` -> 15 (SUM)
- `subtotal(10, [2,4,4,4,5,5,7,9])` -> ~4.571 (VAR.S)
- `subtotal(11, [2,4,4,4,5,5,7,9])` -> ~4.0 (VAR.P)

**SUBTOTAL 101-111 (alias for v0.3):**
- `subtotal(109, [1,2,3])` -> 6 (same as subtotal(9) in v0.3)

**SUBTOTAL errors:**
- `subtotal(0, [1,2])` -> `#VALUE!` (invalid function_num)
- `subtotal(12, [1,2])` -> `#VALUE!`
- `subtotal(100, [1,2])` -> `#VALUE!`
- `subtotal(112, [1,2])` -> `#VALUE!`

**AGGREGATE basic dispatch:**
- `aggregate(9, 0, [1,2,3])` -> 6 (SUM, no ignoring)
- `aggregate(1, 0, [1,2,3])` -> 2 (AVERAGE)
- `aggregate(4, 0, [1,5,3])` -> 5 (MAX)

**AGGREGATE error ignoring:**
- `aggregate(9, 6, [1,#N/A,3])` -> 4 (SUM, ignore errors: skip #N/A)
- `aggregate(1, 6, [1,#N/A,3])` -> 2 (AVERAGE, ignore errors)
- `aggregate(9, 0, [1,#N/A,3])` -> `#N/A` (no ignoring, error propagates)

**AGGREGATE errors:**
- `aggregate(0, 0, [1,2])` -> `#VALUE!`
- `aggregate(20, 0, [1,2])` -> `#VALUE!`
- `aggregate(9, 8, [1,2])` -> `#VALUE!` (options > 7)
- `aggregate(9, -1, [1,2])` -> `#VALUE!`

**Error propagation:**
- `subtotal(#N/A, [1,2])` -> `#N/A`
- `aggregate(#N/A, 0, [1,2])` -> `#N/A`

**Arity errors:**
- `subtotal(9)` -> `#VALUE!` (no refs)
- `aggregate(9, 0)` -> `#VALUE!` (no refs)

### Conformance fixture

Create `tests/fixtures/math/subtotal_aggregate.xlsx`.

**Sheet1 data:**
- A: "Values" header, rows 2-11: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`
- B: "Mixed" header, rows 2-6: `1, "text", 3, TRUE, 5`
- C: "WithError" header, rows 2-5: `1, =NA(), 3, 5`
- D: "FuncNum" header, rows 2-12: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11`

**Sheet2 data:**
- A: "Val" header, rows 2-6: `10, 20, 30, 40, 50`

**Formulas (column E, starting row 2) — 30 formulas:**

SUBTOTAL function_num 1-11 (11):
1. `=SUBTOTAL(1, A2:A11)` -> 5.5 (AVERAGE)
2. `=SUBTOTAL(2, A2:A11)` -> 10 (COUNT)
3. `=SUBTOTAL(3, A2:A11)` -> 10 (COUNTA)
4. `=SUBTOTAL(4, A2:A11)` -> 10 (MAX)
5. `=SUBTOTAL(5, A2:A11)` -> 1 (MIN)
6. `=SUBTOTAL(6, A2:A11)` -> 3628800 (PRODUCT = 10!)
7. `=SUBTOTAL(7, A2:A11)` -> ~3.028 (STDEV.S)
8. `=SUBTOTAL(8, A2:A11)` -> ~2.872 (STDEV.P)
9. `=SUBTOTAL(9, A2:A11)` -> 55 (SUM)
10. `=SUBTOTAL(10, A2:A11)` -> ~9.167 (VAR.S)
11. `=SUBTOTAL(11, A2:A11)` -> ~8.25 (VAR.P)

SUBTOTAL 101-111 alias (1):
12. `=SUBTOTAL(109, A2:A11)` -> 55 (same as SUM in v0.3)

SUBTOTAL with mixed types (2):
13. `=SUBTOTAL(2, B2:B6)` -> 2 (COUNT: only numbers 1, 3... wait, TRUE may count. COUNT counts numbers: 1, 3, 5 = 3)
14. `=SUBTOTAL(9, B2:B6)` -> 9 (SUM: 1+3+5, text/bool ignored in SUM)

SUBTOTAL errors (2):
15. `=SUBTOTAL(0, A2:A11)` -> `#VALUE!`
16. `=SUBTOTAL(12, A2:A11)` -> `#VALUE!`

AGGREGATE basic (3):
17. `=AGGREGATE(9, 0, A2:A11)` -> 55 (SUM)
18. `=AGGREGATE(1, 0, A2:A11)` -> 5.5 (AVERAGE)
19. `=AGGREGATE(4, 0, A2:A11)` -> 10 (MAX)

AGGREGATE ignore errors (3):
20. `=AGGREGATE(9, 6, C2:C5)` -> 9 (SUM: 1+3+5, skip #N/A)
21. `=AGGREGATE(1, 6, C2:C5)` -> 3 (AVERAGE: (1+3+5)/3)
22. `=AGGREGATE(9, 0, C2:C5)` -> `#N/A` (no ignoring)

AGGREGATE errors (2):
23. `=AGGREGATE(0, 0, A2:A11)` -> `#VALUE!`
24. `=AGGREGATE(9, 8, A2:A11)` -> `#VALUE!`

Error propagation (1):
25. `=SUBTOTAL(9, C2:C5)` -> `#N/A` (error propagates)

Nested (2):
26. `=IF(SUBTOTAL(9, A2:A11)=55, "yes", "no")` -> "yes"
27. `=IFERROR(SUBTOTAL(0, A2:A11), "bad")` -> "bad"

Cross-sheet (1):
28. `=SUBTOTAL(9, Sheet2!A2:A6)` -> 150

Row-local SUBTOTAL (1):
29. `=SUBTOTAL(9, A2, A3, A4)` -> 6 (SUM of 1+2+3)

Combined (1):
30. `=SUBTOTAL(9, A2:A11) - SUBTOTAL(1, A2:A11) * SUBTOTAL(2, A2:A11)` -> 55 - 5.5*10 = 0

Note: SUBTOTAL and AGGREGATE do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn subtotal_aggregate()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "SUBTOTAL, AGGREGATE multi-mode aggregate functions (function_num 1-13, basic options)" under `[Unreleased]` |
| `docs/functions.md` | Change SUBTOTAL, AGGREGATE from `.` to `x` (with note: hidden-row and nested-SUBTOTAL ignoring deferred) |
| `docs/roadmap/v0.3/README.md` | Tick the SUBTOTAL / AGGREGATE checkbox |

## Streaming invariant

Does not violate when used on column ranges — prelude-computed, same as the underlying aggregate function. Does not violate when used on row-local cells — pure scalar of current-row values.

**Caveat:** SUBTOTAL/AGGREGATE with function_num 101-111 (ignore hidden rows) is a no-op difference in v0.3 because xlstream does not track row visibility. This is a known limitation, not a streaming invariant violation. If hidden-row tracking is added in the future, the prelude pass would need to be aware of row visibility — but that's architectural, not an invariant violation.

**Nested SUBTOTAL/AGGREGATE exclusion** (where SUBTOTAL skips cells that are themselves SUBTOTAL results) is deferred. In streaming mode, we'd need to mark cells as "SUBTOTAL-produced" during the row pass. This is feasible but adds complexity. Defer to v0.4.
