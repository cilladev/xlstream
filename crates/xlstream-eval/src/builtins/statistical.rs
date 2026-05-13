//! Statistical builtin functions.
//!
//! STDEV.S, STDEV.P, VAR.S, VAR.P and future functions (SKEW, KURT,
//! AVEDEV, etc.) share a common [`collect_numerics`] helper that
//! extracts `f64` values from a `&[Value]` slice using range semantics.

use xlstream_core::{CellError, Value};

/// Extract numeric values from a `&[Value]` slice using range semantics.
///
/// Includes `Number`, `Integer` (cast to f64), and `Date` (serial).
/// Skips `Text`, `Bool`, and `Empty`. Propagates errors immediately.
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
    Ok(variance)
}

/// `VAR.P` — population variance (divides by n).
///
/// Requires at least 1 numeric value. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values.
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
    Ok(variance)
}

/// `STDEV.S` — sample standard deviation (sqrt of [`var_s`]).
///
/// Requires at least 2 numeric values. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 2 numeric values.
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
}
