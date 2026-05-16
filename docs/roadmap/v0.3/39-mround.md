# Feature: MROUND

**Branch:** `feat/mround`
**Effort:** ~1 hour
**Crates:** xlstream-eval

## What

Rounds a number to the nearest multiple of a specified value.

- `MROUND(number, multiple)` — round to nearest multiple of `multiple`.

If `number` and `multiple` have different signs, returns `#NUM!`. If `multiple` is 0, returns 0.

```
=MROUND(10, 3)       -> 9
=MROUND(7.5, 3)      -> 9
=MROUND(-10, -3)     -> -9
=MROUND(1.3, 0.2)    -> 1.4
=MROUND(5, 0)        -> 0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:256-297` — `builtin_ceiling`, `builtin_floor` — 2-arg rounding with significance (closely related pattern: round to multiple)
- `crates/xlstream-eval/src/builtins/math.rs:154-170` — `builtin_mod` — 2-arg with division-by-zero handling
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists MROUND as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:256-297` — CEILING/FLOOR (round to multiple pattern)
- `crates/xlstream-eval/src/builtins/math.rs:154-170` — MOD (2-arg, division handling)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

Pure scalar function — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**MROUND(number, multiple):**
1. Arity check: exactly 2 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate.
3. Extract multiple as f64 via `num_arg`. Error -> propagate.
4. If multiple == 0.0 -> return `Value::Number(0.0)`.
5. If number and multiple have different signs (one positive, one negative) -> return `#NUM!`.
6. Compute: `(number / multiple).round() * multiple`. This rounds to the nearest multiple using standard half-away-from-zero rounding.
7. Return `Value::Number(result)`.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- number > 0 and multiple < 0: `#NUM!`
- number < 0 and multiple > 0: `#NUM!`
- multiple == 0: returns 0 (not an error)
- Error in arg: propagate

## Tests

### Unit tests (in `math.rs`)

**MROUND happy path:**
- `mround(10, 3)` -> 9.0
- `mround(7.5, 3)` -> 9.0 (7.5/3 = 2.5, rounds to 3, 3*3 = 9)
- `mround(-10, -3)` -> -9.0
- `mround(1.3, 0.2)` -> 1.4 (1.3/0.2 = 6.5, rounds to 7, 7*0.2 = 1.4... but may need Decimal for precision)
- `mround(6, 3)` -> 6.0

**MROUND midpoint rounding:**
- `mround(4.5, 3)` -> 6.0 (4.5/3 = 1.5, rounds to 2)
- `mround(1.5, 3)` -> 3.0 (1.5/3 = 0.5, rounds to 1... actually 0.5 rounds to 1, 1*3=3)

**MROUND multiple=0:**
- `mround(5, 0)` -> 0.0
- `mround(0, 0)` -> 0.0

**MROUND number=0:**
- `mround(0, 3)` -> 0.0

**MROUND sign mismatch (#NUM!):**
- `mround(5, -3)` -> `#NUM!`
- `mround(-5, 3)` -> `#NUM!`

**Error propagation:**
- `mround(#N/A, 3)` -> `#N/A`
- `mround(10, #N/A)` -> `#N/A`

**Arity errors:**
- `mround(10)` -> `#VALUE!`
- `mround(10, 3, 1)` -> `#VALUE!`

**Coercion:**
- `mround(TRUE, 1)` -> 1.0 (TRUE = 1.0)
- `mround("10", 3)` -> 9.0

**Type mismatch:**
- `mround("abc", 3)` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/mround.xlsx`.

**Sheet1 data:**
- A: "Number" header, rows 2-10: `10, 7.5, -10, 1.3, 5, 0, 6, 4.5, -5`
- B: "Multiple" header, rows 2-10: `3, 3, -3, 0.2, 0, 3, 3, 3, 3`
- C: "Error" header, row 2: `=NA()`
- D: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `10`
- B: "Mult" header, row 2: `3`

**Formulas (column E, starting row 2) — 22 formulas:**

Happy path (5):
1. `=MROUND(A2, B2)` -> 9 (10, nearest mult of 3)
2. `=MROUND(A3, B3)` -> 9 (7.5, rounds to 9)
3. `=MROUND(A4, B4)` -> -9 (-10, nearest mult of -3)
4. `=MROUND(A5, B5)` -> 1.4 (1.3, nearest mult of 0.2)
5. `=MROUND(A8, B8)` -> 6 (6, exact multiple)

Multiple=0 (1):
6. `=MROUND(A6, B6)` -> 0 (multiple=0 returns 0)

Number=0 (1):
7. `=MROUND(A7, B7)` -> 0 (0 rounds to 0)

Midpoint rounding (1):
8. `=MROUND(A9, B9)` -> 6 (4.5 / 3 = 1.5, rounds to 2)

Sign mismatch (2):
9. `=MROUND(A6, -3)` -> `#NUM!` (5 with -3)
10. `=MROUND(A10, B10)` -> `#NUM!` (-5 with 3, different signs)

Negative both (1):
11. `=MROUND(-7.5, -3)` -> -9

Error propagation (2):
12. `=MROUND(C2, 3)` -> `#N/A`
13. `=MROUND(10, C2)` -> `#N/A`

Type error (1):
14. `=MROUND(D2, 3)` -> `#VALUE!`

Coercion (2):
15. `=MROUND(TRUE, 1)` -> 1
16. `=MROUND("10", 3)` -> 9

Nested (2):
17. `=IF(MROUND(10, 3)=9, "yes", "no")` -> "yes"
18. `=IFERROR(MROUND(5, -3), "err")` -> "err"

Combined (2):
19. `=MROUND(10, 3) + MROUND(10, 5)` -> 9 + 10 = 19
20. `=CEILING(10, 3) - MROUND(10, 3)` -> 12 - 9 = 3

Cross-sheet (1):
21. `=MROUND(Sheet2!A2, Sheet2!B2)` -> 9

Additional (1):
22. `=MROUND(1.5, 3)` -> 3 (0.5 rounds up to 1, 1*3=3)

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn mround()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "MROUND round-to-multiple function" under `[Unreleased]` |
| `docs/functions.md` | Change MROUND from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the MROUND checkbox |

## Streaming invariant

Does not violate. Pure scalar function of its arguments — no cross-row reads, no prelude dependency.
