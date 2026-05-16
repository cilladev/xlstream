# Feature: GCD / LCM

**Branch:** `feat/gcd-lcm`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Greatest common divisor and least common multiple. Both are variadic (1-255 args), operating on non-negative integers. Arguments are truncated to integer before computation.

- `GCD(number1, [number2], ...)` — greatest common divisor.
- `LCM(number1, [number2], ...)` — least common multiple.

```
=GCD(12, 8)          -> 4
=GCD(24, 36, 48)     -> 12
=GCD(5, 0)           -> 5
=GCD(0, 0)           -> 0
=LCM(4, 6)           -> 12
=LCM(5, 10, 15)      -> 30
=LCM(1, 0)           -> 0
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch pattern
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/math.rs:36-42` — `finite_or_num` helper for overflow guard
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch section
- `crates/xlstream-eval/src/builtins/math.rs:396-404` — 1-arg pattern; GCD/LCM differ by being variadic
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper (used per arg in loop)
- `crates/xlstream-eval/src/builtins/mod.rs:158-186` — math dispatch

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion. Variadic: accept 1+ args.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**GCD(number1, [number2], ...):**
1. Arity check: at least 1 arg. Otherwise `#VALUE!`.
2. For each arg: extract as f64 via `num_arg`. Error -> propagate. Truncate to integer. If negative -> `#NUM!`.
3. Accumulate pairwise GCD using Euclidean algorithm: `gcd(a, b) = if b == 0 { a } else { gcd(b, a % b) }`.
4. Start with first arg, accumulate `result = gcd(result, next_arg)` for each subsequent arg.
5. Return `Value::Number(result as f64)`.

**LCM(number1, [number2], ...):**
1. Arity check: at least 1 arg. Otherwise `#VALUE!`.
2. For each arg: extract as f64 via `num_arg`. Error -> propagate. Truncate to integer. If negative -> `#NUM!`.
3. If any arg is 0, return 0 immediately.
4. Accumulate pairwise LCM: `lcm(a, b) = a / gcd(a, b) * b` (divide first to avoid overflow).
5. Start with first arg, accumulate `result = lcm(result, next_arg)` for each subsequent arg.
6. Guard final result with `finite_or_num` (large LCMs can overflow f64).
7. Return `Value::Number(result as f64)`.

**Implementation note:** Use `u64` for internal computation to avoid floating-point issues. Cast args from f64 to u64 after truncation and validation. Cast result back to f64 at the end. For LCM, check overflow before multiplication.

**Error conditions:**
- Wrong arity (0 args): `#VALUE!`
- Negative arg (after truncation): `#NUM!`
- Non-numeric: `#VALUE!` (via `coerce::to_number` failure)
- Error in any arg: propagate
- LCM overflow: `#NUM!` (via `finite_or_num` or explicit u64 overflow check)
- GCD(0, 0) = 0 (not an error)

## Tests

### Unit tests (in `math.rs`)

**GCD happy path:**
- `gcd(12, 8)` -> 4
- `gcd(24, 36)` -> 12
- `gcd(24, 36, 48)` -> 12
- `gcd(7, 13)` -> 1 (coprime)
- `gcd(100, 75, 50)` -> 25

**GCD single arg:**
- `gcd(12)` -> 12

**GCD with zero:**
- `gcd(5, 0)` -> 5
- `gcd(0, 5)` -> 5
- `gcd(0, 0)` -> 0

**GCD fractional truncation:**
- `gcd(12.9, 8.1)` -> 4 (truncated to gcd(12, 8))

**LCM happy path:**
- `lcm(4, 6)` -> 12
- `lcm(5, 10, 15)` -> 30
- `lcm(3, 7)` -> 21
- `lcm(12, 8)` -> 24

**LCM single arg:**
- `lcm(12)` -> 12

**LCM with zero:**
- `lcm(5, 0)` -> 0
- `lcm(0, 0)` -> 0

