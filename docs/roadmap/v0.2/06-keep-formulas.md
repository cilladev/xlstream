# Feature: Preserve Formulas in Output

**Branch:** `feat/keep-formulas`
**Effort:** ~4 hours
**Crates:** xlstream-core, xlstream-eval, xlstream-io, xlstream-cli, xlstream-python

## What

By default, xlstream should preserve formulas in the output xlsx — writing `<f>formula</f><v>result</v>` instead of just `<v>result</v>`. This matches Excel's behavior: opening the output file shows formulas in cells, not just static values. A `values_only` option (CLI: `--values-only`) produces a flat values-only output for downstream data consumers.

```
Current behavior:
  Input:  B2 has =A2*2 with A2=5
  Output: B2 has value 10 (formula erased)

After this change (default):
  Input:  B2 has =A2*2 with A2=5
  Output: B2 has =A2*2 with cached value 10

With --values-only:
  Input:  B2 has =A2*2 with A2=5
  Output: B2 has value 10 (formula erased, same as current)
```

## What already exists

- `SheetHandle::write_formula(row, col, formula, &cached)` in `crates/xlstream-io/src/sheet_handle.rs:96-109` — already writes `<f>formula</f><v>cached</v>` using `rust_xlsxwriter::Formula::set_result()`. Currently unused by the evaluate pipeline.
- `SheetHandle::write_row(row, values)` in `crates/xlstream-io/src/sheet_handle.rs:63-73` — writes values only (no formulas). Currently used by the evaluate pipeline.
- `reader.formulas(sheet)` in `crates/xlstream-io/src/reader.rs:181` — returns `Vec<(row, col, formula_text)>` for all formula cells. Used in `build_plan()` to collect formulas for classification.
- `EvaluateOptions` in `crates/xlstream-core/src/options.rs` — already has `workers`, `iterative_calc`, `max_iterations`, `max_change`. Add `values_only` here.
- The formula text is available during `build_eval_plan()` as `col_formulas: HashMap<u32, Vec<(u32, String)>>`. It's stored per-column. The streaming loop has access to `col_asts` and `topo_order` but NOT the original formula text — that's discarded after parsing.

## Where to look

- `crates/xlstream-core/src/options.rs` — add `values_only: bool` to `EvaluateOptions`
- `crates/xlstream-eval/src/evaluate.rs:63-73` — `EvalPlan` struct, add formula text storage
- `crates/xlstream-eval/src/evaluate.rs:832-834` — `build_eval_plan()` where formula text is collected
- `crates/xlstream-eval/src/evaluate.rs:540-572` — `stream_single_threaded()` write loop
- `crates/xlstream-eval/src/evaluate.rs:740-760` — `run_worker()` write loop
- `crates/xlstream-eval/src/evaluate.rs:133-175` — parallel path secondary sheet writes
- `crates/xlstream-io/src/sheet_handle.rs:96-109` — `write_formula()` method
- `crates/xlstream-io/src/sheet_handle.rs:63-73` — `write_row()` method
- `crates/xlstream-cli/src/main.rs` — CLI args
- `bindings/python/src/lib.rs` — Python kwargs

## Resolution / Evaluation behavior

No classification or evaluation changes. This is purely an output concern.

**Plan build:** `build_eval_plan()` already collects formula text per column (`col_formulas`). Store the formula text map in `EvalPlan` alongside `col_asts`.

**Streaming:** For each formula column in the write step:
- If `values_only` is false (default): call `sh.write_formula(row, col, formula_text, &result)` instead of writing the value directly via `write_row()`
- If `values_only` is true: current behavior (write value only)

**Row-override formulas:** When a row has a per-row AST override (mixed-column formulas), use that row's formula text, not the default column formula.

**Non-formula cells:** Always write as values (no change).

**Parallel path:** Workers send `(row_idx, values, formulas_count)` through the channel. The writer knows which columns are formula columns and has the formula text map — it can look up the text by column index.

## API

### EvaluateOptions
```rust
pub struct EvaluateOptions {
    pub workers: Option<usize>,
    pub iterative_calc: bool,
    pub max_iterations: u32,
    pub max_change: f64,
    pub values_only: bool,  // default: false
}
```

### Python
```python
# Default — formulas preserved
xlstream.evaluate("in.xlsx", "out.xlsx")

# Values only
xlstream.evaluate("in.xlsx", "out.xlsx", values_only=True)
```

### CLI
```bash
# Default — formulas preserved
xlstream evaluate in.xlsx -o out.xlsx

# Values only
xlstream evaluate in.xlsx -o out.xlsx --values-only
```

## Tests

### Classification (unit tests)

No classification changes — not applicable.

### Conformance fixtures

Create `tests/fixtures/issues/keep-formulas.xlsx`:

**Sheet1 — mixed data and formulas:**
- A: data column (numbers)
- B: `=A{r}*2` (row-local formula)
- C: `=SUM(A:A)` (aggregate formula)
- D: data column (text)
- E: `=IF(A{r}>50,"high","low")` (conditional formula)
- 10 data rows

**Tests to verify (custom test, not standard `run_conformance`):**
- Default mode (`values_only=false`): output xlsx has `<f>` elements for formula cells
- Values-only mode (`values_only=true`): output xlsx has NO `<f>` elements
- Cached values in `<v>` are correct in both modes
- Non-formula cells have no `<f>` in either mode
- Cross-sheet formula texts preserved correctly in default mode
- Row-override formula texts (mixed-column) preserved with the correct per-row text
- Formula text is the original input formula (not the rewritten/classified form)

**Verification approach:** Open the output with calamine's `worksheet_formula()` to check if formulas are present. Compare against expected formula strings.

### Additional unit tests

- `EvaluateOptions::default().values_only == false`
- CLI parses `--values-only` correctly
- `write_formula` round-trip: write formula + cached value, read back, verify both present

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add entry: "formulas preserved in output by default; add `--values-only` / `values_only` for static output" |
| `docs/roadmap/v0.2/README.md` | Tick the keep-formulas checkbox |

## Streaming invariant

Does not violate. This is an output-only change — no effect on parsing, classification, prelude, or evaluation. The formula text is read from the input and written to the output alongside the computed result.
