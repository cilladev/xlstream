# Phase 5 — Arithmetic, comparison, concatenation

**Goal:** numeric arithmetic, comparison, unary, and string concat (`&`) fully implemented.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phase 4.

**Reading:** [`docs/architecture/evaluator.md`](../architecture/evaluator.md).

**Output:** formulas like `Deal Value * Quantity`, `(Revenue - Cost) / Quantity`, `UPPER(LEFT(...)) & "-"` work (well — `UPPER` lands in Phase 9, the `&` part works now).

## Checklist

### Binary operators

In `xlstream-eval/src/builtins/arithmetic.rs`:

- [ ] `+` — numeric add; error propagation; text-that-parses coerces; bool coerces.
- [ ] `-` — subtract.
- [ ] `*` — multiply.
- [ ] `/` — divide; zero → `#DIV/0!`.
- [ ] `^` — power; negative-base fractional-exponent → `#NUM!` when appropriate.
- [ ] `&` — string concatenation; coerces both sides to text per Excel rules.
- [ ] Comparison: `=`, `<>`, `<`, `>`, `<=`, `>=`. Case-insensitive text comparison.

### Unary

- [ ] Unary `-`.
- [ ] Unary `+`.
- [ ] Unary `%` (percent — divides by 100).

### Precedence (in parser or at AST evaluation)

- [ ] Verify parser respects Excel precedence:
  1. `:` (range)
  2. `space` (intersection)
  3. `,` (union)
  4. `-` (negation — unary)
  5. `%` (percent)
  6. `^` (exponent)
  7. `*`, `/`
  8. `+`, `-`
  9. `&`
  10. `=`, `<`, `>`, `<=`, `>=`, `<>`
- [ ] **Test: `-2^2 = 4`** (unary minus before exponent). This is the famous Excel trap.

### Coercion

- [ ] `xlstream-core::coerce` module:
  - [ ] `to_number(&Value) -> Result<f64, CellError>` — respects Excel semantics.
  - [ ] `to_text(&Value) -> Cow<'_, str>`.
  - [ ] `to_bool(&Value) -> Result<bool, CellError>`.
- [ ] Each returns `CellError::Value` where Excel would.

### Error propagation

- [ ] `Value::Error(e) + _` → `Value::Error(e)`.
- [ ] `_ + Value::Error(e)` → `Value::Error(e)`.
- [ ] Comparison against error → error propagates.

### Tests — arithmetic

For each op: at least 5 tests.
- [ ] Happy numeric path.
- [ ] Mixed type (number + text-that-parses).
- [ ] Type mismatch (number + non-numeric text) → `#VALUE!`.
- [ ] Error propagation from left operand.
- [ ] Error propagation from right operand.
- [ ] Edge cases: `x/0`, `0^0`, `-(-1)`, `"" & "" = ""`.

### Tests — operator precedence

- [ ] `1 + 2 * 3 = 7`.
- [ ] `(1 + 2) * 3 = 9`.
- [ ] `-2^2 = 4`.
- [ ] `-(-1) = 1`.
- [ ] `"a" & "b" & "c" = "abc"`.
- [ ] `2 ^ 3 ^ 2` — right-associative? Check Excel. Update test.
- [ ] `10 + "5" = 15`.
- [ ] `10 & "5" = "105"`.

### Tests — comparison

- [ ] `"a" = "A"` → TRUE.
- [ ] `1 < "2"` → TRUE (coerces RHS).
- [ ] `"10" > "9"` → FALSE (text compare, lexicographic).
- [ ] `0.1 + 0.2 = 0.3` → TRUE (IEEE with Excel's 15-digit rounding). **Tests this explicitly** since it's a famous gotcha.

### Tests — IEEE rounding

- [ ] `0.1 + 0.2 = 0.3` returns TRUE.
- [ ] `1.0 / 3.0 * 3.0 = 1.0` returns TRUE.
- [ ] Implement Excel's behaviour: when comparing numbers, round to 15 significant digits first.

### Benchmarks

- [ ] `arithmetic_bench.rs`: throughput of `+`, `*`, `/` per 1M ops. Target: > 100M ops/sec single-threaded.

## Done when

Every operator has ≥ 5 passing tests. Precedence tests pass. The `-2^2 = 4` Excel-trap test passes. IEEE rounding for `0.1 + 0.2 = 0.3` passes.
