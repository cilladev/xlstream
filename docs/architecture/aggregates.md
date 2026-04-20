# Aggregates

Aggregates reduce whole columns to scalars during prelude.

## Supported aggregate functions

| Function | Reads | Returns |
|---|---|---|
| `SUM` | column / range | sum |
| `COUNT` | column / range | count of numerics |
| `COUNTA` | column / range | count of non-empty |
| `COUNTBLANK` | column / range | count of empty |
| `AVERAGE` | column / range | mean |
| `MIN` | column / range | min |
| `MAX` | column / range | max |
| `PRODUCT` | column / range | product |
| `SUMIF` | column / range, condition | sum where condition matches |
| `COUNTIF` | column / range, condition | count where condition matches |
| `AVERAGEIF` | column / range, condition | average where condition matches |
| `SUMIFS` | sum range, (cond range, criterion)+ | multi-criteria sum |
| `COUNTIFS` | (cond range, criterion)+ | multi-criteria count |
| `AVERAGEIFS` | avg range, (cond range, criterion)+ | multi-criteria avg |

The `IF` variants support Excel's wildcard syntax (`*`, `?`) and comparison operators (`">10"`, `"<>abc"`).

## Classification

A sub-expression is an **aggregate-only reference** if:
- Its root is a supported aggregate function.
- Every `RangeRef` argument is a whole-column or bounded range in the main sheet or a lookup sheet.
- Criteria arguments (for `*IF*` variants) evaluate to constants at classification time (i.e., not per-row).

A mixed formula (`Deal Value / SUM(Deal Value:Deal Value)`) is supported because each sub-expression is supported — per-row `Deal Value` + aggregate `SUM(col)`.

## Prelude: computing aggregates

### Single-column aggregates

Group all aggregates that read the same column into one pass:

```
for col in columns_needing_aggregates:
    let needs = [Sum, Count, Min, Max]; // example
    let mut sum = 0.0;
    let mut count = 0;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for cell in reader.stream_column(col):
        match cell:
            Number(n) | Integer(n as f64) => {
                sum += n; count += 1;
                min = min.min(n); max = max.max(n);
            },
            Bool(b) => { sum += if b { 1.0 } else { 0.0 }; count += 1; ... },
            _ => { /* skip */ },
    prelude.scalars.insert(("SUM", col), Value::Number(sum));
    prelude.scalars.insert(("COUNT", col), Value::Number(count as f64));
    // ...
```

### Multi-column aggregates (`SUMIFS` etc.)

Group by (sum_column, cond_column1, cond_column2, ...) tuple. Pass all needed columns in a single streaming read, evaluate criteria per row, accumulate.

### `*IF` with criteria

Criteria are Excel mini-expressions: `">100"`, `"=EMEA"`, `"<>blank"`, `"apple*"`. We parse criteria strings at prelude time into a predicate `fn(&Value) -> bool`. Predicates are compiled once per aggregate, reused per cell.

```rust
pub enum Criteria {
    Equals(Value),
    NotEquals(Value),
    Greater(f64),
    GreaterOrEq(f64),
    Less(f64),
    LessOrEq(f64),
    Wildcard(WildcardPattern),
    NonBlank,
    Blank,
}

impl Criteria {
    pub fn matches(&self, v: &Value) -> bool { ... }
}
```

## Edge cases

- **Empty column** — `SUM` returns 0, `AVERAGE` returns `#DIV/0!`, `MIN`/`MAX` return 0 per Excel.
- **Mixed types** — strings that parse as numbers are included in numeric aggregates (per Excel). Strings that don't are skipped.
- **Error in source cell** — `SUM` of a range containing an error returns that error. Matches Excel.
- **Whole-column with millions of empty rows** — calamine only yields used cells; empty rows cost nothing.

## Shared-pass scheduling

Given a set of aggregates to compute, the prelude builds a minimal set of column reads:

```
plan:
  - columns [A, B, C]: accumulators = [SUM(A), COUNT(A), SUMIFS(A where B=x), SUMIFS(A where C=y)]
  - column D: accumulators = [AVERAGE(D), MIN(D)]
```

Each plan step is one streaming pass over the specified columns. For the 400k reference workbook this is 2–5 passes, each a few hundred milliseconds.

Parallelism: plan steps are independent and run on separate rayon workers.

## Prelude storage

See `xlstream-eval/src/prelude.rs` for the canonical definition.

```rust
pub struct Prelude {
    aggregates: HashMap<AggregateKey, Value>,
    conditional_aggregates: HashMap<ConditionalAggKey, HashMap<String, Value>>,
    multi_conditional_aggregates: HashMap<MultiConditionalAggKey, HashMap<String, Value>>,
    lookup_sheets: HashMap<String, LookupSheet>,
    volatile: Option<VolatileData>,  // TODAY, NOW, RAND — single values
    cached_ranges: HashMap<BoundedRangeKey, Vec<Value>>,
}
```

The evaluator queries `prelude.aggregates` (or conditional/multi-conditional variants) by key at row eval time — `O(1)` hashmap hit.

## Interaction with the evaluator

Aggregate function calls in the AST are **rewritten** during classification. `SUM(Deal Value:Deal Value)` becomes `AstNode::PreludeRef(AggKey::Whole { col: ..., kind: Sum })`. The evaluator never re-walks the column; it just dereferences the prelude.

## Forbidden: aggregates over the main sheet row-by-row reference

`=SUM(Deal Value:Deal Value) + Deal Value` is supported (aggregate scalar + row-local cell).

`=SUM(Deal Value2:Deal Value10) + Deal Value` is supported only if the bounded range is closed at prelude time. Bounded ranges where start/end can't be statically resolved (e.g., `A1:INDEX(...)`) are refused.

`=SUM(OFFSET(A1, 0, 0, 10, 1))` — `OFFSET` is unsupported entirely. Refused.

## Tests

Every aggregate has tests for:
- Happy path numeric column.
- Mixed types (include bools, skip text).
- Empty column.
- Column with errors (propagate).
- Criteria variants (`*IF`) with wildcards, comparison operators, blank matching.
- Large-column perf smoke test (< 500 ms on 400k rows).
