# Feature: Preserve Formulas via Template write_formula

**Branch:** `feat/keep-formulas-v3`
**Effort:** ~1.5 days
**Crates:** xlstream-eval, xlstream-io (minor), xlstream-cli, xlstream-python
**Replaces:** PR #83 (per-cell text storage, 900 MB), PR #84 (copy-and-replace, 600-700 MB overhead)

## Problem with prior approaches

**PR #83** stored every formula string in memory: 1M rows x 30 formula cols x ~25 chars = ~900 MB. Rejected.

**PR #84** avoided formula text by copying the input xlsx at the zip level and streaming XML through quick_xml to replace `<v>` values. Eliminated formula text memory but introduced ~600-700 MB of new overhead: ~300 MB results HashMap + ~300-400 MB for reading/writing full sheet XML buffers. Caused a 60-70% wall-clock regression because the entire copy-and-replace pass runs after evaluation.

## Key insight: formulas are column-templated

`build_eval_plan` already groups formulas by column and detects per-row exceptions (`row_overrides`). In real workbooks, >99% of rows in a formula column use the same structural formula — only the row number in cell references changes:

```
Row 2: =A2+B2
Row 3: =A3+B3
Row 4: =A4+B4
```

We don't need 1M formula strings. We need ~30 templates + a fast row-number substitution function.

## New approach: template-based write_formula

Use the existing `stream_single_threaded` / `stream_parallel` write path (rust_xlsxwriter in constant_memory mode) but call `SheetHandle::write_formula()` for formula cells instead of `SheetHandle::write_row()`. Formula text is reconstructed per-row from column templates.

```
For each row:
  For each formula column (in topo order):
    1. Evaluate the formula (existing pipeline, unchanged)
    2. Reconstruct formula text from column template + current row number
    3. Call sh.write_formula(row, col, reconstructed_text, &result)
  For each data column:
    1. Call sh.write_value(row, col, &value)  (existing, unchanged)
```

No results HashMap. No XML manipulation. No second pass. No zip copy. Same write path as `--values-only`, with one extra `write_formula` call per formula cell.

## What already exists

- `SheetHandle::write_formula(row, col, formula, &cached_value)` — already implemented in `xlstream-io/src/sheet_handle.rs`. Uses `rust_xlsxwriter::Formula::new(text).set_result(cached_str)`. Produces `<f>text</f><v>result</v>` in constant_memory mode.
- `build_eval_plan()` in `xlstream-eval/src/evaluate.rs` — already groups formulas by column, identifies per-row exceptions via `ast_streaming_eq`, stores column-representative ASTs in `col_asts` and exceptions in `row_overrides`.
- `reader.formulas(sheet)` — returns `Vec<(row, col, formula_text)>` for all formula cells.
- `EvaluateOptions` with `values_only: bool` — already exists.
- `--values-only` CLI flag and `values_only` Python kwarg — already exist.
- `stream_single_threaded` / `stream_parallel` / `run_worker` — the write pipeline.

## Architecture

### Formula template extraction (during `build_eval_plan`)

For each formula column, `build_eval_plan` already has the first formula text and processes per-row variations. Extend it to also produce a `FormulaTemplate`:

```rust
struct FormulaTemplate {
    /// The original formula text from the first row (with `=` prefix stripped).
    text: String,
    /// Byte positions of row-number substrings within `text` that vary per row.
    /// Each entry: (byte_start, byte_end, base_row_number).
    /// Only relative row references are included — absolute ($) rows are excluded.
    row_refs: Vec<(usize, usize, u32)>,
    /// 1-based row number of the template source row.
    base_row: u32,
}
```

Extraction logic:

1. Take the first formula text for the column (already available at `build_eval_plan` line 1175)
2. Parse it to get cell references with positions (the parser already does this)
3. For each cell reference that has a relative row (not `$`-locked):
   - Record the byte offset of the row-number substring within the formula text
   - Record the base row number
