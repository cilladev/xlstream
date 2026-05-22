# Feature: Centralize Formula Registration (v2 — single registry)

**Branch:** `feat/formula-registry-v2`
**Effort:** ~3-4 days
**Crates:** xlstream-parse, xlstream-eval

## What

Replace 9 scattered function-name registration sites with a single centralized registry. One `define_functions!` macro invocation in xlstream-eval defines every function's metadata AND handler. Parse receives lookups via callback — it never imports eval.

```
Before: adding a function = 5+ file edits across 2 crates, forgetting one = silent bug (#140)
After:  adding a function = 1 impl + 1 registry entry, forgetting handler = won't compile
```

Current behavior: function identity is scattered across `sets.rs` (8 phf_sets), `classify.rs` (branching), `rewrite.rs` (`agg_kind_for()` + `collect_lookup_keys()`), `builtins/mod.rs` (300+ match arms), and `prelude_plan.rs` (`collect_multi_conditional_keys()`). See `docs/roadmap/v0.4/01-formula-registry-cleanup.md` for the full 9-site problem statement.

## What already exists

- `crates/xlstream-parse/src/sets.rs` — 8 `phf_set!` tables for function categories (aggregate, lookup, range-expanding, unsupported, volatile, range-metadata). Public `is_*()` query functions.
- `crates/xlstream-parse/src/classify.rs` — `classify()` walks ASTs, delegates to `sets::is_*()` to route functions. `is_criteria_arg()` maps 8 function names to criteria arg positions.
- `crates/xlstream-parse/src/rewrite.rs` — `AggKind` enum (Sum, Count, ..., Median). `agg_kind_for()` maps 9 function names to AggKind. `collect_lookup_keys()` matches on 6 lookup function names.
- `crates/xlstream-eval/src/builtins/mod.rs` — `dispatch()` with ~200 match arms on function name strings. `eval_args()`, `expand_range()`, `expand_args_for_aggregate()` shared helpers.
- `crates/xlstream-eval/src/prelude_plan.rs` — `collect_multi_conditional_keys()` matches on 8 conditional aggregate names. `FoldState` for prelude aggregation.
- `crates/xlstream-eval/src/builtins/subtotal.rs` — numeric dispatch (1-11 maps to sub-functions).
- `phf` crate already in xlstream-parse's dependencies.
- `bitflags` crate NOT currently a dependency (must add).
- 225 functions currently implemented across v0.1-v0.3.

## Where to look

**Parse crate — types and classification:**
- `crates/xlstream-parse/src/sets.rs` — all 8 phf_set definitions and `is_*()` functions (entire file, 313 lines)
- `crates/xlstream-parse/src/classify.rs:344` — `pub fn classify()` entry point
- `crates/xlstream-parse/src/classify.rs:453-481` — `classify_function()` — the routing cascade that reads sets.rs
- `crates/xlstream-parse/src/classify.rs:513-519` — `is_criteria_arg()` — known exception, stays
- `crates/xlstream-parse/src/rewrite.rs:19-38` — `AggKind` enum definition
- `crates/xlstream-parse/src/rewrite.rs:213-226` — `agg_kind_for()` — 9-arm string match to replace
- `crates/xlstream-parse/src/rewrite.rs:339-413` — `collect_lookup_keys()` — 6 lookup name matches to replace
- `crates/xlstream-parse/src/rewrite.rs:164-205` — `rewrite_node()` — calls `sets::is_aggregate()` and `sets::is_lookup()`
- `crates/xlstream-parse/src/lib.rs:30` — `pub mod sets;` declaration
- `crates/xlstream-parse/src/lib.rs:33-42` — public re-exports from classify, rewrite
- `crates/xlstream-parse/Cargo.toml` — dependencies (phf present, bitflags absent)

