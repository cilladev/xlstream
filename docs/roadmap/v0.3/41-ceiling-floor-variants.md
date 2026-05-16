# Feature: CEILING.MATH / FLOOR.MATH / CEILING.PRECISE / FLOOR.PRECISE / ISO.CEILING

**Branch:** `feat/ceiling-floor-variants`
**Effort:** ~3 hours
**Crates:** xlstream-eval

## What

Extended rounding-to-multiple functions. These are newer variants of CEILING/FLOOR with different sign-handling semantics. Excel ships several variants because the original CEILING/FLOOR have inconsistent sign rules across Excel versions.

- `CEILING.MATH(number, [significance], [mode])` — rounds up (away from zero or toward positive infinity depending on mode). 1-3 args.
- `FLOOR.MATH(number, [significance], [mode])` — rounds down (toward zero or toward negative infinity depending on mode). 1-3 args.
- `CEILING.PRECISE(number, [significance])` — always rounds toward positive infinity. Significance sign is ignored. 1-2 args.
- `FLOOR.PRECISE(number, [significance])` — always rounds toward negative infinity. Significance sign is ignored. 1-2 args.
- `ISO.CEILING(number, [significance])` — identical to CEILING.PRECISE. Alias.

```
=CEILING.MATH(6.3)           -> 7
=CEILING.MATH(-6.7, 2)       -> -6
=CEILING.MATH(-6.7, 2, 1)   -> -8    (mode=1: round away from zero)
=FLOOR.MATH(6.7)             -> 6
=FLOOR.MATH(-6.7, 2)        -> -8
=FLOOR.MATH(-6.7, 2, 1)     -> -6    (mode=1: round toward zero)
=CEILING.PRECISE(4.1, 2)     -> 6
=CEILING.PRECISE(-4.1, 2)    -> -4   (toward positive infinity)
=FLOOR.PRECISE(4.1, 2)       -> 4
=FLOOR.PRECISE(-4.1, 2)      -> -6   (toward negative infinity)
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:256-297` — `builtin_ceiling`, `builtin_floor` — the "compatibility" versions (exact base pattern to extend)
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists all five as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:256-297` — existing CEILING/FLOOR (base pattern, sign-handling to contrast with)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

All five are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**Note on dispatch names:** These functions have dots in their names. In xlsx internal storage they are prefixed with `_xlfn.` (e.g., `_xlfn.CEILING.MATH`). The dispatch in `mod.rs` should match both the bare name and the `_xlfn.` prefixed variant. The parser already strips `_xlfn.` prefixes, so dispatch on the bare dotted name: `"CEILING.MATH"`, `"FLOOR.MATH"`, etc.

**CEILING.MATH(number, [significance], [mode]):**
1. Arity check: 1-3 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate.
3. Extract significance as f64 (default: 1 if number >= 0, -1 if number < 0). Error -> propagate.
4. If significance == 0 -> return 0.
5. Extract mode as f64 (default: 0). Error -> propagate.
6. Significance sign is treated as absolute for rounding direction. The actual rounding:
   - If mode == 0 (or number >= 0): round toward positive infinity: `ceil(number / abs(sig)) * abs(sig)`.
   - If mode != 0 AND number < 0: round toward zero (away from negative infinity is default; mode flips to toward zero): `-floor(abs(number) / abs(sig)) * abs(sig)`.
7. Return `Value::Number(result)`.

**FLOOR.MATH(number, [significance], [mode]):**
1. Arity check: 1-3 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate.
3. Extract significance as f64 (default: 1 if number >= 0, -1 if number < 0). Error -> propagate.
4. If significance == 0 -> return 0.
5. Extract mode as f64 (default: 0). Error -> propagate.
6. Rounding:
   - If mode == 0 (or number >= 0): round toward negative infinity: `floor(number / abs(sig)) * abs(sig)`.
   - If mode != 0 AND number < 0: round toward zero: `-ceil(abs(number) / abs(sig)) * abs(sig)`. Wait — FLOOR.MATH with mode rounds negative numbers toward zero, which means less negative.
7. Return `Value::Number(result)`.

**CEILING.PRECISE(number, [significance]):**
1. Arity check: 1-2 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate.
3. Extract significance as f64 (default: 1). Error -> propagate.
4. If significance == 0 -> return 0.
5. Take `abs(significance)` — sign of significance is ignored.
6. Compute `ceil(number / abs_sig) * abs_sig`. Always rounds toward positive infinity.
7. Return `Value::Number(result)`.

**FLOOR.PRECISE(number, [significance]):**
1. Arity check: 1-2 args. Otherwise `#VALUE!`.
2. Extract number as f64 via `num_arg`. Error -> propagate.
3. Extract significance as f64 (default: 1). Error -> propagate.
4. If significance == 0 -> return 0.
5. Take `abs(significance)` — sign of significance is ignored.
6. Compute `floor(number / abs_sig) * abs_sig`. Always rounds toward negative infinity.
7. Return `Value::Number(result)`.