4. Store in `EvalPlan` as `col_templates: HashMap<u32, FormulaTemplate>`
5. For `row_overrides`: store `(Ast, String)` pairs — the override AST plus its original formula text

### Row-number reconstruction (during streaming write)

For each formula cell at row R:

```rust
fn reconstruct_formula(template: &FormulaTemplate, current_row: u32) -> String {
    // 1-based Excel row for the current data row
    let target_row = current_row + 1;
    let row_delta: i64 = target_row as i64 - template.base_row as i64;
    
    let mut result = String::with_capacity(template.text.len() + 8);
    let mut last_end = 0;
    
    for &(start, end, base_ref_row) in &template.row_refs {
        result.push_str(&template.text[last_end..start]);
        let new_row = (base_ref_row as i64 + row_delta) as u32;
        result.push_str(&new_row.to_string());
        last_end = end;
    }
    result.push_str(&template.text[last_end..]);
    result
}
```

This is O(refs_per_formula) string surgery — fast, no allocation beyond the output string.

### Reference position detection

The formula parser already extracts cell references via `extract_references()`. But we need byte positions within the original text, not just parsed (row, col) values. Two options:

**Option A: regex scan of the template text.** Scan for cell-reference patterns (`[A-Z]+[0-9]+`, `$[A-Z]+[0-9]+`, etc.) and record positions. Simple, ~30 lines, handles all cases.

**Option B: extend the parser to track source positions.** The parser already has span information internally (formualizer-parse tracks positions). Expose it. More robust but requires parser changes.

**Recommendation: Option A.** The formula text is short (~20-50 chars), the patterns are well-defined, and we only run this once per column during `build_plan`. A simple byte scan is sufficient and doesn't couple us to parser internals.

### Handling cell reference patterns

The scanner must identify these patterns in formula text and extract the row-number portion:

| Pattern | Example | Row part | Adjusts? |
|---|---|---|---|
| Relative | `A2` | `2` | Yes |
| Absolute col | `$A2` | `2` | Yes |
| Absolute row | `A$2` | `2` | No |
| Fully absolute | `$A$2` | `2` | No |
| Range start | `A2:B10` | `2` and `10` | Yes (both) |
| Cross-sheet | `Sheet1!A2` | `2` | Yes |
| Whole-column | `A:A` | none | N/A |

Key rule: if `$` immediately precedes the row digits, the row is absolute and must not be adjusted.

### Modified `stream_single_threaded` / `stream_parallel`

The streaming write functions currently call `sh.write_row(row_idx, &row_values)` for every row. Modify to:

```rust
// For formula sheets:
for &fcol in topo_order {
    // ... existing eval_column() call ...
}

if !values_only {
    // Write cell-by-cell: data cells via write_value, formula cells via write_formula
    sh.enforce_row_order(row_idx)?;
    for (col_idx, val) in row_values.iter().enumerate() {
        let col = col_idx as u16;
        if let Some(template) = plan.col_templates.get(&(col_idx as u32)) {
            let formula_text = reconstruct_formula(template, row_idx);
            sh.write_formula(row_idx, col, &formula_text, val)?;
        } else if let Some(override_text) = plan.row_override_texts.get(&(col_idx as u32))
            .and_then(|m| m.get(&row_idx)) {
            sh.write_formula(row_idx, col, override_text, val)?;
        } else {
            sh.write_value(row_idx, col, val)?;
        }
    }
} else {
    sh.write_row(row_idx, &row_values)?;
}
```

This eliminates the need for `collect_single_threaded` / `collect_parallel` entirely.

### No separate code paths

Unlike PR #84 which had four functions (stream/collect x single/parallel), this approach modifies the existing two functions (stream_single_threaded, stream_parallel). The only difference between keep-formulas and values-only is whether `write_formula` or `write_value` is called for formula cells. One code path, one branch per cell.

