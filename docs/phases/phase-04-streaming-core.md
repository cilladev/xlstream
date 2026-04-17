# Phase 4 — Streaming core

**Goal:** end-to-end pipeline that opens an xlsx, classifies formulas, runs an empty prelude, streams rows through a minimal evaluator that resolves cell refs only, and writes output. No builtins yet.

**Estimated effort:** ~1 week.

**Prerequisites:** Phases 2 + 3.

**Reading:** [`docs/architecture/streaming-model.md`](../architecture/streaming-model.md), [`docs/architecture/evaluator.md`](../architecture/evaluator.md).

**Output:** `xlstream-eval::evaluate(input, output, None)` runs end-to-end on a fixture with only cell-reference formulas (e.g. `=A2`, `=B2`). Output has those cells resolved.

## Checklist

### Types

- [ ] `Prelude` struct — empty for now (no aggregates, no lookups).
- [ ] `RowScope<'row>` struct — holds the current row's values + header map + row index.
- [ ] `Interpreter<'ctx>` struct — holds `&Prelude`, `&BuiltinRegistry` (still empty).
- [ ] `BuiltinRegistry` — stub, `phf::phf_map!{}` with zero entries.

### Driver

- [ ] `evaluate(input, output, workers)`:
  1. Open `Reader`.
  2. For the first sheet with formulas (v0.1 assumes one main sheet; multi-sheet in later phases):
     1. Scan: read headers, scan formula cells via `reader.formulas()`.
     2. Parse each formula via `xlstream_parse::parse`.
     3. Classify.
     4. Refuse if any unsupported.
  3. Build prelude (empty for now).
  4. Build intra-row topo order of formula columns.
  5. Open `Writer`.
  6. Copy non-main sheets as-is (pass-through).
  7. For the main sheet, write header row.
  8. Stream rows: for each row, for each formula column in topo order, call `interp.eval(ast, &row_scope)`, store into the row vec.
  9. Write row.
  10. Close writer.
- [ ] Return `EvaluateSummary` with rows processed, duration, peak RSS.

### Interpreter — minimal

- [ ] `eval(node: &AstNode, scope: &RowScope) -> Result<Value, XlStreamError>`:
  - [ ] `Number(n)`, `Text(s)`, `Bool(b)`, `Error(e)` literals → return as-is.
  - [ ] `CellRef` — resolve to current-row value via header map or row-index.
  - [ ] `Function` — look up in registry; registry is empty → return `XlStreamError::Unsupported`.
  - [ ] `Binary`, `Unary` — return `XlStreamError::Unsupported` for now (implemented in Phase 5).
- [ ] Value cloning minimised; `Value::Number`/`Bool`/`Empty`/`Error` are Copy; text clones are Arc'd where feasible.

### Topo order

- [ ] Build intra-row DAG: for each formula column, its dependencies are the other formula columns it references.
- [ ] Topological sort: `Vec<Col>` in evaluation order.
- [ ] Cycle detection → `XlStreamError::CircularReference`.
- [ ] Tests: diamond, linear chain, cycle.

### Tests

- [ ] End-to-end: fixture with `=A2` in column C, produces column C filled with col-A values.
- [ ] End-to-end: fixture with two chained formula cols (`=A2` → `B2`, `=B2 * 1` — wait, `* 1` is arithmetic, skip for this phase. Use `=B2` → `A2` chained through a pass-through wrap).
- [ ] Unsupported formula → clear error with formula text + reason.
- [ ] Missing reference → `#REF!` in output cell.
- [ ] Circular refs refused.

### Performance smoke

- [ ] 10k rows × 10 columns (all `=A2` style, trivially resolvable): < 2 s.
- [ ] Peak RSS: < 50 MB.

### CLI

- [ ] `xlstream-cli evaluate input.xlsx --output out.xlsx` runs end-to-end.
- [ ] `--verbose` flag prints phase timings (classify, prelude, stream).

## Done when

End-to-end pipeline works for cell-ref-only formulas. Topo order + cycle detection tested. CLI runs. Performance smoke hits targets. Path is prepared for builtins in Phases 5–9.