**Eval crate — dispatch and prelude:**
- `crates/xlstream-eval/src/builtins/mod.rs:92-126` — `expand_args_for_aggregate()` and `agg()` helper
- `crates/xlstream-eval/src/builtins/mod.rs:132-369` — the 200-arm dispatch match to replace
- `crates/xlstream-eval/src/builtins/mod.rs:372-729` — handler functions (SUMPRODUCT, VAR.S, etc.) already with correct signature
- `crates/xlstream-eval/src/prelude_plan.rs:260-346` — `collect_multi_conditional_keys()` — known exception, stays
- `crates/xlstream-eval/src/prelude_plan.rs:275` — `_XLFN.` strip in prelude
- `crates/xlstream-eval/src/evaluate.rs:1119` — production caller of `classify()`
- `crates/xlstream-eval/src/evaluate.rs:1169` — second production caller of `classify()`
- `crates/xlstream-eval/src/evaluate.rs:1290` — `strip_xlfn_prefix()` on raw formula text — different layer, stays
- `crates/xlstream-eval/src/builtins/subtotal.rs` — numeric dispatch (1-11), stays

**Test files that call classify() or rewrite() (must add callback param):**
- `crates/xlstream-parse/src/classify.rs` — 66 internal test calls
- `crates/xlstream-parse/tests/classify_aggregate.rs` — 8 calls
- `crates/xlstream-parse/tests/classify_lookup.rs` — 5 calls
- `crates/xlstream-parse/tests/classify_row_local.rs` — 7 calls
- `crates/xlstream-parse/tests/classify_unsupported.rs` — 12 calls
- `crates/xlstream-parse/tests/classify_streaming_sheet.rs` — 4 calls
- `crates/xlstream-parse/tests/classify_mixed.rs` — 4 calls
- `crates/xlstream-parse/tests/rewrite.rs` — 9 classify + 9 rewrite + 5 collect_lookup_keys calls

## Architecture

### Dependency direction

```
xlstream-core  (Value, CellError — no xlstream deps)
      ^
xlstream-parse (FnCaps, FnCategory, FunctionMeta — pure types)
      ^         classify(), rewrite() — take fn_lookup callback
      |
xlstream-eval  (registry.rs — define_functions! macro — THE source of truth)
               (225 entries, each with metadata + handler)
               (passes registry::lookup_meta to classify/rewrite)
```

Parse never imports eval. Parse asks questions via callback. Eval has the answers.

### Data flow for classification

```
evaluate.rs (eval)
  |  calls classify(&ast, &ctx, &registry::lookup_meta)
  v
classify.rs (parse)
  |  calls fn_lookup("SUM")
  v
registry.rs (eval)
  |  returns &FunctionMeta { caps: PURE|AGG_COERCE|..., category: Aggregate, agg_kind: Some(Sum) }
  v
classify.rs (parse)
  |  reads caps, routes to classify_aggregate()
  v
returns Classification::AggregateOnly
```

### Data flow for dispatch

```
interp.rs (eval)
  |  calls registry::dispatch("SUM", args, interp, scope)
  v
registry.rs (eval)
  |  lookup("SUM") -> FunctionEntry { meta, handler: handle_sum }
  |  calls (entry.handler)(args, interp, scope)
  v
builtins/mod.rs (eval)
  |  handle_sum() -> expand_args_for_aggregate() -> aggregate::sum()
  v
returns Value::Number(42.0)
```

### Types in parse (pure data, no functions)

```rust
// xlstream-parse/src/function_meta.rs

bitflags! {
    pub struct FnCaps: u8 {
        const PURE           = 0b0000_0001;  // deterministic, no side effects
        const SHORT_CIRCUIT  = 0b0000_0010;  // lazy arg eval (IF, AND, OR)
        const RANGE_EXPAND   = 0b0000_0100;  // expand range refs before call
        const AGG_COERCE     = 0b0000_1000;  // coerce scalar bools/text in agg context
        const NEEDS_PRELUDE  = 0b0001_0000;  // requires pass-1 data
        const LOOKUP         = 0b0010_0000;  // uses prelude-loaded index
        const VOLATILE       = 0b0100_0000;  // changes between runs (TODAY, NOW)
        const RANGE_METADATA = 0b1000_0000;  // range args inspected for metadata only
    }
}

pub enum FnCategory {
    Math, String, Date, Lookup, Aggregate, Conditional,
    Info, Financial, Statistical, Engineering, Database, Compatibility,
}

pub struct FunctionMeta {
    pub name: &'static str,
    pub caps: FnCaps,
    pub category: FnCategory,
    pub agg_kind: Option<AggKind>,
}
```

### Registry in eval (THE source of truth)