**LCM fractional truncation:**
- `lcm(4.9, 6.1)` -> 12 (truncated to lcm(4, 6))

**Domain errors (#NUM!):**
- `gcd(-1, 5)` -> `#NUM!` (negative)
- `lcm(-1, 5)` -> `#NUM!`
- `gcd(5, -1)` -> `#NUM!`

**Error propagation:**
- `gcd(#N/A, 5)` -> `#N/A`
- `lcm(5, #N/A)` -> `#N/A`

**Arity errors:**
- `gcd()` -> `#VALUE!`
- `lcm()` -> `#VALUE!`

**Coercion:**
- `gcd(TRUE, 1)` -> 1 (TRUE = 1)
- `lcm("4", 6)` -> 12

**Type mismatch:**
- `gcd("abc", 5)` -> `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/gcd_lcm.xlsx`.

**Sheet1 data:**
- A: "Num1" header, rows 2-9: `12, 24, 7, 5, 0, 100, 4, 3`
- B: "Num2" header, rows 2-9: `8, 36, 13, 0, 0, 75, 6, 7`
- C: "Num3" header, rows 2-4: `(empty), 48, (empty)`
- D: "Error" header, row 2: `=NA()`
- E: "Text" header, row 2: `"abc"`
- F: "Neg" header, row 2: `-1`

**Sheet2 data:**
- A: "Val1" header, row 2: `12`
- B: "Val2" header, row 2: `8`

**Formulas (column G, starting row 2) — 25 formulas:**

GCD happy path (5):
1. `=GCD(A2, B2)` -> 4 (gcd(12, 8))
2. `=GCD(A3, B3)` -> 12 (gcd(24, 36))
3. `=GCD(A3, B3, C3)` -> 12 (gcd(24, 36, 48))
4. `=GCD(A4, B4)` -> 1 (gcd(7, 13), coprime)
5. `=GCD(A7, B7, 50)` -> 25 (gcd(100, 75, 50))

GCD with zero (2):
6. `=GCD(A5, B5)` -> 5 (gcd(5, 0))
7. `=GCD(A6, B6)` -> 0 (gcd(0, 0))

GCD single arg (1):
8. `=GCD(A2)` -> 12

LCM happy path (4):
9. `=LCM(A8, B8)` -> 12 (lcm(4, 6))
10. `=LCM(A9, B9)` -> 21 (lcm(3, 7))
11. `=LCM(5, 10, 15)` -> 30
12. `=LCM(A2, B2)` -> 24 (lcm(12, 8))

LCM with zero (1):
13. `=LCM(A5, B5)` -> 0 (lcm(5, 0))

LCM single arg (1):
14. `=LCM(A2)` -> 12

Domain errors (2):
15. `=GCD(F2, 5)` -> `#NUM!` (negative)
16. `=LCM(F2, 5)` -> `#NUM!` (negative)

Error propagation (2):
17. `=GCD(D2, 5)` -> `#N/A`
18. `=LCM(5, D2)` -> `#N/A`

Type error (1):
19. `=GCD(E2, 5)` -> `#VALUE!`

Coercion (2):
20. `=GCD(TRUE, 1)` -> 1
21. `=LCM("4", 6)` -> 12

Nested (2):
22. `=IF(GCD(12, 8)=4, "yes", "no")` -> "yes"
23. `=IFERROR(GCD(-1, 5), "neg")` -> "neg"

Cross-sheet (1):
24. `=GCD(Sheet2!A2, Sheet2!B2)` -> 4

Combined (1):
25. `=GCD(12, 8) * LCM(12, 8) / 12` -> 4 * 24 / 12 = 8 (identity: gcd*lcm = a*b)

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn gcd_lcm()` in `conformance/math.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "GCD, LCM greatest common divisor / least common multiple" under `[Unreleased]` |
| `docs/functions.md` | Change GCD, LCM from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the GCD / LCM checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of their arguments — no cross-row reads, no prelude dependency. Variadic but all args are evaluated eagerly.
