//! Builtin function dispatch.
//!
//! Entry point: `dispatch`. The interpreter calls this for every
//! `NodeView::Function` node. Returns `Some(value)` if the function is
//! known, `None` otherwise (caller falls back to `#VALUE!`).

pub mod aggregate;
mod compatibility;
pub(crate) mod conditional;
pub mod convert;
mod database;
pub(crate) mod date;
pub mod engineering;
pub mod financial;
pub mod info;
pub(crate) mod lookup;
pub(crate) mod math;
pub(crate) mod multi_conditional;
mod specfn;
pub mod statistical;
pub(crate) mod string;
pub(crate) mod subtotal;

use xlstream_core::{coerce, CellError, Value};
use xlstream_parse::{NodeRef, NodeView};

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// Evaluate all arguments eagerly, returning a `Vec<Value>`.
///
/// Used by pure builtins (string, math, etc.) that don't need
/// short-circuit evaluation.
pub(crate) fn eval_args(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Vec<Value> {
    args.iter().map(|a| interp.eval(*a, scope)).collect()
}

/// Expand a single AST node into a `Vec<Value>`.
///
/// For `RangeRef` nodes this resolves from lookup sheets or the prelude
/// bounded range cache. For any other node it evaluates normally and
/// returns a single-element vec.
pub(crate) fn expand_range(
    node: NodeRef<'_>,
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Vec<Value> {
    use xlstream_core::CellError;
    use xlstream_parse::NodeView;

    match node.view() {
        NodeView::RangeRef { sheet, start_row, end_row, start_col, .. } => {
            let sc = start_col.unwrap_or(1);

            // Try lookup sheet (already fully loaded).
            if let Some(sheet_name) = sheet {
                if let Some(ls) = interp.prelude().lookup_sheet(sheet_name) {
                    let sr = start_row.map_or(0, |r| (r - 1) as usize);
                    let er = end_row.map_or(ls.num_rows().saturating_sub(1), |r| (r - 1) as usize);
                    let col = (sc - 1) as usize;
                    return (sr..=er)
                        .map(|r| ls.cell(r, col).cloned().unwrap_or(Value::Empty))
                        .collect();
                }
            }

            // Fall back to cached bounded range.
            if let (Some(sr), Some(er)) = (start_row, end_row) {
                let key = crate::prelude::BoundedRangeKey {
                    sheet: sheet
                        .map(ToString::to_string)
                        .or_else(|| interp.current_sheet().map(ToString::to_string)),
                    col: sc,
                    start_row: sr,
                    end_row: er,
                };
                if let Some(values) = interp.prelude().get_cached_range(&key) {
                    return values.clone();
                }
            }

            vec![Value::Error(CellError::Ref)]
        }
        _ => vec![interp.eval(node, scope)],
    }
}

/// Expand args for aggregate builtins with Excel's scalar coercion.
///
/// Range args use range semantics (bools/text skipped by the aggregate
/// function itself). Scalar args are coerced to numbers via
/// [`coerce::to_number`]: `TRUE`→1, `FALSE`→0, numeric text→number,
/// non-numeric text→`#VALUE!`, errors propagate. This matches Excel
/// where `SUM(TRUE,1)` = 2 but `SUM(A1:A2)` with A1=TRUE, A2=1 = 1.
pub(crate) fn expand_args_for_aggregate(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Vec<Value> {
    args.iter()
        .flat_map(|&a| {
            if matches!(a.view(), NodeView::RangeRef { .. }) {
                expand_range(a, interp, scope)
            } else {
                let val = interp.eval(a, scope);
                let coerced = match coerce::to_number(&val) {
                    Ok(n) => Value::Number(n),
                    Err(e) => Value::Error(e),
                };
                vec![coerced]
            }
        })
        .collect()
}

/// `SUMPRODUCT(array1, array2, ...)` — sum of element-wise products.
///
/// Expands each arg via [`expand_range`] to collect arrays, then
/// delegates to [`aggregate::sumproduct`].
pub(crate) fn handle_sumproduct(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }

    let arrays: Vec<Vec<Value>> = args.iter().map(|&a| expand_range(a, interp, scope)).collect();
    let slices: Vec<&[Value]> = arrays.iter().map(Vec::as_slice).collect();
    aggregate::sumproduct(&slices)
}

/// `VAR.S(range, ...)` — sample variance. Range-expanding.
pub(crate) fn handle_var_s(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::var_s(&values).map_or_else(Value::Error, Value::Number)
}

/// `VAR.P(range, ...)` — population variance. Range-expanding.
pub(crate) fn handle_var_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::var_p(&values).map_or_else(Value::Error, Value::Number)
}

/// `STDEV.S(range, ...)` — sample standard deviation. Range-expanding.
pub(crate) fn handle_stdev_s(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::stdev_s(&values).map_or_else(Value::Error, Value::Number)
}

/// `STDEV.P(range, ...)` — population standard deviation. Range-expanding.
pub(crate) fn handle_stdev_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::stdev_p(&values).map_or_else(Value::Error, Value::Number)
}

