# Evaluator

The evaluator is the interpreter for our AST. Given a parsed formula, a row scope, and the prelude context, it returns a `Value`.

## Types

```rust
pub struct Interpreter<'ctx> {
    prelude: &'ctx Prelude,
}

pub struct RowScope<'row> {
    values: &'row [Value],         // current row's cells, indexed by column
    header_to_col: &'row HeaderMap, // resolve column name -> index
    row_idx: u32,
}

impl<'ctx> Interpreter<'ctx> {
    pub fn eval(&self, node: NodeRef<'_>, scope: &RowScope<'_>) -> Value { ... }
}
```

## AST walk

```rust
fn eval(&self, node: NodeRef<'_>, scope: &RowScope<'_>) -> Value {
    match node.view() {
        NodeView::Number(n)                => Value::Number(n),
        NodeView::Text(s)                  => Value::Text(s.into()),
        NodeView::Bool(b)                  => Value::Bool(b),
        NodeView::Error(e)                 => Value::Error(e),
        NodeView::CellRef { col, .. }      => scope.get(col),
        NodeView::RangeRef { .. }          => Value::Error(CellError::Ref),
        NodeView::BinaryOp { op }          => self.eval_binary(op, node, scope),
        NodeView::UnaryOp { op }           => self.eval_unary(op, node, scope),
        NodeView::Function { name }        => crate::builtins::dispatch(name, &node.args(), self, scope)
                                                  .unwrap_or(Value::Error(CellError::Value)),
        NodeView::PreludeRef(key)          => self.resolve_prelude(key),
        // NamedRef, ExternalRef, TableRef -> #NAME?
        // Array -> #VALUE!
    }
}
```

Cell errors are not `Result::Err` -- they are `Value::Error(CellError)`. `eval()` always returns `Value` directly.

## Builtin dispatch

A single `dispatch()` function in `builtins/mod.rs` maps function names to implementations via match arms:

```rust
pub(crate) fn dispatch(
    name: &str,
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Option<Value> {
    let upper = name.to_ascii_uppercase();
    let normalized = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
    match normalized {
        "IF"      => Some(conditional::builtin_if(args, interp, scope)),
        "VLOOKUP" => Some(lookup::builtin_vlookup(args, interp, scope)),
        "ROUND"   => Some(math::builtin_round(&eval_args(args, interp, scope))),
        // ...
        _ => None,
    }
}
```

Returns `None` for unknown functions; the caller falls back to `#VALUE!`. No registry struct or static map -- just a match expression.

Note: `phf` is used in xlstream-parse for function-set lookups during classification, but not in xlstream-eval for dispatch.

## Function categories

### Eager: `(&[Value]) -> Value`

Take all arguments pre-evaluated via `eval_args()`. Used for most arithmetic, string, math, date, info, and financial builtins.

Example:
```rust
// dispatch arm:
"ROUND" => Some(math::builtin_round(&eval_args(args, interp, scope))),

// implementation:
pub fn builtin_round(args: &[Value]) -> Value {
    // ...
}
```

### Lazy: takes `&[NodeRef<'_>]`, interpreter, scope

Needs access to the raw AST nodes for short-circuit evaluation, or to the interpreter/prelude for lookups.

Example:
```rust
pub(crate) fn builtin_if(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let cond = interp.eval(args[0], scope);
    let cond_bool = match coerce::to_bool(&cond) {
        Ok(b) => b,
        Err(e) => return Value::Error(e),
    };
    if cond_bool {
        interp.eval(args[1], scope)
    } else if args.len() > 2 {
        interp.eval(args[2], scope)
    } else {
        Value::Bool(false)
    }
}
```

## Coercion rules

Excel is loose about types. Our coercion tables match Excel exactly:

- **Number context**: `true` -> 1, `false` -> 0, text-that-parses-as-number -> number, empty -> 0, date -> serial number, error -> propagate.
- **Text context**: all values stringified per Excel's rules.
- **Bool context**: 0 -> false, nonzero -> true, `"TRUE"` / `"FALSE"` -> bool (case-insensitive), other text -> `#VALUE!`.

Coercion helpers live in `xlstream-core::coerce`. Every function uses them -- no ad-hoc conversions.

## Error propagation

```rust
Value::Error(e) + anything = Value::Error(e)   // unless a lazy fn intercepts
```

Eager functions propagate errors by checking inputs explicitly. Lazy functions that "catch" errors (`IFERROR`, `IFNA`) check the evaluated result:

```rust
let val = interp.eval(args[0], scope);
if matches!(val, Value::Error(_)) {
    interp.eval(args[1], scope)
} else {
    val
}
```

## Short-circuit evaluation

`AND`, `OR`, `IF`, `IFS`, `SWITCH`, `IFERROR`, `IFNA` are lazy and short-circuit. The interpreter does NOT pre-evaluate their arguments.

This matters for correctness: `IF(A=0, 0, 1/A)` must not evaluate `1/A` when `A=0`, otherwise we return `#DIV/0!` instead of `0`.

## Thread safety

- `Interpreter` is `Send + Sync` because it holds only an immutable reference to `prelude`.
- `RowScope` is `Send + Sync`-able in principle, but practically built per thread per row.
- `Prelude` is immutable after construction, shared across threads via `&Prelude`.

Row parallelism spawns N workers, each with its own `Interpreter` (cheap to clone -- it's one reference), each iterating its row range.

## Testing a builtin

Every builtin lives in its own module with its own tests:

```rust
// xlstream-eval/src/builtins/arithmetic.rs
#[cfg(test)]
mod tests {
    use super::*;
    use xlstream_core::Value;

    #[test]
    fn sum_numbers() {
        assert_eq!(sum(&[Value::Number(1.0), Value::Number(2.0)]).unwrap(), Value::Number(3.0));
    }

    #[test]
    fn sum_empty_returns_zero() {
        assert_eq!(sum(&[]).unwrap(), Value::Number(0.0));
    }

    #[test]
    fn sum_propagates_error() {
        let res = sum(&[Value::Number(1.0), Value::Error(CellError::Div0)]);
        assert_eq!(res, Err(CellError::Div0));
    }

    #[test]
    fn sum_coerces_bool() {
        assert_eq!(sum(&[Value::Bool(true), Value::Bool(true)]).unwrap(), Value::Number(2.0));
    }

    #[test]
    fn sum_numeric_text() {
        assert_eq!(sum(&[Value::Text("1.5".into())]).unwrap(), Value::Number(1.5));
    }

    #[test]
    fn sum_non_numeric_text_errors() {
        assert_eq!(sum(&[Value::Text("abc".into())]), Err(CellError::Value));
    }
}
```

Every new builtin lands with **at least 5 unit tests**: happy path, empty, error propagation, coercion, type edge case.

## Performance rules for builtins

1. **No allocation in the pure path.** `Value::Number`, `Value::Bool` are enum variants without heap data. Only text and array returns allocate.
2. **Early exit on error.** Check and propagate.
3. **No dynamic dispatch.** Direct function calls, not `Box<dyn Fn>`.
4. **Precompiled constants.** e.g. `ln(10)` is `const LN_10: f64 = ...`.
