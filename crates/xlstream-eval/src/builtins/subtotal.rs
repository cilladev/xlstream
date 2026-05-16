//! SUBTOTAL and AGGREGATE meta-dispatch functions.

use xlstream_core::{CellError, Value};
use xlstream_parse::NodeRef;

use crate::builtins::{aggregate, expand_range, statistical};
use crate::interp::Interpreter;
use crate::scope::RowScope;

/// Resolve SUBTOTAL `function_num` (1-11 or 101-111) to a canonical 1-11 index.
///
/// Returns `None` for out-of-range or non-finite values.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn resolve_subtotal_fn(raw: f64) -> Option<u8> {
    if !raw.is_finite() {
        return None;
    }
    let n = raw as i32;
    match n {
        1..=11 => Some(n as u8),
        101..=111 => Some((n - 100) as u8),
        _ => None,
    }
}

/// Dispatch a resolved `function_num` (1-11) against a collected value slice.
fn dispatch_subtotal(fn_num: u8, values: &[Value]) -> Value {
    match fn_num {
        1 => aggregate::average(values).unwrap_or_else(Value::Error),
        2 => aggregate::count(values).unwrap_or_else(Value::Error),
        3 => aggregate::counta(values).unwrap_or_else(Value::Error),
        4 => aggregate::max(values).unwrap_or_else(Value::Error),
        5 => aggregate::min(values).unwrap_or_else(Value::Error),
        6 => aggregate::product(values).unwrap_or_else(Value::Error),
        7 => statistical::stdev_s(values).map_or_else(Value::Error, Value::Number),
        8 => statistical::stdev_p(values).map_or_else(Value::Error, Value::Number),
        9 => aggregate::sum(values).unwrap_or_else(Value::Error),
        10 => statistical::var_s(values).map_or_else(Value::Error, Value::Number),
        11 => statistical::var_p(values).map_or_else(Value::Error, Value::Number),
        _ => Value::Error(CellError::Value),
    }
}

/// Dispatch AGGREGATE `function_num` (1-13) against a collected value slice.
///
/// `function_num` 14-19 are not implemented in v0.3 — returns `#VALUE!`.
fn dispatch_aggregate(fn_num: u8, values: &[Value]) -> Value {
    match fn_num {
        1..=11 => dispatch_subtotal(fn_num, values),
        12 => aggregate::median(values).unwrap_or_else(Value::Error),
        13 => statistical::mode_sngl(values).map_or_else(Value::Error, Value::Number),
        // 14-19 (LARGE, SMALL, PERCENTILE, QUARTILE) deferred to v0.4
        _ => Value::Error(CellError::Value),
    }
}

/// Filter errors out of a value slice (for AGGREGATE "ignore errors" options).
fn filter_errors(values: &[Value]) -> Vec<Value> {
    values.iter().filter(|v| !matches!(v, Value::Error(_))).cloned().collect()
}

/// Returns true if the AGGREGATE options value means "ignore errors."
///
/// Options bit layout:
/// - 0: ignore nothing (only nested SUBTOTAL/AGGREGATE — deferred)
/// - 1: ignore hidden rows + nested
/// - 2: ignore errors + nested
/// - 3: ignore hidden + errors + nested
/// - 4: ignore nothing
/// - 5: ignore hidden rows
/// - 6: ignore errors
/// - 7: ignore hidden + errors
fn options_ignore_errors(options: u8) -> bool {
    matches!(options, 2 | 3 | 6 | 7)
}

/// `SUBTOTAL(function_num, ref1, [ref2], ...)` — multi-mode aggregate.
///
/// Dispatches to one of 11 aggregate functions based on `function_num`.
/// `function_num` 1-11 maps to AVERAGE, COUNT, COUNTA, MAX, MIN, PRODUCT,
/// STDEV.S, STDEV.P, SUM, VAR.S, VAR.P. `function_num` 101-111 behaves
/// identically in v0.3 (hidden-row awareness not implemented).
///
/// # Errors
///
/// Returns `#VALUE!` if fewer than 2 args, or `function_num` is out of range.
/// Propagates errors from the `function_num` argument or the underlying
/// aggregate function.
///
/// # Examples
///
/// ```
/// // SUBTOTAL is dispatched through the interpreter; see unit tests
/// // for direct dispatch_subtotal calls.
/// ```
pub(crate) fn builtin_subtotal(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 {
        return Value::Error(CellError::Value);
    }
    let fn_num_val = interp.eval(args[0], scope);
    let raw = match xlstream_core::coerce::to_number(&fn_num_val) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let Some(fn_num) = resolve_subtotal_fn(raw) else {
        return Value::Error(CellError::Value);
    };
    let values: Vec<Value> =
        args[1..].iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    dispatch_subtotal(fn_num, &values)
}

