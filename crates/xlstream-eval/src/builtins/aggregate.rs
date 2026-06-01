//! Pure aggregate functions operating on `&[Value]` slices.
//!
//! These implement **range semantics**: numeric values are consumed,
//! text and booleans are skipped (not coerced), errors propagate
//! immediately (except in [`countblank`] which ignores everything
//! non-empty).
//!
//! Each function takes a slice of values (typically a full column from
//! the prelude pass) and returns a single [`Value`].

use xlstream_core::{CellError, Value};

/// `SUM` — sum of numeric values. Text/bool skipped. Errors propagate.
///
/// Empty range returns `0`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::sum;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Text("x".into())];
/// assert_eq!(sum(&vals), Value::Number(3.0));
/// ```
#[must_use]
pub fn sum(values: &[Value]) -> Value {
    let mut total = 0.0_f64;
    for v in values {
        match v {
            Value::Error(e) => return Value::Error(*e),
            Value::Number(n) => total += n,
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => total += *i as f64,
            Value::Date(d) => total += d.serial,
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    Value::Number(total)
}

/// `COUNT` — count of numeric values. Text/bool/empty skipped. Errors
/// skipped.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::count;
/// let vals = [Value::Number(1.0), Value::Text("x".into()), Value::Number(2.0)];
/// assert_eq!(count(&vals), Value::Number(2.0));
/// ```
#[must_use]
pub fn count(values: &[Value]) -> Value {
    let mut n = 0_u64;
    for v in values {
        match v {
            Value::Number(_) | Value::Integer(_) | Value::Date(_) => n += 1,
            Value::Text(_) | Value::Bool(_) | Value::Empty | Value::Error(_) => {}
        }
    }
    #[allow(clippy::cast_precision_loss)]
    Value::Number(n as f64)
}

/// `COUNTA` — count of non-empty values. Errors count. Only `Empty` is
/// excluded.
///
/// # Examples
///
/// ```
/// use xlstream_core::{CellError, Value};
/// use xlstream_eval::builtins::aggregate::counta;
/// let vals = [Value::Number(1.0), Value::Empty, Value::Text("x".into())];
/// assert_eq!(counta(&vals), Value::Number(2.0));
/// ```
#[must_use]
pub fn counta(values: &[Value]) -> Value {
    let mut n = 0_u64;
    for v in values {
        if !matches!(v, Value::Empty) {
            n += 1;
        }
    }
    #[allow(clippy::cast_precision_loss)]
    Value::Number(n as f64)
}

/// `COUNTBLANK` — count of empty/blank values. Errors and text are
/// non-blank.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::countblank;
/// let vals = [Value::Empty, Value::Number(0.0), Value::Empty];
/// assert_eq!(countblank(&vals), Value::Number(2.0));
/// ```
#[must_use]
pub fn countblank(values: &[Value]) -> Value {
    let mut n = 0_u64;
    for v in values {
        if matches!(v, Value::Empty) {
            n += 1;
        }
    }
    #[allow(clippy::cast_precision_loss)]
    Value::Number(n as f64)
}

/// `AVERAGE` — arithmetic mean of numeric values. Text/bool skipped.
/// Errors propagate.
///
/// Empty numeric range returns `#DIV/0!`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::average;
/// let vals = [Value::Number(10.0), Value::Number(20.0)];
/// assert_eq!(average(&vals), Value::Number(15.0));
/// ```
#[must_use]
pub fn average(values: &[Value]) -> Value {
    let mut total = 0.0_f64;
    let mut cnt = 0_u64;
    for v in values {
        match v {
            Value::Error(e) => return Value::Error(*e),
            Value::Number(n) => {
                total += n;
                cnt += 1;
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => {
                total += *i as f64;
                cnt += 1;
            }
            Value::Date(d) => {
                total += d.serial;
                cnt += 1;
            }
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    if cnt == 0 {
        return Value::Error(CellError::Div0);
    }
    #[allow(clippy::cast_precision_loss)]
    Value::Number(total / cnt as f64)
}

/// `MIN` — minimum of numeric values. Text/bool skipped. Errors propagate.
///
/// Empty numeric range returns `0` (Excel behaviour).
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::min;
/// let vals = [Value::Number(3.0), Value::Number(1.0), Value::Number(2.0)];
/// assert_eq!(min(&vals), Value::Number(1.0));
/// ```
#[must_use]
pub fn min(values: &[Value]) -> Value {
    let mut result: Option<f64> = None;
    for v in values {
        match v {
            Value::Error(e) => return Value::Error(*e),
            Value::Number(n) => {
                result = Some(result.map_or(*n, |cur: f64| cur.min(*n)));
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => {
                let f = *i as f64;
                result = Some(result.map_or(f, |cur: f64| cur.min(f)));
            }
            Value::Date(d) => {
                result = Some(result.map_or(d.serial, |cur: f64| cur.min(d.serial)));
            }
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    Value::Number(result.unwrap_or(0.0))
}

/// `MAX` — maximum of numeric values. Text/bool skipped. Errors propagate.
///
/// Empty numeric range returns `0` (Excel behaviour).
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::max;
/// let vals = [Value::Number(3.0), Value::Number(1.0), Value::Number(2.0)];
/// assert_eq!(max(&vals), Value::Number(3.0));
/// ```
#[must_use]
pub fn max(values: &[Value]) -> Value {
    let mut result: Option<f64> = None;
    for v in values {
        match v {
            Value::Error(e) => return Value::Error(*e),
            Value::Number(n) => {
                result = Some(result.map_or(*n, |cur: f64| cur.max(*n)));
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => {
                let f = *i as f64;
                result = Some(result.map_or(f, |cur: f64| cur.max(f)));
            }
            Value::Date(d) => {
                result = Some(result.map_or(d.serial, |cur: f64| cur.max(d.serial)));
            }
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    Value::Number(result.unwrap_or(0.0))
}

/// `PRODUCT` — product of numeric values. Text/bool skipped. Errors
/// propagate.
///
/// If no numeric values exist, returns `0` (Excel behaviour).
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::product;
/// let vals = [Value::Number(2.0), Value::Number(3.0), Value::Number(4.0)];
/// assert_eq!(product(&vals), Value::Number(24.0));
/// ```
#[must_use]
pub fn product(values: &[Value]) -> Value {
    let mut result = 1.0_f64;
    let mut has_numeric = false;
    for v in values {
        match v {
            Value::Error(e) => return Value::Error(*e),
            Value::Number(n) => {
                result *= n;
                has_numeric = true;
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => {
                result *= *i as f64;
                has_numeric = true;
            }
            Value::Date(d) => {
                result *= d.serial;
                has_numeric = true;
            }
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    if has_numeric {
        Value::Number(result)
    } else {
        Value::Number(0.0)
    }
}

/// `MEDIAN` — median of numeric values. Text/bool skipped. Errors
/// propagate.
///
/// Returns `#NUM!` if no numeric values exist.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::median;
/// let vals = [Value::Number(1.0), Value::Number(3.0), Value::Number(2.0)];
/// assert_eq!(median(&vals), Value::Number(2.0));
/// ```
#[must_use]
pub fn median(values: &[Value]) -> Value {
    let mut nums = Vec::new();
    for v in values {
        match v {
            Value::Error(e) => return Value::Error(*e),
            Value::Number(n) => nums.push(*n),
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => nums.push(*i as f64),
            Value::Date(d) => nums.push(d.serial),
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    if nums.is_empty() {
        return Value::Error(CellError::Num);
    }
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = nums.len() / 2;
    if nums.len() % 2 == 0 {
        Value::Number(f64::midpoint(nums[mid - 1], nums[mid]))
    } else {
        Value::Number(nums[mid])
    }
}

/// `SUMPRODUCT` — sum of element-wise products of arrays.
///
/// With a single array, sums the array. All arrays must have the same
/// length. Values are coerced via [`xlstream_core::coerce::to_number`]:
/// booleans become 1/0, empty becomes 0, errors propagate, text that
/// cannot parse as a number returns `#VALUE!`.
///
/// Returns `Value::Error(CellError::Value)` if no arrays are provided
/// or if arrays have different lengths. Error values and non-numeric
/// text propagate as `Value::Error`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::aggregate::sumproduct;
/// let a = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
/// let b = [Value::Number(4.0), Value::Number(5.0), Value::Number(6.0)];
/// assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Number(32.0));
/// ```
#[must_use]
pub fn sumproduct(arrays: &[&[Value]]) -> Value {
    if arrays.is_empty() {
        return Value::Error(CellError::Value);
    }

    let len = arrays[0].len();
    for arr in &arrays[1..] {
        if arr.len() != len {
            return Value::Error(CellError::Value);
        }
    }

    let mut total = 0.0_f64;
    for i in 0..len {
        let mut product = 1.0_f64;
        for arr in arrays {
            let n = match &arr[i] {
                Value::Bool(_) | Value::Text(_) | Value::Empty => 0.0,
                Value::Error(e) => return Value::Error(*e),
                v => match xlstream_core::coerce::to_number(v) {
                    Ok(n) => n,
                    Err(e) => return Value::Error(e),
                },
            };
            product *= n;
        }
        total += product;
    }

    Value::Number(total)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};

    use super::*;

    // ===== SUM =====

    #[test]
    fn sum_numbers() {
        let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(sum(&vals), Value::Number(6.0));
    }

    #[test]
    fn sum_empty_returns_zero() {
        assert_eq!(sum(&[]), Value::Number(0.0));
    }

    #[test]
    fn sum_skips_text() {
        let vals = [Value::Number(5.0), Value::Text("x".into())];
        assert_eq!(sum(&vals), Value::Number(5.0));
    }

    #[test]
    fn sum_skips_bool() {
        let vals = [Value::Number(5.0), Value::Bool(true)];
        assert_eq!(sum(&vals), Value::Number(5.0));
    }

    #[test]
    fn sum_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Div0)];
        assert_eq!(sum(&vals), Value::Error(CellError::Div0));
    }

    #[test]
    fn sum_skips_empty() {
        let vals = [Value::Number(3.0), Value::Empty, Value::Number(7.0)];
        assert_eq!(sum(&vals), Value::Number(10.0));
    }

    // ===== COUNT =====

    #[test]
    fn count_numbers_only() {
        let vals = [Value::Number(1.0), Value::Text("x".into()), Value::Number(2.0)];
        assert_eq!(count(&vals), Value::Number(2.0));
    }

    #[test]
    fn count_empty_returns_zero() {
        assert_eq!(count(&[]), Value::Number(0.0));
    }

    #[test]
    fn count_skips_text_bool_empty() {
        let vals = [Value::Text("a".into()), Value::Bool(true), Value::Empty];
        assert_eq!(count(&vals), Value::Number(0.0));
    }

    #[test]
    fn count_skips_errors() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(count(&vals), Value::Number(1.0));
    }

    // ===== COUNTA =====

    #[test]
    fn counta_counts_non_empty() {
        let vals = [Value::Number(1.0), Value::Empty, Value::Text("x".into())];
        assert_eq!(counta(&vals), Value::Number(2.0));
    }

    #[test]
    fn counta_counts_errors() {
        let vals = [Value::Error(CellError::Na), Value::Empty];
        assert_eq!(counta(&vals), Value::Number(1.0));
    }

    #[test]
    fn counta_counts_bool() {
        let vals = [Value::Bool(false), Value::Empty];
        assert_eq!(counta(&vals), Value::Number(1.0));
    }

    #[test]
    fn counta_empty_returns_zero() {
        assert_eq!(counta(&[]), Value::Number(0.0));
    }

    // ===== COUNTBLANK =====

    #[test]
    fn countblank_counts_empty() {
        let vals = [Value::Empty, Value::Number(0.0), Value::Empty];
        assert_eq!(countblank(&vals), Value::Number(2.0));
    }

    #[test]
    fn countblank_ignores_non_empty() {
        let vals = [Value::Number(1.0), Value::Text("x".into()), Value::Bool(true)];
        assert_eq!(countblank(&vals), Value::Number(0.0));
    }

    #[test]
    fn countblank_ignores_errors() {
        let vals = [Value::Error(CellError::Na), Value::Empty];
        assert_eq!(countblank(&vals), Value::Number(1.0));
    }

    #[test]
    fn countblank_empty_range() {
        assert_eq!(countblank(&[]), Value::Number(0.0));
    }

    // ===== AVERAGE =====

    #[test]
    fn average_numbers() {
        let vals = [Value::Number(10.0), Value::Number(20.0)];
        assert_eq!(average(&vals), Value::Number(15.0));
    }

    #[test]
    fn average_skips_text() {
        let vals = [Value::Number(10.0), Value::Text("x".into()), Value::Number(20.0)];
        assert_eq!(average(&vals), Value::Number(15.0));
    }

    #[test]
    fn average_empty_range_returns_div0() {
        assert_eq!(average(&[]), Value::Error(CellError::Div0));
    }

    #[test]
    fn average_all_text_returns_div0() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(average(&vals), Value::Error(CellError::Div0));
    }

    #[test]
    fn average_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(average(&vals), Value::Error(CellError::Na));
    }

    // ===== MIN =====

    #[test]
    fn min_numbers() {
        let vals = [Value::Number(3.0), Value::Number(1.0), Value::Number(2.0)];
        assert_eq!(min(&vals), Value::Number(1.0));
    }

    #[test]
    fn min_empty_returns_zero() {
        assert_eq!(min(&[]), Value::Number(0.0));
    }

    #[test]
    fn min_skips_text() {
        let vals = [Value::Number(5.0), Value::Text("x".into()), Value::Number(3.0)];
        assert_eq!(min(&vals), Value::Number(3.0));
    }

    #[test]
    fn min_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Ref)];
        assert_eq!(min(&vals), Value::Error(CellError::Ref));
    }

    #[test]
    fn min_negative_numbers() {
        let vals = [Value::Number(-5.0), Value::Number(-1.0), Value::Number(-3.0)];
        assert_eq!(min(&vals), Value::Number(-5.0));
    }

    // ===== MAX =====

    #[test]
    fn max_numbers() {
        let vals = [Value::Number(3.0), Value::Number(1.0), Value::Number(2.0)];
        assert_eq!(max(&vals), Value::Number(3.0));
    }

    #[test]
    fn max_empty_returns_zero() {
        assert_eq!(max(&[]), Value::Number(0.0));
    }

    #[test]
    fn max_skips_text() {
        let vals = [Value::Number(5.0), Value::Text("x".into()), Value::Number(3.0)];
        assert_eq!(max(&vals), Value::Number(5.0));
    }

    #[test]
    fn max_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Value)];
        assert_eq!(max(&vals), Value::Error(CellError::Value));
    }

    #[test]
    fn max_negative_numbers() {
        let vals = [Value::Number(-5.0), Value::Number(-1.0), Value::Number(-3.0)];
        assert_eq!(max(&vals), Value::Number(-1.0));
    }

    // ===== PRODUCT =====

    #[test]
    fn product_numbers() {
        let vals = [Value::Number(2.0), Value::Number(3.0), Value::Number(4.0)];
        assert_eq!(product(&vals), Value::Number(24.0));
    }

    #[test]
    fn product_empty_returns_zero() {
        assert_eq!(product(&[]), Value::Number(0.0));
    }

    #[test]
    fn product_skips_text() {
        let vals = [Value::Number(2.0), Value::Text("x".into()), Value::Number(5.0)];
        assert_eq!(product(&vals), Value::Number(10.0));
    }

    #[test]
    fn product_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Num)];
        assert_eq!(product(&vals), Value::Error(CellError::Num));
    }

    #[test]
    fn product_with_zero() {
        let vals = [Value::Number(5.0), Value::Number(0.0), Value::Number(3.0)];
        assert_eq!(product(&vals), Value::Number(0.0));
    }

    #[test]
    fn product_all_text_returns_zero() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(product(&vals), Value::Number(0.0));
    }

    // ===== MEDIAN =====

    #[test]
    fn median_odd_count() {
        let vals = [Value::Number(1.0), Value::Number(3.0), Value::Number(2.0)];
        assert_eq!(median(&vals), Value::Number(2.0));
    }

    #[test]
    fn median_even_count() {
        let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0), Value::Number(4.0)];
        assert_eq!(median(&vals), Value::Number(2.5));
    }

    #[test]
    fn median_single() {
        let vals = [Value::Number(42.0)];
        assert_eq!(median(&vals), Value::Number(42.0));
    }

    #[test]
    fn median_empty_returns_num_error() {
        assert_eq!(median(&[]), Value::Error(CellError::Num));
    }

    #[test]
    fn median_skips_text() {
        let vals = [Value::Number(1.0), Value::Text("x".into()), Value::Number(5.0)];
        assert_eq!(median(&vals), Value::Number(3.0));
    }

    #[test]
    fn median_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(median(&vals), Value::Error(CellError::Na));
    }

    #[test]
    fn median_unsorted_input() {
        let vals = [
            Value::Number(5.0),
            Value::Number(1.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(2.0),
        ];
        assert_eq!(median(&vals), Value::Number(3.0));
    }

    // ===== Mixed type tests =====

    #[test]
    fn sum_with_integer() {
        let vals = [Value::Number(1.0), Value::Integer(2)];
        assert_eq!(sum(&vals), Value::Number(3.0));
    }

    #[test]
    fn count_with_integer() {
        let vals = [Value::Integer(1), Value::Text("x".into())];
        assert_eq!(count(&vals), Value::Number(1.0));
    }

    #[test]
    fn min_with_integer() {
        let vals = [Value::Number(5.0), Value::Integer(2)];
        assert_eq!(min(&vals), Value::Number(2.0));
    }

    #[test]
    fn max_with_integer() {
        let vals = [Value::Number(5.0), Value::Integer(10)];
        assert_eq!(max(&vals), Value::Number(10.0));
    }

    // ===== SUMPRODUCT =====

    #[test]
    fn sumproduct_two_equal_ranges() {
        let a = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        let b = [Value::Number(4.0), Value::Number(5.0), Value::Number(6.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Number(32.0));
    }

    #[test]
    fn sumproduct_single_range_sums() {
        let a = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(sumproduct(&[&a[..]]), Value::Number(6.0));
    }

    #[test]
    fn sumproduct_three_ranges() {
        let a = [Value::Number(1.0), Value::Number(2.0)];
        let b = [Value::Number(3.0), Value::Number(4.0)];
        let c = [Value::Number(5.0), Value::Number(6.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..], &c[..]]), Value::Number(63.0));
    }

    #[test]
    fn sumproduct_mismatched_lengths_returns_value_error() {
        let a = [Value::Number(1.0), Value::Number(2.0)];
        let b = [Value::Number(3.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Error(CellError::Value));
    }

    #[test]
    fn sumproduct_empty_args_returns_value_error() {
        assert_eq!(sumproduct(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn sumproduct_bool_treated_as_zero() {
        let a = [Value::Bool(true), Value::Bool(false), Value::Bool(true)];
        let b = [Value::Number(10.0), Value::Number(20.0), Value::Number(30.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Number(0.0));
    }

    #[test]
    fn sumproduct_empty_cells_treated_as_zero() {
        let a = [Value::Number(5.0), Value::Empty, Value::Number(3.0)];
        let b = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Number(28.0));
    }

    #[test]
    fn sumproduct_error_propagates() {
        let a = [Value::Number(1.0), Value::Error(CellError::Na)];
        let b = [Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Error(CellError::Na));
    }

    #[test]
    fn sumproduct_text_treated_as_zero() {
        let a = [Value::Number(1.0), Value::Text("x".into())];
        let b = [Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(sumproduct(&[&a[..], &b[..]]), Value::Number(2.0));
    }
}
