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

- [ ] `extract_references(ast: &Ast) -> References` walks the AST and returns:
  - [ ] All `CellRef`s.
  - [ ] All `RangeRef`s.
  - [ ] All `SheetRef`s.
  - [ ] All function names called (for classification).
- [ ] `References` uses `SmallVec<[T; N]>` with N sized to the P99 case.
- [ ] Tests cover: single cell, range, cross-sheet, whole-column, nested functions.

### Classification

- [ ] Implement `classify(ast, ctx) -> Classification`:
  - [ ] `RowLocal` — references only current-row cells; no function in `UNSUPPORTED` or `REQUIRES_PRELUDE` sets.
  - [ ] `AggregateOnly` — root is a supported aggregate function; all range args are whole-column or fixed.
  - [ ] `LookupOnly` — supported lookup function with cross-sheet range into a lookup sheet.
  - [ ] `Mixed` — recurses; ok if every sub-expression classifies.
  - [ ] `Unsupported(UnsupportedReason)` — with a specific reason.
- [ ] `UnsupportedReason` enum: `ForwardRowRef`, `CircularRef`, `UnsupportedFunction(String)`, `UnboundedRange`, `NonStaticCriteria`, `DynamicArray`, `VolatileUnsupported`, `TableReference`, `NamedRange`, `ExternalReference`, etc.
- [ ] Support-set constants:
  - [ ] `UNSUPPORTED_FUNCTIONS`: `OFFSET`, `INDIRECT`, `FILTER`, `UNIQUE`, `SORT`, `SORTBY`, `SEQUENCE`, `RANDARRAY`, `LAMBDA`, `LET`, `HYPERLINK` (as function), `WEBSERVICE`, `CUBEVALUE`, ...
  - [ ] `AGGREGATE_FUNCTIONS`: `SUM`, `COUNT`, `COUNTA`, `AVERAGE`, `MIN`, `MAX`, `PRODUCT`, `SUMIF`, `COUNTIF`, `AVERAGEIF`, `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`, `MINIFS`, `MAXIFS`, `MEDIAN`.
  - [ ] `LOOKUP_FUNCTIONS`: `VLOOKUP`, `HLOOKUP`, `XLOOKUP`, `MATCH`, `XMATCH`, `INDEX`, `CHOOSE`. Pure Excel only — no custom extensions.

### AST rewrite

- [ ] After classification, rewrite the AST to replace supported aggregate/lookup sub-expressions with `PreludeRef(key)` nodes.
- [ ] Add `AstNode::PreludeRef(PreludeKey)` variant.
- [ ] `PreludeKey` encodes aggregate vs lookup plus parameters.
- [ ] Rewriting is a pure function; golden tests for input → rewritten AST.

### Tests

- [ ] 30+ classification tests covering:
  - [ ] Simple arithmetic — RowLocal.
  - [ ] `SUM(A:A)` — AggregateOnly.
  - [ ] `VLOOKUP(A, Sheet2!A:C, 2, FALSE)` — LookupOnly.
  - [ ] `Deal Value / SUM(Deal Value:Deal Value)` — Mixed.
  - [ ] `OFFSET(A1, 1, 0)` — Unsupported.
  - [ ] `INDIRECT("A1")` — Unsupported.
  - [ ] `A3` where current row is 5 — Unsupported (forward ref from past row).
  - [ ] `FILTER(A:A, B:B>0)` — Unsupported (dynamic array).
  - [ ] Circular reference — Unsupported.
  - [ ] `VLOOKUP(A&B, Sheet2!D:E, 2, FALSE)` — LookupOnly, concatenated-key via helper column on lookup sheet.
  - [ ] Nested functions mixing supported + unsupported → Unsupported.
  - [ ] `Table1[Column]` — Unsupported (table reference).
  - [ ] `MyNamedRange` — Unsupported (named range).
  - [ ] `[Book2.xlsx]Sheet1!A1` — Unsupported (external reference).
- [ ] Reference-extraction tests for each variant.
- [ ] AST rewrite golden tests.

### Error messages

- [ ] Every `Unsupported` path produces a user-facing message that:
  - [ ] Quotes the formula text.
  - [ ] Names the specific reason.
  - [ ] Includes a doc link (placeholder URL for v0.1; real links land with the docs site in Phase 13/14).
- [ ] Tests assert on message substrings.

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