## What to keep from PR #84

- `values_only: bool` in `EvaluateOptions` (already on main or trivial to re-add)
- `--values-only` CLI flag
- `values_only` Python kwarg
- Integration test structure (rewrite tests for new approach)
- `value_to_result_string()` in `xlstream-io/src/convert.rs` (used by `write_formula`)

## What to drop from PR #84

- `formula_preserve.rs` (578 lines) — entire module
- `CellResult` type and `value_to_cell_result()` — not needed
- `collect_single_threaded()` — not needed
- `collect_parallel()` — not needed
- `a1_to_col_row()` — not needed (only used by formula_preserve)
- Direct `quick-xml` and `zip` dependencies in xlstream-io
- `SheetResults` type alias

## What to add

| Item | Location | Lines (est.) |
|---|---|---|
| `FormulaTemplate` struct | `xlstream-eval/src/evaluate.rs` | ~15 |
| `reconstruct_formula()` | `xlstream-eval/src/evaluate.rs` | ~25 |
| `extract_row_refs()` (byte position scanner) | `xlstream-eval/src/evaluate.rs` | ~50 |
| Template extraction in `build_eval_plan()` | `xlstream-eval/src/evaluate.rs` | ~20 |
| `col_templates` + `row_override_texts` in `EvalPlan` | `xlstream-eval/src/evaluate.rs` | ~5 |
| Write-path branching in `stream_single_threaded` | `xlstream-eval/src/evaluate.rs` | ~15 |
| Write-path branching in `run_worker` / parallel path | `xlstream-eval/src/evaluate.rs` | ~15 |
| `write_formula_row()` helper on SheetHandle | `xlstream-io/src/sheet_handle.rs` | ~20 |
| **Total new code** | | **~165** |

