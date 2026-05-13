//! Statistical builtin functions.
//!
//! STDEV.S, STDEV.P, VAR.S, VAR.P and future functions (SKEW, KURT,
//! AVEDEV, etc.) share a common [`collect_numerics`] helper that
//! extracts `f64` values from a `&[Value]` slice using range semantics.

use xlstream_core::{CellError, Value};

fn finite_or_num(v: f64) -> Result<f64, CellError> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(CellError::Num)
    }
}

/// Extract numeric values from a `&[Value]` slice using range semantics.
///
/// Includes `Number`, `Integer` (cast to f64), and `Date` (serial).
/// Skips `Text`, `Bool`, and `Empty`. Propagates errors immediately.
/// NaN/Infinity values pass through — callers must guard their output.
///
/// Reused by variance, standard deviation, skewness, kurtosis, etc.
///
/// # Errors
///
/// Returns `Err(CellError)` if any value is an error variant.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::collect_numerics;
/// let vals = [Value::Number(1.0), Value::Text("x".into()), Value::Number(3.0)];
/// assert_eq!(collect_numerics(&vals).unwrap(), vec![1.0, 3.0]);
/// ```
pub fn collect_numerics(values: &[Value]) -> Result<Vec<f64>, CellError> {
    let mut nums = Vec::with_capacity(values.len());
    for v in values {
        match v {
            Value::Error(e) => return Err(*e),
            Value::Number(n) => nums.push(*n),
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => nums.push(*i as f64),
            Value::Date(d) => nums.push(d.serial),
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    Ok(nums)
}

/// `VAR.S` — sample variance (divides by n-1).
///
/// Requires at least 2 numeric values. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 2 numeric values.
/// Returns `Err(CellError::Num)` if the result overflows to NaN/Infinity.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::var_s;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = var_s(&vals).unwrap();
/// assert!((result - 6.666_666_666_666_667).abs() < 1e-9);
/// ```
pub fn var_s(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let n = nums.len();
    if n < 2 {
        return Err(CellError::Div0);
    }
    #[allow(clippy::cast_precision_loss)]
    let mean = nums.iter().sum::<f64>() / n as f64;
    #[allow(clippy::cast_precision_loss)]
    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    finite_or_num(variance)
}

/// `VAR.P` — population variance (divides by n).
///
/// Requires at least 1 numeric value. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values.
/// Returns `Err(CellError::Num)` if the result overflows to NaN/Infinity.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::var_p;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = var_p(&vals).unwrap();
/// assert!((result - 5.0).abs() < 1e-9);
/// ```
pub fn var_p(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let n = nums.len();
    if n == 0 {
        return Err(CellError::Div0);
    }
    #[allow(clippy::cast_precision_loss)]
    let mean = nums.iter().sum::<f64>() / n as f64;
    #[allow(clippy::cast_precision_loss)]
    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    finite_or_num(variance)
}

/// `STDEV.S` — sample standard deviation (sqrt of [`var_s`]).
///
/// Requires at least 2 numeric values. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 2 numeric values.
/// Returns `Err(CellError::Num)` if the underlying variance overflows.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::stdev_s;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = stdev_s(&vals).unwrap();
/// assert!((result - 2.581_988_897_471_611).abs() < 1e-9);
/// ```
pub fn stdev_s(values: &[Value]) -> Result<f64, CellError> {
    var_s(values).map(f64::sqrt)
}

/// `STDEV.P` — population standard deviation (sqrt of [`var_p`]).
///
/// Requires at least 1 numeric value. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values.
/// Returns `Err(CellError::Num)` if the underlying variance overflows.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::stdev_p;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = stdev_p(&vals).unwrap();
/// assert!((result - 2.236_067_977_499_79).abs() < 1e-9);
/// ```
pub fn stdev_p(values: &[Value]) -> Result<f64, CellError> {
    var_p(values).map(f64::sqrt)
}

/// `SKEW` — sample skewness (adjusted).
///
/// Formula: `[n / ((n-1)(n-2))] * sum[(xi - mean) / stdev_s]^3`
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 3 numeric values,
/// if standard deviation is zero, or if any input is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::skew;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// let result = skew(&vals).unwrap();
/// assert!(result.abs() < 1e-10);
/// ```
pub fn skew(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let n = nums.len();
    if n < 3 {
        return Err(CellError::Div0);
    }

    #[allow(clippy::cast_precision_loss)]
    let nf = n as f64;
    let mean = nums.iter().sum::<f64>() / nf;

    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (nf - 1.0);
    let stdev = variance.sqrt();
    if stdev == 0.0 {
        return Err(CellError::Div0);
    }

    let m3: f64 = nums.iter().map(|x| ((x - mean) / stdev).powi(3)).sum();
    let adjustment = nf / ((nf - 1.0) * (nf - 2.0));

    Ok(adjustment * m3)
}

/// `SKEW.P` — population skewness.
///
/// Formula: `(1/n) * sum[(xi - mean) / stdev_p]^3`
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values, if standard
/// deviation is zero, or if any input is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::skew_p;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// let result = skew_p(&vals).unwrap();
/// assert!(result.abs() < 1e-10);
/// ```
pub fn skew_p(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let n = nums.len();
    if n == 0 {
        return Err(CellError::Div0);
    }

    #[allow(clippy::cast_precision_loss)]
    let nf = n as f64;
    let mean = nums.iter().sum::<f64>() / nf;

    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / nf;
    let stdev = variance.sqrt();
    if stdev == 0.0 {
        return Err(CellError::Div0);
    }

    let m3: f64 = nums.iter().map(|x| ((x - mean) / stdev).powi(3)).sum();

    Ok(m3 / nf)
}

/// `KURT` — excess kurtosis (sample-adjusted).
///
/// Formula: `[(n(n+1)) / ((n-1)(n-2)(n-3))] * sum[(xi - mean) / stdev_s]^4
///           - [3(n-1)^2 / ((n-2)(n-3))]`
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 4 numeric values,
/// if standard deviation is zero, or if any input is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::kurt;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// let result = kurt(&vals).unwrap();
/// assert!((result - (-1.2)).abs() < 1e-10);
/// ```
pub fn kurt(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let n = nums.len();
    if n < 4 {
        return Err(CellError::Div0);
    }

    #[allow(clippy::cast_precision_loss)]
    let nf = n as f64;
    let mean = nums.iter().sum::<f64>() / nf;

    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (nf - 1.0);
    let stdev = variance.sqrt();
    if stdev == 0.0 {
        return Err(CellError::Div0);
    }

    let m4: f64 = nums.iter().map(|x| ((x - mean) / stdev).powi(4)).sum();
    let term1 = (nf * (nf + 1.0)) / ((nf - 1.0) * (nf - 2.0) * (nf - 3.0));
    let term2 = (3.0 * (nf - 1.0).powi(2)) / ((nf - 2.0) * (nf - 3.0));

    Ok(term1 * m4 - term2)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};

    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9, "expected {expected}, got {actual}");
    }

    // ===== collect_numerics =====

    #[test]
    fn collect_numerics_empty_input() {
        assert_eq!(collect_numerics(&[]).unwrap(), Vec::<f64>::new());
    }

    #[test]
    fn collect_numerics_all_text_returns_empty() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(collect_numerics(&vals).unwrap(), Vec::<f64>::new());
    }

    #[test]
    fn collect_numerics_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(collect_numerics(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn collect_numerics_mixed_types() {
        let vals = [
            Value::Number(1.0),
            Value::Text("x".into()),
            Value::Integer(2),
            Value::Bool(true),
            Value::Empty,
            Value::Number(3.0),
        ];
        assert_eq!(collect_numerics(&vals).unwrap(), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn collect_numerics_includes_date_serial() {
        let date = xlstream_core::ExcelDate { serial: 45000.0 };
        let vals = [Value::Number(1.0), Value::Date(date)];
        assert_eq!(collect_numerics(&vals).unwrap(), vec![1.0, 45000.0]);
    }

    // ===== VAR.S =====

    #[test]
    fn var_s_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(var_s(&vals).unwrap(), 6.666_666_666_666_667);
    }

    #[test]
    fn var_s_two_values() {
        let vals = [Value::Number(3.0), Value::Number(7.0)];
        assert_close(var_s(&vals).unwrap(), 8.0);
    }

    #[test]
    fn var_s_all_same_returns_zero() {
        let vals = [Value::Number(5.0), Value::Number(5.0), Value::Number(5.0)];
        assert_close(var_s(&vals).unwrap(), 0.0);
    }

    #[test]
    fn var_s_single_value_returns_div0() {
        let vals = [Value::Number(5.0)];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_s_empty_returns_div0() {
        assert_eq!(var_s(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_s_all_text_returns_div0() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_s_skips_text_and_bool() {
        let vals =
            [Value::Number(1.0), Value::Text("text".into()), Value::Number(3.0), Value::Bool(true)];
        assert_close(var_s(&vals).unwrap(), 2.0);
    }

    #[test]
    fn var_s_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn var_s_negative_numbers() {
        let vals = [Value::Number(-2.0), Value::Number(-4.0), Value::Number(-6.0)];
        assert_close(var_s(&vals).unwrap(), 4.0);
    }

    // ===== VAR.P =====

    #[test]
    fn var_p_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(var_p(&vals).unwrap(), 5.0);
    }

    #[test]
    fn var_p_single_value_returns_zero() {
        let vals = [Value::Number(5.0)];
        assert_close(var_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn var_p_empty_returns_div0() {
        assert_eq!(var_p(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_p_all_same_returns_zero() {
        let vals = [Value::Number(7.0), Value::Number(7.0), Value::Number(7.0)];
        assert_close(var_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn var_p_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Div0)];
        assert_eq!(var_p(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_p_skips_text_and_bool() {
        let vals =
            [Value::Number(1.0), Value::Text("x".into()), Value::Number(3.0), Value::Bool(false)];
        assert_close(var_p(&vals).unwrap(), 1.0);
    }

    // ===== STDEV.S =====

    #[test]
    fn stdev_s_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(stdev_s(&vals).unwrap(), 2.581_988_897_471_611);
    }

    #[test]
    fn stdev_s_two_values() {
        let vals = [Value::Number(3.0), Value::Number(7.0)];
        assert_close(stdev_s(&vals).unwrap(), 2.828_427_124_746_190_3);
    }

    #[test]
    fn stdev_s_all_same_returns_zero() {
        let vals = [Value::Number(5.0), Value::Number(5.0), Value::Number(5.0)];
        assert_close(stdev_s(&vals).unwrap(), 0.0);
    }

    #[test]
    fn stdev_s_single_value_returns_div0() {
        let vals = [Value::Number(5.0)];
        assert_eq!(stdev_s(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn stdev_s_empty_returns_div0() {
        assert_eq!(stdev_s(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn stdev_s_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(stdev_s(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn stdev_s_negative_numbers() {
        let vals = [Value::Number(-2.0), Value::Number(-4.0), Value::Number(-6.0)];
        assert_close(stdev_s(&vals).unwrap(), 2.0);
    }

    // ===== STDEV.P =====

    #[test]
    fn stdev_p_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(stdev_p(&vals).unwrap(), 2.236_067_977_499_79);
    }

    #[test]
    fn stdev_p_single_value_returns_zero() {
        let vals = [Value::Number(5.0)];
        assert_close(stdev_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn stdev_p_empty_returns_div0() {
        assert_eq!(stdev_p(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn stdev_p_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Ref)];
        assert_eq!(stdev_p(&vals).unwrap_err(), CellError::Ref);
    }

    // ===== Numerical stability =====

    #[test]
    fn var_p_large_numbers_stable() {
        let vals = [Value::Number(1e10), Value::Number(1e10 + 1.0)];
        assert_close(var_p(&vals).unwrap(), 0.25);
    }

    #[test]
    fn var_s_overflow_returns_num() {
        let vals = [Value::Number(f64::MAX), Value::Number(f64::MAX)];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Num);
    }

    #[test]
    fn var_p_infinity_returns_num() {
        let vals = [Value::Number(f64::INFINITY), Value::Number(1.0)];
        assert_eq!(var_p(&vals).unwrap_err(), CellError::Num);
    }

    // ===== SKEW =====

    #[test]
    fn skew_symmetric_is_zero() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(skew(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_right_skewed_is_positive() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(skew(&vals).unwrap() > 0.0);
    }

    #[test]
    fn skew_left_skewed_is_negative() {
        let vals: Vec<Value> =
            [1.0, 97.0, 98.0, 99.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(skew(&vals).unwrap() < 0.0);
    }

    #[test]
    fn skew_minimum_n_three() {
        let vals: Vec<Value> = [10.0, 20.0, 30.0].iter().map(|&n| Value::Number(n)).collect();
        skew(&vals).unwrap();
    }

    #[test]
    fn skew_below_minimum_returns_div0() {
        let vals = [Value::Number(1.0), Value::Number(2.0)];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_single_value_returns_div0() {
        let vals = [Value::Number(1.0)];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_empty_returns_div0() {
        assert_eq!(skew(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_all_same_returns_div0() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(skew(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_skips_text() {
        let vals = [
            Value::Number(1.0),
            Value::Text("text".into()),
            Value::Number(3.0),
            Value::Number(5.0),
        ];
        assert_close(skew(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_propagates_error() {
        let vals = [
            Value::Number(1.0),
            Value::Error(CellError::Na),
            Value::Number(3.0),
            Value::Number(5.0),
        ];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Na);
    }

    // ===== SKEW.P =====

    #[test]
    fn skew_p_symmetric_is_zero() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(skew_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_p_right_skewed_is_positive() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(skew_p(&vals).unwrap() > 0.0);
    }

    #[test]
    fn skew_p_with_two_values() {
        let vals = [Value::Number(1.0), Value::Number(3.0)];
        skew_p(&vals).unwrap();
    }

    #[test]
    fn skew_p_single_value_div0() {
        let vals = [Value::Number(5.0)];
        assert_eq!(skew_p(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_p_empty_returns_div0() {
        assert_eq!(skew_p(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_p_all_same_returns_div0() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(skew_p(&vals).unwrap_err(), CellError::Div0);
    }

    // ===== KURT =====

    #[test]
    fn kurt_uniform_is_negative_1_2() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_right_skewed_is_positive() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(kurt(&vals).unwrap() > 0.0);
    }

    #[test]
    fn kurt_uniform_ten_values() {
        let vals: Vec<Value> = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0]
            .iter()
            .map(|&n| Value::Number(n))
            .collect();
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_minimum_n_four() {
        let vals: Vec<Value> = [1.0, 2.0, 3.0, 4.0].iter().map(|&n| Value::Number(n)).collect();
        kurt(&vals).unwrap();
    }

    #[test]
    fn kurt_below_minimum_returns_div0() {
        let vals: Vec<Value> = [1.0, 2.0, 3.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn kurt_empty_returns_div0() {
        assert_eq!(kurt(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn kurt_all_same_returns_div0() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn kurt_skips_bool() {
        let vals = [
            Value::Number(1.0),
            Value::Bool(true),
            Value::Number(3.0),
            Value::Number(5.0),
            Value::Number(7.0),
        ];
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_propagates_error() {
        let vals = [
            Value::Number(1.0),
            Value::Error(CellError::Na),
            Value::Number(3.0),
            Value::Number(5.0),
            Value::Number(7.0),
        ];
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Na);
    }
}
