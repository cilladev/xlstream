# Phase 5 — Arithmetic, comparison, concatenation

**Goal:** numeric arithmetic, comparison, unary, and string concat (`&`) fully implemented.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phase 4.

**Reading:** [`docs/architecture/evaluator.md`](../architecture/evaluator.md).

**Output:** formulas like `Deal Value * Quantity`, `(Revenue - Cost) / Quantity`, `UPPER(LEFT(...)) & "-"` work (well — `UPPER` lands in Phase 9, the `&` part works now).

## Checklist

### Binary operators

In `xlstream-eval/src/ops.rs`:

- [x] `+` — numeric add; error propagation; text-that-parses coerces; bool coerces.
- [x] `-` — subtract.
- [x] `*` — multiply.
- [x] `/` — divide; zero -> `#DIV/0!`.
- [x] `^` — power; negative-base fractional-exponent -> `#NUM!` when appropriate.
- [x] `&` — string concatenation; coerces both sides to text per Excel rules.
- [x] Comparison: `=`, `<>`, `<`, `>`, `<=`, `>=`. Case-insensitive text comparison.

### Unary

- [x] Unary `-`.
- [x] Unary `+`.
- [x] Unary `%` (percent — divides by 100).

### Precedence (in parser or at AST evaluation)

- [x] Verify parser respects Excel precedence:
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
- [x] **Test: `-2^2`** — formualizer-parse gives `^` higher precedence than unary minus (standard math, not Excel). `-2^2 = -4`, not 4. `(-2)^2 = 4` with explicit parens. Parser divergence from Excel; fix is Phase 2 scope.

### Coercion

- [x] `xlstream-core::coerce` module:
  - [x] `to_number(&Value) -> Result<f64, CellError>` — respects Excel semantics.
  - [x] `to_text(&Value) -> Cow<'_, str>`.
  - [x] `to_bool(&Value) -> Result<bool, CellError>`.
- [x] Each returns `CellError::Value` where Excel would.

### Error propagation

- [x] `Value::Error(e) + _` -> `Value::Error(e)`.
- [x] `_ + Value::Error(e)` -> `Value::Error(e)`.
- [x] Comparison against error -> error propagates.

### Tests — arithmetic

For each op: at least 5 tests.
- [x] Happy numeric path.
- [x] Mixed type (number + text-that-parses).
- [x] Type mismatch (number + non-numeric text) -> `#VALUE!`.
- [x] Error propagation from left operand.
- [x] Error propagation from right operand.
- [x] Edge cases: `x/0`, `0^0`, `-(-1)`, `"" & "" = ""`.

### Tests — operator precedence

- [x] `1 + 2 * 3 = 7`.
- [x] `(1 + 2) * 3 = 9`.
- [x] `-2^2 = -4` (parser precedence); `(-2)^2 = 4` (explicit parens).
- [x] `-(-1) = 1`.
- [x] `"a" & "b" & "c" = "abc"`.
- [ ] `2 ^ 3 ^ 2` — right-associative? Check Excel. Update test.
- [x] `10 + "5" = 15`.
- [x] `10 & "5" = "105"`.

### Tests — comparison

- [x] `"a" = "A"` -> TRUE.
- [x] `1 < "2"` -> TRUE (coerces RHS).
- [x] `"10" > "9"` -> FALSE (text compare, lexicographic).
- [x] `0.1 + 0.2 = 0.3` -> TRUE (IEEE with Excel's 15-digit rounding).

### Tests — IEEE rounding

- [x] `0.1 + 0.2 = 0.3` returns TRUE.
- [x] `1.0 / 3.0 * 3.0 = 1.0` returns TRUE.
- [x] Implement Excel's behaviour: when comparing numbers, round to 15 significant digits first.

### Benchmarks

- [ ] `arithmetic_bench.rs`: throughput of `+`, `*`, `/` per 1M ops. Target: > 100M ops/sec single-threaded. **Deferred to Phase 12 — criterion bench harness not set up yet.**

## Done when

Every operator has >= 5 passing tests. Precedence tests pass. IEEE rounding for `0.1 + 0.2 = 0.3` passes.
