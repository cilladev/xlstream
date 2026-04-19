//! Conditional and logical builtin functions (Phase 6).
//!
//! All functions in this module are *stateful*: they receive unevaluated
//! `NodeRef` arguments and decide which to evaluate, enabling short-circuit
//! semantics for `IF`, `AND`, `OR`, etc.

use xlstream_core::{coerce, CellError, Value};
use xlstream_parse::NodeRef;

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// `IF(cond, then, else?)` — short-circuit conditional.
///
/// Evaluates `cond` and coerces to bool. If true, evaluates and returns
/// `then`. If false, evaluates and returns `else` (or `FALSE` if omitted).
/// The unused branch is never evaluated.
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

/// `IFS(cond1, val1, cond2, val2, ...)` — multi-branch conditional.
///
/// Evaluates conditions in order. Returns the value paired with the first
/// true condition. Short-circuits: conditions after the first match and
/// all non-matching values are never evaluated. Returns `#N/A` if no
/// condition matches.
pub(crate) fn builtin_ifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() % 2 != 0 {
        return Value::Error(CellError::Value);
    }
    for pair in args.chunks(2) {
        let cond = interp.eval(pair[0], scope);
        let cond_bool = match coerce::to_bool(&cond) {
            Ok(b) => b,
            Err(e) => return Value::Error(e),
        };
        if cond_bool {
            return interp.eval(pair[1], scope);
        }
    }
    Value::Error(CellError::Na)
}

/// `SWITCH(expr, val1, result1, val2, result2, ..., default?)` — value match.
///
/// Evaluates `expr` once, then compares against each `valN` in order using
/// Excel `=` semantics. Returns the first matching `resultN`. If no match
/// and a default is present (odd remaining arg count), returns the default.
/// Otherwise returns `#N/A`.
pub(crate) fn builtin_switch(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 3 {
        return Value::Error(CellError::Value);
    }

    let expr = interp.eval(args[0], scope);
    if let Value::Error(e) = &expr {
        return Value::Error(*e);
    }

    let remaining = &args[1..];
    let has_default = remaining.len() % 2 == 1;
    let pairs_end = if has_default { remaining.len() - 1 } else { remaining.len() };

    for pair in remaining[..pairs_end].chunks(2) {
        let val = interp.eval(pair[0], scope);
        if let Value::Error(e) = &val {
            return Value::Error(*e);
        }
        if crate::ops::values_equal(&expr, &val) {
            return interp.eval(pair[1], scope);
        }
    }

    if has_default {
        interp.eval(remaining[remaining.len() - 1], scope)
    } else {
        Value::Error(CellError::Na)
    }
}

/// `IFERROR(expr, fallback)` — catch any cell error.
///
/// Evaluates `expr`. If the result is `Value::Error(_)`, evaluates and
/// returns `fallback`. Otherwise returns the value.
pub(crate) fn builtin_iferror(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let val = interp.eval(args[0], scope);
    if matches!(val, Value::Error(_)) {
        interp.eval(args[1], scope)
    } else {
        val
    }
}

/// `IFNA(expr, fallback)` — catch only `#N/A`.
///
/// Like [`builtin_iferror`] but only intercepts `CellError::Na`. All
/// other errors propagate unchanged.
pub(crate) fn builtin_ifna(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let val = interp.eval(args[0], scope);
    if matches!(val, Value::Error(CellError::Na)) {
        interp.eval(args[1], scope)
    } else {
        val
    }
}

/// `AND(a1, a2, ..., aN)` — logical conjunction with short-circuit.
///
/// Returns `TRUE` if all arguments coerce to true. Short-circuits on the
/// first false value. Empty args return `#VALUE!`. Errors propagate.
pub(crate) fn builtin_and(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    for arg in args {
        let val = interp.eval(*arg, scope);
        match coerce::to_bool(&val) {
            Ok(false) => return Value::Bool(false),
            Ok(true) => {}
            Err(e) => return Value::Error(e),
        }
    }
    Value::Bool(true)
}

/// `OR(a1, a2, ..., aN)` — logical disjunction with short-circuit.
///
/// Returns `TRUE` if any argument coerces to true. Short-circuits on the
/// first true value. Empty args return `#VALUE!`. Errors propagate.
pub(crate) fn builtin_or(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    for arg in args {
        let val = interp.eval(*arg, scope);
        match coerce::to_bool(&val) {
            Ok(true) => return Value::Bool(true),
            Ok(false) => {}
            Err(e) => return Value::Error(e),
        }
    }
    Value::Bool(false)
}

/// `NOT(x)` — boolean inversion.
///
/// Coerces the single argument to bool and inverts it.
pub(crate) fn builtin_not(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let val = interp.eval(args[0], scope);
    match coerce::to_bool(&val) {
        Ok(b) => Value::Bool(!b),
        Err(e) => Value::Error(e),
    }
}

