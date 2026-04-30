# Feature: SUMPRODUCT

**Branch:** `feat/sumproduct`
**Effort:** ~1 day
**Crates:** xlstream-parse, xlstream-eval

## What

`SUMPRODUCT` returns the sum of element-wise products of two or more arrays. With a single array, it sums the array. The arrays must be bounded ranges (not whole-column refs).

```
=SUMPRODUCT(A1:A3, B1:B3)  → A1*B1 + A2*B2 + A3*B3
=SUMPRODUCT(A1:A5)         → A1 + A2 + A3 + A4 + A5
=SUMPRODUCT((A1:A3>0)*B1:B3)  → conditional sum (common Excel idiom)
```

Current behavior: `Classification::Unsupported` — the function is not in `UNSUPPORTED_FUNCTIONS` but has no dispatch entry, so it falls through to `#VALUE!` at eval time.

## What already exists

- `BoundedRangeKey` struct in `crates/xlstream-eval/src/prelude.rs:104-134` — caches bounded single-column ranges during prelude
- `get_cached_range()` in `crates/xlstream-eval/src/prelude.rs:188-190` — retrieves cached range data
- `collect_bounded_range_keys()` in `crates/xlstream-eval/src/prelude_plan.rs:509-561` — walks AST to find bounded ranges
- `expand_range()` in `crates/xlstream-eval/src/builtins/mod.rs:36-75` — resolves `RangeRef` nodes to `Vec<Value>` from lookup sheets or cached ranges
- `RANGE_EXPANDING_FUNCTIONS` set in `crates/xlstream-parse/src/sets.rs:138-141` — functions whose range args get cached during prelude
- SUMPRODUCT is already in `AGGREGATE_FUNCTIONS` set in `crates/xlstream-parse/src/sets.rs:41`

## Where to look

- `crates/xlstream-parse/src/sets.rs:138-141` — add SUMPRODUCT to `RANGE_EXPANDING_FUNCTIONS`
- `crates/xlstream-parse/src/classify.rs` — verify classification as Aggregate or RowLocal with range args
- `crates/xlstream-eval/src/builtins/mod.rs:83-203` — dispatch match block, add SUMPRODUCT arm
- `crates/xlstream-eval/src/builtins/mod.rs:36-75` — `expand_range()` for resolving bounded ranges to values
- `crates/xlstream-eval/src/prelude_plan.rs:509-561` — `collect_bounded_range_keys()` for prelude caching
- `crates/xlstream-eval/src/builtins/aggregate.rs` — existing aggregate builtins for pattern reference
- `crates/xlstream-eval/src/builtins/multi_conditional.rs` — alternative pattern with range expansion

## Resolution / Evaluation behavior

SUMPRODUCT operates on bounded ranges that are known at classification time. The ranges must be cached during prelude (pass 1) so they're available during streaming (pass 2).

**Classification:** SUMPRODUCT's args are bounded range refs. It classifies as an aggregate function (already in `AGGREGATE_FUNCTIONS`). Its range args need bounded-range caching, so it must also be in `RANGE_EXPANDING_FUNCTIONS`.

**Prelude:** `collect_bounded_range_keys()` extracts `BoundedRangeKey` entries for each range argument. The prelude pass reads and caches those ranges.

**Row eval:** `expand_range()` retrieves each cached range as `Vec<Value>`. The builtin multiplies corresponding elements pairwise and sums the products. If ranges have different lengths, return `#VALUE!`.

Does not violate the streaming invariant — all range data is loaded during prelude.

## Tests

### Classification (unit tests)

**Happy path:**
- `SUMPRODUCT(A1:A10, B1:B10)` — two bounded ranges, classifies as streamable
- `SUMPRODUCT(A1:A5)` — single range, classifies as streamable
- `SUMPRODUCT(A1:A3, B1:B3, C1:C3)` — three ranges, classifies as streamable

**Edge cases:**
- `sumproduct(a1:a5, b1:b5)` — case insensitive function name
- `SUMPRODUCT(A:A, B:B)` — whole-column refs; should classify as aggregate (unbounded range handling)
- `SUMPRODUCT(Sheet2!A1:A5, B1:B5)` — cross-sheet range in first arg
- `IF(D2, SUMPRODUCT(A1:A3, B1:B3), 0)` — nested inside IF
- `SUMPRODUCT()` — no args, should produce `#VALUE!`
- `SUMPRODUCT(A1:A3, B1:B5)` — mismatched range sizes, should produce `#VALUE!`

**Regression guards:**
- Existing aggregate functions (SUM, SUMIF, SUMIFS, etc.) must not change behavior
- Existing bounded-range functions (IRR, NPV, CONCAT, NETWORKDAYS) must not change behavior

### Evaluation (integration tests)

- Two equal-length ranges with numeric values — verify sum of pairwise products
- Single range — verify simple sum
- Three ranges — verify element-wise triple product then sum
- Range containing `#N/A` — verify error propagation
- Range with boolean values — verify TRUE=1, FALSE=0 coercion
- Mismatched range lengths — verify `#VALUE!`
- Empty cells in range — verify treated as 0

### Conformance fixture

Create `tests/fixtures/aggregate/sumproduct.xlsx` with data + SUMPRODUCT formulas. Generate with openpyxl, recalculate with LibreOffice headless. Add `#[test] fn sumproduct()` in `conformance/aggregate.rs`.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add SUMPRODUCT entry under `[Unreleased]` |
| `docs/functions.md` | Tick SUMPRODUCT as implemented |
| `docs/roadmap/v0.2/README.md` | Tick the checkbox |

## Streaming invariant

Does not violate. All range data is loaded during prelude pass 1. Row evaluation reads from cached prelude data only.
