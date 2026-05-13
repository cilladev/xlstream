# Feature: COMBIN / COMBINA

**Branch:** `feat/combin`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Combination functions.

- `COMBIN(number, number_chosen)` — combinations without repetition: `n! / (k! * (n-k)!)`
- `COMBINA(number, number_chosen)` — combinations with repetition: `(n+k-1)! / (k! * (n-1)!)`

```
=COMBIN(5, 3)   → 10
=COMBIN(10, 2)  → 45
=COMBINA(5, 3)  → 35     (= COMBIN(5+3-1, 3) = COMBIN(7, 3))
=COMBINA(4, 2)  → 10     (= COMBIN(5, 2))
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — scalar math builtins with `eval_args` dispatch
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper
- `crates/xlstream-eval/src/builtins/mod.rs` — dispatch
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home
- `crates/xlstream-eval/src/builtins/math.rs:55-70` — reference 2-arg pattern
- `crates/xlstream-eval/src/builtins/mod.rs:158-181` — math dispatch

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion.

**Classification:** RowLocal.

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch.

**Implementation approach:**

COMBIN: compute `n! / (k! * (n-k)!)` without computing full factorials. Use the multiplicative formula: `product(n-k+1..=n) / product(1..=k)`. Accumulate with running division to avoid intermediate overflow.

COMBINA: `COMBINA(n, k)` = `COMBIN(n+k-1, k)`. Delegate to COMBIN after adjusting n.

Both arguments are truncated to integer.

**Value handling:** Uses `num_arg` for coercion.

**Error conditions:**
- n < 0 or k < 0 → `#NUM!`
- COMBIN: k > n → `#NUM!`
- COMBINA: n = 0 and k = 0 → 1
- Non-numeric → `#VALUE!`
- Wrong arity → `#VALUE!`
- Overflow → `#NUM!` (via `finite_or_num`)

## Tests

### Unit tests (in `math.rs`)

**COMBIN happy path:**
- `combin(5, 3)` → 10
- `combin(10, 2)` → 45
- `combin(5, 0)` → 1
- `combin(5, 5)` → 1
- `combin(10, 1)` → 10
- `combin(20, 10)` → 184756

**COMBIN edge cases:**
- k > n: `combin(3, 5)` → `#NUM!`
- n = 0, k = 0: `combin(0, 0)` → 1
- Negative: `combin(-1, 1)` → `#NUM!`
- Fractional truncated: `combin(5.9, 3.1)` → `combin(5, 3)` = 10
- Large: `combin(100, 50)` → verify against Excel

**COMBINA happy path:**
- `combina(5, 3)` → 35
- `combina(4, 2)` → 10
- `combina(1, 5)` → 1 (= COMBIN(5, 5))
- `combina(5, 0)` → 1

**COMBINA edge cases:**
- `combina(0, 0)` → 1
- `combina(0, 1)` → 0 (= COMBIN(-1+1, 1) = COMBIN(0, 1) = 0)
- Negative: `combina(-1, 1)` → `#NUM!`

**Type handling:**
- Error propagation: `combin(#N/A, 3)` → `#N/A`
- Wrong args: `combin(5)` → `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/combin.xlsx`.

**Sheet1 data:**
- A: "n" rows 2-8: `5, 10, 0, 20, 100, 5, 4`
- B: "k" rows 2-8: `3, 2, 0, 10, 50, 5, 2`

**Formulas (column C) — 18 formulas:**

COMBIN (7):
1. `=COMBIN(A2, B2)` → 10
2. `=COMBIN(A3, B3)` → 45
3. `=COMBIN(A4, B4)` → 1
4. `=COMBIN(A5, B5)` → 184756
5. `=COMBIN(A6, B6)` → large value (verify in Excel)
6. `=COMBIN(A7, B7)` → 1
7. `=COMBIN(3, 5)` → `#NUM!`

COMBINA (5):
8. `=COMBINA(A2, B2)` → 35
9. `=COMBINA(A8, B8)` → 10
10. `=COMBINA(1, 5)` → 1
11. `=COMBINA(5, 0)` → 1
12. `=COMBINA(0, 0)` → 1

Nested (2):
13. `=IF(COMBIN(10, 5)>200, "many", "few")` → "many"
14. `=IFERROR(COMBIN(3, 5), "invalid")` → "invalid"

Cross-sheet (1):
15. `=COMBIN(Sheet2!A2, Sheet2!B2)`

Combined (3):
16. `=COMBIN(5, 3) + COMBIN(5, 2)` → 10 + 10 = 20
17. `=PERMUT(5, 3) / COMBIN(5, 3)` → 60 / 10 = 6 (= 3!)
18. `=COMBINA(5, 3) - COMBIN(7, 3)` → 35 - 35 = 0

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "COMBIN, COMBINA combination functions" under `[Unreleased]` |
| `docs/functions.md` | Change COMBIN, COMBINA from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the COMBIN / COMBINA checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of their arguments.
