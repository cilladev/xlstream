# Phase 2 — Parser integration

**Goal:** real formula parsing via `formualizer-parse`, with our classification layer on top.

**Estimated effort:** 2–3 days.

**Prerequisites:** Phase 1 complete.

**Reading:** [`docs/architecture/parse-reuse.md`](../architecture/parse-reuse.md), [`docs/architecture/streaming-model.md`](../architecture/streaming-model.md).

**Output:** `xlstream-parse::parse(expr)` returns a real AST; `classify(ast, ctx)` returns a valid `Classification` for every formula shape in our corpus.

## Checklist

### Parser

- [x] Integrate `formualizer-parse`: call its parser, map its AST to our `Ast` type.
  - [x] Define our `Ast` as either a re-export of the upstream type or a thin wrapper. Keep wrapper if we'll extend with extra fields (source spans, rewritten nodes).
- [x] `parse(expr: &str)` returns `Result<Ast, XlStreamError>` with `FormulaParse` variant on error.
- [x] Include source-error context: line/column if available from upstream, otherwise the offending substring.
- [x] Rustdoc + doctest.

### Reference extraction

- [x] `extract_references(ast: &Ast) -> References` walks the AST and returns:
  - [x] All `CellRef`s.
  - [x] All `RangeRef`s.
  - [x] All `SheetRef`s.
  - [x] All function names called (for classification).
- [x] `References` uses `SmallVec<[T; N]>` with N sized to the P99 case.
- [x] Tests cover: single cell, range, cross-sheet, whole-column, nested functions.

### Classification

- [x] Implement `classify(ast, ctx) -> Classification`:
  - [x] `RowLocal` — references only current-row cells; no function in `UNSUPPORTED` or `REQUIRES_PRELUDE` sets.
  - [x] `AggregateOnly` — root is a supported aggregate function; all range args are whole-column or fixed.
  - [x] `LookupOnly` — supported lookup function with cross-sheet range into a lookup sheet.
  - [x] `Mixed` — recurses; ok if every sub-expression classifies.
  - [x] `Unsupported(UnsupportedReason)` — with a specific reason.
- [x] `UnsupportedReason` enum: `NonCurrentRowRef`, `CircularRef`, `UnsupportedFunction(String)`, `UnboundedRange`, `NonStaticCriteria`, `DynamicArray`, `VolatileUnsupported`, `TableReference`, `NamedRange`, `ExternalReference`, `NestedUnsupported`, `LookupSheetNotPrepared`, `LookupIntoStreamingSheet`.
- [x] Support-set constants:
  - [x] `UNSUPPORTED_FUNCTIONS`: `OFFSET`, `INDIRECT`, `FILTER`, `UNIQUE`, `SORT`, `SORTBY`, `SEQUENCE`, `RANDARRAY`, `LAMBDA`, `LET`, `HYPERLINK`, `WEBSERVICE`, `CUBEVALUE`, `CUBEMEMBER`, `CUBESET`, `RAND`, `RANDBETWEEN`.
  - [x] `AGGREGATE_FUNCTIONS`: `SUM`, `COUNT`, `COUNTA`, `AVERAGE`, `MIN`, `MAX`, `PRODUCT`, `SUMIF`, `COUNTIF`, `AVERAGEIF`, `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`, `MINIFS`, `MAXIFS`, `MEDIAN`.
  - [x] `LOOKUP_FUNCTIONS`: `VLOOKUP`, `HLOOKUP`, `XLOOKUP`, `MATCH`, `XMATCH`, `INDEX`, `CHOOSE`.
  - [x] `VOLATILE_STREAMING_OK`: `TODAY`, `NOW`.

### AST rewrite

- [x] After classification, rewrite the AST to replace supported aggregate/lookup sub-expressions with `PreludeRef(key)` nodes.
- [x] Add `AstNode::PreludeRef(PreludeKey)` variant.
- [x] `PreludeKey` encodes aggregate vs lookup plus parameters.
- [x] Rewriting is a pure function; golden tests for input → rewritten AST.

### Tests

- [x] 30+ classification tests covering:
  - [x] Simple arithmetic — RowLocal.
  - [x] `SUM(A:A)` — AggregateOnly.
  - [x] `VLOOKUP(A, Sheet2!A:C, 2, FALSE)` — LookupOnly.
  - [x] `Deal Value / SUM(Deal Value:Deal Value)` — Mixed.
  - [x] `OFFSET(A1, 1, 0)` — Unsupported.
  - [x] `INDIRECT("A1")` — Unsupported.
  - [x] `A3` where current row is 5 — Unsupported (non-current-row ref).
  - [x] `FILTER(A:A, B:B>0)` — Unsupported (dynamic array).
  - [x] Circular reference — Unsupported.
  - [x] `VLOOKUP(A&B, Sheet2!D:E, 2, FALSE)` — LookupOnly, concatenated-key via helper column on lookup sheet.
  - [x] Nested functions mixing supported + unsupported → Unsupported.
  - [x] `Table1[Column]` — Unsupported (table reference).
  - [x] `MyNamedRange` — Unsupported (named range).
  - [x] `[Book2.xlsx]Sheet1!A1` — Unsupported (external reference).
- [x] Reference-extraction tests for each variant.
- [x] AST rewrite golden tests.

### Error messages

- [x] Every `Unsupported` path produces a user-facing message that:
  - [x] Quotes the formula text.
  - [x] Names the specific reason.
  - [x] Includes a doc link (placeholder URL for v0.1; real links land with the docs site in Phase 13/14).
- [x] Tests assert on message substrings.

### CLI integration

- [ ] `xlstream-cli classify "FORMULA"` parses and classifies a formula string passed as argument.
- [ ] Useful smoke test for development: "what would xlstream do with this formula?"
- [ ] File-level classification (`classify path.xlsx`) defers to Phase 3 when I/O is wired up.

## Verification

```bash
cargo test -p xlstream-parse --all-features
cargo test --doc -p xlstream-parse
cargo run -p xlstream-cli -- classify "SUM(A:A)"
cargo run -p xlstream-cli -- classify "VLOOKUP(A1, Sheet2!A:C, 2, FALSE)"
cargo run -p xlstream-cli -- classify "OFFSET(A1, 1, 0)"
```

Each `classify` invocation should print the formula text + classification verdict.

## Done when

All classification tests pass (30+, including table/named-range/external refusals). Reference extraction covers every AST shape. CLI `classify` works on formula strings.