/// `XOR(a1, a2, ..., aN)` — exclusive or (parity).
///
/// Returns `TRUE` if an odd number of arguments coerce to true.
/// Unlike AND/OR, XOR evaluates all arguments (no short-circuit).
/// Empty args return `#VALUE!`. Errors propagate.
pub(crate) fn builtin_xor(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let mut count = 0u32;
    for arg in args {
        let val = interp.eval(*arg, scope);
        match coerce::to_bool(&val) {
            Ok(true) => count += 1,
            Ok(false) => {}
            Err(e) => return Value::Error(e),
        }
    }
    Value::Bool(count % 2 == 1)
}

/// `TRUE()` — returns `Value::Bool(true)`.
///
/// Zero-arg builtin. Returns `#VALUE!` if called with arguments.
pub(crate) fn builtin_true(args: &[NodeRef<'_>]) -> Value {
    if args.is_empty() {
        Value::Bool(true)
    } else {
        Value::Error(CellError::Value)
    }
}

/// `FALSE()` — returns `Value::Bool(false)`.
///
/// Zero-arg builtin. Returns `#VALUE!` if called with arguments.
pub(crate) fn builtin_false(args: &[NodeRef<'_>]) -> Value {
    if args.is_empty() {
        Value::Bool(false)
    } else {
        Value::Error(CellError::Value)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};
    use xlstream_parse::parse;

    use crate::{Interpreter, Prelude, RowScope};

    fn eval(formula: &str, row: &[Value]) -> Value {
        let prelude = Prelude::empty();
        let interp = Interpreter::new(&prelude);
        let ast = parse(formula).unwrap();
        let scope = RowScope::new(row, 0);
        interp.eval(ast.root(), &scope)
    }

    // ===== IF =====

    #[test]
    fn if_true_branch_returns_then_value() {
        assert_eq!(eval("IF(TRUE, 1, 2)", &[]), Value::Number(1.0));
    }

    #[test]
    fn if_false_branch_returns_else_value() {
        assert_eq!(eval("IF(FALSE, 1, 2)", &[]), Value::Number(2.0));
    }

    #[test]
    fn if_two_args_false_returns_false() {
        assert_eq!(eval("IF(FALSE, 1)", &[]), Value::Bool(false));
    }

    #[test]
    fn if_short_circuit_true_does_not_evaluate_else() {
        assert_eq!(eval("IF(TRUE, 0, 1/0)", &[]), Value::Number(0.0));
    }

    #[test]
    fn if_short_circuit_avoids_div_zero_with_cell_ref() {
        let row = vec![Value::Number(0.0)];
        assert_eq!(eval("IF(A1=0, 0, 1/A1)", &row), Value::Number(0.0));
    }

    #[test]
    fn if_error_cond_propagates() {
        assert_eq!(eval("IF(#N/A, 1, 2)", &[]), Value::Error(CellError::Na));
    }

    #[test]
    fn if_numeric_cond_zero_is_false() {
        assert_eq!(eval("IF(0, 1, 2)", &[]), Value::Number(2.0));
    }

    #[test]
    fn if_numeric_cond_nonzero_is_true() {
        assert_eq!(eval("IF(1, \"yes\", \"no\")", &[]), Value::Text("yes".into()));
    }

    #[test]
    fn if_string_cond_true() {
        assert_eq!(eval("IF(\"TRUE\", 1, 2)", &[]), Value::Number(1.0));
    }

    #[test]
    fn if_string_cond_other_returns_value_error() {
        assert_eq!(eval("IF(\"abc\", 1, 2)", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn if_wrong_arg_count_one_arg() {
        assert_eq!(eval("IF(TRUE)", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn if_wrong_arg_count_four_args() {
        assert_eq!(eval("IF(TRUE, 1, 2, 3)", &[]), Value::Error(CellError::Value));
    }

    // ===== IFS =====

    #[test]
    fn ifs_first_match_returns_value() {
        assert_eq!(eval("IFS(TRUE, \"a\", TRUE, \"b\")", &[]), Value::Text("a".into()));
    }

    #[test]
    fn ifs_second_match() {
        assert_eq!(eval("IFS(FALSE, \"a\", TRUE, \"b\")", &[]), Value::Text("b".into()));
    }

    #[test]
    fn ifs_no_match_returns_na() {
        assert_eq!(eval("IFS(FALSE, \"a\", FALSE, \"b\")", &[]), Value::Error(CellError::Na));
    }

    #[test]
    fn ifs_short_circuit_does_not_evaluate_after_match() {
        assert_eq!(eval("IFS(TRUE, 1, TRUE, 1/0)", &[]), Value::Number(1.0));
    }

    #[test]
    fn ifs_error_in_cond_propagates() {
        assert_eq!(eval("IFS(#DIV/0!, \"a\")", &[]), Value::Error(CellError::Div0));
    }

    #[test]
    fn ifs_numeric_cond_coercion() {
        assert_eq!(eval("IFS(1, \"yes\")", &[]), Value::Text("yes".into()));
    }

    #[test]
    fn ifs_odd_args_returns_value_error() {
        assert_eq!(eval("IFS(TRUE)", &[]), Value::Error(CellError::Value));
    }

    // ===== SWITCH =====

    #[test]
    fn switch_first_match() {
        assert_eq!(eval("SWITCH(1, 1, \"a\", 2, \"b\")", &[]), Value::Text("a".into()));
    }

    #[test]
    fn switch_second_match() {
        assert_eq!(eval("SWITCH(2, 1, \"a\", 2, \"b\")", &[]), Value::Text("b".into()));
    }

    #[test]
    fn switch_no_match_no_default_returns_na() {
        assert_eq!(eval("SWITCH(3, 1, \"a\", 2, \"b\")", &[]), Value::Error(CellError::Na));
    }

    #[test]
    fn switch_no_match_with_default() {
        assert_eq!(
            eval("SWITCH(3, 1, \"a\", 2, \"b\", \"default\")", &[]),
            Value::Text("default".into())
        );
    }

    #[test]
    fn switch_with_cell_ref() {
        let row = vec![Value::Number(1.0)];
        assert_eq!(eval("SWITCH(A1, 0, \"zero\", 1, \"one\")", &row), Value::Text("one".into()));
    }

    #[test]
    fn switch_error_expr_propagates() {
        assert_eq!(eval("SWITCH(#REF!, 1, \"a\")", &[]), Value::Error(CellError::Ref));
    }

    #[test]
    fn switch_text_case_insensitive() {
        assert_eq!(eval("SWITCH(\"a\", \"A\", \"matched\")", &[]), Value::Text("matched".into()));
    }

    #[test]
    fn switch_too_few_args() {
        assert_eq!(eval("SWITCH(1, 2)", &[]), Value::Error(CellError::Value));
    }

    // ===== IFERROR =====

    #[test]
    fn iferror_catches_div0() {
        assert_eq!(eval("IFERROR(1/0, \"err\")", &[]), Value::Text("err".into()));
    }

    #[test]
    fn iferror_catches_value_error() {
        assert_eq!(eval("IFERROR(#VALUE!, \"err\")", &[]), Value::Text("err".into()));
    }

    #[test]
    fn iferror_catches_na() {
        assert_eq!(eval("IFERROR(#N/A, \"fallback\")", &[]), Value::Text("fallback".into()));
    }

    #[test]
    fn iferror_passthrough_non_error() {
        assert_eq!(eval("IFERROR(42, \"err\")", &[]), Value::Number(42.0));
    }

    #[test]
    fn iferror_passthrough_text() {
        assert_eq!(eval("IFERROR(\"ok\", \"err\")", &[]), Value::Text("ok".into()));
    }

    #[test]
    fn iferror_wrong_arg_count() {
        assert_eq!(eval("IFERROR(1)", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn iferror_fallback_can_be_error() {
        assert_eq!(eval("IFERROR(1/0, #N/A)", &[]), Value::Error(CellError::Na));
    }

    // ===== IFNA =====

    #[test]
    fn ifna_catches_na() {
        assert_eq!(eval("IFNA(#N/A, \"fallback\")", &[]), Value::Text("fallback".into()));
    }

    #[test]
    fn ifna_does_not_catch_div0() {
        assert_eq!(eval("IFNA(1/0, \"fallback\")", &[]), Value::Error(CellError::Div0));
    }

    #[test]
    fn ifna_does_not_catch_value_error() {
        assert_eq!(eval("IFNA(#VALUE!, \"fallback\")", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn ifna_passthrough_non_error() {
        assert_eq!(eval("IFNA(42, \"fallback\")", &[]), Value::Number(42.0));
    }

    #[test]
    fn ifna_wrong_arg_count() {
        assert_eq!(eval("IFNA(1)", &[]), Value::Error(CellError::Value));
    }

    // ===== AND =====

    #[test]
    fn and_all_true() {
        assert_eq!(eval("AND(TRUE, TRUE)", &[]), Value::Bool(true));
    }

    #[test]
    fn and_one_false() {
        assert_eq!(eval("AND(TRUE, FALSE)", &[]), Value::Bool(false));
    }

    #[test]
    fn and_short_circuit_on_false() {
        assert_eq!(eval("AND(FALSE, 1/0)", &[]), Value::Bool(false));
    }

    #[test]
    fn and_empty_args_returns_value_error() {
        assert_eq!(eval("AND()", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn and_numeric_coercion() {
        assert_eq!(eval("AND(1, 1, 1)", &[]), Value::Bool(true));
    }

    #[test]
    fn and_numeric_zero_is_false() {
        assert_eq!(eval("AND(1, 0)", &[]), Value::Bool(false));
    }

    #[test]
    fn and_error_propagation() {
        assert_eq!(eval("AND(#N/A, TRUE)", &[]), Value::Error(CellError::Na));
    }

    #[test]
    fn and_string_coercion_true() {
        assert_eq!(eval("AND(\"TRUE\", \"true\")", &[]), Value::Bool(true));
    }

    #[test]
    fn and_string_non_bool_returns_value_error() {
        assert_eq!(eval("AND(\"abc\")", &[]), Value::Error(CellError::Value));
    }

    // ===== OR =====

    #[test]
    fn or_all_false() {
        assert_eq!(eval("OR(FALSE, FALSE)", &[]), Value::Bool(false));
    }

    #[test]
    fn or_one_true() {
        assert_eq!(eval("OR(FALSE, TRUE)", &[]), Value::Bool(true));
    }

    #[test]
    fn or_short_circuit_on_true() {
        assert_eq!(eval("OR(TRUE, 1/0)", &[]), Value::Bool(true));
    }

    #[test]
    fn or_empty_args_returns_value_error() {
        assert_eq!(eval("OR()", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn or_numeric_coercion() {
        assert_eq!(eval("OR(0, 0, 1)", &[]), Value::Bool(true));
    }

    #[test]
    fn or_all_zero_is_false() {
        assert_eq!(eval("OR(0, 0)", &[]), Value::Bool(false));
    }

    #[test]
    fn or_error_propagation() {
        assert_eq!(eval("OR(#DIV/0!, FALSE)", &[]), Value::Error(CellError::Div0));
    }

    // ===== NOT =====

    #[test]
    fn not_true_returns_false() {
        assert_eq!(eval("NOT(TRUE)", &[]), Value::Bool(false));
    }

    #[test]
    fn not_false_returns_true() {
        assert_eq!(eval("NOT(FALSE)", &[]), Value::Bool(true));
    }

    #[test]
    fn not_numeric_zero_returns_true() {
        assert_eq!(eval("NOT(0)", &[]), Value::Bool(true));
    }

    #[test]
    fn not_numeric_nonzero_returns_false() {
        assert_eq!(eval("NOT(1)", &[]), Value::Bool(false));
    }

    #[test]
    fn not_error_propagates() {
        assert_eq!(eval("NOT(#N/A)", &[]), Value::Error(CellError::Na));
    }

    #[test]
    fn not_wrong_arg_count_zero() {
        assert_eq!(eval("NOT()", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn not_wrong_arg_count_two() {
        assert_eq!(eval("NOT(TRUE, FALSE)", &[]), Value::Error(CellError::Value));
    }

    // ===== XOR =====

    #[test]
    fn xor_one_true_returns_true() {
        assert_eq!(eval("XOR(TRUE, FALSE)", &[]), Value::Bool(true));
    }

    #[test]
    fn xor_two_true_returns_false() {
        assert_eq!(eval("XOR(TRUE, TRUE)", &[]), Value::Bool(false));
    }

    #[test]
    fn xor_all_false_returns_false() {
        assert_eq!(eval("XOR(FALSE, FALSE)", &[]), Value::Bool(false));
    }

    #[test]
    fn xor_three_true_returns_true() {
        assert_eq!(eval("XOR(TRUE, TRUE, TRUE)", &[]), Value::Bool(true));
    }

    #[test]
    fn xor_empty_args_returns_value_error() {
        assert_eq!(eval("XOR()", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn xor_error_propagation() {
        assert_eq!(eval("XOR(#DIV/0!, TRUE)", &[]), Value::Error(CellError::Div0));
    }

    #[test]
    fn xor_numeric_coercion() {
        assert_eq!(eval("XOR(1, 0)", &[]), Value::Bool(true));
    }

    // ===== TRUE / FALSE =====

    #[test]
    fn true_returns_bool_true() {
        assert_eq!(eval("TRUE()", &[]), Value::Bool(true));
    }

    #[test]
    fn false_returns_bool_false() {
        assert_eq!(eval("FALSE()", &[]), Value::Bool(false));
    }

    #[test]
    fn true_with_args_returns_value_error() {
        assert_eq!(eval("TRUE(1)", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn false_with_args_returns_value_error() {
        assert_eq!(eval("FALSE(1)", &[]), Value::Error(CellError::Value));
    }

    #[test]
    fn true_function_in_arithmetic() {
        assert_eq!(eval("TRUE()+1", &[]), Value::Number(2.0));
    }

    #[test]
    fn false_function_in_arithmetic() {
        assert_eq!(eval("FALSE()+1", &[]), Value::Number(1.0));
    }
}
