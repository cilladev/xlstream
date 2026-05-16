# Feature: DEGREES / RADIANS

**Branch:** `feat/degrees-radians`
**Effort:** ~1 hour
**Crates:** xlstream-eval

## What

Angle unit conversion functions. Both take a single numeric argument and perform a linear conversion between radians and degrees.

- `DEGREES(angle)` — convert radians to degrees: `angle * 180 / PI`
- `RADIANS(angle)` — convert degrees to radians: `angle * PI / 180`

```
=DEGREES(PI())        -> 180
=DEGREES(1)           -> 57.295779513...
=RADIANS(180)         -> 3.141592654... (PI)
=RADIANS(90)          -> 1.570796327... (PI/2)
=RADIANS(0)           -> 0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:396-404` — `builtin_sin` — exact 1-arg pattern to follow (trivial, no domain check)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:396-404` — SIN pattern (simplest 1-arg, no domain check)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**DEGREES(angle):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `angle * 180.0 / std::f64::consts::PI`.
4. Return `Value::Number(result)`.

**RADIANS(angle):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`.
2. Extract arg as f64 via `num_arg`. Error -> propagate.
3. Compute `angle * std::f64::consts::PI / 180.0`.
4. Return `Value::Number(result)`.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- Error in arg: propagate
- No domain restrictions. Both are pure linear operations, no overflow for reasonable inputs. Extreme values could overflow f64 but this mirrors Excel behavior.

## Tests

### Unit tests (in `math.rs`)

**DEGREES happy path:**
- `degrees(PI)` -> 180.0
- `degrees(PI/2)` -> 90.0
- `degrees(0)` -> 0.0
- `degrees(1)` -> ~57.2957795131
- `degrees(-PI)` -> -180.0

**RADIANS happy path:**
- `radians(180)` -> PI (~3.14159265359)
- `radians(90)` -> PI/2 (~1.57079632679)
- `radians(0)` -> 0.0
- `radians(360)` -> 2*PI (~6.28318530718)
- `radians(-180)` -> -PI

**Round-trip identity:**
- `degrees(radians(45))` -> 45.0
- `radians(degrees(1))` -> 1.0

**Error propagation:**
- `degrees(#N/A)` -> `#N/A`
- `radians(#N/A)` -> `#N/A`

**Arity errors:**
- `degrees()` -> `#VALUE!`
- `degrees(1, 2)` -> `#VALUE!`
- `radians()` -> `#VALUE!`

**Coercion:**
- `degrees(TRUE)` -> ~57.296 (TRUE = 1.0)
- `radians("180")` -> PI

**Type mismatch:**
- `degrees("abc")` -> `#VALUE!`
- `radians("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/degrees_radians.xlsx`.

**Sheet1 data:**
- A: "Angle" header, rows 2-8: `0, 1, -1, 180, 90, 360, 45`
- B: "Pi" header, row 2: `=PI()`
- C: "Error" header, row 2: `=NA()`
- D: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `90`

**Formulas (column E, starting row 2) — 20 formulas:**

DEGREES happy path (4):
1. `=DEGREES(B2)` -> 180 (PI -> 180)
2. `=DEGREES(A2)` -> 0
3. `=DEGREES(A3)` -> ~57.296 (1 radian)
4. `=DEGREES(A4)` -> ~-57.296 (-1 radian)

RADIANS happy path (4):
5. `=RADIANS(A5)` -> ~3.14159 (180 degrees)
6. `=RADIANS(A6)` -> ~1.5708 (90 degrees)
7. `=RADIANS(A2)` -> 0
8. `=RADIANS(A7)` -> ~6.28319 (360 degrees)

Negative (1):
9. `=RADIANS(-180)` -> ~-3.14159

Round-trip (2):
10. `=DEGREES(RADIANS(A8))` -> 45
11. `=RADIANS(DEGREES(1))` -> 1

Error propagation (2):
12. `=DEGREES(C2)` -> `#N/A`
13. `=RADIANS(C2)` -> `#N/A`

Type error (1):
14. `=DEGREES(D2)` -> `#VALUE!`

Coercion (2):
15. `=DEGREES(TRUE)` -> ~57.296
16. `=RADIANS("180")` -> ~3.14159

Nested (2):
17. `=SIN(RADIANS(90))` -> 1
18. `=IF(DEGREES(B2)=180, "half", "other")` -> "half"

Cross-sheet (1):
19. `=RADIANS(Sheet2!A2)` -> ~1.5708 (90 degrees)

Combined (1):
20. `=DEGREES(B2) + RADIANS(180)` -> 180 + PI -> ~183.14159

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn degrees_radians()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "DEGREES, RADIANS angle conversion functions" under `[Unreleased]` |
| `docs/functions.md` | Change DEGREES, RADIANS from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the DEGREES / RADIANS checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of a single argument — no cross-row reads, no prelude dependency.
