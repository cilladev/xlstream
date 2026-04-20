# Parse reuse

`formualizer-parse` is our formula parser. We wrap it behind a thin adapter, `xlstream-parse`, so we can swap later if needed.

## Why reuse

Writing a correct formula parser is weeks of work and endless edge cases:
- `_xlfn.` and `_xlfn._xlws.` prefixes for modern functions (XLOOKUP, IFS, etc.).
- Cross-sheet references with quoted names: `'Tax Rates'!A:B`.
- Whole-column references: `A:A`, `$A:$A`.
- Array constants: `{1,2;3,4}`.
- Text-inside-string handling.
- Tokenizer/parser handoff with the correct precedence table.

`formualizer-parse` has done all this. It's a dependency of `formualizer` and exposed as its own crate on crates.io.

## What we reuse

- `formualizer_parse::tokenizer::Tokenizer`.
- `formualizer_parse::parser::Parser` (or the newer `BatchParser` if available).
- `formualizer_parse::ast::Ast` and node enum.

## What we wrap

Our `xlstream-parse` adapter:

```rust
pub fn parse(text: &str) -> Result<Ast, FormulaParseError> { ... }

pub struct References {
    pub cells: SmallVec<[CellRef; 8]>,
    pub ranges: SmallVec<[RangeRef; 4]>,
    pub sheets: SmallVec<[SheetRef; 2]>,
    pub functions: SmallVec<[FnRef; 8]>,
}

pub fn extract_references(ast: &Ast) -> References { ... }

pub enum Classification {
    RowLocal,
    AggregateOnly,
    LookupOnly,
    Mixed,
    Unsupported(UnsupportedReason),
}
// UnsupportedReason exposes a doc_link() -> &'static str method.

pub fn classify(ast: &Ast, ctx: &ClassificationContext) -> Classification { ... }
```

`ClassificationContext` contains:
- The main sheet's column headers → column indices.
- The set of known lookup sheets (determined in the pass-0 scan).
- The current cell's address (for circular-ref detection).
- A whitelist of supported functions.

## AST rewriting

After classification, we rewrite the AST to replace supported aggregate/lookup sub-expressions with `PreludeRef` nodes:

```
original:
   =Deal Value / SUM(Deal Value:Deal Value) * 100
   └─ BinaryOp(*)
      ├─ BinaryOp(/)
      │  ├─ CellRef(Deal Value, current_row)
      │  └─ FunctionCall(SUM, [RangeRef(Deal Value:Deal Value)])
      └─ Number(100)

rewritten:
   └─ BinaryOp(*)
      ├─ BinaryOp(/)
      │  ├─ CellRef(Deal Value, current_row)
      │  └─ PreludeRef(AggregateKey { kind: AggKind::Sum, col="Deal Value" })
      └─ Number(100)
```

The rewritten AST is cached by formula text. 4M evaluations of the same formula use the same cached rewritten AST.

## Why a rewrite

- The evaluator can dispatch `PreludeRef` via an immediate hashmap lookup — no re-walk.
- Separation of concerns: classification decides "what goes where," the evaluator just executes.
- Easier to test: rewriting is a pure function, assertable by golden AST output.

## Swapping the parser

If `formualizer-parse` becomes unmaintained or diverges:

1. Swap `formualizer-parse` dependency in `xlstream-parse/Cargo.toml`.
2. Update the adapter functions.
3. The rest of the codebase doesn't change.

Good practice: keep adapter functions pure — no `formualizer_parse` types in the public signatures of `xlstream-parse`. We lean on a mapped internal AST.

## Known friction

- `formualizer-parse`'s AST types may change across versions. Pin to an exact version (`= "0.4.3"`, not `"^0.4.3"`) until we know upstream's stability policy.
- Our internal AST may eventually want information `formualizer-parse` doesn't expose (e.g., source-position spans). We add it ourselves in the adapter.

## Tests

Classification has its own test file with ~30 cases covering every shape:
```rust
#[test] fn row_local_arithmetic() { assert!(matches!(classify("A * 2"), Classification::RowLocal)); }
#[test] fn aggregate_whole_column() { ... }
#[test] fn vlookup_classified() { ... }
#[test] fn offset_refused() {
    assert!(matches!(
        classify("OFFSET(A1, 1, 0)"),
        Classification::Unsupported(_)
    ));
}
// ...
```