/// `AGGREGATE(function_num, options, ref1, [ref2], ...)` — extended SUBTOTAL.
///
/// Dispatches to one of 13 aggregate functions (1-13). Options 0-7
/// control what to ignore; v0.3 implements error-ignoring (options 2, 3,
/// 6, 7) and treats hidden-row/nested-SUBTOTAL bits as no-ops.
///
/// # Errors
///
/// Returns `#VALUE!` if fewer than 3 args, `function_num` not in 1-19, or
/// options not in 0-7. Propagates errors from scalar args.
///
/// # Examples
///
/// ```
/// // AGGREGATE is dispatched through the interpreter; see unit tests.
/// ```
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn builtin_aggregate(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 3 {
        return Value::Error(CellError::Value);
    }
    let fn_val = interp.eval(args[0], scope);
    let raw_fn = match xlstream_core::coerce::to_number(&fn_val) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    if !raw_fn.is_finite() {
        return Value::Error(CellError::Value);
    }
    let fn_num = raw_fn as i32;
    if !(1..=19).contains(&fn_num) {
        return Value::Error(CellError::Value);
    }

    let opt_val = interp.eval(args[1], scope);
    let raw_opt = match xlstream_core::coerce::to_number(&opt_val) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    if !raw_opt.is_finite() {
        return Value::Error(CellError::Value);
    }
    let options = raw_opt as i32;
    if !(0..=7).contains(&options) {
        return Value::Error(CellError::Value);
    }

    let values: Vec<Value> =
        args[2..].iter().flat_map(|&a| expand_range(a, interp, scope)).collect();

    let effective =
        if options_ignore_errors(options as u8) { filter_errors(&values) } else { values };

    dispatch_aggregate(fn_num as u8, &effective)
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]

    use xlstream_core::{CellError, Value};

    use super::*;

    fn nums(vals: &[f64]) -> Vec<Value> {
        vals.iter().map(|&n| Value::Number(n)).collect()
    }

    // ===== resolve_subtotal_fn =====

    #[test]
    fn resolve_subtotal_fn_valid_range() {
        for i in 1..=11 {
            assert_eq!(resolve_subtotal_fn(f64::from(i)), Some(i as u8));
        }
    }

    #[test]
    fn resolve_subtotal_fn_hidden_row_range() {
        for i in 101..=111 {
            assert_eq!(resolve_subtotal_fn(f64::from(i)), Some((i - 100) as u8));
        }
    }

    #[test]
    fn resolve_subtotal_fn_out_of_range() {
        assert_eq!(resolve_subtotal_fn(0.0), None);
        assert_eq!(resolve_subtotal_fn(12.0), None);
        assert_eq!(resolve_subtotal_fn(100.0), None);
        assert_eq!(resolve_subtotal_fn(112.0), None);
    }

    #[test]
    fn resolve_subtotal_fn_non_finite() {
        assert_eq!(resolve_subtotal_fn(f64::NAN), None);
        assert_eq!(resolve_subtotal_fn(f64::INFINITY), None);
        assert_eq!(resolve_subtotal_fn(f64::NEG_INFINITY), None);
    }

    // ===== dispatch_subtotal: all 11 modes =====

    #[test]
    fn subtotal_1_average() {
        assert_eq!(dispatch_subtotal(1, &nums(&[1.0, 2.0, 3.0, 4.0, 5.0])), Value::Number(3.0));
    }

    #[test]
    fn subtotal_2_count() {
        let vals = vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Text("a".into()),
            Value::Bool(true),
            Value::Number(5.0),
        ];
        assert_eq!(dispatch_subtotal(2, &vals), Value::Number(3.0));
    }

    #[test]
    fn subtotal_3_counta() {
        let vals = vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Text("a".into()),
            Value::Bool(true),
            Value::Number(5.0),
        ];
        assert_eq!(dispatch_subtotal(3, &vals), Value::Number(5.0));
    }

    #[test]
    fn subtotal_4_max() {
        assert_eq!(dispatch_subtotal(4, &nums(&[1.0, 2.0, 3.0, 4.0, 5.0])), Value::Number(5.0));
    }

    #[test]
    fn subtotal_5_min() {
        assert_eq!(dispatch_subtotal(5, &nums(&[1.0, 2.0, 3.0, 4.0, 5.0])), Value::Number(1.0));
    }

    #[test]
    fn subtotal_6_product() {
        assert_eq!(dispatch_subtotal(6, &nums(&[1.0, 2.0, 3.0, 4.0, 5.0])), Value::Number(120.0));
    }

    #[test]
    fn subtotal_7_stdev_s() {
        let v = dispatch_subtotal(7, &nums(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]));
        match v {
            Value::Number(n) => assert!((n - 2.138_089_935_299_395).abs() < 1e-6),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn subtotal_8_stdev_p() {
        let v = dispatch_subtotal(8, &nums(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]));
        match v {
            Value::Number(n) => assert!((n - 2.0).abs() < 1e-6),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn subtotal_9_sum() {
        assert_eq!(dispatch_subtotal(9, &nums(&[1.0, 2.0, 3.0, 4.0, 5.0])), Value::Number(15.0));
    }

    #[test]
    fn subtotal_10_var_s() {
        let v = dispatch_subtotal(10, &nums(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]));
        match v {
            Value::Number(n) => assert!((n - 4.571_428_571_428_571).abs() < 1e-6),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn subtotal_11_var_p() {
        let v = dispatch_subtotal(11, &nums(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]));
        match v {
            Value::Number(n) => assert!((n - 4.0).abs() < 1e-6),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    // ===== SUBTOTAL 101-111 alias =====

    #[test]
    fn subtotal_109_same_as_9() {
        assert_eq!(resolve_subtotal_fn(109.0), Some(9));
        assert_eq!(dispatch_subtotal(9, &nums(&[1.0, 2.0, 3.0])), Value::Number(6.0));
    }

    // ===== dispatch_subtotal errors =====

    #[test]
    fn subtotal_error_propagation() {
        let vals = vec![Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(dispatch_subtotal(9, &vals), Value::Error(CellError::Na));
    }

    #[test]
    fn subtotal_invalid_fn_num() {
        assert_eq!(dispatch_subtotal(0, &nums(&[1.0])), Value::Error(CellError::Value));
        assert_eq!(dispatch_subtotal(12, &nums(&[1.0])), Value::Error(CellError::Value));
    }

    // ===== dispatch_aggregate: basic =====

    #[test]
    fn aggregate_9_sum() {
        assert_eq!(dispatch_aggregate(9, &nums(&[1.0, 2.0, 3.0])), Value::Number(6.0));
    }

    #[test]
    fn aggregate_1_average() {
        assert_eq!(dispatch_aggregate(1, &nums(&[1.0, 2.0, 3.0])), Value::Number(2.0));
    }

    #[test]
    fn aggregate_4_max() {
        assert_eq!(dispatch_aggregate(4, &nums(&[1.0, 5.0, 3.0])), Value::Number(5.0));
    }

    #[test]
    fn aggregate_12_median() {
        assert_eq!(dispatch_aggregate(12, &nums(&[1.0, 3.0, 5.0, 7.0, 9.0])), Value::Number(5.0));
    }

    #[test]
    fn aggregate_13_mode_sngl() {
        assert_eq!(dispatch_aggregate(13, &nums(&[1.0, 2.0, 2.0, 3.0])), Value::Number(2.0));
    }

    // ===== filter_errors =====

    #[test]
    fn filter_errors_removes_errors() {
        let vals = vec![Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        let filtered = filter_errors(&vals);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0], Value::Number(1.0));
        assert_eq!(filtered[1], Value::Number(3.0));
    }

    #[test]
    fn filter_errors_preserves_non_errors() {
        let vals = vec![Value::Number(1.0), Value::Text("x".into()), Value::Bool(true)];
        assert_eq!(filter_errors(&vals).len(), 3);
    }

    // ===== options_ignore_errors =====

    #[test]
    fn options_ignore_errors_correct() {
        assert!(!options_ignore_errors(0));
        assert!(!options_ignore_errors(1));
        assert!(options_ignore_errors(2));
        assert!(options_ignore_errors(3));
        assert!(!options_ignore_errors(4));
        assert!(!options_ignore_errors(5));
        assert!(options_ignore_errors(6));
        assert!(options_ignore_errors(7));
    }

    // ===== AGGREGATE with error ignoring =====

    #[test]
    fn aggregate_sum_ignore_errors() {
        let vals = vec![Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        let filtered = filter_errors(&vals);
        assert_eq!(dispatch_aggregate(9, &filtered), Value::Number(4.0));
    }

    #[test]
    fn aggregate_average_ignore_errors() {
        let vals = vec![Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        let filtered = filter_errors(&vals);
        assert_eq!(dispatch_aggregate(1, &filtered), Value::Number(2.0));
    }

    #[test]
    fn aggregate_sum_no_ignore_propagates() {
        let vals = vec![Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(dispatch_aggregate(9, &vals), Value::Error(CellError::Na));
    }

    // ===== AGGREGATE invalid function_num =====

    #[test]
    fn aggregate_fn_0_returns_value_error() {
        assert_eq!(dispatch_aggregate(0, &nums(&[1.0])), Value::Error(CellError::Value));
    }

    #[test]
    fn aggregate_fn_14_returns_value_error() {
        assert_eq!(dispatch_aggregate(14, &nums(&[1.0])), Value::Error(CellError::Value));
    }
}
