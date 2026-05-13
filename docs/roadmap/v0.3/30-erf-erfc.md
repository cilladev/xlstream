# Feature: ERF / ERFC / ERF.PRECISE / ERFC.PRECISE

**Branch:** `feat/erf-erfc`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Error function and its complement — fundamental engineering/statistical functions.

- `ERF(lower_limit, [upper_limit])` — error function. 1 arg: erf(x). 2 args: erf(upper) - erf(lower).
- `ERFC(x)` — complementary error function: 1 - erf(x). Single arg only.
- `ERF.PRECISE(x)` — identical to ERF(x) with 1 arg. Compatibility alias.
- `ERFC.PRECISE(x)` — identical to ERFC(x). Compatibility alias.

```
=ERF(1)              → 0.842700793        (erf(1))
=ERF(0)              → 0
=ERF(0, 1)           → 0.842700793        (erf(1) - erf(0))
=ERF(1, 2)           → 0.152621472        (erf(2) - erf(1))
=ERFC(1)             → 0.157299207        (1 - erf(1))
=ERFC(0)             → 1
=ERF.PRECISE(1)      → 0.842700793        (same as ERF(1))
=ERFC.PRECISE(1)     → 0.157299207        (same as ERFC(1))
```

**Key identities:**
- `erf(0) = 0`
- `erf(inf) = 1`, `erf(-inf) = -1`
- `erfc(x) = 1 - erf(x)`
- `erfc(0) = 1`
- `erf(-x) = -erf(x)` (odd function)
- Two-arg ERF: `ERF(a, b) = erf(b) - erf(a)`. Both args can be negative.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. Specs 23-29 (base conversion, BASE, COMPLEX/IMREAL/IMAGINARY, DELTA/GESTEP) will have landed here, establishing the engineering dispatch pattern.
- `crates/xlstream-eval/src/builtins/specfn.rs:233-257` — `erf_approx` function (relocated from `statistical.rs` in this PR). `pub(super)`, shared by both `statistical.rs` and `engineering.rs`.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). Engineering dispatch arms already present from specs 23-29.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists ERF, ERFC, ERF.PRECISE, ERFC.PRECISE as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (base conversion + comparison functions from specs 23-29 already present)
- `crates/xlstream-eval/src/builtins/specfn.rs:233-257` — `erf_approx` (shared, relocated from statistical.rs)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:159-175` — math builtins dispatch pattern (pure, eager eval)
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

All four functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. All args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Same pattern as math builtins.

**Prerequisite refactor:** Done — `erf_approx` relocated to `specfn.rs` as `pub(super)`. Both `statistical.rs` and `engineering.rs` import it.

**ERF(lower_limit, [upper_limit]):**
1. Arity check: 1-2 args. Otherwise `#VALUE!`
2. If 1 arg: extract as f64 via `num_arg_ce`. Return `Value::Number(erf_approx(x))`.
3. If 2 args: extract both as f64. Return `Value::Number(erf_approx(upper) - erf_approx(lower))`.

