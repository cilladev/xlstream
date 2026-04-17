# Evaluator

The evaluator is the interpreter for our AST. Given a parsed formula, a row scope, and the prelude context, it returns a `Value`.

## Types

```rust
pub struct Interpreter<'ctx> {
    prelude: &'ctx Prelude,
    functions: &'static BuiltinRegistry,
}

pub struct RowScope<'row> {
    values: &'row [Value],         // current row's cells, indexed by column
    header_to_col: &'row HeaderMap, // resolve column name → index
    row_idx: u32,
}

impl<'ctx> Interpreter<'ctx> {
    pub fn eval<'row>(&self, node: &AstNode, scope: &RowScope<'row>) -> Result<Value, XlStreamError> { ... }
}
```

## AST walk

```rust
fn eval(&self, node: &AstNode, scope: &RowScope) -> Result<Value, XlStreamError> {
    match node {
        AstNode::Number(n)     => Ok(Value::Number(*n)),
        AstNode::Text(s)       => Ok(Value::Text(s.clone())),
        AstNode::Bool(b)       => Ok(Value::Bool(*b)),
        AstNode::Error(e)      => Ok(Value::Error(*e)),
        AstNode::CellRef(r)    => self.resolve_cell(r, scope),
        AstNode::RangeRef(r)   => self.resolve_range(r, scope),
        AstNode::Binary(op, l, r) => self.eval_binary(*op, l, r, scope),
        AstNode::Unary(op, x)  => self.eval_unary(*op, x, scope),
        AstNode::Function(name, args) => self.call_builtin(name, args, scope),
    }
}
```

## Builtin registry

Compile-time perfect-hash map from function name (upper-case) to function pointer. Built with `phf::phf_map!`:

```rust
pub type BuiltinFn = fn(&[Value]) -> Result<Value, CellError>;

pub static BUILTINS: phf::Map<&'static str, BuiltinFn> = phf_map! {
    "SUM"       => arithmetic::sum,
    "IF"        => conditional::if_,
    "VLOOKUP"   => lookup::vlookup,
    // ...
};
```

Some functions are not pure `(&[Value]) -> Value` — they need the prelude or scope:

```rust
pub type StatefulBuiltinFn =
    fn(&[AstNode], &Interpreter, &RowScope) -> Result<Value, XlStreamError>;

// VLOOKUP with lookup sheet, IF with short-circuit, etc.
pub static STATEFUL_BUILTINS: phf::Map<&'static str, StatefulBuiltinFn> = phf_map! {
    "IF"        => conditional::if_stateful,
    "IFS"       => conditional::ifs_stateful,
    "VLOOKUP"   => lookup::vlookup_stateful,
    // ...
};
```

Lookup order: stateful first (early exit, custom arg handling), then pure. Stateful sees the raw AST so it can short-circuit / lazy-evaluate its arguments.

## Function categories

### Pure: `(&[Value]) -> Result<Value, CellError>`

Take all arguments pre-evaluated. Used for most arithmetic / string / math functions.

Example:
```rust
pub fn sum(args: &[Value]) -> Result<Value, CellError> {
    let mut total = 0.0;
    for v in args {
        match v {
            Value::Number(n) => total += n,
            Value::Integer(i) => total += *i as f64,
            Value::Empty => (),
            Value::Bool(b) => total += if *b { 1.0 } else { 0.0 },
            Value::Text(s) => {
                if let Ok(n) = s.parse::<f64>() { total += n }
                else { return Err(CellError::Value); }
            },
            Value::Error(e) => return Err(*e),
            Value::Date(d) => total += d.serial(),
        }
    }
    Ok(Value::Number(total))
}
```

### Stateful: takes `&[AstNode]`, interpreter, scope

Needs access to the AST for short-circuit, or to the prelude for looking up an aggregate, or to the lookup index.

Example:
```rust
pub fn if_stateful(
    args: &[AstNode],
    interp: &Interpreter,
    scope: &RowScope,
) -> Result<Value, XlStreamError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(XlStreamError::Classification {
            address: "...".into(),
            message: format!("IF expects 2 or 3 args, got {}", args.len()),
        });
    }
    let cond = interp.eval(&args[0], scope)?;
    let cond = coerce_to_bool(&cond)?;
    if cond {
        interp.eval(&args[1], scope)
    } else if let Some(else_branch) = args.get(2) {
        interp.eval(else_branch, scope)
    } else {
        Ok(Value::Bool(false))
    }
}
```

## Coercion rules

Excel is loose about types. Our coercion tables match Excel exactly:

- **Number context**: `true` → 1, `false` → 0, text-that-parses-as-number → number, empty → 0, date → serial number, error → propagate.
- **Text context**: all values stringified per Excel's rules.
- **Bool context**: 0 → false, nonzero → true, `"TRUE"` / `"FALSE"` → bool (case-insensitive), other text → `#VALUE!`.

Coercion helpers live in `xlstream-core::coerce`. Every function uses them — no ad-hoc conversions.

## Error propagation

```rust
Value::Error(e) + anything = Value::Error(e)   // unless stateful fn intercepts
```

Pure functions do this automatically via the `?` operator on `Result<Value, CellError>`.

Stateful functions that "catch" errors (`IFERROR`, `IFNA`) check explicitly:
```rust
match interp.eval(&args[0], scope) {
    Ok(Value::Error(_)) | Err(_) => interp.eval(&args[1], scope),
    Ok(v) => Ok(v),
}
```

## Short-circuit evaluation

`AND`, `OR`, `IF`, `IFS`, `IFERROR`, `IFNA` are stateful and short-circuit. The interpreter does NOT pre-evaluate their arguments.

This matters for correctness: `IF(A=0, 0, 1/A)` must not evaluate `1/A` when `A=0`, otherwise we return `#DIV/0!` instead of `0`.

## Thread safety

- `Interpreter` is `Send + Sync` because it holds only immutable references to `prelude` and `functions`.
- `RowScope` is `Send + Sync`-able in principle, but practically built per thread per row.
- `Prelude` is immutable after construction, shared across threads via `&Prelude`.

Row parallelism spawns N workers, each with its own `Interpreter` (cheap to clone — it's two references), each iterating its row range.

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
2. **Early exit on error.** `?` operator.
3. **No dynamic dispatch.** `fn` pointer, not `Box<dyn Fn>`.
4. **Precompiled constants.** e.g. `ln(10)` is `const LN_10: f64 = ...`.
