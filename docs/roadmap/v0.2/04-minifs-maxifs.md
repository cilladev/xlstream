# Feature: MINIFS / MAXIFS

**Branch:** `feat/minifs-maxifs`
**Effort:** ~0.5 day
**Crates:** xlstream-parse, xlstream-eval

## What

`MINIFS` returns the minimum value in a range that meets one or more criteria. `MAXIFS` returns the maximum. Same argument pattern as `SUMIFS`/`COUNTIFS`/`AVERAGEIFS`.

```
=MINIFS(B:B, A:A, "EMEA")           → smallest B where A = "EMEA"
=MAXIFS(B:B, A:A, ">100", C:C, "Q1") → largest B where A > 100 and C = "Q1"
```

Current behavior: parser classifies them correctly (already in `AGGREGATE_FUNCTIONS` and criteria-arg detection in `classify.rs:503`). But no dispatch entry exists in the evaluator — falls through to `#VALUE!`.

## What already exists

- `MINIFS`/`MAXIFS` in `AGGREGATE_FUNCTIONS` set — `crates/xlstream-parse/src/sets.rs:41`
- Criteria-arg classification — `crates/xlstream-parse/src/classify.rs:503` handles MINIFS/MAXIFS identical to SUMIFS
- `MultiConditionalAggKey` struct — `crates/xlstream-eval/src/prelude.rs:70-103` with `AggKind` field
- `AggKind` enum — `crates/xlstream-parse/src/rewrite.rs:19-38` (currently has Sum, Count, CountA, Average, Min, Max, Product, Median, CountBlank)
- `builtin_sumifs()` — `crates/xlstream-eval/src/builtins/multi_conditional.rs:46-82` — exact pattern to follow
- `builtin_countifs()` — `crates/xlstream-eval/src/builtins/multi_conditional.rs:88-128`
- `builtin_averageifs()` — `crates/xlstream-eval/src/builtins/multi_conditional.rs:129+`
- `extract_sumifs_key()` — `crates/xlstream-eval/src/prelude_plan.rs:350-370` — key extraction pattern
- `collect_multi_conditional_keys()` — `crates/xlstream-eval/src/prelude_plan.rs` — walks AST to find multi-conditional keys
- Prelude computation of multi-conditional aggregates — already handles Sum, Count, Average via `AggKind`

## Where to look

- `crates/xlstream-parse/src/rewrite.rs:19-38` — `AggKind` enum (may need MinIfs/MaxIfs variants or reuse Min/Max)
- `crates/xlstream-eval/src/builtins/multi_conditional.rs` — add `builtin_minifs()` and `builtin_maxifs()` following SUMIFS pattern
- `crates/xlstream-eval/src/builtins/mod.rs:103-105` — add dispatch arms for MINIFS, MAXIFS
- `crates/xlstream-eval/src/prelude_plan.rs:350-421` — add `extract_minifs_key()` and `extract_maxifs_key()` key extractors
- `crates/xlstream-eval/src/prelude_plan.rs` — `collect_multi_conditional_keys()` to recognize MINIFS/MAXIFS
- `crates/xlstream-eval/src/prelude.rs` — prelude computation logic for multi-conditional aggregates (verify Min/Max accumulation works)

## Resolution / Evaluation behavior

Identical to SUMIFS/COUNTIFS/AVERAGEIFS. These are whole-column aggregates computed during prelude.

**Classification:** Already handled. `classify.rs:503` recognizes MINIFS/MAXIFS with the same criteria-arg pattern as SUMIFS.

**Prelude:** `collect_multi_conditional_keys()` must recognize MINIFS/MAXIFS function names and extract `MultiConditionalAggKey` entries with the appropriate `AggKind`. During the prelude scan, the multi-conditional accumulator groups rows by composite criteria key and tracks the min/max of the target column.

**Row eval:** `builtin_minifs()`/`builtin_maxifs()` evaluate criteria args from the current row, build the composite key, and look up the pre-computed min/max from the prelude. Same as `builtin_sumifs()` but with `AggKind::Min`/`AggKind::Max`.

**Argument layout:**
- `MINIFS(min_range, criteria_range1, criteria1, ...)` — same as SUMIFS
- `MAXIFS(max_range, criteria_range1, criteria1, ...)` — same as SUMIFS

## Tests

### Classification (unit tests)

**Happy path:**
- `MINIFS(B:B, A:A, "x")` — one criteria pair
- `MAXIFS(B:B, A:A, ">0", C:C, "Q1")` — two criteria pairs
- `MINIFS(B:B, A:A, D2)` — row-local criteria value

**Edge cases:**
- `minifs(b:b, a:a, "x")` — case insensitive
- `MINIFS(B:B)` — missing criteria pair, should produce `#VALUE!`
- `MINIFS(B:B, A:A)` — incomplete pair, should produce `#VALUE!`
- `MAXIFS(Sheet2!B:B, Sheet2!A:A, "x")` — cross-sheet ranges
- `IF(D2, MINIFS(B:B, A:A, E2), 0)` — nested in IF
- `MINIFS(B:B, A:A, "x", C:C, ">0", D:D, "y")` — three criteria pairs
- Error propagation: criteria value is `#N/A`

**Regression guards:**
- SUMIFS, COUNTIFS, AVERAGEIFS behavior unchanged
- Existing multi-conditional prelude computation unchanged

### Evaluation (integration tests)

- MINIFS with one criteria — verify returns minimum matching value
- MAXIFS with one criteria — verify returns maximum matching value
- Two criteria pairs — verify AND-logic (both must match)
- No matching rows — MINIFS returns 0, MAXIFS returns 0
- All matching rows — returns global min/max of target column
- Row-local criteria (`MINIFS(B:B, A:A, E2)`) — criteria changes per row
- Mixed with SUMIFS in same workbook — both compute correctly

### Conformance fixture

Create `tests/fixtures/aggregate/minifs_maxifs.xlsx` with data + MINIFS/MAXIFS formulas. Generate with openpyxl, recalculate with LibreOffice headless. Add `#[test] fn minifs_maxifs()` in `conformance/aggregate.rs`.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add MINIFS/MAXIFS entry under `[Unreleased]` |
| `docs/functions.md` | Tick MINIFS and MAXIFS as implemented |
| `docs/roadmap/v0.2/README.md` | Tick the checkbox |

## Streaming invariant

Does not violate. MINIFS/MAXIFS are whole-column aggregates pre-computed during prelude pass 1. Row evaluation reads from pre-computed lookup tables only.
