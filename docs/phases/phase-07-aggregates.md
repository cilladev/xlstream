# Phase 7 — Aggregates

**Goal:** whole-column and criteria-based aggregates pre-computed at prelude; referenced as scalars during row eval.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phases 3 + 4 + 5.

**Reading:** [`docs/architecture/aggregates.md`](../architecture/aggregates.md).

**Output:** `=SUM(Deal Value:Deal Value)`, `=SUMIF(Region:Region, "EMEA", Deal Value:Deal Value)` work efficiently.

## Checklist

### Classification

- [ ] Extend classifier to recognise aggregate calls with whole-column range args → `AggregateOnly` or contributor to `Mixed`.
- [ ] Recognise `*IF`, `*IFS` variants; capture criteria as `Criteria` enums at classification time (where criteria is a constant-resolvable string).
- [ ] Refuse criteria that depend on per-row values at classification time.

### `Criteria` parsing

- [ ] `Criteria::parse(s: &str) -> Criteria`:
  - [ ] `"=5"` → `Equals(Value::Number(5.0))`.
  - [ ] `">10"` → `Greater(10.0)`.
  - [ ] `"<>abc"` → `NotEquals(Value::Text("abc".into()))`.
  - [ ] `"ap*le"` → `Wildcard(...)`.
  - [ ] `""` → `Blank`.
  - [ ] `"<>"` → `NonBlank`.
  - [ ] Otherwise → `Equals(Value::Text(s.into()))`.
- [ ] Unit tests for every form.

### Prelude builder

- [ ] `PreludePlan` — set of (sheet, column, aggregates-needed) tuples.
- [ ] Group by `(sheet, columns)` to produce one streaming read per column-combo.
- [ ] `execute_plan(reader, plan) -> Prelude` streams and accumulates into the prelude store.

### Aggregate functions

In `xlstream-eval/src/builtins/aggregate.rs`:

- [ ] `SUM` — numeric sum; skip text/bool unless part of an aggregate-with-coercion (bool → 1/0) — match Excel.
- [ ] `COUNT` — count of numeric cells only.
- [ ] `COUNTA` — count of non-empty.
- [ ] `COUNTBLANK` — count of empty.
- [ ] `AVERAGE` — mean; empty → `#DIV/0!`.
- [ ] `MIN`, `MAX` — over numerics; empty → 0.
- [ ] `PRODUCT` — multiplication; empty → 0.
- [ ] `MEDIAN` — middle of sorted numerics.
- [ ] `SUMIF(range, criteria, sum_range?)` — conditional sum.
- [ ] `COUNTIF(range, criteria)`.
- [ ] `AVERAGEIF(range, criteria, avg_range?)`.
- [ ] `SUMIFS(sum_range, (crit_range, crit)+)`.
- [ ] `COUNTIFS(...)`.
- [ ] `AVERAGEIFS(...)`.
- [ ] `MINIFS(...)`, `MAXIFS(...)`.

### Prelude storage

- [ ] `Prelude::aggregates: HashMap<AggKey, Value>`.
- [ ] `AggKey::Whole { sheet, col, kind }`.
- [ ] `AggKey::Conditional { sheet, sum_col, criteria_cols, criteria_hash }`.

### AST rewrite

- [ ] Classifier replaces aggregate sub-ASTs with `AstNode::PreludeRef(AggKey)`.
- [ ] Evaluator's `eval` dispatches `PreludeRef` to a hashmap lookup in `Prelude::aggregates`. O(1).

### Tests

- [ ] Each aggregate: happy path, empty range, error propagation, mixed types.
- [ ] `SUMIF` with numeric criteria (`">10"`).
- [ ] `SUMIF` with wildcard (`"ap*"`).
- [ ] `SUMIFS` with 2 criteria ranges.
- [ ] `COUNTIF` with blank / non-blank.
- [ ] Criteria respects case-insensitivity for text.
- [ ] Aggregates over a range with one error → propagate that error.
- [ ] Large column performance smoke: SUM over 400k cells < 500 ms.

## Integration tests

- [ ] Fixture mirroring xlformula's `Pct of Total` column: `Deal Value / SUM(Deal Value:Deal Value) * 100`.
- [ ] Assertion: output values match Excel-computed golden to within IEEE precision.

## Done when

All aggregates work. `Pct of Total` fixture computes correctly. Large-column aggregates hit the < 500 ms target.
