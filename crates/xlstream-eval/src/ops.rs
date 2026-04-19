//! Operator dispatch for binary and unary operators.

use xlstream_core::coerce;
use xlstream_core::{CellError, Value};

/// Evaluate a binary operator on two pre-evaluated values.
///
/// Error propagation: if either operand is an error, the error propagates
/// (left checked first). Operands are coerced per the operator's context.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::ops::eval_binary;
/// assert_eq!(eval_binary("+", &Value::Number(1.0), &Value::Number(2.0)), Value::Number(3.0));
/// ```
#[must_use]
pub fn eval_binary(op: &str, left: &Value, right: &Value) -> Value {
    match op {
        "+" | "-" | "*" | "/" | "^" => eval_arithmetic(op, left, right),
        "&" => eval_concat(left, right),
        "=" | "<>" | "<" | ">" | "<=" | ">=" => eval_comparison(op, left, right),
        _ => Value::Error(CellError::Value),
    }
}

/// Evaluate a unary operator on a pre-evaluated value.
///
/// - `-`: negate (coerce to number)
/// - `+`: identity (coerce to number — Excel forces numeric coercion)
/// - `%`: percent (coerce to number, divide by 100)
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::ops::eval_unary;
/// assert_eq!(eval_unary("-", &Value::Number(5.0)), Value::Number(-5.0));
/// assert_eq!(eval_unary("%", &Value::Number(50.0)), Value::Number(0.5));
/// ```
#[must_use]
pub fn eval_unary(op: &str, operand: &Value) -> Value {
    if let Value::Error(e) = operand {
        return Value::Error(*e);
    }

    let n = match coerce::to_number(operand) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };

    match op {
        "-" => Value::Number(-n),
        "+" => Value::Number(n),
        "%" => Value::Number(n / 100.0),
        _ => Value::Error(CellError::Value),
    }
}

fn eval_arithmetic(op: &str, left: &Value, right: &Value) -> Value {
    if let Value::Error(e) = left {
        return Value::Error(*e);
    }
    if let Value::Error(e) = right {
        return Value::Error(*e);
    }

    let a = match coerce::to_number(left) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let b = match coerce::to_number(right) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };

    let result = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                return Value::Error(CellError::Div0);
            }
            a / b
        }
        "^" => {
            if a == 0.0 && b < 0.0 {
                return Value::Error(CellError::Div0);
            }
            let r = a.powf(b);
            if r.is_nan() || r.is_infinite() {
                return Value::Error(CellError::Num);
            }
            r
        }
        _ => return Value::Error(CellError::Value),
    };

    if result.is_nan() || result.is_infinite() {
        Value::Error(CellError::Num)
    } else {
        Value::Number(result)
    }
}

fn eval_concat(left: &Value, right: &Value) -> Value {
    if let Value::Error(e) = left {
        return Value::Error(*e);
    }
    if let Value::Error(e) = right {
        return Value::Error(*e);
    }

    let a = coerce::to_text(left);
    let b = coerce::to_text(right);
    let mut result = String::with_capacity(a.len() + b.len());
    result.push_str(&a);
    result.push_str(&b);
    Value::Text(result.into_boxed_str())
}

fn eval_comparison(op: &str, left: &Value, right: &Value) -> Value {
    if let Value::Error(e) = left {
        return Value::Error(*e);
    }
    if let Value::Error(e) = right {
        return Value::Error(*e);
    }

    let ordering = compare_values(left, right);

    let result = match op {
        "=" => ordering == std::cmp::Ordering::Equal,
        "<>" => ordering != std::cmp::Ordering::Equal,
        "<" => ordering == std::cmp::Ordering::Less,
        ">" => ordering == std::cmp::Ordering::Greater,
        "<=" => ordering != std::cmp::Ordering::Greater,
        ">=" => ordering != std::cmp::Ordering::Less,
        _ => return Value::Error(CellError::Value),
    };

    Value::Bool(result)
}

/// Excel comparison type tier. Different tiers never coerce —
/// they use fixed ordering: number < text < boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CompareTier {
    Numeric = 0,
    Text = 1,
    Boolean = 2,
}

