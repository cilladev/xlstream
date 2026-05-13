# Feature: FACT / FACTDOUBLE

**Branch:** `feat/fact`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Factorial functions.

- `FACT(number)` — factorial: `n!` = `n * (n-1) * ... * 1`
- `FACTDOUBLE(number)` — double factorial: `n!!` = `n * (n-2) * (n-4) * ... * 1` (or 2)

```
=FACT(5)         → 120       (5*4*3*2*1)
=FACT(0)         → 1
=FACT(170)       → 7.257e306 (largest before f64 overflow)
=FACTDOUBLE(5)   → 15        (5*3*1)
=FACTDOUBLE(6)   → 48        (6*4*2)
=FACTDOUBLE(0)   → 1
=FACTDOUBLE(-1)  → 1
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper
- `crates/xlstream-eval/src/builtins/mod.rs` — dispatch
- Not in `UNSUPPORTED_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:55-70` — reference 1-arg pattern (builtin_int, builtin_abs)
- `crates/xlstream-eval/src/builtins/mod.rs:158-181` — math dispatch

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**Implementation approach:**

FACT: loop from 1 to n, multiply. Use f64 accumulator. For n > 170, the result overflows f64 → return `#NUM!`. Argument truncated to integer.

FACTDOUBLE: loop from n down to 1 (step -2), multiply. Use f64 accumulator. Argument truncated to integer. Excel defines `FACTDOUBLE(-1) = 1` and `FACTDOUBLE(0) = 1`.

Both should check for `finite_or_num` on the result to catch overflow.

**Value handling:** Uses `num_arg` for coercion.

**Error conditions:**
- FACT: n < 0 → `#NUM!`. n > 170 → `#NUM!` (overflow).
- FACTDOUBLE: n < -1 → `#NUM!`. Large n → `#NUM!` (overflow).
- Non-numeric → `#VALUE!`
- Wrong arity (not exactly 1 arg) → `#VALUE!`

## Tests

### Unit tests (in `math.rs`)

**FACT happy path:**
- `fact(0)` → 1
- `fact(1)` → 1
- `fact(5)` → 120
- `fact(10)` → 3628800
- `fact(20)` → 2432902008176640000
- `fact(170)` → ~7.257e306 (largest representable)

**FACT edge cases:**
- Negative: `fact(-1)` → `#NUM!`
- Overflow: `fact(171)` → `#NUM!`
- Fractional truncated: `fact(5.9)` → 120 (same as fact(5))
- `fact(0)` → 1

**FACTDOUBLE happy path:**
- `factdouble(0)` → 1
- `factdouble(1)` → 1
- `factdouble(5)` → 15 (5*3*1)
- `factdouble(6)` → 48 (6*4*2)
- `factdouble(7)` → 105 (7*5*3*1)
- `factdouble(10)` → 3840 (10*8*6*4*2)

**FACTDOUBLE edge cases:**
- `factdouble(-1)` → 1 (Excel convention)
- Negative < -1: `factdouble(-2)` → `#NUM!`
- Fractional truncated: `factdouble(5.9)` → 15

**Type handling:**
- Error propagation: `fact(#N/A)` → `#N/A`
- Wrong args: `fact()` → `#VALUE!`
- `fact(1, 2)` → `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/fact.xlsx`.

**Sheet1 data:**
- A: "n" rows 2-10: `0, 1, 5, 10, 20, 170, -1, 6, 7`

**Formulas (column B) — 20 formulas:**

FACT (8):
1. `=FACT(A2)` → 1
2. `=FACT(A3)` → 1
3. `=FACT(A4)` → 120
4. `=FACT(A5)` → 3628800
5. `=FACT(A6)` → 2432902008176640000
6. `=FACT(A7)` → ~7.257e306
7. `=FACT(A8)` → `#NUM!` (n=-1)
8. `=FACT(171)` → `#NUM!`

FACTDOUBLE (7):
9. `=FACTDOUBLE(A2)` → 1
10. `=FACTDOUBLE(A3)` → 1
11. `=FACTDOUBLE(A4)` → 15
12. `=FACTDOUBLE(A9)` → 48
13. `=FACTDOUBLE(A10)` → 105
14. `=FACTDOUBLE(-1)` → 1
15. `=FACTDOUBLE(-2)` → `#NUM!`

Nested (2):
16. `=IF(FACT(5)>100, "big", "small")` → "big"
17. `=IFERROR(FACT(-1), "n/a")` → "n/a"

Cross-sheet (1):
18. `=FACT(Sheet2!A2)` (add Sheet2 with data)

Combined (2):
19. `=FACT(5) / (FACT(3) * FACT(2))` → 10 (= COMBIN(5,3))
20. `=FACTDOUBLE(5) * FACTDOUBLE(4)` → 15 * 8 = 120 (= FACT(5))

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "FACT, FACTDOUBLE factorial functions" under `[Unreleased]` |
| `docs/functions.md` | Change FACT, FACTDOUBLE from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the FACT / FACTDOUBLE checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of a single argument.
