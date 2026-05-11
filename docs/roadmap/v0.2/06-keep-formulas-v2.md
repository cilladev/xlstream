# Feature: Preserve Formulas via Copy-and-Replace

**Branch:** `feat/keep-formulas-v2`
**Effort:** ~1 day
**Crates:** xlstream-core, xlstream-eval, xlstream-io, xlstream-cli, xlstream-python
**Replaces:** PR #83 (formula text storage approach — 3.5 GB memory regression)

## Problem with PR #83

PR #83 stores all formula text strings in memory (`HashMap<u32, HashMap<u32, String>>`). For 1M rows x 30 formula cols = ~900 MB of string clones. Even with `--values-only`, the text is allocated during `build_eval_plan` and immediately discarded. Result: 1.8 GB → 3.5 GB RSS regression.

## New approach: copy-and-replace

Don't store formula text. Don't use `rust_xlsxwriter` for the keep-formulas mode. Instead:

1. Copy the input xlsx to the output path (zip-level copy)
2. Open the output zip, read each sheet's XML
3. Stream the XML with `quick_xml` — pass everything through unchanged
4. For cells with `<f>` (formula): evaluate the formula, replace the `<v>` value text
5. For cells without `<f>` (data): pass through unchanged
6. Write modified XML back to the zip

The `<f>` element is **never read into Rust memory** — it flows from input XML to output XML untouched. Only `<v>` content is replaced. Zero formula text storage. Memory overhead: ~10 KB for the XML streaming buffer.

For `--values-only` mode: use the current rust_xlsxwriter approach (write from scratch, no `<f>` elements). Same as today's behavior. No regression.

```
Default (keep formulas):
  input.xlsx ──copy──> output.xlsx
  open output.xlsx zip
  for each sheet XML:
    quick_xml stream: pass through all elements
    when <v> inside a <c> with <f>: replace value text with evaluated result
  rewrite zip entry

--values-only:
  current approach (rust_xlsxwriter from scratch)
  unchanged, no regression
```

## What already exists