fn value_tier(v: &Value) -> CompareTier {
    match v {
        Value::Number(_) | Value::Integer(_) | Value::Date(_) | Value::Empty => {
            CompareTier::Numeric
        }
        Value::Text(_) => CompareTier::Text,
        Value::Bool(_) => CompareTier::Boolean,
        Value::Error(_) => CompareTier::Numeric,
    }
}

fn compare_values(left: &Value, right: &Value) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    // Empty equals both 0 and ""
    match (left, right) {
        (Value::Empty, Value::Text(s)) | (Value::Text(s), Value::Empty) if s.is_empty() => {
            return Ordering::Equal;
        }
        _ => {}
    }

    let left_tier = value_tier(left);
    let right_tier = value_tier(right);

    if left_tier != right_tier {
        return left_tier.cmp(&right_tier);
    }

    match left_tier {
        CompareTier::Numeric => {
            let a = as_compare_number(left);
            let b = as_compare_number(right);
            compare_numbers(a, b)
        }
        CompareTier::Text => {
            let a = coerce::to_text(left);
            let b = coerce::to_text(right);
            a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase())
        }
        CompareTier::Boolean => {
            let a = matches!(left, Value::Bool(true));
            let b = matches!(right, Value::Bool(true));
            a.cmp(&b)
        }
    }
}

fn compare_numbers(a: f64, b: f64) -> std::cmp::Ordering {
    let a = round_to_15_sig_digits(a);
    let b = round_to_15_sig_digits(b);
    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
}

fn as_compare_number(v: &Value) -> f64 {
    match v {
        Value::Number(n) => *n,
        #[allow(clippy::cast_precision_loss)]
        Value::Integer(i) => *i as f64,
        Value::Date(d) => d.serial,
        Value::Empty | Value::Bool(_) | Value::Text(_) | Value::Error(_) => 0.0,
    }
}

