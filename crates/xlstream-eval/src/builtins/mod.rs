//! Builtin function dispatch.
//!
//! Entry point: `dispatch`. The interpreter calls this for every
//! `NodeView::Function` node. Returns `Some(value)` if the function is
//! known, `None` otherwise (caller falls back to `#VALUE!`).

pub mod aggregate;
mod compatibility;
mod conditional;
pub mod convert;
mod database;
pub(crate) mod date;
pub mod engineering;
pub mod financial;
pub mod info;
mod lookup;
pub(crate) mod math;
mod multi_conditional;
mod specfn;
pub mod statistical;
pub(crate) mod string;
mod subtotal;

use xlstream_core::{coerce, CellError, Value};
use xlstream_parse::{NodeRef, NodeView};

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// Evaluate all arguments eagerly, returning a `Vec<Value>`.
///
/// Used by pure builtins (string, math, etc.) that don't need
/// short-circuit evaluation.
fn eval_args(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Vec<Value> {
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
fn expand_args_for_aggregate(
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
fn builtin_sumproduct(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }

    let arrays: Vec<Vec<Value>> = args.iter().map(|&a| expand_range(a, interp, scope)).collect();
    let slices: Vec<&[Value]> = arrays.iter().map(Vec::as_slice).collect();
    aggregate::sumproduct(&slices).unwrap_or_else(Value::Error)
}

/// `VAR.S(range, ...)` — sample variance. Range-expanding.
fn builtin_var_s(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::var_s(&values).map_or_else(Value::Error, Value::Number)
}

/// `VAR.P(range, ...)` — population variance. Range-expanding.
fn builtin_var_p(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::var_p(&values).map_or_else(Value::Error, Value::Number)
}

/// `STDEV.S(range, ...)` — sample standard deviation. Range-expanding.
fn builtin_stdev_s(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::stdev_s(&values).map_or_else(Value::Error, Value::Number)
}

/// `STDEV.P(range, ...)` — population standard deviation. Range-expanding.
fn builtin_stdev_p(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::stdev_p(&values).map_or_else(Value::Error, Value::Number)
}

/// `SKEW(range, ...)` — sample skewness. Range-expanding.
fn builtin_skew(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::skew(&values).map_or_else(Value::Error, Value::Number)
}

/// `SKEW.P(range, ...)` — population skewness. Range-expanding.
fn builtin_skew_p(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::skew_p(&values).map_or_else(Value::Error, Value::Number)
}

/// `KURT(range, ...)` — excess kurtosis. Range-expanding.
fn builtin_kurt(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::kurt(&values).map_or_else(Value::Error, Value::Number)
}

/// `MODE.SNGL(range, ...)` — most frequent value. Range-expanding.
fn builtin_mode_sngl(
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
fn builtin_avedev(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    let values: Vec<Value> = args.iter().flat_map(|&a| expand_range(a, interp, scope)).collect();
    statistical::builtin_avedev(&values).map_or_else(Value::Error, Value::Number)
}

/// `PERCENTILE.INC(range, k)` — inclusive percentile. Two-arg: range + scalar.
fn builtin_percentile_inc(
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
fn builtin_percentile_exc(
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
fn builtin_quartile_inc(
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
fn builtin_quartile_exc(
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
fn builtin_large_small(
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
fn builtin_rank_eq(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
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
fn builtin_rank_avg(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
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
fn builtin_expon_dist(
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
fn builtin_correl(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let xs = expand_range(args[0], interp, scope);
    let ys = expand_range(args[1], interp, scope);
    statistical::correl(&xs, &ys).map_or_else(Value::Error, Value::Number)
}

/// `COVARIANCE.P(array1, array2)` — population covariance. Two-arg range-expanding.
fn builtin_covariance_p(
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
fn builtin_covariance_s(
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
fn builtin_slope(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let ys = expand_range(args[0], interp, scope);
    let xs = expand_range(args[1], interp, scope);
    statistical::slope(&ys, &xs).map_or_else(Value::Error, Value::Number)
}

/// `INTERCEPT(known_ys, known_xs)` — y-intercept of least-squares line.
fn builtin_intercept(
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
fn builtin_rsq(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let ys = expand_range(args[0], interp, scope);
    let xs = expand_range(args[1], interp, scope);
    statistical::rsq(&ys, &xs).map_or_else(Value::Error, Value::Number)
}

/// `FORECAST.LINEAR(x, known_ys, known_xs)` — predict Y from X.
fn builtin_forecast_linear(
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

pub(crate) fn handle_if(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_if(args, interp, scope)
}

pub(crate) fn handle_ifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_ifs(args, interp, scope)
}

pub(crate) fn handle_switch(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_switch(args, interp, scope)
}

pub(crate) fn handle_iferror(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_iferror(args, interp, scope)
}

pub(crate) fn handle_ifna(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_ifna(args, interp, scope)
}

pub(crate) fn handle_and(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_and(args, interp, scope)
}

pub(crate) fn handle_or(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_or(args, interp, scope)
}

pub(crate) fn handle_not(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_not(args, interp, scope)
}

pub(crate) fn handle_xor(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    conditional::builtin_xor(args, interp, scope)
}

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

pub(crate) fn handle_sumifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_sumifs(args, interp, scope)
}

pub(crate) fn handle_countifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_countifs(args, interp, scope)
}

pub(crate) fn handle_averageifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_averageifs(args, interp, scope)
}

pub(crate) fn handle_sumif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_sumif(args, interp, scope)
}

pub(crate) fn handle_countif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_countif(args, interp, scope)
}

pub(crate) fn handle_averageif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_averageif(args, interp, scope)
}

pub(crate) fn handle_minifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_minifs(args, interp, scope)
}

pub(crate) fn handle_maxifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    multi_conditional::builtin_maxifs(args, interp, scope)
}

// -- Simple aggregates --

pub(crate) fn handle_sum(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::sum(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_count(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::count(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_counta(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::counta(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_countblank(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::countblank(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_average(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::average(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_min(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::min(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_max(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::max(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_median(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::median(&vals).unwrap_or_else(Value::Error)
}

pub(crate) fn handle_product(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    let vals = expand_args_for_aggregate(args, interp, scope);
    aggregate::product(&vals).unwrap_or_else(Value::Error)
}

// -- Lookup --

pub(crate) fn handle_vlookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_vlookup(args, interp, scope)
}

pub(crate) fn handle_hlookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_hlookup(args, interp, scope)
}

pub(crate) fn handle_xlookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_xlookup(args, interp, scope)
}

pub(crate) fn handle_match(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_match(args, interp, scope)
}

pub(crate) fn handle_xmatch(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_xmatch(args, interp, scope)
}

pub(crate) fn handle_index(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_index(args, interp, scope)
}

pub(crate) fn handle_choose(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    lookup::builtin_choose(args, interp, scope)
}

// -- Date (volatile) --

pub(crate) fn handle_today(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_today(args, interp, scope)
}

pub(crate) fn handle_now(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_now(args, interp, scope)
}

// -- Date (pure, eager eval) --

pub(crate) fn handle_date(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_date(&eval_args(args, interp, scope))
}

pub(crate) fn handle_year(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_year(&eval_args(args, interp, scope))
}

pub(crate) fn handle_month(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_month(&eval_args(args, interp, scope))
}

pub(crate) fn handle_day(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_day(&eval_args(args, interp, scope))
}

pub(crate) fn handle_weekday(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_weekday(&eval_args(args, interp, scope))
}

pub(crate) fn handle_edate(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_edate(&eval_args(args, interp, scope))
}

pub(crate) fn handle_eomonth(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_eomonth(&eval_args(args, interp, scope))
}

pub(crate) fn handle_datedif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_datedif(&eval_args(args, interp, scope))
}

// -- Date (range-expanding) --

pub(crate) fn handle_networkdays(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_networkdays(args, interp, scope)
}

pub(crate) fn handle_workday(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    date::builtin_workday(args, interp, scope)
}

// -- String (pure, eager eval) --

pub(crate) fn handle_left(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_left(&eval_args(args, interp, scope))
}

pub(crate) fn handle_right(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_right(&eval_args(args, interp, scope))
}

pub(crate) fn handle_mid(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_mid(&eval_args(args, interp, scope))
}

pub(crate) fn handle_len(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_len(&eval_args(args, interp, scope))
}

pub(crate) fn handle_upper(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_upper(&eval_args(args, interp, scope))
}

pub(crate) fn handle_lower(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_lower(&eval_args(args, interp, scope))
}

pub(crate) fn handle_proper(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_proper(&eval_args(args, interp, scope))
}

pub(crate) fn handle_trim(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_trim(&eval_args(args, interp, scope))
}

pub(crate) fn handle_clean(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_clean(&eval_args(args, interp, scope))
}

pub(crate) fn handle_find(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_find(&eval_args(args, interp, scope))
}

pub(crate) fn handle_search(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_search(&eval_args(args, interp, scope))
}

pub(crate) fn handle_substitute(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_substitute(&eval_args(args, interp, scope))
}

pub(crate) fn handle_replace(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_replace(&eval_args(args, interp, scope))
}

pub(crate) fn handle_text(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_text(&eval_args(args, interp, scope))
}

pub(crate) fn handle_value(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_value(&eval_args(args, interp, scope))
}

pub(crate) fn handle_exact(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_exact(&eval_args(args, interp, scope))
}

// -- String (range-expanding) --

pub(crate) fn handle_concat(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_concat(args, interp, scope)
}

pub(crate) fn handle_textjoin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    string::builtin_textjoin(args, interp, scope)
}

// -- Math (pure, eager eval) --

pub(crate) fn handle_round(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_round(&eval_args(args, interp, scope))
}

pub(crate) fn handle_roundup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_roundup(&eval_args(args, interp, scope))
}

pub(crate) fn handle_rounddown(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_rounddown(&eval_args(args, interp, scope))
}

pub(crate) fn handle_int(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_int(&eval_args(args, interp, scope))
}

pub(crate) fn handle_mod(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_mod(&eval_args(args, interp, scope))
}

pub(crate) fn handle_abs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_abs(&eval_args(args, interp, scope))
}

pub(crate) fn handle_sign(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_sign(&eval_args(args, interp, scope))
}

pub(crate) fn handle_sqrt(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_sqrt(&eval_args(args, interp, scope))
}

pub(crate) fn handle_power(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_power(&eval_args(args, interp, scope))
}

pub(crate) fn handle_ceiling(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_ceiling(&eval_args(args, interp, scope))
}

pub(crate) fn handle_floor(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_floor(&eval_args(args, interp, scope))
}

pub(crate) fn handle_even(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_even(&eval_args(args, interp, scope))
}

pub(crate) fn handle_odd(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_odd(&eval_args(args, interp, scope))
}

pub(crate) fn handle_trunc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_trunc(&eval_args(args, interp, scope))
}

pub(crate) fn handle_mround(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_mround(&eval_args(args, interp, scope))
}

pub(crate) fn handle_ceiling_math(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_ceiling_math(&eval_args(args, interp, scope))
}

pub(crate) fn handle_floor_math(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_floor_math(&eval_args(args, interp, scope))
}

pub(crate) fn handle_ceiling_precise(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_ceiling_precise(&eval_args(args, interp, scope))
}

pub(crate) fn handle_floor_precise(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_floor_precise(&eval_args(args, interp, scope))
}

pub(crate) fn handle_pi(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_pi(&eval_args(args, interp, scope))
}

pub(crate) fn handle_ln(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_ln(&eval_args(args, interp, scope))
}

pub(crate) fn handle_log(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_log(&eval_args(args, interp, scope))
}

pub(crate) fn handle_log10(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_log10(&eval_args(args, interp, scope))
}

pub(crate) fn handle_exp(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_exp(&eval_args(args, interp, scope))
}

pub(crate) fn handle_sin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_sin(&eval_args(args, interp, scope))
}

pub(crate) fn handle_cos(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_cos(&eval_args(args, interp, scope))
}

pub(crate) fn handle_tan(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_tan(&eval_args(args, interp, scope))
}

pub(crate) fn handle_asin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_asin(&eval_args(args, interp, scope))
}

pub(crate) fn handle_acos(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_acos(&eval_args(args, interp, scope))
}

pub(crate) fn handle_atan(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_atan(&eval_args(args, interp, scope))
}

pub(crate) fn handle_atan2(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_atan2(&eval_args(args, interp, scope))
}

pub(crate) fn handle_fact(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_fact(&eval_args(args, interp, scope))
}

pub(crate) fn handle_factdouble(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_factdouble(&eval_args(args, interp, scope))
}

pub(crate) fn handle_permut(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_permut(&eval_args(args, interp, scope))
}

pub(crate) fn handle_permutationa(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_permutationa(&eval_args(args, interp, scope))
}

pub(crate) fn handle_combin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_combin(&eval_args(args, interp, scope))
}

pub(crate) fn handle_combina(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_combina(&eval_args(args, interp, scope))
}

pub(crate) fn handle_gcd(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_gcd(&eval_args(args, interp, scope))
}

pub(crate) fn handle_lcm(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_lcm(&eval_args(args, interp, scope))
}

pub(crate) fn handle_roman(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_roman(&eval_args(args, interp, scope))
}

pub(crate) fn handle_arabic(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_arabic(&eval_args(args, interp, scope))
}

pub(crate) fn handle_acosh(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_acosh(&eval_args(args, interp, scope))
}

pub(crate) fn handle_asinh(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_asinh(&eval_args(args, interp, scope))
}

pub(crate) fn handle_atanh(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_atanh(&eval_args(args, interp, scope))
}

pub(crate) fn handle_cosh(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_cosh(&eval_args(args, interp, scope))
}

pub(crate) fn handle_sinh(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_sinh(&eval_args(args, interp, scope))
}

pub(crate) fn handle_tanh(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_tanh(&eval_args(args, interp, scope))
}

pub(crate) fn handle_cot(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_cot(&eval_args(args, interp, scope))
}

pub(crate) fn handle_csc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_csc(&eval_args(args, interp, scope))
}

pub(crate) fn handle_sec(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_sec(&eval_args(args, interp, scope))
}

pub(crate) fn handle_coth(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_coth(&eval_args(args, interp, scope))
}

pub(crate) fn handle_csch(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_csch(&eval_args(args, interp, scope))
}

pub(crate) fn handle_sech(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_sech(&eval_args(args, interp, scope))
}

pub(crate) fn handle_degrees(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_degrees(&eval_args(args, interp, scope))
}

pub(crate) fn handle_radians(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    math::builtin_radians(&eval_args(args, interp, scope))
}

// -- Info (pure, eager eval) --

pub(crate) fn handle_isblank(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_isblank(&eval_args(args, interp, scope))
}

pub(crate) fn handle_isnumber(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_isnumber(&eval_args(args, interp, scope))
}

pub(crate) fn handle_istext(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_istext(&eval_args(args, interp, scope))
}

pub(crate) fn handle_iserror(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_iserror(&eval_args(args, interp, scope))
}

pub(crate) fn handle_isna(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_isna(&eval_args(args, interp, scope))
}

pub(crate) fn handle_islogical(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_islogical(&eval_args(args, interp, scope))
}

pub(crate) fn handle_isnontext(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_isnontext(&eval_args(args, interp, scope))
}

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

pub(crate) fn handle_na(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_na(&eval_args(args, interp, scope))
}

pub(crate) fn handle_type(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    info::builtin_type(&eval_args(args, interp, scope))
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

pub(crate) fn handle_pmt(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    financial::builtin_pmt(&eval_args(args, interp, scope))
}

pub(crate) fn handle_pv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    financial::builtin_pv(&eval_args(args, interp, scope))
}

pub(crate) fn handle_fv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    financial::builtin_fv(&eval_args(args, interp, scope))
}

pub(crate) fn handle_rate(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    financial::builtin_rate(&eval_args(args, interp, scope))
}

// -- Financial (range-expanding) --

pub(crate) fn handle_npv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    financial::builtin_npv(args, interp, scope)
}

pub(crate) fn handle_irr(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    financial::builtin_irr(args, interp, scope)
}

// -- Meta-dispatch (SUBTOTAL, AGGREGATE) --

pub(crate) fn handle_subtotal(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    subtotal::builtin_subtotal(args, interp, scope)
}

pub(crate) fn handle_aggregate(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    subtotal::builtin_aggregate(args, interp, scope)
}

// -- Statistical (range-expanding) --

pub(crate) fn handle_sumproduct(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_sumproduct(args, interp, scope)
}

pub(crate) fn handle_avedev(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_avedev(args, interp, scope)
}

pub(crate) fn handle_large(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_large_small(args, interp, scope, true)
}

pub(crate) fn handle_small(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_large_small(args, interp, scope, false)
}

pub(crate) fn handle_var_s(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_var_s(args, interp, scope)
}

pub(crate) fn handle_var_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_var_p(args, interp, scope)
}

pub(crate) fn handle_stdev_s(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_stdev_s(args, interp, scope)
}

pub(crate) fn handle_stdev_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_stdev_p(args, interp, scope)
}

pub(crate) fn handle_skew(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_skew(args, interp, scope)
}

pub(crate) fn handle_skew_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_skew_p(args, interp, scope)
}

pub(crate) fn handle_kurt(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_kurt(args, interp, scope)
}

pub(crate) fn handle_mode_sngl(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_mode_sngl(args, interp, scope)
}

pub(crate) fn handle_percentile_inc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_percentile_inc(args, interp, scope)
}

pub(crate) fn handle_percentile_exc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_percentile_exc(args, interp, scope)
}

pub(crate) fn handle_quartile_inc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_quartile_inc(args, interp, scope)
}

