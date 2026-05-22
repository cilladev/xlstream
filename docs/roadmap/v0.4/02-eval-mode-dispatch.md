# Feature: EvalMode Dispatch

**Branch:** `feat/eval-mode-dispatch`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

Replace ~134 boilerplate handler wrappers in `builtins/mod.rs` with a 3-variant `EvalMode` enum on each registry entry. The registry's `dispatch()` function matches on the variant and handles arg preparation (eval_args / expand_args_for_aggregate) generically. ~34 Custom wrappers stay as the glue layer between eval types and pure-math sub-modules.

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

### EvalMode enum

```rust
// registry.rs
pub(crate) enum EvalMode {
    /// eval_args() then call fn(&[Value]) -> Value
    Eager(fn(&[Value]) -> Value),
    /// expand_args_for_aggregate() then call fn(&[Value]) -> Value
    Aggregate(fn(&[Value]) -> Value),
    /// handler does everything — arg unpacking, coercion, delegation
    Custom(fn(&[NodeRef], &Interpreter, &RowScope) -> Value),
}
```

### FunctionEntry change

```rust
pub struct FunctionEntry {
    pub meta: FunctionMeta,
    pub aliases: &'static [&'static str],
    pub eval_mode: EvalMode,  // replaces `handler: Handler`
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
    Some(match entry.eval_mode {
        EvalMode::Eager(f) => f(&eval_args(args, interp, scope)),
        EvalMode::Aggregate(f) => {
            let vals = expand_args_for_aggregate(args, interp, scope);
            f(&vals).unwrap_or_else(Value::Error)
        }
        EvalMode::Custom(f) => f(args, interp, scope),
    })
}
```

### Registry entry examples

```rust
// Eager — no wrapper needed, points at sub-module fn directly
FunctionEntry {
    meta: ...,
    aliases: &[],
    eval_mode: EvalMode::Eager(crate::builtins::math::builtin_round),
},

// Aggregate — no wrapper needed
FunctionEntry {
    meta: ...,
    aliases: &[],
    eval_mode: EvalMode::Aggregate(crate::builtins::aggregate::sum_value),
},

// Custom — keeps existing handler in mod.rs
FunctionEntry {
    meta: ...,
    aliases: &[],
    eval_mode: EvalMode::Custom(crate::builtins::handle_rank_eq),
},
```

## What stays in mod.rs

~34 Custom handlers that contain real glue logic (arg validation, coercion, `expand_range` calls). These are the bridge between eval types (`NodeRef`, `Interpreter`) and pure-math sub-modules (`&[Value]`, `&[f64]`). They stay because moving them into sub-modules would pollute the pure-math layer with eval-aware imports.

## What changes in sub-modules

### Aggregate functions

`aggregate::sum`, `aggregate::count`, etc. currently return `Result<Value, CellError>`. `EvalMode::Aggregate` needs `fn(&[Value]) -> Value`. Two options:

1. Add `_value` wrappers: `pub fn sum_value(vals: &[Value]) -> Value { sum(vals).unwrap_or_else(Value::Error) }`
2. Change return type to `Value` directly

Option 1 is safer — doesn't break existing callers. Option 2 is cleaner if no other code calls `sum()` with Result handling.

### Statistical Result-returning functions

`builtin_poisson_dist`, `builtin_binom_dist`, etc. return `Result<f64, CellError>`. Same two options. Currently wrapped by eager handlers that do `.map_or_else(Value::Error, Value::Number)`. With EvalMode::Eager these need to return `Value` directly.

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
| `docs/roadmap/v0.4/README.md` | Tick the EvalMode dispatch checkbox |