/// `SKEW(range, ...)` — sample skewness. Range-expanding.
pub(crate) fn handle_skew(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::skew(&values).map_or_else(Value::Error, Value::Number)
}

/// `SKEW.P(range, ...)` — population skewness. Range-expanding.
pub(crate) fn handle_skew_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::skew_p(&values).map_or_else(Value::Error, Value::Number)
}

/// `KURT(range, ...)` — excess kurtosis. Range-expanding.
pub(crate) fn handle_kurt(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::kurt(&values).map_or_else(Value::Error, Value::Number)
}

/// `MODE.SNGL(range, ...)` — most frequent value. Range-expanding.
pub(crate) fn handle_mode_sngl(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::mode_sngl(&values).map_or_else(Value::Error, Value::Number)
}

/// `AVEDEV(range, ...)` — average absolute deviation. Range-expanding.
pub(crate) fn handle_avedev(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::builtin_avedev(&values).map_or_else(Value::Error, Value::Number)
}

/// `PERCENTILE.INC(range, k)` — inclusive percentile. Two-arg: range + scalar.
pub(crate) fn handle_percentile_inc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = expand_range(args[0], interp, scope);
    let k = match xlstream_core::coerce::to_number(&interp.eval(args[1], scope)) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    statistical::percentile_inc(&values, k).map_or_else(Value::Error, Value::Number)
}

/// `PERCENTILE.EXC(range, k)` — exclusive percentile. Two-arg: range + scalar.
pub(crate) fn handle_percentile_exc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = expand_range(args[0], interp, scope);
    let k = match xlstream_core::coerce::to_number(&interp.eval(args[1], scope)) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    statistical::percentile_exc(&values, k).map_or_else(Value::Error, Value::Number)
}

/// `QUARTILE.INC(range, quart)` — inclusive quartile. Two-arg: range + scalar.
pub(crate) fn handle_quartile_inc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = expand_range(args[0], interp, scope);
    let q = match xlstream_core::coerce::to_number(&interp.eval(args[1], scope)) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    if !(0.0..=4.0).contains(&q) {
        return Value::Error(CellError::Num);
    }
    #[allow(clippy::cast_possible_truncation)]
    let quart = q as i32;
    statistical::quartile_inc(&values, quart).map_or_else(Value::Error, Value::Number)
}

/// `QUARTILE.EXC(range, quart)` — exclusive quartile. Two-arg: range + scalar.
pub(crate) fn handle_quartile_exc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = expand_range(args[0], interp, scope);
    let q = match xlstream_core::coerce::to_number(&interp.eval(args[1], scope)) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    if !(1.0..=3.0).contains(&q) {
        return Value::Error(CellError::Num);
    }
    #[allow(clippy::cast_possible_truncation)]
    let quart = q as i32;
    statistical::quartile_exc(&values, quart).map_or_else(Value::Error, Value::Number)
}

/// `LARGE(array, k)` / `SMALL(array, k)` — k-th largest or smallest.
pub(crate) fn handle_large_small(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
    descending: bool,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = expand_range(args[0], interp, scope);
    let k_val = interp.eval(args[1], scope);
    let f = if descending { statistical::large } else { statistical::small };
    f(&values, &k_val).map_or_else(Value::Error, Value::Number)
}

/// `RANK.EQ(number, ref, [order])` — rank with ties getting top rank.
pub(crate) fn handle_rank_eq(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let number = match coerce::to_number(&interp.eval(args[0], scope)) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let range_values = expand_range(args[1], interp, scope);
    let nums = match statistical::collect_numerics(&range_values) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let ascending = if args.len() == 3 {
        match coerce::to_number(&interp.eval(args[2], scope)) {
            Ok(o) => o != 0.0,
            Err(e) => return Value::Error(e),
        }
    } else {
        false
    };
    statistical::rank_eq(number, &nums, ascending).map_or_else(Value::Error, Value::Number)
}

/// `RANK.AVG(number, ref, [order])` — rank with ties getting average rank.
pub(crate) fn handle_rank_avg(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let number = match coerce::to_number(&interp.eval(args[0], scope)) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let range_values = expand_range(args[1], interp, scope);
    let nums = match statistical::collect_numerics(&range_values) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let ascending = if args.len() == 3 {
        match coerce::to_number(&interp.eval(args[2], scope)) {
            Ok(o) => o != 0.0,
            Err(e) => return Value::Error(e),
        }
    } else {
        false
    };
    statistical::rank_avg(number, &nums, ascending).map_or_else(Value::Error, Value::Number)
}