**ISO.CEILING(number, [significance]):**
1. Alias for CEILING.PRECISE. Dispatch to same implementation.

**Error conditions:**
- Wrong arity: `#VALUE!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- Error in arg: propagate
- significance == 0: returns 0 (not an error for these variants)
- No `#NUM!` for sign mismatch (unlike compatibility CEILING/FLOOR) — these variants handle sign gracefully

## Tests

### Unit tests (in `math.rs`)

**CEILING.MATH happy path:**
- `ceiling_math(6.3)` -> 7.0
- `ceiling_math(6.3, 2)` -> 8.0
- `ceiling_math(-6.3)` -> -6.0 (toward positive infinity)
- `ceiling_math(-6.7, 2)` -> -6.0

**CEILING.MATH with mode:**
- `ceiling_math(-6.7, 2, 0)` -> -6.0 (toward positive infinity)
- `ceiling_math(-6.7, 2, 1)` -> -8.0 (away from zero)

**CEILING.MATH edge:**
- `ceiling_math(0)` -> 0.0
- `ceiling_math(5, 0)` -> 0.0

**FLOOR.MATH happy path:**
- `floor_math(6.7)` -> 6.0
- `floor_math(6.7, 2)` -> 6.0
- `floor_math(-6.7)` -> -7.0 (toward negative infinity)
- `floor_math(-6.7, 2)` -> -8.0

**FLOOR.MATH with mode:**
- `floor_math(-6.7, 2, 0)` -> -8.0 (toward negative infinity)
- `floor_math(-6.7, 2, 1)` -> -6.0 (toward zero)

**FLOOR.MATH edge:**
- `floor_math(0)` -> 0.0
- `floor_math(5, 0)` -> 0.0

**CEILING.PRECISE happy path:**
- `ceiling_precise(4.1, 2)` -> 6.0
- `ceiling_precise(-4.1, 2)` -> -4.0 (toward positive infinity)
- `ceiling_precise(4.1)` -> 5.0 (default sig=1)
- `ceiling_precise(-4.1, -2)` -> -4.0 (sign of significance ignored)

**FLOOR.PRECISE happy path:**
- `floor_precise(4.1, 2)` -> 4.0
- `floor_precise(-4.1, 2)` -> -6.0 (toward negative infinity)
- `floor_precise(4.1)` -> 4.0 (default sig=1)
- `floor_precise(-4.1, -2)` -> -6.0 (sign of significance ignored)

**ISO.CEILING = CEILING.PRECISE:**
- `iso_ceiling(4.1, 2)` -> 6.0
- `iso_ceiling(-4.1, 2)` -> -4.0

**Error propagation:**
- `ceiling_math(#N/A)` -> `#N/A`
- `floor_precise(#N/A, 2)` -> `#N/A`

**Arity errors:**
- `ceiling_math()` -> `#VALUE!`
- `ceiling_math(1, 2, 3, 4)` -> `#VALUE!`
- `ceiling_precise()` -> `#VALUE!`
- `ceiling_precise(1, 2, 3)` -> `#VALUE!`

**Coercion:**
- `ceiling_math(TRUE)` -> 1.0
- `floor_precise("4.1", 2)` -> 4.0

