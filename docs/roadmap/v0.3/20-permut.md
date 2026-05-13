# Feature: PERMUT / PERMUTA

**Branch:** `feat/permut`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Permutation functions.

- `PERMUT(number, number_chosen)` — number of permutations of n objects taken k at a time: `n! / (n-k)!`
- `PERMUTA(number, number_chosen)` — permutations with repetition: `n^k`

Note: `PERMUTATIONA` is the full Excel name; the roadmap uses `PERMUTA` as shorthand.

```
=PERMUT(5, 3)   → 60     (5*4*3)
=PERMUT(10, 2)  → 90     (10*9)
=PERMUTA(5, 3)  → 125    (5^3)
=PERMUTA(3, 3)  → 27     (3^3)
```

Current behavior: no dispatch entry — returns `#VALUE!`.

## What already exists

- `crates/xlstream-eval/src/builtins/math.rs` — pure scalar math builtins (ROUND, SQRT, POWER, etc.) with `eval_args` dispatch pattern. PERMUT/PERMUTA fit this pattern.
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with `eval_args` for scalar builtins
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists PERMUT as `.` and PERMUTATIONA as `.` for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/math.rs` — implementation home (scalar math functions)
- `crates/xlstream-eval/src/builtins/math.rs:55-70` — `builtin_round` — reference pattern for 2-arg scalar math
- `crates/xlstream-eval/src/builtins/math.rs:10-30` — `num_arg` helper for per-argument coercion
- `crates/xlstream-eval/src/builtins/mod.rs:158-181` — dispatch for math builtins

## Resolution / Evaluation behavior

Both are pure scalar functions — row-local, no range expansion needed.

**Classification:** RowLocal (default).

**Prelude:** Nothing needed.

**Row eval:** `eval_args` dispatch — evaluate all arguments eagerly, pass `&[Value]` to the builtin.

**Implementation approach:**

PERMUT: compute `n! / (n-k)!` = `n * (n-1) * ... * (n-k+1)` (k multiplications, no factorial needed). Both n and k must be non-negative integers, k <= n.

PERMUTA: compute `n^k` = `n.powf(k)`. Both n and k must be non-negative integers.

Both arguments are truncated to integer (Excel truncates, doesn't round).

**Value handling:** Uses `coerce::to_number` via `num_arg` for each argument.

**Error conditions:**
- n < 0 or k < 0 → `#NUM!`
- PERMUT: k > n → `#NUM!`
- Non-numeric argument → `#VALUE!`
- Wrong arity → `#VALUE!`
- Result overflow → `#NUM!` (via `finite_or_num`)

## Tests

### Unit tests (in `math.rs`)

**PERMUT happy path:**
- `permut(5, 3)` → 60
- `permut(10, 2)` → 90
- `permut(5, 0)` → 1 (0 chosen = 1 way)
- `permut(5, 5)` → 120 (= 5!)
- `permut(1, 1)` → 1

**PERMUT edge cases:**
- k > n: `permut(3, 5)` → `#NUM!`
- n = 0, k = 0: `permut(0, 0)` → 1
- Negative n: `permut(-1, 1)` → `#NUM!`
- Negative k: `permut(5, -1)` → `#NUM!`
- Fractional truncated: `permut(5.9, 3.1)` → same as `permut(5, 3)` = 60
- Large n: `permut(170, 1)` → 170

**PERMUTA happy path:**
- `permuta(5, 3)` → 125
- `permuta(3, 3)` → 27
- `permuta(10, 0)` → 1 (any^0 = 1)
- `permuta(1, 10)` → 1 (1^any = 1)

**PERMUTA edge cases:**
- `permuta(0, 0)` → 1 (0^0 = 1 in Excel)
- Negative n: → `#NUM!`
- Overflow: `permuta(1000, 1000)` → `#NUM!`

**Type handling / error propagation:**
- Error propagation: `permut(#N/A, 3)` → `#N/A`
- Wrong arg count: `permut(5)` → `#VALUE!`

### Conformance fixture

Create `tests/fixtures/math/permut.xlsx`.

**Sheet1 data:**
- A: "n" rows 2-8: `5, 10, 0, 1, 3, 5, 170`
- B: "k" rows 2-8: `3, 2, 0, 1, 5, 0, 1`

**Formulas (column C) — 18 formulas:**

PERMUT (8):
1. `=PERMUT(A2, B2)` → 60
2. `=PERMUT(A3, B3)` → 90
3. `=PERMUT(A4, B4)` → 1
4. `=PERMUT(A5, B5)` → 1
5. `=PERMUT(5, 5)` → 120
6. `=PERMUT(A7, B7)` → 1
7. `=PERMUT(A8, B8)` → 170
8. `=PERMUT(3, 5)` → `#NUM!`

PERMUTATIONA (5):
9. `=PERMUTATIONA(5, 3)` → 125
10. `=PERMUTATIONA(3, 3)` → 27
11. `=PERMUTATIONA(10, 0)` → 1
12. `=PERMUTATIONA(1, 10)` → 1
13. `=PERMUTATIONA(0, 0)` → 1

Nested (2):
14. `=IF(PERMUT(5, 3)>50, "many", "few")` → "many"
15. `=IFERROR(PERMUT(3, 5), "invalid")` → "invalid"

Cross-sheet (1):
16. `=PERMUT(Sheet2!A2, Sheet2!B2)` (add Sheet2 with data)

Combined (2):
17. `=PERMUT(5, 3) / PERMUT(5, 5)` → 0.5 (= 1/C(5,3)... actually 60/120 = 0.5)
18. `=PERMUTATIONA(2, 3) + PERMUT(4, 2)` → 8 + 12 = 20

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "PERMUT, PERMUTATIONA permutation functions" under `[Unreleased]` |
| `docs/functions.md` | Change PERMUT, PERMUTATIONA from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the PERMUT / PERMUTA checkbox |

## Streaming invariant

Does not violate. Both are pure scalar functions of their arguments.