**ERFC(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`
2. Extract arg 0 as f64 via `num_arg_ce`.
3. Return `Value::Number(1.0 - erf_approx(x))`.

**ERF.PRECISE(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`
2. Identical to ERF with 1 arg. Delegate directly.

**ERFC.PRECISE(x):**
1. Arity check: exactly 1 arg. Otherwise `#VALUE!`
2. Identical to ERFC. Delegate directly.

**Error conditions:**
- Wrong arity: `#VALUE!` (ERF requires 1-2; ERFC, ERF.PRECISE, ERFC.PRECISE require exactly 1)
- Non-numeric args: `#VALUE!` (via `coerce::to_number` failure)
- Error in any arg: propagate

**Edge cases:**
- `ERF(0)` → 0.0 exactly
- `ERFC(0)` → 1.0 exactly
- Large positive x (e.g. 10): `ERF(10)` → 1.0 (within floating-point precision)
- Large negative x: `ERF(-10)` → -1.0
- `ERFC(10)` → ~0.0 (very small but non-negative)
- `ERFC(-10)` → ~2.0
- Negative lower_limit in 2-arg ERF is valid: `ERF(-1, 1)` → `erf(1) - erf(-1)` = `2 * erf(1)`
- Same value: `ERF(1, 1)` → 0.0
- Reversed args: `ERF(2, 1)` → negative (erf(1) - erf(2) < 0) — valid, Excel allows this
- Boolean coercion: `ERF(TRUE)` → erf(1.0)
- Text coercion: `ERF("0.5")` → erf(0.5). `ERF("abc")` → `#VALUE!`

**Dispatch names:** ERF.PRECISE and ERFC.PRECISE need `_xlfn.` prefix handling in LibreOffice-saved files. The dispatch normalizer already strips `_XLFN.` prefix (line 95 of mod.rs), so match on `"ERF.PRECISE"` and `"ERFC.PRECISE"`.

## Tests

### Unit tests (in `engineering.rs`)

**ERF 1-arg happy path:**
- `erf(0.0)` → 0.0
- `erf(1.0)` → ~0.842700793 (within 1e-6)
- `erf(0.5)` → ~0.520499878 (within 1e-6)
- `erf(2.0)` → ~0.995322265 (within 1e-6)
- `erf(3.0)` → ~0.999977910 (within 1e-6)

**ERF 1-arg negative (odd function):**
- `erf(-1.0)` → ~-0.842700793
- `erf(-0.5)` → ~-0.520499878

**ERF 2-arg happy path:**
- `erf(0, 1)` → ~0.842700793
- `erf(1, 2)` → ~0.152621472
- `erf(0, 0)` → 0.0

**ERF 2-arg with negatives:**
- `erf(-1, 1)` → ~1.685401586 (2 * erf(1))
- `erf(-2, -1)` → ~0.152621472 (erf(-1) - erf(-2))
- `erf(2, 1)` → ~-0.152621472 (reversed — negative result)

**ERF edge cases:**
- `erf(10.0)` → 1.0 (saturated)
- `erf(-10.0)` → -1.0

**ERFC happy path:**
- `erfc(0.0)` → 1.0
- `erfc(1.0)` → ~0.157299207
- `erfc(2.0)` → ~0.004677735
- `erfc(-1.0)` → ~1.842700793

**ERFC edge cases:**
- `erfc(10.0)` → ~0.0 (very small, >= 0.0)
- `erfc(-10.0)` → ~2.0

**ERF.PRECISE:**
- `erf_precise(1.0)` → same as `erf(1.0)`
- `erf_precise(0.0)` → 0.0

**ERFC.PRECISE:**
- `erfc_precise(1.0)` → same as `erfc(1.0)`
- `erfc_precise(0.0)` → 1.0

**Error cases:**
- ERF with 0 args → `#VALUE!`
- ERF with 3 args → `#VALUE!`
- ERFC with 0 args → `#VALUE!`
- ERFC with 2 args → `#VALUE!`
- ERF.PRECISE with 2 args → `#VALUE!`
- ERFC.PRECISE with 0 args → `#VALUE!`
- Non-numeric text → `#VALUE!`
- Error propagation: `ERF(#N/A)` → `#N/A`

**Coercion:**
- `erf(TRUE)` → erf(1.0)
- `erf("0.5")` → erf(0.5)

### Conformance fixture

Create `tests/fixtures/engineering/erf_erfc.xlsx`.

**Sheet1 data:**
- A: "X" header, rows 2-10: `0, 0.5, 1, 1.5, 2, 3, -1, -0.5, 10`
- B: "Lower" header, rows 2-6: `0, 1, -1, -2, 2`
- C: "Upper" header, rows 2-6: `1, 2, 1, -1, 1`
- D: "Error" header, row 2: `=NA()`
- E: "Text" header, rows 2-3: `"abc"`, `"0.5"`

**Sheet2 data:**
- A: "Val" header, rows 2-3: `1, 0`

**Formulas (column F, starting row 2) — 30 formulas:**

ERF 1-arg happy path (6):
1. `=ERF(A2)` → 0 (erf(0))
2. `=ERF(A3)` → ~0.520500 (erf(0.5))
3. `=ERF(A4)` → ~0.842701 (erf(1))
4. `=ERF(A5)` → ~0.966105 (erf(1.5))
5. `=ERF(A6)` → ~0.995322 (erf(2))
6. `=ERF(A7)` → ~0.999978 (erf(3))

ERF 1-arg negative (2):
7. `=ERF(A8)` → ~-0.842701 (erf(-1))
8. `=ERF(A9)` → ~-0.520500 (erf(-0.5))

ERF 1-arg large (1):
9. `=ERF(A10)` → 1 (erf(10) saturates)

ERF 2-arg happy path (3):
10. `=ERF(B2, C2)` → ~0.842701 (erf(1) - erf(0))
11. `=ERF(B3, C3)` → ~0.152622 (erf(2) - erf(1))
12. `=ERF(B2, B2)` → 0 (erf(0) - erf(0))

ERF 2-arg with negatives (2):
13. `=ERF(B4, C4)` → ~1.685401 (erf(1) - erf(-1) = 2*erf(1))
14. `=ERF(B5, C5)` → ~0.152622 (erf(-1) - erf(-2))

ERF 2-arg reversed (1):
15. `=ERF(B6, C6)` → ~-0.152622 (erf(1) - erf(2), negative)

ERFC happy path (4):
16. `=ERFC(A2)` → 1 (1 - erf(0))
17. `=ERFC(A4)` → ~0.157299 (1 - erf(1))
18. `=ERFC(A6)` → ~0.004678 (1 - erf(2))
19. `=ERFC(A8)` → ~1.842701 (1 - erf(-1))

ERFC edge (1):
20. `=ERFC(A10)` → ~0 (1 - erf(10), nearly 0)

ERF.PRECISE (2):
21. `=ERF.PRECISE(A4)` → ~0.842701 (same as ERF(1))
22. `=ERF.PRECISE(A2)` → 0

ERFC.PRECISE (2):
23. `=ERFC.PRECISE(A4)` → ~0.157299 (same as ERFC(1))
24. `=ERFC.PRECISE(A2)` → 1

Error propagation (2):
25. `=ERF(D2)` → `#N/A`
26. `=ERFC(D2)` → `#N/A`

Non-numeric text (1):
27. `=ERF(E2)` → `#VALUE!` ("abc" not numeric)

Text coercion (1):
28. `=ERF(E3)` → ~0.520500 ("0.5" coerces)

Nested / combined (2):
29. `=ERF(A4) + ERFC(A4)` → 1 (identity: erf(x) + erfc(x) = 1)
30. `=IF(ERFC(A4)<0.5, "tail", "body")` → "tail"

Cross-sheet (1):
31. `=ERF(Sheet2!A2)` → ~0.842701 (erf(1))

ERF.PRECISE and ERFC.PRECISE need `_xlfn.` prefix in LibreOffice-saved files. The dispatch normalizer handles this.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn erf_erfc()` in `conformance/engineering.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "ERF, ERFC, ERF.PRECISE, ERFC.PRECISE error function" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change ERF, ERFC, ERF.PRECISE, ERFC.PRECISE from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the ERF / ERFC checkbox |

## Streaming invariant

Does not violate. All four functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency. The `erf_approx` relocation is a refactor of existing code with no behavioral change.
