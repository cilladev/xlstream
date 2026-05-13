# Feature: DELTA / GESTEP

**Branch:** `feat/delta-gestep`
**Effort:** ~1 hour
**Crates:** xlstream-eval

## What

Kronecker delta and unit step — the two simplest engineering functions.

- `DELTA(number1, [number2])` — returns 1 if number1 == number2, else 0
- `GESTEP(number, [step])` — returns 1 if number >= step, else 0

```
=DELTA(5, 5)       → 1
=DELTA(5, 4)       → 0
=DELTA(5)           → 0          (default number2 = 0)
=DELTA(0)           → 1
=GESTEP(5, 4)       → 1
=GESTEP(-4, -5)     → 1
=GESTEP(-1)          → 0          (default step = 0)
=GESTEP(0)           → 1
```

Both return a number (0 or 1), not a boolean. Second argument defaults to 0 for both functions. Comparison uses exact IEEE 754 float equality (`==`) for DELTA and `>=` for GESTEP — no epsilon tolerance. This matches Excel.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — empty module (module doc only, lines 1-5). First engineering functions to land here.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). No dispatch arms yet for engineering functions.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper (extracts f64 from args)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- Not in dispatch
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (currently empty)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:159-175` — math builtins dispatch pattern (pure, eager eval)
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

Both functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. All args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Same pattern as math builtins.

**DELTA(number1, [number2]):**
1. Arity check: 1-2 args. Otherwise `#VALUE!`
2. Extract arg 0 as f64 via `num_arg_ce`. Error → propagate.
3. Extract arg 1 as f64 via `num_arg_ce`, defaulting to 0.0 if absent.
4. Return `Value::Number(1.0)` if `number1 == number2` (exact IEEE 754 equality), else `Value::Number(0.0)`

**GESTEP(number, [step]):**
1. Arity check: 1-2 args. Otherwise `#VALUE!`
2. Extract arg 0 as f64 via `num_arg_ce`. Error → propagate.
3. Extract arg 1 as f64 via `num_arg_ce`, defaulting to 0.0 if absent.
4. Return `Value::Number(1.0)` if `number >= step`, else `Value::Number(0.0)`

**Error conditions:**
- Wrong arity: `#VALUE!` (DELTA requires 1-2, GESTEP requires 1-2)
- Non-numeric args: `#VALUE!` (via `coerce::to_number` failure)
- Error in any arg: propagate
- NaN inputs: `DELTA(NaN, NaN)` → 0 (NaN != NaN in IEEE 754). `GESTEP(NaN, 0)` → 0 (NaN >= 0 is false). Excel does not produce NaN from user input, but our engine should handle it gracefully if it appears.

**Edge cases:**
- Negative zero: `DELTA(0.0, -0.0)` → 1 (IEEE 754: `0.0 == -0.0` is true)
- Infinity: `DELTA(INF, INF)` → 1. `GESTEP(INF, INF)` → 1. `GESTEP(-INF, INF)` → 0.
- Very close floats: `DELTA(1.0, 1.0000000000000002)` → 0 (no epsilon — exact equality)
- Boolean coercion: `DELTA(TRUE, 1)` → 1 (TRUE coerces to 1.0). `GESTEP(FALSE, 0)` → 1.
- Text coercion: `DELTA("5", 5)` → 1 (text "5" coerces to 5.0). `DELTA("abc", 0)` → `#VALUE!`.

## Tests

### Unit tests (in `engineering.rs`)

**DELTA happy path:**
- `delta(5.0, 5.0)` → 1.0
- `delta(5.0, 4.0)` → 0.0
- `delta(0.0, 0.0)` → 1.0
- `delta(-3.0, -3.0)` → 1.0
- `delta(1.5, 1.5)` → 1.0

**DELTA default second arg:**
- `delta(0.0)` → 1.0 (default number2 = 0)
- `delta(5.0)` → 0.0
- `delta(-1.0)` → 0.0

**DELTA edge cases:**
- `delta(0.0, -0.0)` → 1.0 (negative zero equals zero)
- `delta(f64::INFINITY, f64::INFINITY)` → 1.0
- `delta(f64::NEG_INFINITY, f64::NEG_INFINITY)` → 1.0
- `delta(f64::INFINITY, f64::NEG_INFINITY)` → 0.0
- `delta(f64::NAN, f64::NAN)` → 0.0 (NaN != NaN)
- `delta(1.0, 1.0 + f64::EPSILON)` → 0.0 (no epsilon tolerance)

**DELTA errors:**
- Non-numeric text → `#VALUE!`
- Error input → propagate (`#N/A` → `#N/A`)
- 0 args → `#VALUE!`
- 3 args → `#VALUE!`