fn round_to_15_sig_digits(n: f64) -> f64 {
    if n == 0.0 || !n.is_finite() {
        return n;
    }
    #[allow(clippy::cast_possible_truncation)]
    let d = n.abs().log10().floor() as i32 + 1;
    let exp = 15 - d;
    // Guard against 10^exp overflow for extreme exponents
    if !(-308..=308).contains(&exp) {
        return n;
    }
    let power = 10_f64.powi(exp);
    (n * power).round() / power
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};

    use super::*;

    // ===== Arithmetic =====

    #[test]
    fn add_two_numbers() {
        assert_eq!(eval_binary("+", &Value::Number(1.0), &Value::Number(2.0)), Value::Number(3.0));
    }

    #[test]
    fn add_number_and_bool() {
        assert_eq!(eval_binary("+", &Value::Number(1.0), &Value::Bool(true)), Value::Number(2.0));
    }

    #[test]
    fn add_number_and_numeric_text() {
        assert_eq!(
            eval_binary("+", &Value::Number(10.0), &Value::Text("5".into())),
            Value::Number(15.0)
        );
    }

    #[test]
    fn add_number_and_non_numeric_text() {
        assert_eq!(
            eval_binary("+", &Value::Number(1.0), &Value::Text("abc".into())),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn add_error_left_propagates() {
        assert_eq!(
            eval_binary("+", &Value::Error(CellError::Div0), &Value::Number(1.0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn add_error_right_propagates() {
        assert_eq!(
            eval_binary("+", &Value::Number(1.0), &Value::Error(CellError::Na)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn add_empty_is_zero() {
        assert_eq!(eval_binary("+", &Value::Number(5.0), &Value::Empty), Value::Number(5.0));
    }

    // -- Subtraction --

    #[test]
    fn sub_two_numbers() {
        assert_eq!(eval_binary("-", &Value::Number(5.0), &Value::Number(3.0)), Value::Number(2.0));
    }

    #[test]
    fn sub_error_propagates() {
        assert_eq!(
            eval_binary("-", &Value::Error(CellError::Ref), &Value::Number(1.0)),
            Value::Error(CellError::Ref)
        );
    }

    #[test]
    fn sub_bool_coercion() {
        assert_eq!(eval_binary("-", &Value::Number(5.0), &Value::Bool(true)), Value::Number(4.0));
    }

    #[test]
    fn sub_non_numeric_text() {
        assert_eq!(
            eval_binary("-", &Value::Number(1.0), &Value::Text("x".into())),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn sub_error_right_propagates() {
        assert_eq!(
            eval_binary("-", &Value::Number(1.0), &Value::Error(CellError::Num)),
            Value::Error(CellError::Num)
        );
    }

    // -- Multiplication --

    #[test]
    fn mul_two_numbers() {
        assert_eq!(eval_binary("*", &Value::Number(3.0), &Value::Number(4.0)), Value::Number(12.0));
    }

    #[test]
    fn mul_by_zero() {
        assert_eq!(
            eval_binary("*", &Value::Number(100.0), &Value::Number(0.0)),
            Value::Number(0.0)
        );
    }

    #[test]
    fn mul_bool_coercion() {
        assert_eq!(eval_binary("*", &Value::Bool(true), &Value::Number(7.0)), Value::Number(7.0));
    }

    #[test]
    fn mul_error_left_propagates() {
        assert_eq!(
            eval_binary("*", &Value::Error(CellError::Value), &Value::Number(1.0)),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn mul_non_numeric_text() {
        assert_eq!(
            eval_binary("*", &Value::Text("abc".into()), &Value::Number(1.0)),
            Value::Error(CellError::Value)
        );
    }

    // -- Division --

    #[test]
    fn div_two_numbers() {
        assert_eq!(eval_binary("/", &Value::Number(10.0), &Value::Number(2.0)), Value::Number(5.0));
    }

    #[test]
    fn div_by_zero() {
        assert_eq!(
            eval_binary("/", &Value::Number(1.0), &Value::Number(0.0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn div_zero_by_zero() {
        assert_eq!(
            eval_binary("/", &Value::Number(0.0), &Value::Number(0.0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn div_error_propagates() {
        assert_eq!(
            eval_binary("/", &Value::Error(CellError::Na), &Value::Number(1.0)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn div_non_numeric_text() {
        assert_eq!(
            eval_binary("/", &Value::Number(1.0), &Value::Text("x".into())),
            Value::Error(CellError::Value)
        );
    }

    // -- Power --

    #[test]
    fn pow_two_numbers() {
        assert_eq!(eval_binary("^", &Value::Number(2.0), &Value::Number(3.0)), Value::Number(8.0));
    }

    #[test]
    fn pow_zero_to_zero() {
        assert_eq!(eval_binary("^", &Value::Number(0.0), &Value::Number(0.0)), Value::Number(1.0));
    }

    #[test]
    fn pow_negative_base_integer_exponent() {
        assert_eq!(eval_binary("^", &Value::Number(-2.0), &Value::Number(2.0)), Value::Number(4.0));
    }

    #[test]
    fn pow_negative_base_fractional_exponent() {
        assert_eq!(
            eval_binary("^", &Value::Number(-4.0), &Value::Number(0.5)),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn pow_error_propagates() {
        assert_eq!(
            eval_binary("^", &Value::Error(CellError::Div0), &Value::Number(2.0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn unknown_binary_op_returns_value_error() {
        assert_eq!(
            eval_binary("??", &Value::Number(1.0), &Value::Number(2.0)),
            Value::Error(CellError::Value)
        );
    }

    // ===== Unary =====

    #[test]
    fn unary_negate_number() {
        assert_eq!(eval_unary("-", &Value::Number(5.0)), Value::Number(-5.0));
    }

    #[test]
    fn unary_negate_bool() {
        assert_eq!(eval_unary("-", &Value::Bool(true)), Value::Number(-1.0));
    }

    #[test]
    fn unary_negate_error_propagates() {
        assert_eq!(eval_unary("-", &Value::Error(CellError::Na)), Value::Error(CellError::Na));
    }

    #[test]
    fn unary_negate_text_numeric() {
        assert_eq!(eval_unary("-", &Value::Text("3".into())), Value::Number(-3.0));
    }

    #[test]
    fn unary_negate_text_non_numeric() {
        assert_eq!(eval_unary("-", &Value::Text("abc".into())), Value::Error(CellError::Value));
    }

    #[test]
    fn unary_plus_number() {
        assert_eq!(eval_unary("+", &Value::Number(5.0)), Value::Number(5.0));
    }

    #[test]
    fn unary_plus_bool_coerces() {
        assert_eq!(eval_unary("+", &Value::Bool(true)), Value::Number(1.0));
    }

    #[test]
    fn unary_plus_error_propagates() {
        assert_eq!(eval_unary("+", &Value::Error(CellError::Div0)), Value::Error(CellError::Div0));
    }

    #[test]
    fn unary_percent_number() {
        assert_eq!(eval_unary("%", &Value::Number(50.0)), Value::Number(0.5));
    }

    #[test]
    fn unary_percent_bool() {
        assert_eq!(eval_unary("%", &Value::Bool(true)), Value::Number(0.01));
    }

    #[test]
    fn unary_percent_error_propagates() {
        assert_eq!(eval_unary("%", &Value::Error(CellError::Ref)), Value::Error(CellError::Ref));
    }

    #[test]
    fn unknown_unary_op_returns_value_error() {
        assert_eq!(eval_unary("??", &Value::Number(1.0)), Value::Error(CellError::Value));
    }

    // ===== Concatenation =====

    #[test]
    fn concat_two_texts() {
        assert_eq!(
            eval_binary("&", &Value::Text("a".into()), &Value::Text("b".into())),
            Value::Text("ab".into())
        );
    }

    #[test]
    fn concat_number_and_text() {
        assert_eq!(
            eval_binary("&", &Value::Number(10.0), &Value::Text("5".into())),
            Value::Text("105".into())
        );
    }

    #[test]
    fn concat_bool_and_text() {
        assert_eq!(
            eval_binary("&", &Value::Bool(true), &Value::Text("!".into())),
            Value::Text("TRUE!".into())
        );
    }

    #[test]
    fn concat_empty_and_empty() {
        assert_eq!(eval_binary("&", &Value::Empty, &Value::Empty), Value::Text("".into()));
    }

    #[test]
    fn concat_error_left_propagates() {
        assert_eq!(
            eval_binary("&", &Value::Error(CellError::Div0), &Value::Text("x".into())),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn concat_error_right_propagates() {
        assert_eq!(
            eval_binary("&", &Value::Text("x".into()), &Value::Error(CellError::Na)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn concat_number_formats_without_trailing_zeros() {
        assert_eq!(
            eval_binary("&", &Value::Number(1.0), &Value::Text("".into())),
            Value::Text("1".into())
        );
    }

    // ===== Comparison =====

    #[test]
    fn ieee_round_zero_point_one_plus_zero_point_two() {
        let sum = 0.1_f64 + 0.2_f64;
        let rounded = round_to_15_sig_digits(sum);
        assert_eq!(rounded, 0.3);
    }

    #[test]
    fn ieee_round_one_third_times_three() {
        let result = 1.0_f64 / 3.0 * 3.0;
        let rounded = round_to_15_sig_digits(result);
        assert_eq!(rounded, 1.0);
    }

    #[test]
    fn ieee_round_exact_number_unchanged() {
        assert_eq!(round_to_15_sig_digits(42.0), 42.0);
    }

    #[test]
    fn ieee_round_zero() {
        assert_eq!(round_to_15_sig_digits(0.0), 0.0);
    }

    #[test]
    fn eq_two_numbers() {
        assert_eq!(eval_binary("=", &Value::Number(1.0), &Value::Number(1.0)), Value::Bool(true));
    }

    #[test]
    fn eq_different_numbers() {
        assert_eq!(eval_binary("=", &Value::Number(1.0), &Value::Number(2.0)), Value::Bool(false));
    }

    #[test]
    fn eq_text_case_insensitive() {
        assert_eq!(
            eval_binary("=", &Value::Text("a".into()), &Value::Text("A".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn eq_ieee_rounding() {
        let sum = Value::Number(0.1 + 0.2);
        assert_eq!(eval_binary("=", &sum, &Value::Number(0.3)), Value::Bool(true));
    }

    #[test]
    fn eq_error_left_propagates() {
        assert_eq!(
            eval_binary("=", &Value::Error(CellError::Na), &Value::Number(1.0)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn eq_error_right_propagates() {
        assert_eq!(
            eval_binary("=", &Value::Number(1.0), &Value::Error(CellError::Div0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn ne_two_numbers() {
        assert_eq!(eval_binary("<>", &Value::Number(1.0), &Value::Number(2.0)), Value::Bool(true));
    }

    #[test]
    fn ne_same_numbers() {
        assert_eq!(eval_binary("<>", &Value::Number(1.0), &Value::Number(1.0)), Value::Bool(false));
    }

    #[test]
    fn ne_text_case_insensitive() {
        assert_eq!(
            eval_binary("<>", &Value::Text("abc".into()), &Value::Text("ABC".into())),
            Value::Bool(false)
        );
    }

    #[test]
    fn ne_error_propagates() {
        assert_eq!(
            eval_binary("<>", &Value::Error(CellError::Value), &Value::Number(1.0)),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn ne_mixed_type_number_text() {
        assert_eq!(
            eval_binary("<>", &Value::Number(10.0), &Value::Text("abc".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn lt_numbers() {
        assert_eq!(eval_binary("<", &Value::Number(1.0), &Value::Number(2.0)), Value::Bool(true));
    }

    #[test]
    fn lt_equal_numbers() {
        assert_eq!(eval_binary("<", &Value::Number(2.0), &Value::Number(2.0)), Value::Bool(false));
    }

    #[test]
    fn lt_text_lexicographic() {
        assert_eq!(
            eval_binary("<", &Value::Text("a".into()), &Value::Text("b".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn lt_error_propagates() {
        assert_eq!(
            eval_binary("<", &Value::Error(CellError::Num), &Value::Number(1.0)),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn lt_number_less_than_non_numeric_text() {
        assert_eq!(
            eval_binary("<", &Value::Number(999.0), &Value::Text("abc".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn gt_numbers() {
        assert_eq!(eval_binary(">", &Value::Number(3.0), &Value::Number(2.0)), Value::Bool(true));
    }

    #[test]
    fn gt_text_greater_than_number() {
        assert_eq!(
            eval_binary(">", &Value::Text("abc".into()), &Value::Number(999.0)),
            Value::Bool(true)
        );
    }

    #[test]
    fn gt_error_propagates() {
        assert_eq!(
            eval_binary(">", &Value::Error(CellError::Na), &Value::Number(1.0)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn gt_text_case_insensitive() {
        assert_eq!(
            eval_binary(">", &Value::Text("B".into()), &Value::Text("a".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn gt_text_lexicographic_ten_vs_nine() {
        assert_eq!(
            eval_binary(">", &Value::Text("10".into()), &Value::Text("9".into())),
            Value::Bool(false)
        );
    }

    #[test]
    fn gt_two_numeric_texts_compared_as_text() {
        assert_eq!(
            eval_binary(">", &Value::Text("10".into()), &Value::Text("9".into())),
            Value::Bool(false)
        );
    }

    #[test]
    fn le_equal() {
        assert_eq!(eval_binary("<=", &Value::Number(5.0), &Value::Number(5.0)), Value::Bool(true));
    }

    #[test]
    fn le_less() {
        assert_eq!(eval_binary("<=", &Value::Number(3.0), &Value::Number(5.0)), Value::Bool(true));
    }

    #[test]
    fn le_greater() {
        assert_eq!(eval_binary("<=", &Value::Number(7.0), &Value::Number(5.0)), Value::Bool(false));
    }

    #[test]
    fn le_error_propagates() {
        assert_eq!(
            eval_binary("<=", &Value::Error(CellError::Div0), &Value::Number(1.0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn le_text_case_insensitive() {
        assert_eq!(
            eval_binary("<=", &Value::Text("a".into()), &Value::Text("A".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn ge_equal() {
        assert_eq!(eval_binary(">=", &Value::Number(5.0), &Value::Number(5.0)), Value::Bool(true));
    }

    #[test]
    fn ge_greater() {
        assert_eq!(eval_binary(">=", &Value::Number(7.0), &Value::Number(5.0)), Value::Bool(true));
    }

    #[test]
    fn ge_less() {
        assert_eq!(eval_binary(">=", &Value::Number(3.0), &Value::Number(5.0)), Value::Bool(false));
    }

    #[test]
    fn ge_error_propagates() {
        assert_eq!(
            eval_binary(">=", &Value::Number(1.0), &Value::Error(CellError::Na)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn ge_text_case_insensitive() {
        assert_eq!(
            eval_binary(">=", &Value::Text("A".into()), &Value::Text("a".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn eq_empty_and_zero() {
        assert_eq!(eval_binary("=", &Value::Empty, &Value::Number(0.0)), Value::Bool(true));
    }

    #[test]
    fn eq_empty_and_empty_string() {
        assert_eq!(eval_binary("=", &Value::Empty, &Value::Text("".into())), Value::Bool(true));
    }

    #[test]
    fn eq_empty_and_empty() {
        assert_eq!(eval_binary("=", &Value::Empty, &Value::Empty), Value::Bool(true));
    }

    #[test]
    fn lt_number_less_than_any_text_by_tier() {
        // Number < Text always, regardless of text content (type ordering)
        assert_eq!(
            eval_binary("<", &Value::Number(999.0), &Value::Text("1".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn eq_number_and_numeric_text_different_tiers() {
        // 1 = "1" is FALSE in Excel — different type tiers
        assert_eq!(
            eval_binary("=", &Value::Number(1.0), &Value::Text("1".into())),
            Value::Bool(false)
        );
    }

    // -- Boolean tier tests --

    #[test]
    fn eq_bool_true_and_number_one_different_tiers() {
        // TRUE = 1 is FALSE in Excel — bool and number are different tiers
        assert_eq!(eval_binary("=", &Value::Bool(true), &Value::Number(1.0)), Value::Bool(false));
    }

    #[test]
    fn gt_bool_outranks_number() {
        // TRUE > 1000 is TRUE in Excel — boolean tier > number tier
        assert_eq!(eval_binary(">", &Value::Bool(true), &Value::Number(1000.0)), Value::Bool(true));
    }

    #[test]
    fn gt_bool_outranks_text() {
        // TRUE > "zzz" is TRUE in Excel — boolean tier > text tier
        assert_eq!(
            eval_binary(">", &Value::Bool(true), &Value::Text("zzz".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn eq_bool_true_and_bool_true() {
        assert_eq!(eval_binary("=", &Value::Bool(true), &Value::Bool(true)), Value::Bool(true));
    }

    #[test]
    fn lt_bool_false_less_than_bool_true() {
        assert_eq!(eval_binary("<", &Value::Bool(false), &Value::Bool(true)), Value::Bool(true));
    }

    // -- NaN/inf text --

    #[test]
    fn nan_text_is_non_numeric() {
        // "NaN" should not parse as a number — Excel treats it as text
        assert_eq!(
            eval_binary("+", &Value::Number(1.0), &Value::Text("NaN".into())),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn inf_text_is_non_numeric() {
        assert_eq!(
            eval_binary("+", &Value::Number(1.0), &Value::Text("inf".into())),
            Value::Error(CellError::Value)
        );
    }

    // -- 0^negative --

    #[test]
    fn pow_zero_to_negative_returns_div0() {
        assert_eq!(
            eval_binary("^", &Value::Number(0.0), &Value::Number(-1.0)),
            Value::Error(CellError::Div0)
        );
    }

    // -- IEEE rounding edge case --

    #[test]
    fn ieee_round_very_small_number_does_not_overflow() {
        let tiny = 1e-300_f64;
        let rounded = round_to_15_sig_digits(tiny);
        assert!(rounded.is_finite());
    }
}
