# Feature: Named ranges

**Branch:** `feat/named-ranges`
**Effort:** ~1 day
**Crates:** xlstream-parse, xlstream-eval

## What

Resolve `MyRange` in formulas via Excel's `<definedNames>` at classification time. After resolution, the formula becomes a regular range/cell reference that the existing pipeline handles.

```
=SUM(SalesData)                    → SUM(Sheet1!$A$1:$A$100)
=VLOOKUP(A2, Rates, 2, FALSE)     → VLOOKUP(A2, Lookup!$B$2:$D$50, 2, FALSE)
=TaxRate * B2                      → 0.15 * B2
```

Currently returns `Unsupported(NamedRange)`.

## What already exists

- **Parser:** recognizes named ranges → `Reference::Named(name)`
- **Classifier:** catches them at `classify.rs:436` → `Disposition::Unsupported(NamedRange)`
- **Calamine:** exposes `defined_names() -> &[(String, String)]` — name/value pairs from `workbook.xml`

## Where to look

- `crates/xlstream-eval/src/evaluate.rs` — where defined_names should be read and passed to classification context
- `crates/xlstream-parse/src/classify.rs:436` — current rejection point (`Reference::Named → Unsupported`)
- `crates/xlstream-parse/src/classify.rs` — `ClassificationContext` struct (needs `named_ranges` field)
- `crates/xlstream-parse/src/references.rs:48` — `Reference::Named(String)` variant
- `refs/calamine/src/lib.rs` — `defined_names()` API

## Resolution cases

Three types of named range values:
- **Range:** `Sheet1!$A$1:$A$100` → classify as range ref (may be aggregate, lookup, or row-local depending on context)
- **Cell:** `Sheet1!$A$1` → classify as cell ref
- **Constant:** `0.15` → classify as RowLocal (literal value)

Unknown names (not in `defined_names`) should still return `Unsupported(NamedRange)`.

## Tests

### Classification (unit tests in classify.rs)

**Happy path:**
- `SUM(SalesData)` where `SalesData` = `Sheet1!A:A` → `AggregateOnly`
- `TaxRate * B2` where `TaxRate` = `0.15` → `RowLocal`
- `VLOOKUP(A2, Rates, 2, FALSE)` where `Rates` = `Lookup!A:D` → `Lookup`
- `SalesData + 1` where `SalesData` = `Sheet1!$A$1` (single cell, same row) → `RowLocal`

**Case insensitivity:**
- `SUM(salesdata)` where `SalesData` defined → resolves (Excel names are case-insensitive)
- `SUM(SALESDATA)` where `SalesData` defined → resolves

**Constants:**
- `TaxRate * B2` where `TaxRate` = `0.15` → resolves to literal, classifies as `RowLocal`
- `TaxRate` where `TaxRate` = `"EMEA"` (text constant) → resolves to text literal

**Cross-sheet:**
- `SUM(RegionData)` where `RegionData` = `Lookup!B:B` → classifies correctly with cross-sheet range

**Unknown / missing:**
- Unknown name (not in `defined_names`) → `Unsupported(NamedRange)` (preserved)
- Empty name string → `Unsupported(NamedRange)`

**Nested:**
- `IF(A2>0, SalesData, 0)` where `SalesData` resolves to a range → `Mixed` or appropriate
- `SUMIF(SalesData, "EMEA", AmountData)` where both are named ranges → both resolve

**Edge cases:**
- Named range pointing to another sheet that doesn't exist in workbook → appropriate error
- Named range with `$` absolute refs (`Sheet1!$A$1:$A$100`) → `$` stripped during resolution
- Named range value with spaces (e.g., `'Sheet Name'!A:A`) → sheet name with spaces handled
- Circular named range (name references itself) → should not infinite loop, return error
- Named range used inside VLOOKUP table_array arg → classifies as Lookup

### Evaluation (integration tests)

- Workbook with 3 named ranges (range, cell, constant) + formulas referencing each → correct output values
- Workbook with named range pointing to lookup sheet + `VLOOKUP(A2, MyLookup, 2, FALSE)` → correct lookup
- Workbook with named range in aggregate: `SUMIF(RegionCol, "EMEA", AmountCol)` where `RegionCol` and `AmountCol` are named → correct sum
- Workbook with no named ranges + formulas using explicit refs → unchanged behavior (regression guard)

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "Named range support" to next release section |
| `docs/functions.md` | Update named range status |
| `docs/architecture/streaming-model.md` | Change "Unsupported (v0.2 candidate)" → "Supported" |
| `docs/roadmap/v0.2/README.md` | Tick the checkbox |

## Streaming invariant

Not violated. Resolution happens at classification time, before evaluation. Resolved references go through the same prelude/row-eval paths as explicit references.