```rust
// xlstream-eval/src/registry.rs

define_functions! {
    "SUM" => {
        category: Aggregate,
        caps: PURE | RANGE_EXPAND | AGG_COERCE | NEEDS_PRELUDE,
        agg_kind: Sum,
        handler: handle_sum,
    },
    "IF" => {
        category: Conditional,
        caps: SHORT_CIRCUIT,
        handler: conditional::builtin_if,
    },
    "SQRT" => {
        category: Math,
        caps: PURE,
        handler: handle_sqrt,
    },
    // ... all 225 entries
}
```

The `define_functions!` macro generates:
- `static ENTRIES: &[FunctionEntry]` — array of metadata + handler
- `pub fn lookup(name: &str) -> Option<&'static FunctionEntry>` — uppercase + `_XLFN.` strip + match
- `pub fn lookup_meta(name: &str) -> Option<&'static FunctionMeta>` — metadata only (passed to parse)
- `pub fn dispatch(name, args, interp, scope) -> Option<Value>` — lookup + call handler
- `pub fn all() -> &'static [FunctionEntry]` — for iteration in tests

### API changes to classify() and rewrite()

```rust
// BEFORE
pub fn classify(ast: &Ast, ctx: &ClassificationContext) -> Classification

// AFTER
pub fn classify(
    ast: &Ast,
    ctx: &ClassificationContext,
    fn_lookup: &dyn Fn(&str) -> Option<&FunctionMeta>,
) -> Classification
```

Same pattern for internal functions that need metadata: `classify_function()`, `disposition()` thread the callback through. `rewrite()` and `rewrite_node()` also take the callback for `agg_kind_for()` and `is_aggregate`/`is_lookup` checks.

`collect_lookup_keys()` takes the callback to replace its 6-name string match with `caps.contains(LOOKUP)`.

### Flag combinations (every function type)

| Function type | Caps | Examples |
|---|---|---|
| Pure math/string/info | `PURE` | ABS, UPPER, ISBLANK, ROUND |
| Result-returning stats | `PURE` | NORM.DIST, BINOM.DIST, T.DIST |
| Short-circuit conditional | `SHORT_CIRCUIT` | IF, IFS, SWITCH, IFERROR, IFNA |
| Pure conditional | `PURE` | NOT, XOR, TRUE, FALSE |
| Short-circuit + range-expanding | `SHORT_CIRCUIT \| RANGE_EXPAND` | AND, OR |
| Pure range-expanding | `PURE \| RANGE_EXPAND` | SUMPRODUCT, CONCAT, VAR.S, CORREL |
| Simple aggregate | `PURE \| RANGE_EXPAND \| AGG_COERCE \| NEEDS_PRELUDE` | SUM, COUNT, AVERAGE, MIN, MAX |
| Conditional aggregate | `SHORT_CIRCUIT \| NEEDS_PRELUDE` | SUMIF, COUNTIF, SUMIFS |
| Lookup | `LOOKUP \| NEEDS_PRELUDE` | VLOOKUP, XLOOKUP, INDEX |
| Variable binding (v0.4) | `SHORT_CIRCUIT` | LET |
| Database aggregate (v0.5) | `RANGE_EXPAND \| NEEDS_PRELUDE` | DSUM, DAVERAGE |
| Volatile | `VOLATILE` | TODAY, NOW |
| Range metadata | `RANGE_METADATA` | ROW, COLUMN, ROWS, COLUMNS |
| Meta-dispatch | `SHORT_CIRCUIT \| RANGE_EXPAND` | SUBTOTAL, AGGREGATE |

### Streamability derivation

```
NeedsPrelude = caps.contains(NEEDS_PRELUDE)
RowLocal     = !NeedsPrelude
```

LOOKUP implies NEEDS_PRELUDE (enforced by flag consistency test).

### Classifier routing (replaces sets::is_*() cascade)

```rust
fn classify_function(name, args, ctx, fn_lookup) -> Disposition {
    if sets::is_unsupported(name) { return Unsupported(...); }

    if let Some(meta) = fn_lookup(name) {
        if meta.caps.contains(FnCaps::VOLATILE)       { return RowLocal; }
        if meta.caps.contains(FnCaps::RANGE_METADATA)  { return RowLocal; }
        if meta.category == FnCategory::Aggregate       { return classify_aggregate(...); }
        if meta.caps.contains(FnCaps::LOOKUP)           { return fold_fn_args(Lookup); }
        if meta.caps.contains(FnCaps::RANGE_EXPAND)     { return fold_args(RangeExpanding); }
        return fold_args(None);
    }

    fold_args(None)  // unknown function
}
```

