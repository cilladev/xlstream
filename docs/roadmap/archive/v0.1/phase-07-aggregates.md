# Phase 7 — Aggregates

**Goal:** whole-column and criteria-based aggregates pre-computed at prelude; referenced as scalars during row eval.

**Estimated effort:** 3-4 days.

**Prerequisites:** Phases 3 + 4 + 5.

**Reading:** [`docs/architecture/aggregates.md`](../architecture/aggregates.md).

**Output:** `=SUM(Deal Value:Deal Value)`, `=SUMIF(Region:Region, "EMEA", Deal Value:Deal Value)` work efficiently.

## Checklist

### Classification

- [x] Extend classifier to recognise aggregate calls with whole-column range args -> `AggregateOnly` or contributor to `Mixed`.
- [x] Recognise `*IF`, `*IFS` variants; capture criteria as `Criteria` enums at classification time (where criteria is a constant-resolvable string).
- [x] Refuse criteria that depend on per-row values at classification time.

### `Criteria` parsing

- [x] `Criteria::parse(s: &str) -> Criteria`:
  - [x] `"=5"` -> `Equals(Value::Number(5.0))`.
  - [x] `">10"` -> `Greater(10.0)`.
  - [x] `"<>abc"` -> `NotEquals(Value::Text("abc".into()))`.
  - [x] `"ap*le"` -> `Wildcard(...)`.
  - [x] `""` -> `Blank`.
  - [x] `"<>"` -> `NonBlank`.
  - [x] Otherwise -> `Equals(Value::Text(s.into()))`.
- [x] Unit tests for every form.

### Prelude builder

- [x] `PreludePlan` — set of (sheet, column, aggregates-needed) tuples.
- [x] Group by `(sheet, columns)` to produce one streaming read per column-combo.
- [x] `execute_plan(reader, plan) -> Prelude` streams and accumulates into the prelude store.

### Aggregate functions

In `xlstream-eval/src/builtins/aggregate.rs`:

- [x] `SUM` — numeric sum; skip text/bool unless part of an aggregate-with-coercion (bool -> 1/0) — match Excel.
- [x] `COUNT` — count of numeric cells only.
- [x] `COUNTA` — count of non-empty.
- [x] `COUNTBLANK` — count of empty.
- [x] `AVERAGE` — mean; empty -> `#DIV/0!`.
- [x] `MIN`, `MAX` — over numerics; empty -> 0.
- [x] `PRODUCT` — multiplication; empty -> 0.
- [x] `MEDIAN` — middle of sorted numerics.
- [x] `SUMIF(range, criteria, sum_range?)` — conditional sum.
- [x] `COUNTIF(range, criteria)`.
- [x] `AVERAGEIF(range, criteria, avg_range?)`.
- [x] `SUMIFS(sum_range, (crit_range, crit)+)` — multi-criteria sum.
- [x] `COUNTIFS(...)` — multi-criteria count.
- [x] `AVERAGEIFS(...)` — multi-criteria average.
- [ ] `MINIFS(...)`, `MAXIFS(...)` — deferred (v0.2). (deferred: v0.2)

### Prelude storage

- [x] `Prelude::aggregates: HashMap<AggKey, Value>`.
- [x] `AggKey::Whole { sheet, col, kind }`.
- [x] `AggKey::Conditional { sheet, sum_col, criteria_cols, criteria_hash }`.

### AST rewrite

- [x] Classifier replaces aggregate sub-ASTs with `AstNode::PreludeRef(AggKey)`.
- [x] Evaluator's `eval` dispatches `PreludeRef` to a hashmap lookup in `Prelude::aggregates`. O(1).

### Tests

- [x] Each aggregate: happy path, empty range, error propagation, mixed types.
- [x] `SUMIF` with numeric criteria (`">10"`).
- [x] `SUMIF` with wildcard (`"ap*"`).
- [ ] `SUMIFS` with 2 criteria ranges — deferred.
- [x] `COUNTIF` with blank / non-blank.
- [x] Criteria respects case-insensitivity for text.
- [x] Aggregates over a range with one error -> propagate that error.
- [ ] Large column performance smoke: SUM over 400k cells < 500 ms — deferred to Phase 12. (deferred: covered by Phase 12 benchmarks)

## Integration tests

- [x] Fixture mirroring xlformula's `Pct of Total` column: `Deal Value / SUM(Deal Value:Deal Value) * 100`.
- [x] Assertion: output values match Excel-computed golden to within IEEE precision.

## Done when

All simple aggregates work. `Pct of Total` fixture computes correctly. SUMIF/COUNTIF/AVERAGEIF work with static criteria. SUMIFS/COUNTIFS/AVERAGEIFS/MINIFS/MAXIFS deferred to follow-up.