**Type mismatch:**
- `ceiling_math("abc")` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/ceiling_floor_variants.xlsx`.

**Sheet1 data:**
- A: "Number" header, rows 2-10: `6.3, 6.7, -6.3, -6.7, 4.1, -4.1, 0, 5, -8.5`
- B: "Sig" header, rows 2-6: `2, 2, 2, 2, 3`
- C: "Mode" header, rows 2-3: `0, 1`
- D: "Error" header, row 2: `=NA()`
- E: "Text" header, row 2: `"abc"`

**Sheet2 data:**
- A: "Val" header, row 2: `6.3`

**Formulas (column F, starting row 2) — 30 formulas:**

CEILING.MATH (6):
1. `=_xlfn.CEILING.MATH(A2)` -> 7 (6.3, default sig=1)
2. `=_xlfn.CEILING.MATH(A2, B2)` -> 8 (6.3, sig=2)
3. `=_xlfn.CEILING.MATH(A4, B3)` -> -6 (-6.3, sig=2, toward +inf)
4. `=_xlfn.CEILING.MATH(A5, B4, C2)` -> -6 (-6.7, sig=2, mode=0)
5. `=_xlfn.CEILING.MATH(A5, B4, C3)` -> -8 (-6.7, sig=2, mode=1, away from zero)
6. `=_xlfn.CEILING.MATH(A8)` -> 0 (0)

FLOOR.MATH (6):
7. `=_xlfn.FLOOR.MATH(A3)` -> 6 (6.7, default sig=1)
8. `=_xlfn.FLOOR.MATH(A3, B2)` -> 6 (6.7, sig=2)
9. `=_xlfn.FLOOR.MATH(A5, B4)` -> -8 (-6.7, sig=2, toward -inf)
10. `=_xlfn.FLOOR.MATH(A5, B4, C2)` -> -8 (-6.7, sig=2, mode=0)
11. `=_xlfn.FLOOR.MATH(A5, B4, C3)` -> -6 (-6.7, sig=2, mode=1, toward zero)
12. `=_xlfn.FLOOR.MATH(A8)` -> 0 (0)

CEILING.PRECISE (4):
13. `=_xlfn.CEILING.PRECISE(A6, B5)` -> 6 (4.1, sig=2)
14. `=_xlfn.CEILING.PRECISE(A7, B5)` -> -4 (-4.1, sig=2, toward +inf)
15. `=_xlfn.CEILING.PRECISE(A6)` -> 5 (4.1, default sig=1)
16. `=_xlfn.CEILING.PRECISE(A7, -2)` -> -4 (sign of sig ignored)

FLOOR.PRECISE (4):
17. `=_xlfn.FLOOR.PRECISE(A6, B5)` -> 4 (4.1, sig=2)
18. `=_xlfn.FLOOR.PRECISE(A7, B5)` -> -6 (-4.1, sig=2, toward -inf)
19. `=_xlfn.FLOOR.PRECISE(A6)` -> 4 (4.1, default sig=1)
20. `=_xlfn.FLOOR.PRECISE(A7, -2)` -> -6 (sign of sig ignored)

ISO.CEILING = CEILING.PRECISE (2):
21. `=_xlfn.ISO.CEILING(A6, B5)` -> 6
22. `=_xlfn.ISO.CEILING(A7, B5)` -> -4

sig=0 (1):
23. `=_xlfn.CEILING.MATH(A9, 0)` -> 0

Error propagation (2):
24. `=_xlfn.CEILING.MATH(D2)` -> `#N/A`
25. `=_xlfn.FLOOR.PRECISE(D2, 2)` -> `#N/A`

Type error (1):
26. `=_xlfn.CEILING.MATH(E2)` -> `#VALUE!`

Coercion (1):
27. `=_xlfn.CEILING.MATH(TRUE)` -> 1

Nested (1):
28. `=IF(_xlfn.CEILING.MATH(6.3)=7, "yes", "no")` -> "yes"

Cross-sheet (1):
29. `=_xlfn.CEILING.MATH(Sheet2!A2)` -> 7

Combined (1):
30. `=_xlfn.CEILING.PRECISE(4.1, 2) - _xlfn.FLOOR.PRECISE(4.1, 2)` -> 6 - 4 = 2

Note: These functions require the `_xlfn.` prefix in xlsx files. The parser strips this prefix before dispatch.

**Fixture workflow:**
1. Generate with openpyxl (use `_xlfn.` prefix in formula strings)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn ceiling_floor_variants()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "CEILING.MATH, FLOOR.MATH, CEILING.PRECISE, FLOOR.PRECISE, ISO.CEILING rounding variants" under `[Unreleased]` |
| `docs/functions.md` | Change CEILING.MATH, FLOOR.MATH, CEILING.PRECISE, FLOOR.PRECISE, ISO.CEILING from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the CEILING.MATH / FLOOR.MATH / CEILING.PRECISE / FLOOR.PRECISE / ISO.CEILING checkbox |

## Streaming invariant

Does not violate. All five are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