**DELTA coercion:**
- `delta(TRUE, 1)` → 1.0 (bool coerces)
- `delta("5", 5)` → 1.0 (text coerces)

**GESTEP happy path:**
- `gestep(5.0, 4.0)` → 1.0
- `gestep(4.0, 5.0)` → 0.0
- `gestep(5.0, 5.0)` → 1.0 (equal counts)
- `gestep(-4.0, -5.0)` → 1.0
- `gestep(0.0, 0.0)` → 1.0

**GESTEP default second arg:**
- `gestep(0.0)` → 1.0 (default step = 0)
- `gestep(1.0)` → 1.0
- `gestep(-1.0)` → 0.0

**GESTEP edge cases:**
- `gestep(0.0, -0.0)` → 1.0
- `gestep(-0.0, 0.0)` → 1.0 (0.0 >= -0.0 is true, and -0.0 >= 0.0 is also true)
- `gestep(f64::INFINITY, 0.0)` → 1.0
- `gestep(f64::NEG_INFINITY, 0.0)` → 0.0
- `gestep(f64::NAN, 0.0)` → 0.0

**GESTEP errors:**
- Non-numeric text → `#VALUE!`
- Error input → propagate
- 0 args → `#VALUE!`
- 3 args → `#VALUE!`

**GESTEP coercion:**
- `gestep(TRUE, 0)` → 1.0 (TRUE = 1.0 >= 0)
- `gestep("5", 4)` → 1.0

### Conformance fixture

Create `tests/fixtures/engineering/delta_gestep.xlsx`.

**Sheet1 data:**
- A: "Num1" header, rows 2-9: `5, 0, -3, 1.5, 1, 0, -1, 100`
- B: "Num2" header, rows 2-9: `5, 0, -3, 1.5, 0, 0, 0, 99`
- C: "Step" header, rows 2-7: `4, 0, -5, 5, -1, 0`
- D: "Error" header, row 2: `=NA()`
- E: "Text" header, rows 2-3: `"abc"`, `"5"`

**Sheet2 data:**
- A: "Val" header, rows 2-3: `5, 0`

**Formulas (column F, starting row 2) — 25 formulas:**

DELTA happy path (4):
1. `=DELTA(A2, B2)` → 1 (5 == 5)
2. `=DELTA(A3, B3)` → 1 (0 == 0)
3. `=DELTA(A4, B4)` → 1 (-3 == -3)
4. `=DELTA(A5, B5)` → 1 (1.5 == 1.5)

DELTA not equal (2):
5. `=DELTA(A6, B6)` → 0 (1 != 0)
6. `=DELTA(A9, B9)` → 0 (100 != 99)

DELTA default second arg (3):
7. `=DELTA(A3)` → 1 (0 == 0 default)
8. `=DELTA(A2)` → 0 (5 != 0 default)
9. `=DELTA(A8)` → 0 (-1 != 0 default)

GESTEP happy path (4):
10. `=GESTEP(A2, C2)` → 1 (5 >= 4)
11. `=GESTEP(A3, C3)` → 1 (0 >= 0)
12. `=GESTEP(A8, C4)` → 1 (-1 >= -5)
13. `=GESTEP(A3, C5)` → 0 (0 >= 5 is false)

GESTEP equal (1):
14. `=GESTEP(A2, A2)` → 1 (5 >= 5)

GESTEP default step (3):
15. `=GESTEP(A2)` → 1 (5 >= 0)
16. `=GESTEP(A3)` → 1 (0 >= 0)
17. `=GESTEP(A8)` → 0 (-1 >= 0 is false)

Error propagation (2):
18. `=DELTA(D2, 0)` → `#N/A`
19. `=GESTEP(D2, 0)` → `#N/A`

Non-numeric text (2):
20. `=DELTA(E2, 0)` → `#VALUE!` ("abc" not numeric)
21. `=GESTEP(E2, 0)` → `#VALUE!`

Text coercion (2):
22. `=DELTA(E3, 5)` → 1 ("5" coerces to 5.0)
23. `=GESTEP(E3, 4)` → 1 ("5" coerces to 5.0 >= 4)

Nested / combined (2):
24. `=IF(DELTA(A2, B2)=1, "match", "differ")` → "match"
25. `=DELTA(A2, B2) + GESTEP(A2, C2)` → 2 (1 + 1)

Cross-sheet (1):
26. `=DELTA(Sheet2!A2, A2)` → 1 (both 5)

DELTA and GESTEP do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn delta_gestep()` in `conformance/engineering.rs` (new file if HEX2DEC hasn't landed yet, or append)

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "DELTA, GESTEP Kronecker delta and unit step" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change DELTA and GESTEP from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the DELTA / GESTEP checkbox |

## Streaming invariant

Does not violate. Both functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