- `EvaluateOptions` with `values_only: bool` (from PR #83, keep this)
- `--values-only` CLI flag and `values_only` Python kwarg (from PR #83, keep this)
- `quick-xml` is already a transitive dependency (via calamine)
- `zip` crate is already a transitive dependency (via calamine)
- The evaluation pipeline (prelude, classification, topo sort, streaming eval) is unchanged — we just change HOW results are written, not how they're computed

## What changes from PR #83

| PR #83 approach | Copy-and-replace approach |
|---|---|
| Store all formula texts in EvalPlan (~900 MB) | No formula text storage (0 MB) |
| `write_row_with_formulas()` via rust_xlsxwriter | Stream XML via quick_xml, replace `<v>` only |
| `build_row_formula_map()` per row | No per-row map needed |
| `can_round_trip_as_formula()` fallback | Not needed — `<f>` is never touched, all types work |
| Text/Error/Date formulas silently lose `<f>` | All formulas preserved regardless of result type |

## Where to look

- `crates/xlstream-io/src/writer.rs` — current `Writer` struct (rust_xlsxwriter wrapper)
- `crates/xlstream-eval/src/evaluate.rs` — `evaluate()` entry point, `stream_single_threaded()`, `stream_parallel()`
- `crates/xlstream-core/src/options.rs` — `EvaluateOptions` (already has `values_only`)
- `refs/calamine/src/xlsx/mod.rs` — how calamine reads xlsx XML (for reference)
- `quick-xml` docs — `Reader`, `Writer`, `Event` types

## Architecture

### Two write modes in `evaluate()`

```rust
if options.values_only {
    // Current approach: rust_xlsxwriter from scratch
    // No <f> elements, just <v> values
    // Zero formula text memory
    let mut writer = Writer::create(output)?;
    stream_single_threaded(&mut reader, &mut writer, &plan)?;
    writer.finish()?;
} else {
    // New approach: copy input, replace <v> values
    // <f> elements pass through untouched
    // Zero formula text memory
    std::fs::copy(input, output)?;
    replace_values_in_place(output, &plan, &mut reader)?;
}
```

### The `replace_values_in_place` function

Opens the output xlsx as a zip. For each worksheet XML entry:

1. Read the XML into a buffer (or stream if possible)
2. Use `quick_xml::Reader` + `quick_xml::Writer` to stream through events
3. Track current row/col from `<row r="N">` and `<c r="B5">` attributes
4. When inside a `<c>` that has a `<f>` child AND is a formula column in the eval plan:
   - Pass `<f>` through unchanged
   - Replace `<v>` text content with the evaluated result for that (row, col)
5. All other elements: pass through byte-for-byte

### How evaluated results are available

The evaluation still happens via the existing streaming pipeline. But instead of writing results to rust_xlsxwriter during streaming, store them in a `HashMap<(u32, u32), String>` (row, col → result string).

Wait — that's the same memory problem. For 1M rows x 30 formula cols = 30M entries.

**Better:** do the evaluation AND the XML replacement in a single streaming pass. Read the input XML row by row. For each row, calamine has already given us the cell values. Evaluate the formula columns. Then write the XML for that row with replaced `<v>` values.

But we can't stream the input twice (calamine consumes it). And we can't interleave calamine streaming with raw XML reading — they'd compete for the same zip reader.

**The actual approach:**

1. Run the evaluation pipeline as today — streaming, row by row
2. Instead of writing via rust_xlsxwriter, accumulate **only the formula cell results** per row: `(row, col, result_string)`
3. After streaming is complete, do a second pass over the raw XML to replace `<v>` values
4. The results are the same strings that `write_value` would have produced — just stored temporarily

Memory for results: 1M rows x 30 formula cols x ~10 bytes per result string = ~300 MB. Better than 900 MB for formula text, but still significant.

**Even better — stream both passes simultaneously:**

Actually, the simplest correct approach:

1. Phase 1 (current): evaluate all formulas, store results in a flat `Vec<(u32, u16, String)>` sorted by (row, col). For 1M x 30 = 30M entries x ~20 bytes = ~600 MB. Still too much.

**The right approach: process one row at a time during the XML replace pass.**

The evaluation pipeline streams row by row. The XML replace also processes row by row. Interleave them:

For each `<row>` in the XML:
1. Read the row's cells from the XML
2. Evaluate formula columns using the existing eval pipeline
3. Write the row's XML with replaced `<v>` values
4. Discard the row — no accumulation

This requires reading the sheet XML ourselves (not via calamine) and evaluating as we go. The prelude pass still uses calamine. The row-streaming pass replaces calamine's `cells()` with raw XML reading.

This is a deeper change — replacing the calamine streaming reader with our own XML reader for the default mode. But it's the only way to get zero extra memory.

## Simplified approach (recommended)

Keep it simple. Accept ~300 MB for result storage. The formula TEXT (900 MB) is the big savings — result VALUES are small strings (numbers).

1. Evaluate via current pipeline (calamine streaming + eval). Store results as `Vec<(u32, u16, String)>`.
2. Copy input xlsx to output.
3. Stream output sheet XML via quick_xml. For each `<v>` inside a formula cell, look up the result and replace.

**Memory comparison:**

| Approach | Formula text | Result values | Total extra |
|---|---|---|---|
| PR #83 | 900 MB | 0 (written inline) | 900 MB |
| Copy-and-replace | 0 | 300 MB | 300 MB |
| Current (values-only) | 0 | 0 | 0 |

300 MB is 3x better than 900 MB. And the results are small fixed-size strings (numbers), not variable-length formula text. For the 700k reference workbook (20 formula cols), it's ~140 MB.

For `--values-only`: zero extra memory (current approach unchanged).

## Implementation plan

### What to keep from PR #83
- `values_only: bool` in `EvaluateOptions`
- `--values-only` CLI flag
- `values_only` Python kwarg
- Writer-level tests for `write_row_with_formulas` (useful for future)

### What to remove from PR #83
- `formula_texts` field in `EvalPlan` and `SheetEvalPlan`
- `build_row_formula_map()` helper
- `can_round_trip_as_formula()` helper
- Formula text cloning in `build_eval_plan()`
- Formula text threading through parallel path

### What to add
- `replace_sheet_values()` function in `xlstream-io` — streams XML, replaces `<v>` values
- `copy_and_replace()` function in `xlstream-eval` — copies zip, calls replace per sheet
- Result accumulation during eval: `HashMap<(u32, u16), String>` per sheet
- Direct `zip` and `quick-xml` dependencies in `xlstream-io` (already transitive, make explicit)

## Tests

### Unit tests
- `replace_sheet_values()` on a small XML fragment — verify `<f>` untouched, `<v>` replaced
- `replace_sheet_values()` with data-only cells — verify pass-through
- `replace_sheet_values()` with mixed formula/data rows

### Writer-level tests
- Keep existing `write_row_with_formulas` tests (useful for values-only mode)

### Integration tests
- Default mode: evaluate fixture, verify output has `<f>` elements via `reader.formulas()`
- Default mode: verify cached `<v>` values are correct (not the old cached values)
- `values_only=true`: verify no `<f>` in output (same as current)
- Text-producing formula: verify `<f>` IS preserved (unlike PR #83 which dropped it)
- Error-producing formula: verify `<f>` IS preserved
- Date formula: verify `<f>` IS preserved
- Cross-sheet formulas: verify `<f>` preserved on all sheets
- Mixed-column formulas: verify per-row `<f>` preserved

### Memory verification
- Run bench-report with keep-formulas default — RSS should be close to values-only baseline
- Verify no regression on `--values-only` path

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add entry under `[Unreleased]` |
| `docs/roadmap/v0.2/README.md` | Tick the keep-formulas checkbox |

## Streaming invariant

Does not violate. The copy-and-replace pass happens AFTER evaluation is complete. The evaluation pipeline is unchanged. The XML streaming pass reads and writes sequentially — no random access.

## Advantages over PR #83

1. **Zero formula text memory** — `<f>` flows from input XML to output XML without entering Rust
2. **All value types preserved** — Text, Error, Date formulas keep their `<f>` (no `can_round_trip_as_formula` fallback)
3. **Simpler code** — no formula text storage, no per-row formula map building, no round-trip checks
4. **Correct output** — every `<f>` from input appears in output, byte-for-byte identical
5. **Preserves formatting** — cell styles, column widths, chart references all survive (they're in the XML we pass through)
