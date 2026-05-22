# Plan: Centralize Formula Registration

> **Superseded by [`02-formula-registry-cleanup-v2.md`](02-formula-registry-cleanup-v2.md).**

**Status:** Superseded
**Scope:** Architectural refactor — touches xlstream-parse + xlstream-eval
**Motivation:** Issue #140 exposed that adding/fixing a function requires touching 5+ files with redundant string matching. There's no single source of truth for "what functions exist and how do they behave."

## Problem

Function identity is scattered across 9 locations with no shared type:

| # | File | What it stores | Format |
|---|---|---|---|
| 1 | `xlstream-parse/src/sets.rs` | Category membership (aggregate, lookup, unsupported, ...) | `phf_set!` of string literals |
| 2 | `xlstream-parse/src/classify.rs` | Classification logic | Delegates to sets.rs, custom branching |
| 3 | `xlstream-parse/src/classify.rs` | `is_criteria_arg()` (lines 513-519) | Match on 8 function names + arg position |
| 4 | `xlstream-parse/src/rewrite.rs` | `AggKind` / `LookupKind` enums + `agg_kind_for()` | Enum + match on strings |
| 5 | `xlstream-parse/src/rewrite.rs` | `collect_lookup_keys()` (lines 345-413) | Match on "VLOOKUP", "HLOOKUP", etc. as strings |
| 6 | `xlstream-eval/src/prelude_plan.rs` | Prelude fold finalization per `AggKind` | Match on AggKind |
| 7 | `xlstream-eval/src/prelude_plan.rs` | `collect_multi_conditional_keys()` (lines 260-346) | Match on 8 function names as strings |
| 8 | `xlstream-eval/src/builtins/mod.rs` | Runtime dispatch (300+ match arms) | Match on strings |
| 9 | `xlstream-eval/src/builtins/subtotal.rs` | SUBTOTAL/AGGREGATE numeric dispatch | Match on i32 -> function |

`_XLFN.` prefix stripping happens in 3 additional places:
- `builtins/mod.rs:99` — `upper.strip_prefix("_XLFN.")`
- `prelude_plan.rs:275` — same pattern
- `evaluate.rs:1290` — `formula.replace("_xlfn.", "")` at formula-text level