Net: ~165 lines added, ~980 lines removed (from PR #84). Net reduction: ~815 lines.

## Memory comparison (1M rows x 30 formula cols)

| Approach | Formula text | Results map | XML buffers | Total extra |
|---|---|---|---|---|
| PR #83 (per-cell text) | 900 MB | 0 | 0 | 900 MB |
| PR #84 (copy-and-replace) | 0 | 300 MB | 300-400 MB | 600-700 MB |
| **v3 (template write_formula)** | **~750 bytes** | **0** | **0** | **~0 MB** |

## Wall-clock comparison

| Approach | Overhead vs values-only | Why |
|---|---|---|
| PR #84 (copy-and-replace) | +60-70% | Second pass: HashMap accumulation + zip copy + XML read/replace/write |
| **v3 (template write_formula)** | **~0%** | Same write path; one extra `write_formula` call per formula cell |

## What this loses vs copy-and-replace (PR #84)

PR #84 preserves all non-formula content byte-for-byte from the input: cell formatting, charts, images, column widths, conditional formatting, data validation, print settings.

The template approach writes from scratch via rust_xlsxwriter. All of the above are lost. This is the explicit trade-off: **zero memory overhead and zero wall-clock regression** in exchange for formatting loss.

For xlstream's stated use case (batch evaluation of business workbooks for data processing), this is the right trade-off. Formatting preservation can be added later as an opt-in `--preserve-formatting` flag using the copy-and-replace approach from PR #84.

## Edge cases

### Formulas with no relative row references

Example: `=SUM(Lookup1!A:A)` — whole-column reference, no row number to adjust. The template has an empty `row_refs` vec. `reconstruct_formula` returns the template text unchanged. Works correctly.

### Absolute row references

Example: `=SUM($A$1:A2)` — the `$1` is absolute (not adjusted), the `2` in `A2` is relative (adjusted). The scanner skips row numbers preceded by `$`. Works correctly.

### Cross-sheet references

Example: `=VLOOKUP(A2, Lookup1!A:D, 2, FALSE)` — only `A2` has a row to adjust. `Lookup1!A:D` is a whole-column range. Works correctly.

### Row overrides

When `build_eval_plan` detects a formula that differs from the column template (fails `ast_streaming_eq`), it stores the per-row AST in `row_overrides`. Extend to also store the per-row formula text in `row_override_texts: HashMap<u32, HashMap<u32, String>>`. At write time, use the override text directly instead of reconstructing from the template.

Memory for overrides: if 1% of 1M rows across 30 cols have overrides = 300K strings x ~25 chars = ~7.5 MB. Negligible.

### Formula text won't be byte-for-byte identical

The reconstructed formula text may differ from the original in:
- `_xlfn.` / `_xlfn._xlws.` prefixes stripped by `strip_xlfn_prefix`
- Whitespace normalization
- Case differences

This is acceptable. Excel normalizes formula text on open — it doesn't depend on exact byte representation. The formulas are functionally identical.

### Formulas in non-main sheets

Secondary sheets with formulas are handled by `secondary_plans` in `EvalPlan`. Extend the template extraction to run for secondary sheets too. Same logic, same data structures.

## Tests

### Unit tests

- `extract_row_refs()` on various formula patterns:
  - Simple: `"A2+B2"` → refs at positions for `2` and `2`
  - Absolute row: `"$A$2+B2"` → only the `B2` row
  - Mixed: `"SUM($A$1:A2)"` → only the `A2` row
  - Cross-sheet: `"Lookup1!A2"` → the `2`
  - Whole-column: `"SUM(A:A)"` → empty
  - Multi-digit row: `"A100"` → ref at `100`
  - No refs: `"1+2"` → empty
- `reconstruct_formula()`:
  - Template `"A2+B2"` at base_row=2, target row 5 → `"A5+B5"`
  - Template with absolute refs — absolute parts unchanged
  - Template with no refs — returns template unchanged
  - Large row numbers (1048576)
- `FormulaTemplate` construction from `build_eval_plan` output

### Integration tests

- Default mode: evaluate fixture, verify output has `<f>` elements
- Default mode: verify cached `<v>` values match evaluated results
- `values_only=true`: verify no `<f>` in output
- All value types produce correct `<f>` + `<v>`: number, text, boolean, error, date
- Cross-sheet formulas: verify `<f>` on all sheets
- Row overrides: verify override text used (not template)
- Self-referential formulas: verify correct `<f>` and converged `<v>`
- Round-trip: evaluate, reopen, verify both formulas and values

### Performance verification

- Run `make bench-report` with default (keep-formulas) — RSS and wall-clock should match values-only baseline within noise
- Verify no regression on `--values-only` path

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add entry under `[Unreleased]` |
| `docs/roadmap/v0.2/README.md` | Tick the keep-formulas checkbox |

## Streaming invariant

Not violated. The write path is the same `stream_single_threaded` / `stream_parallel` pipeline — row-by-row, forward-only. `write_formula` is called inline during the existing row loop, not in a separate pass.

## Advantages over PR #84

1. **Zero extra memory** — no results HashMap, no XML buffers, ~750 bytes of templates
2. **Zero wall-clock regression** — same write path as values-only
3. **Less code** — ~165 lines added, ~980 removed (net -815)
4. **No code duplication** — modifies existing stream functions instead of adding parallel collect functions
5. **No new dependencies** — no direct quick-xml or zip deps needed
6. **Simpler architecture** — one code path with a branch per cell, not two entirely separate write strategies

## Trade-off vs PR #84

| | PR #84 (copy-and-replace) | v3 (template write_formula) |
|---|---|---|
| Memory overhead | 600-700 MB | ~0 MB |
| Wall-clock overhead | +60-70% | ~0% |
| Formatting preserved | Yes | No |
| Code added | +980 lines | +165 lines |
| New dependencies | quick-xml, zip (direct) | None |
| Code paths | 4 functions (stream/collect x single/parallel) | 2 functions (existing, modified) |