pub(crate) fn handle_quartile_exc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_quartile_exc(args, interp, scope)
}

pub(crate) fn handle_rank_eq(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_rank_eq(args, interp, scope)
}

pub(crate) fn handle_rank_avg(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_rank_avg(args, interp, scope)
}

pub(crate) fn handle_expon_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_expon_dist(args, interp, scope)
}

pub(crate) fn handle_correl(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_correl(args, interp, scope)
}

pub(crate) fn handle_covariance_p(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_covariance_p(args, interp, scope)
}

pub(crate) fn handle_covariance_s(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_covariance_s(args, interp, scope)
}

pub(crate) fn handle_slope(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_slope(args, interp, scope)
}

pub(crate) fn handle_intercept(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_intercept(args, interp, scope)
}

pub(crate) fn handle_rsq(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_rsq(args, interp, scope)
}

pub(crate) fn handle_forecast_linear(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    builtin_forecast_linear(args, interp, scope)
}

// -- Statistical (pure, eager eval with Result) --

pub(crate) fn handle_poisson_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_poisson_dist(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

pub(crate) fn handle_t_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_t_dist(&eval_args(args, interp, scope))
}

pub(crate) fn handle_t_dist_rt(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_t_dist_rt(&eval_args(args, interp, scope))
}

pub(crate) fn handle_t_dist_2t(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_t_dist_2t(&eval_args(args, interp, scope))
}