/// `EXPON.DIST(x, lambda, cumulative)` — exponential distribution.
pub(crate) fn handle_expon_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let evaled = eval_args(args, interp, scope);
    if evaled.len() != 3 {
        return Value::Error(CellError::Value);
    }
    let x = match coerce::to_number(&evaled[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let lambda = match coerce::to_number(&evaled[1]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let cumulative = match coerce::to_bool(&evaled[2]) {
        Ok(b) => b,
        Err(e) => return Value::Error(e),
    };
    statistical::expon_dist(x, lambda, cumulative).map_or_else(Value::Error, Value::Number)
}

/// `CORREL(array1, array2)` — Pearson correlation coefficient. Two-arg range-expanding.
pub(crate) fn handle_correl(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let xs = expand_range(args[0], interp, scope);
    let ys = expand_range(args[1], interp, scope);
    statistical::correl(&xs, &ys).map_or_else(Value::Error, Value::Number)
}

/// `COVARIANCE.P(array1, array2)` — population covariance. Two-arg range-expanding.
pub(crate) fn handle_covariance_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let xs = expand_range(args[0], interp, scope);
    let ys = expand_range(args[1], interp, scope);
    statistical::covariance_p(&xs, &ys).map_or_else(Value::Error, Value::Number)
}

/// `COVARIANCE.S(array1, array2)` — sample covariance. Two-arg range-expanding.
pub(crate) fn handle_covariance_s(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let xs = expand_range(args[0], interp, scope);
    let ys = expand_range(args[1], interp, scope);
    statistical::covariance_s(&xs, &ys).map_or_else(Value::Error, Value::Number)
}

/// `SLOPE(known_ys, known_xs)` — slope of least-squares regression line.
pub(crate) fn handle_slope(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let ys = expand_range(args[0], interp, scope);
    let xs = expand_range(args[1], interp, scope);
    statistical::slope(&ys, &xs).map_or_else(Value::Error, Value::Number)
}

/// `INTERCEPT(known_ys, known_xs)` — y-intercept of least-squares line.
pub(crate) fn handle_intercept(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let ys = expand_range(args[0], interp, scope);
    let xs = expand_range(args[1], interp, scope);
    statistical::intercept(&ys, &xs).map_or_else(Value::Error, Value::Number)
}

/// `RSQ(known_ys, known_xs)` — coefficient of determination.
pub(crate) fn handle_rsq(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let ys = expand_range(args[0], interp, scope);
    let xs = expand_range(args[1], interp, scope);
    statistical::rsq(&ys, &xs).map_or_else(Value::Error, Value::Number)
}

/// `FORECAST.LINEAR(x, known_ys, known_xs)` — predict Y from X.
pub(crate) fn handle_forecast_linear(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 3 {
        return Value::Error(CellError::Value);
    }
    let x = match coerce::to_number(&interp.eval(args[0], scope)) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let ys = expand_range(args[1], interp, scope);
    let xs = expand_range(args[2], interp, scope);
    statistical::forecast_linear(x, &ys, &xs).map_or_else(Value::Error, Value::Number)
}

// ---------------------------------------------------------------------------
// Handler wrappers for the centralized registry.
//
// Each wrapper has the uniform signature
//   fn(&[NodeRef<'_>], &Interpreter<'_>, &RowScope<'_>) -> Value
// and delegates to the underlying builtin implementation.
// ---------------------------------------------------------------------------

// -- Conditional --

pub(crate) fn handle_true(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_true(args)
}

pub(crate) fn handle_false(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_false(args)
}

// -- Multi-conditional aggregates --

// -- Simple aggregates --

// -- Lookup --

// -- Date (volatile) --

// -- Date (pure, eager eval) --

// -- Date (range-expanding) --

// -- String (pure, eager eval) --

// -- String (range-expanding) --

// -- Math (pure, eager eval) --

// -- Info (pure, eager eval) --

pub(crate) fn handle_isref(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let is_ref = matches!(args[0].view(), NodeView::CellRef { .. } | NodeView::RangeRef { .. });
    Value::Bool(is_ref)
}

// -- Info (range metadata) --

pub(crate) fn handle_row(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() > 1 {
        return Value::Error(CellError::Value);
    }
    if args.is_empty() {
        return Value::Number(f64::from(scope.row_idx().saturating_add(1)));
    }
    info::builtin_row(args[0])
}

pub(crate) fn handle_column(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() > 1 {
        return Value::Error(CellError::Value);
    }
    if args.is_empty() {
        return Value::Number(f64::from(scope.col_idx().saturating_add(1)));
    }
    info::builtin_column(args[0])
}

pub(crate) fn handle_rows(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    info::builtin_rows(args[0])
}

pub(crate) fn handle_columns(
    args: &[NodeRef<'_>],
    _interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    info::builtin_columns(args[0])
}

// -- Financial (pure, eager eval) --

// -- Financial (range-expanding) --

// -- Meta-dispatch (SUBTOTAL, AGGREGATE) --

// -- Statistical (range-expanding) --

pub(crate) fn handle_large(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    handle_large_small(args, interp, scope, true)
}

pub(crate) fn handle_small(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    handle_large_small(args, interp, scope, false)
}

// -- Engineering (pure, eager eval) --

// -- Conversion --
