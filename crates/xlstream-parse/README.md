# xlstream-parse

[![Crates.io](https://img.shields.io/crates/v/xlstream-parse.svg)](https://crates.io/crates/xlstream-parse)
[![docs.rs](https://docs.rs/xlstream-parse/badge.svg)](https://docs.rs/xlstream-parse)

Formula parser and streaming classifier for the [xlstream](https://github.com/cilladev/xlstream) evaluator. Wraps [`formualizer-parse`](https://crates.io/crates/formualizer-parse) and adds a classification layer that labels each AST with a streaming verdict.

Use this crate directly if you need to analyze formula structure without evaluating. For evaluation, depend on [`xlstream-eval`](https://crates.io/crates/xlstream-eval).

## What it provides

- **`parse(text) -> Result<Ast>`** -- parse formula text into an AST
- **`classify(ast, context) -> Classification`** -- assign streaming verdict: `RowLocal`, `AggregateOnly`, `LookupOnly`, `Mixed`, `Unsupported(reason)`
- **`extract_references(ast) -> References`** -- find all cell/range/sheet/table references
- **`resolve_named_ranges(ast, names) -> Ast`** -- rewrite named range refs to resolved cell/range refs
- **`rewrite(ast, ctx, verdict) -> Ast`** -- aggregate-to-prelude-ref, lookup key extraction
- **`AggregateKey`**, **`LookupKey`**, **`BoundedRangeKey`** -- prelude planning keys

## Example

```rust
use xlstream_parse::{parse, classify, ClassificationContext};

let ast = parse("SUM(A:A)").unwrap();
let ctx = ClassificationContext::for_cell("Sheet1", 2, 1);
let verdict = classify(&ast, &ctx);
// verdict == Classification::AggregateOnly
```

## Dependencies

`formualizer-parse`, `formualizer-common`, `xlstream-core`, `smallvec`, `phf`.

## License

Dual-licensed under Apache-2.0 or MIT, at your option.
