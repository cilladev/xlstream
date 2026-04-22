# xlstream-parse

Formula parser adapter. Wraps `formualizer-parse` and adds a classification layer that labels each AST with a streaming verdict.

## What it owns

- **`parse(text) -> Result<Ast>`** -- parse formula text into an AST
- **`classify(ast, context) -> Classification`** -- assign streaming verdict
- **`Classification`** -- `RowLocal`, `AggregateOnly`, `LookupOnly`, `Mixed`, `Unsupported(reason)`
- **`UnsupportedReason`** -- specific rejection reasons with `doc_link()` for user-facing errors
- **`ClassificationContext`** -- inputs for classification (main sheet, lookup sheets, etc.)
- **`extract_references(ast) -> References`** -- find all cell/range/sheet references in a formula
- **`rewrite`** -- AST transformations: aggregate-to-prelude-ref, lookup key extraction, named range resolution
- **`AggregateKey`**, **`LookupKey`**, **`BoundedRangeKey`** -- prelude planning keys

## What it does NOT own

- Evaluation of formulas (that's `xlstream-eval`).
- I/O (that's `xlstream-io`).

## Dependencies

`formualizer-parse`, `formualizer-common`, `xlstream-core`, `smallvec`, `phf`.

## Why this layer exists

Decouples the rest of the codebase from the upstream parser. If we ever fork or swap the parser, this is the only crate that changes.
