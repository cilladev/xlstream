# Phase 6 — Conditional, logical

**Goal:** `IF`, `IFS`, `SWITCH`, `IFERROR`, `IFNA`, `AND`, `OR`, `NOT`, `XOR`, `TRUE`, `FALSE` all work with short-circuit evaluation.

**Estimated effort:** 3 days.

**Prerequisites:** Phase 5.

**Output:** `=IF(Revenue > 10000, "Big", "Small")`, `=IFERROR(VLOOKUP(...), "Not found")` style formulas work.

## Checklist

### IF

In `xlstream-eval/src/builtins/conditional.rs` as **stateful** builtins (they take `&[NodeRef]`, not pre-evaluated values — for short-circuit):

- [x] `IF(cond, then, else?)`:
  - [x] Evaluate `cond`, coerce to bool.
  - [x] If true, evaluate `then` and return.
  - [x] If false, evaluate `else` or return `FALSE`.
  - [x] Short-circuit: never evaluate the unused branch.
- [x] Test: `IF(A=0, 0, 1/A)` — when `A=0`, does NOT produce `#DIV/0!`.

### IFS

- [x] `IFS(cond1, value1, cond2, value2, ...)`:
  - [x] Evaluate cond1; if true return value1.
  - [x] Otherwise try cond2; if true return value2. Etc.
  - [x] If no condition matches → `#N/A`.
  - [x] Short-circuit: only evaluate conds up to the first match; only evaluate the matching value.

### SWITCH

- [x] `SWITCH(expression, val1, result1, val2, result2, ..., default?)`:
  - [x] Evaluate `expression` once.
  - [x] Compare against val1, val2, ... in order; return first matching `resultN`.
  - [x] No match → `default` if given, else `#N/A`.

### IFERROR, IFNA

- [x] `IFERROR(expr, fallback)`:
  - [x] Evaluate `expr`; if result is `Value::Error(_)` or the evaluation returned `Err(CellError)`, evaluate and return `fallback`.
  - [x] Otherwise return the value.
- [x] `IFNA(expr, fallback)`:
  - [x] Same but only for `CellError::Na`. Other errors propagate.

### AND, OR, NOT, XOR

- [x] `AND(a1, a2, ..., aN)`:
  - [x] Short-circuit on first FALSE.
  - [x] Empty args → `#VALUE!`.
  - [x] Propagate errors.
- [x] `OR(...)`:
  - [x] Short-circuit on first TRUE.
- [x] `NOT(x)`:
  - [x] Coerce to bool, invert.
- [x] `XOR(...)`:
  - [x] Parity: true iff odd number of truthy args.

### TRUE, FALSE

- [x] `TRUE()` and `FALSE()` zero-arg builtins return `Value::Bool(true)` / `Value::Bool(false)`.
- [x] Parser should already handle the `TRUE` / `FALSE` literal tokens; confirm.

### Tests

Each function: at least 5 unit tests + 1 integration test.

#### IF

- [x] Happy path true branch.
- [x] Happy path false branch.
- [x] 2-arg form (no else) with false → `FALSE`.
- [x] Short-circuit: `IF(A=0, 0, 1/A)` when `A=0` does NOT error.
- [x] Cond is an error → propagate.
- [x] Cond is a number: 0 → false, nonzero → true.
- [x] Cond is a string: `"TRUE"` → true, `"FALSE"` → false, other → `#VALUE!`.

#### IFS, SWITCH similar coverage.

#### IFERROR

- [x] Catches `#DIV/0!`.
- [x] Catches `#VALUE!`.
- [x] Pass-through for non-error.
- [x] Fallback receives the correct value.
- [x] IFNA only catches `#N/A`, not `#VALUE!`.

#### AND / OR / NOT

- [x] `AND(TRUE, TRUE) = TRUE`.
- [x] `AND(TRUE, FALSE) = FALSE`.
- [x] `OR(FALSE, TRUE) = TRUE`.
- [x] `NOT(TRUE) = FALSE`.
- [x] `AND(1, 1, 1) = TRUE` (numeric coercion).
- [x] `AND("TRUE", "true") = TRUE` (case-insensitive).
- [x] Short-circuit: second arg with side effect not evaluated when first is false (verified via `AND(FALSE, 1/0)` returning FALSE without `#DIV/0!`).

## Integration tests

- [x] Fixture with `IFS(Deal Value > 100000, "Platinum", Deal Value > 50000, "Gold", Deal Value > 10000, "Silver", TRUE, "Bronze")` — matches xlformula's benchmark.
- [ ] Fixture with `IFERROR(VLOOKUP(...), "N/A")` — needs a lookup sheet; depends on Phase 8 too. Defer this one until Phase 8 lands.

## Done when

All conditionals work. Short-circuit behaviour verified. IEEE rounding from Phase 5 still holds when used in comparisons inside `IF`. `IF(A=0, 0, 1/A)` test passes (does NOT produce `#DIV/0!`).
