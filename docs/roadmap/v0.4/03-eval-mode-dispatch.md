# Feature: Dispatch Enum

**Branch:** `feat/eval-mode-dispatch`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

Replace ~134 boilerplate handler wrappers in `builtins/mod.rs` with a 3-variant `Dispatch` enum on each registry entry. The registry's `dispatch()` function matches on the variant and handles arg preparation (eval_args / expand_args_for_aggregate) generically. ~41 Custom wrappers stay as the glue layer between eval types and pure-math sub-modules.

```
Before: adding an eager function = 1 registry entry + 1 handle_* wrapper in mod.rs
After:  adding an eager function = 1 registry entry pointing at the sub-module fn
```

## Why

The registry PR centralized function registration but left ~134 trivial wrappers in `builtins/mod.rs` that all follow one of two patterns:

```rust
// Pattern A: 125 eager wrappers
pub(crate) fn handle_round(args, interp, scope) -> Value {
    math::builtin_round(&eval_args(args, interp, scope))
}

// Pattern B: 9 aggregate wrappers
pub(crate) fn handle_sum(args, interp, scope) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::sum(&vals).unwrap_or_else(Value::Error)
}
```

These are pure boilerplate — the only difference is which arg-preparation function to call before delegating.

## Design

### Dispatch enum

```rust
// registry.rs
pub(crate) enum Dispatch {
    /// eval_args() first, then call with &[Value]
    Eager(fn(&[Value]) -> Value),
    /// expand_args_for_aggregate() first, then call with &[Value]
    Aggregate(fn(&[Value]) -> Value),
    /// no arg prep — handler receives raw &[NodeRef] and does everything
    Custom(fn(&[NodeRef], &Interpreter, &RowScope) -> Value),
}
```

### FunctionEntry change

```rust
pub struct FunctionEntry {
    pub meta: FunctionMeta,
    pub aliases: &'static [&'static str],
    pub(crate) dispatch: Dispatch,
}
```

### Generic dispatch

```rust
pub(crate) fn dispatch(
    name: &str,
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Option<Value> {
    let entry = lookup(name)?;
    Some(match &entry.dispatch {
        Dispatch::Eager(f) => f(&eval_args(args, interp, scope)),
        Dispatch::Aggregate(f) => f(&expand_args_for_aggregate(args, interp, scope)),
        Dispatch::Custom(f) => f(args, interp, scope),
    })
}
```

### Registry entry examples

```rust
// Eager — no wrapper needed, points at sub-module fn directly
FunctionEntry {
    meta: ...,
    aliases: &[],
    dispatch: Dispatch::Eager(crate::builtins::math::builtin_round),
},

// Aggregate — no wrapper needed
FunctionEntry {
    meta: ...,
    aliases: &[],
    dispatch: Dispatch::Aggregate(crate::builtins::aggregate::sum_as_value),
},

// Custom — keeps existing handler in mod.rs
FunctionEntry {
    meta: ...,
    aliases: &[],
    dispatch: Dispatch::Custom(crate::builtins::handle_rank_eq),
},
```

## What stays in mod.rs

~41 Custom handlers that contain real glue logic (arg validation, coercion, `expand_range` calls). These are the bridge between eval types (`NodeRef`, `Interpreter`) and pure-math sub-modules (`&[Value]`, `&[f64]`). They stay because moving them into sub-modules would pollute the pure-math layer with eval-aware imports.

## What changes in sub-modules

### Aggregate functions

`aggregate::sum`, `aggregate::count`, etc. currently return `Result<Value, CellError>`. `Dispatch::Aggregate` needs `fn(&[Value]) -> Value`. Add thin `_as_value` wrappers:

```rust
pub fn sum_as_value(vals: &[Value]) -> Value { sum(vals).unwrap_or_else(Value::Error) }
```

### Statistical Result-returning functions

`builtin_poisson_dist`, `builtin_binom_dist`, etc. return `Result<f64, CellError>`. Change to return `Value` directly so they fit `Dispatch::Eager`. Wrap the internal `Result` at the end of each function.

## Layering invariant

**Do NOT move Custom handler logic into sub-modules.** The boundary is:

```
mod.rs          — eval-aware glue (NodeRef, Interpreter, expand_range)
sub-modules     — pure math on &[Value] or &[f64]
```

Sub-modules must not import `NodeRef`, `Interpreter`, `RowScope`, or `expand_range`.

## Tests

No new tests needed — pure refactor. All ~2700 existing tests must pass.

## Docs to update

| File | Change |
|---|---|
| `CHANGELOG.md` | Add entry under `[Unreleased]` |
| `docs/roadmap/v0.4/README.md` | Tick the "EvalMode dispatch" checkbox |