pub(crate) fn handle_t_inv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_t_inv(&eval_args(args, interp, scope))
}

pub(crate) fn handle_t_inv_2t(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_t_inv_2t(&eval_args(args, interp, scope))
}

pub(crate) fn handle_binom_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_binom_dist(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

pub(crate) fn handle_binom_inv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_binom_inv(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

pub(crate) fn handle_norm_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_norm_dist(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

pub(crate) fn handle_norm_inv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_norm_inv(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

pub(crate) fn handle_norm_s_dist(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_norm_s_dist(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

pub(crate) fn handle_norm_s_inv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    statistical::builtin_norm_s_inv(&eval_args(args, interp, scope))
        .map_or_else(Value::Error, Value::Number)
}

// -- Engineering (pure, eager eval) --

pub(crate) fn handle_hex2dec(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_hex2dec(&eval_args(args, interp, scope))
}

pub(crate) fn handle_dec2hex(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_dec2hex(&eval_args(args, interp, scope))
}

pub(crate) fn handle_complex(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_complex(&eval_args(args, interp, scope))
}

pub(crate) fn handle_imreal(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_imreal(&eval_args(args, interp, scope))
}

pub(crate) fn handle_imaginary(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_imaginary(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bitand(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bitand(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bitor(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bitor(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bitxor(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bitxor(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bitlshift(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bitlshift(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bitrshift(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bitrshift(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bin2dec(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bin2dec(&eval_args(args, interp, scope))
}

pub(crate) fn handle_dec2bin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_dec2bin(&eval_args(args, interp, scope))
}

pub(crate) fn handle_oct2dec(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_oct2dec(&eval_args(args, interp, scope))
}

pub(crate) fn handle_dec2oct(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_dec2oct(&eval_args(args, interp, scope))
}

pub(crate) fn handle_hex2bin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_hex2bin(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bin2hex(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bin2hex(&eval_args(args, interp, scope))
}

pub(crate) fn handle_hex2oct(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_hex2oct(&eval_args(args, interp, scope))
}

pub(crate) fn handle_oct2hex(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_oct2hex(&eval_args(args, interp, scope))
}

pub(crate) fn handle_bin2oct(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_bin2oct(&eval_args(args, interp, scope))
}

pub(crate) fn handle_oct2bin(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_oct2bin(&eval_args(args, interp, scope))
}

pub(crate) fn handle_base(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_base(&eval_args(args, interp, scope))
}

pub(crate) fn handle_delta(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_delta(&eval_args(args, interp, scope))
}

pub(crate) fn handle_gestep(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_gestep(&eval_args(args, interp, scope))
}

pub(crate) fn handle_erf(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_erf(&eval_args(args, interp, scope))
}

pub(crate) fn handle_erfc(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_erfc(&eval_args(args, interp, scope))
}

pub(crate) fn handle_erf_precise(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_erf_precise(&eval_args(args, interp, scope))
}

pub(crate) fn handle_erfc_precise(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    engineering::builtin_erfc_precise(&eval_args(args, interp, scope))
}

// -- Conversion --

pub(crate) fn handle_convert(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    convert::builtin_convert(&eval_args(args, interp, scope))
}