**Consequences:**
- Forgetting one site = silent bug (exactly what #140 was).
- No compile-time guarantee that a function registered in sets.rs has a dispatch entry.
- Boolean/coercion semantics (range vs scalar) are ad-hoc per call site.
- Adding a new function requires 5+ edits across 2 crates.

## Design Goal

> One canonical definition per function. Everything else derives from it.

## Key Design Insight: Bitflags Over Enum

v1-v2 of this plan used an `EvalStrategy` enum. It failed immediately — conditional aggregates didn't fit any variant, forcing `ConditionalAggregate` and `DatabaseAggregate` additions. Every new function combination would require a new variant.

Formualizer solved this with composable bitflags. A function declares capabilities, not a bucket. The interpreter reads the flags it cares about. SUMIF is `SHORT_CIRCUIT | NEEDS_PRELUDE`. SUM is `PURE | RANGE_EXPAND | AGG_COERCE | NEEDS_PRELUDE`. LET is `SHORT_CIRCUIT`. No "which variant" problem.

We adopt bitflags for capabilities and a unified handler signature (one fn pointer type, not an enum of signatures). We do NOT adopt formualizer's trait objects, DashMap, runtime registration, namespaces, or parallelism flags.

## Resolved Design Decisions

| Decision | Answer | Rationale |
|---|---|---|
| Where does the registry live? | Split: metadata in `xlstream-parse`, handlers in `xlstream-eval` | Handler fn pointers need types from eval/core. Parse owns metadata. Eval owns dispatch table. Phase 4 completeness test bridges them. |
| Capabilities: enum or bitflags? | Bitflags | Enum forces exclusive buckets. Bitflags compose. Eliminates the "which variant" problem for all current and future functions. |
| Handler signature: multiple variants or unified? | Unified: `fn(&[NodeRef], &Interpreter, &RowScope) -> Value` | One fn pointer type. No `FnHandler` enum. No 4-signature problem. Pure-eager functions use `eval_args` internally. Dispatch is a single function pointer call — no match on strategy. |
| Centralized or decentralized registration? | Centralized const array | One array in `function_registry.rs`. Greppable, diffable, compiler-checked. |
| Compile-time or runtime init? | phf (compile-time) | Already a dependency. Same pattern, same perf, zero new deps. |
| `_XLFN.` normalization | Strip once at registry `lookup()` entry point | Registry stores canonical names only. `aliases` field handles legacy names (CONCATENATE -> CONCAT). All 3 existing strip sites and redundant `_XLFN.XLOOKUP`/`_XLFN.XMATCH` match arms deleted. |
| Multi-conditional prelude dispatch | Stays as separate match, explicitly excluded | SUMIF/COUNTIF/etc. use `MultiConditionalAggKey` — fundamentally different key shape. Phase 4's grep-based lint flags as known exception. |
| `is_criteria_arg()` in classify.rs | Stays as separate match, explicitly excluded | Function-name-to-arg-position mapping for 8 functions. Too specific for a generic registry field. Phase 4's grep-based lint flags as known exception. |

## FunctionDef Type (xlstream-parse)

```rust
bitflags! {
    pub struct FnCaps: u16 {
        const PURE          = 0b0000_0001;  // deterministic, no side effects
        const SHORT_CIRCUIT = 0b0000_0010;  // lazy arg eval (IF, AND, OR, LET)
        const RANGE_EXPAND  = 0b0000_0100;  // expand range refs before call
        const AGG_COERCE    = 0b0000_1000;  // coerce scalar bools/text in agg context
        const NEEDS_PRELUDE = 0b0001_0000;  // requires pass-1 data
        const LOOKUP        = 0b0010_0000;  // uses prelude-loaded index (implies NEEDS_PRELUDE)
        const VOLATILE      = 0b0100_0000;  // changes between runs (TODAY, NOW)
    }
}

pub struct FunctionDef {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub category: FnCategory,
    pub caps: FnCaps,
    pub agg_kind: Option<AggKind>,
}

pub enum FnCategory {
    Math, String, Date, Lookup, Aggregate, Conditional,
    Info, Financial, Statistical, Engineering, Database,
    Compatibility,
}
```

### Common flag combinations

| Function type | Caps | Examples |
|---|---|---|
| Pure math/string/info | `PURE` | ABS, UPPER, ISBLANK, ROUND |
| Result-returning stats | `PURE` | NORM.DIST, BINOM.DIST, T.DIST |
| Short-circuit conditional | `SHORT_CIRCUIT` | IF, IFS, IFERROR |
| Short-circuit + range-expanding | `SHORT_CIRCUIT \| RANGE_EXPAND` | AND, OR, CONCAT, TEXTJOIN |
| Simple aggregate | `PURE \| RANGE_EXPAND \| AGG_COERCE \| NEEDS_PRELUDE` | SUM, COUNT, AVERAGE, MIN, MAX |
| Range-expanding | `PURE \| RANGE_EXPAND` | SUMPRODUCT, VAR.S, CORREL |
| Conditional aggregate | `SHORT_CIRCUIT \| NEEDS_PRELUDE` | SUMIF, COUNTIF, SUMIFS |
| Lookup | `LOOKUP \| NEEDS_PRELUDE` | VLOOKUP, XLOOKUP, INDEX |
| Variable binding | `SHORT_CIRCUIT` | LET |
| Database aggregate | `RANGE_EXPAND \| NEEDS_PRELUDE` | DSUM, DAVERAGE |
| Volatile | `VOLATILE` | TODAY, NOW |
| Custom (bespoke arg inspection) | varies | ROW, COLUMN, ISREF |

### Streamability derivation

```
NeedsPrelude = caps.contains(NEEDS_PRELUDE)
RowLocal     = !NeedsPrelude
```

One flag, one check. `LOOKUP` implies `NEEDS_PRELUDE` (enforced by Phase 4 flag consistency test), so checking `NEEDS_PRELUDE` alone is sufficient.

## Dispatch Table (xlstream-eval)

```rust
pub type Handler = fn(&[NodeRef<'_>], &Interpreter<'_>, &RowScope<'_>) -> Value;

static DISPATCH: phf::Map<&str, Handler> = ...;

pub fn dispatch(name: &str, args: &[NodeRef], interp: &Interpreter, scope: &RowScope) -> Option<Value> {
    let _meta = function_registry::lookup(name)?;
    let handler = DISPATCH.get(name)?;
    Some(handler(args, interp, scope))
}
```

One phf probe for metadata, one for handler. No match on strategy. No arg preparation in the framework. The handler owns its arg evaluation — pure-eager functions call `eval_args` internally, aggregate functions call `expand_args_for_aggregate`, short-circuit functions eval lazily. Helper functions (`eval_args`, `expand_range`, `expand_args_for_aggregate`) are shared utilities, not framework routing.

The bitflags inform the **classifier** and **streamability derivation**, not the dispatch hot path.

### What about boilerplate in pure-eager handlers?

A pure-eager function like SQRT currently dispatches as:
```rust
"SQRT" => Some(math::builtin_sqrt(&eval_args(args, interp, scope)))
```

With the unified signature, the handler wraps this:
```rust
fn builtin_sqrt(args: &[NodeRef], interp: &Interpreter, scope: &RowScope) -> Value {
    let vals = eval_args(args, interp, scope);
    math::sqrt_impl(&vals)
}
```

One extra line per function. For ~120 pure-eager functions, that's 120 thin wrappers. The alternative (framework-level arg prep based on flags) reintroduces the strategy-match in dispatch and couples flags to handler signatures. The wrapper approach is simpler — each handler is self-contained, testable, and the dispatch path has zero branching.

## How Users Add a New Function

**Before (5+ files):**
1. Write impl in `builtins/<category>.rs`
2. Add match arm in `builtins/mod.rs` (300+ line match)
3. Add to `sets.rs` phf set (if aggregate/lookup/range-expanding)
4. Add to `rewrite.rs` `agg_kind_for()` (if aggregate)
5. Add to `prelude_plan.rs` fold (if aggregate)
6. Tick `docs/functions.md`

**After (2 steps):**
1. Write handler in `builtins/<category>.rs` with the unified signature
2. Add `FunctionDef` in parse's `function_registry.rs` + handler entry in eval's `DISPATCH` table

Phase 4 completeness test catches any mismatch between the two tables.

## Migration Path

Each phase is one PR. No phase changes behavior — pure refactors.

### Phase 1: `FunctionDef` type + registry in xlstream-parse

- Add `function_registry.rs` with `FnCaps` bitflags, `FunctionDef`, `FnCategory` types
- `FnCategory` includes `Database` for v0.5 forward-compatibility
- Populate the central const array from current `sets.rs` content
- Build `phf::Map<&str, &FunctionDef>` via build script
- Registry stores canonical names only; single `_XLFN.` normalization at `lookup()`
- Make `classify.rs` read from registry instead of sets.rs
- Keep `sets.rs` as a thin shim (delegates to registry) until all callers migrate
- Delete `sets.rs` once no direct callers remain
- Add `bitflags` dependency (or use a manual `u16` with associated constants if we want zero deps)

### Phase 2: Derive dispatch from registry in xlstream-eval

- Add `Handler` type alias and `DISPATCH` phf map
- Convert each builtin to the unified handler signature (thin wrappers for pure-eager)
- **Add thin handlers for SUM, COUNT, COUNTA, COUNTBLANK, AVERAGE, MIN, MAX, MEDIAN** — these 8 aggregate functions currently have no dispatch entry (they work only via PreludeRef rewriting for whole-column args). Each handler is a one-liner calling `expand_args_for_aggregate` + `aggregate::fn_name`. ~20 lines total. This also fixes `SUM(1,2,3)` returning `#VALUE!` (issue #140's root cause).
- Replace the 300+ arm match in `builtins/mod.rs` with `DISPATCH.get(name)?(args, interp, scope)`
- Migrate `collect_lookup_keys()` in `rewrite.rs`: registry lookup replaces the function-name match (`caps.contains(LOOKUP)` instead of matching on "VLOOKUP"); per-function key extraction functions (`extract_vlookup_key`, `extract_xlookup_key`, etc.) stay as-is — key extraction is inherently per-function
- Remove all 3 `_XLFN.` strip sites; replace with single normalization at registry entry point
- Remove redundant `_XLFN.XLOOKUP` / `_XLFN.XMATCH` match arms

### Phase 3: Derive prelude handling from registry

- Move `AggKind` onto `FunctionDef` as `agg_kind: Option<AggKind>`
- Delete `agg_kind_for()` string-matching function in `rewrite.rs`
- Replace with `function_registry::lookup(name)?.agg_kind`
- `prelude_plan.rs` fold logic unchanged — AggKind remains the prelude execution type

### Phase 4: Compile-time completeness tests

Add tests that iterate every `FunctionDef` and assert:
- Every `FunctionDef` has a corresponding entry in eval's `DISPATCH` table
- Every entry with `agg_kind: Some(_)` has `AGG_COERCE` in caps
- Every function in `docs/functions.md` marked as shipped has a registry entry
- Grep-based lint: no function name appears as a hardcoded string match outside the registry, except known exceptions:
  - `collect_multi_conditional_keys()` — 8 function names (SUMIF/COUNTIF/etc.)
  - `is_criteria_arg()` — 8 function names + arg position mapping
  - `subtotal.rs` — numeric dispatch (inherently different, not string-based)
- Streamability derivation test: for every `FunctionDef`, verify that `caps.contains(NEEDS_PRELUDE)` matches the current classification behavior
- Flag consistency:
  - `AGG_COERCE` implies `RANGE_EXPAND`
  - `LOOKUP` implies `NEEDS_PRELUDE`

## What This Does NOT Change

- Individual function implementations (math.rs, string.rs, etc.) stay as-is
- The streaming invariant is unaffected
- Prelude/pass-1 architecture is unchanged
- `collect_multi_conditional_keys()` in `prelude_plan.rs` — 8 function names stay as a separate match. They use `MultiConditionalAggKey` with a fundamentally different key shape. Phase 4's grep-based lint flags as known exception.
- `is_criteria_arg()` in `classify.rs` — 8 function names + arg position mapping stays as a separate match. Phase 4's grep-based lint flags as known exception.
- SUBTOTAL/AGGREGATE numeric dispatch in `subtotal.rs` — inherently different (routes by integer, not string)
- No new dependencies beyond `bitflags` (or zero if using manual constants)
- No performance regression — dispatch is now a single phf probe + function pointer call (faster than 300-arm match)

## What We Took From Formualizer (and What We Didn't)

**Adopted:**
- Bitflags for capabilities — composable, eliminates "which variant" problem
- Unified handler signature — one fn pointer type, handler owns arg evaluation
- Registry-level `_XLFN.` normalization — strip once at lookup, store canonical names only

**Not adopted:**
- `trait Function` + `Arc<dyn Function>` — we don't need dynamic dispatch or runtime registration
- `DashMap` + `once_cell::Lazy` — our function set is known at compile time, phf is better
- Decentralized `register_builtins()` — centralized table gives us completeness tests for free
- Namespaces — we only have Excel functions
- SIMD_OK, GPU_OK, PARALLEL_ARGS, PARALLEL_CHUNKS — not relevant to streaming eval
- `ArgumentHandle` with lazy `.value()` — our `NodeRef` + `Interpreter` already provides this; the helper functions (`eval_args`, `expand_range`) are the equivalent