### Known exceptions (not migrated)

| Site | Why it stays |
|---|---|
| `is_criteria_arg()` in classify.rs | 7 lines, 8 functions, 2 patterns. Too specific for registry. |
| `collect_multi_conditional_keys()` in prelude_plan.rs | `MultiConditionalAggKey` has a fundamentally different shape. 8 string matches stay. |
| SUBTOTAL/AGGREGATE numeric dispatch in subtotal.rs | Routes by integer arg, not function name. |
| `strip_xlfn_prefix()` in evaluate.rs | Raw formula text before parsing. Different layer. |

### Test strategy for parse

Parse integration tests add eval as a dev-dependency to use the real registry:

```toml
# crates/xlstream-parse/Cargo.toml
[dev-dependencies]
xlstream-eval = { path = "../xlstream-eval" }
```

Integration tests pass `&xlstream_eval::registry::lookup_meta`. Unit tests in classify.rs use a small mock lookup with only the functions each test needs.

## Tests

### Registry tests (unit tests in eval)

**Happy path:**
- `lookup_sum_returns_correct_metadata` — caps, category, agg_kind all correct
- `lookup_case_insensitive` — "sum", "SUM", "Sum" all return same entry
- `lookup_xlfn_prefix_stripped` — "_XLFN.XLOOKUP" resolves to XLOOKUP entry
- `dispatch_sqrt_computes_correctly` — dispatch("SQRT", [Number(4)]) returns Number(2)
- `dispatch_sum_aggregate` — dispatch("SUM", [1, 2, 3]) returns Number(6)

**Edge cases:**
- `lookup_unknown_returns_none` — "NOTAFUNCTION" returns None
- `lookup_alias_resolves_to_canonical` — "CONCATENATE" resolves to CONCAT entry
- `lookup_double_xlfn_prefix` — "_XLFN._XLFN.SUM" strips correctly
- `all_entries_have_uppercase_names` — iterate all(), assert name == name.to_uppercase()
- `all_aliases_resolve_to_canonical_entry` — each alias's lookup returns same meta as canonical

**Flag consistency:**
- `lookup_implies_needs_prelude` — every entry with LOOKUP also has NEEDS_PRELUDE
- `agg_coerce_implies_range_expand` — every entry with AGG_COERCE also has RANGE_EXPAND
- `agg_kind_implies_agg_coerce` — every entry with agg_kind: Some(_) has AGG_COERCE
- `no_duplicate_names` — no two entries share a canonical name or alias

### Classification tests (existing, updated with callback)

All ~115 existing classify/rewrite tests in parse must pass with the new callback API. No behavior change.

**New tests:**
- `unknown_function_without_registry_entry_classifies_as_row_local` — unknown functions fall through to `fold_args(None)`
- `volatile_flag_classifies_as_row_local` — TODAY, NOW
- `range_metadata_flag_classifies_as_row_local` — ROW, COLUMN, ROWS, COLUMNS with range args
- `aggregate_category_routes_to_classify_aggregate` — SUM, SUMIF both route correctly
- `lookup_flag_routes_to_fold_fn_args_lookup` — VLOOKUP, XLOOKUP

### Dispatch tests (existing, unchanged)

All ~2245 existing eval tests must pass. The dispatch mechanism changes but behavior does not.

### Conformance

No new conformance fixtures. This is a pure refactor — no new functions, no behavior change. Existing conformance suite validates correctness.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add entry: "Centralize formula registration — single registry replaces 9 scattered sites" |
| `docs/roadmap/v0.4/README.md` | Tick the "Formula registry clean up" checkbox |
| `docs/roadmap/v0.4/01-formula-registry-cleanup.md` | Update status to "Superseded by 02-formula-registry-cleanup-v2.md" |
| `crates/xlstream-parse/src/lib.rs` | Add `pub mod function_meta;`, update re-exports |
| `crates/xlstream-eval/src/lib.rs` | Add `pub mod registry;` |

## Streaming invariant

Not violated. This is a pure refactor of where function metadata and dispatch logic live. No change to classification semantics, prelude computation, or row-level evaluation. The streaming invariant is unaffected.
