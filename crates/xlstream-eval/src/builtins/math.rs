//! Math builtin functions (Phase 9, Chunk 2).
//!
//! Pure functions: each takes `&[Value]` and returns `Value`.
//! Error propagation is explicit — `Value::Error` inputs are checked
//! before calling `coerce::to_number`.

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use xlstream_core::{coerce, CellError, Value};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Extract a numeric argument at `idx`, propagating errors.
///
/// Returns `Err(Value::Error(_))` if the value at `idx` is an error or
/// non-numeric text.
fn num_arg(args: &[Value], idx: usize) -> Result<f64, Value> {
    match coerce::to_number(args.get(idx).unwrap_or(&Value::Empty)) {
        Ok(n) => Ok(n),
        Err(e) => Err(Value::Error(e)),
    }
}

/// Convert f64 to Decimal via shortest-roundtrip string to preserve
/// the user-visible decimal representation (avoids IEEE 754 artifacts
/// like 2.345 becoming 2.3449999...).
fn to_decimal(x: f64) -> Option<Decimal> {
    Decimal::from_str(&format!("{x}")).ok()
}

/// Decimal-space rounding with half-away-from-zero, matching Excel.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn decimal_round(x: f64, digits: i32) -> f64 {
    let Some(d) = to_decimal(x) else { return x };
    if digits >= 0 {
        d.round_dp_with_strategy(digits as u32, RoundingStrategy::MidpointAwayFromZero)
            .to_f64()
            .unwrap_or(x)
    } else {
        let factor = Decimal::from(10i64.pow((-digits) as u32));
        ((d / factor).round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero) * factor)
            .to_f64()
            .unwrap_or(x)
    }
}

// ---------------------------------------------------------------------------
// ROUND / ROUNDUP / ROUNDDOWN
// ---------------------------------------------------------------------------

/// `ROUND(x, digits)` — round half away from zero (Excel convention).
/// Uses decimal arithmetic to match Excel's rounding of values like 2.345.
pub(crate) fn builtin_round(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let digits = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    #[allow(clippy::cast_possible_truncation)]
    Value::Number(decimal_round(x, digits.round() as i32))
}

/// `ROUNDUP(x, digits)` — round away from zero.
pub(crate) fn builtin_roundup(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let digits = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let Some(d) = to_decimal(x) else {
        return Value::Number(x);
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let dp = digits.round().max(0.0) as u32;
    let rounded = if x >= 0.0 {
        d.round_dp_with_strategy(dp, RoundingStrategy::AwayFromZero)
    } else {
        -(-d).round_dp_with_strategy(dp, RoundingStrategy::AwayFromZero)
    };
    Value::Number(rounded.to_f64().unwrap_or(x))
}

/// `ROUNDDOWN(x, digits)` — round toward zero (truncate).
pub(crate) fn builtin_rounddown(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let digits = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let Some(d) = to_decimal(x) else {
        return Value::Number(x);
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let dp = digits.round().max(0.0) as u32;
    let rounded = d.round_dp_with_strategy(dp, RoundingStrategy::ToZero);
    Value::Number(rounded.to_f64().unwrap_or(x))
}

// ---------------------------------------------------------------------------
// INT / MOD
// ---------------------------------------------------------------------------

/// `INT(x)` — floor toward negative infinity.
pub(crate) fn builtin_int(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    Value::Number(x.floor())
}

/// `MOD(x, y)` — modulo with sign-of-divisor convention.
///
/// `MOD(x, 0)` returns `#DIV/0!`.
pub(crate) fn builtin_mod(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let y = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if y == 0.0 {
        return Value::Error(CellError::Div0);
    }
    Value::Number(x - y * (x / y).floor())
}

// ---------------------------------------------------------------------------
// ABS / SIGN / SQRT / POWER
// ---------------------------------------------------------------------------

/// `ABS(x)` — absolute value.
pub(crate) fn builtin_abs(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    Value::Number(x.abs())
}

/// `SIGN(x)` — returns -1, 0, or 1.
pub(crate) fn builtin_sign(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let s = if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    };
    Value::Number(s)
}

/// `SQRT(x)` — square root. Negative input returns `#NUM!`.
pub(crate) fn builtin_sqrt(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if x < 0.0 {
        return Value::Error(CellError::Num);
    }
    Value::Number(x.sqrt())
}

/// `POWER(base, exp)` — exponentiation. NaN results return `#NUM!`.
pub(crate) fn builtin_power(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let base = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let exp = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if base == 0.0 && exp < 0.0 {
        return Value::Error(CellError::Div0);
    }
    let result = base.powf(exp);
    if result.is_nan() || result.is_infinite() {
        return Value::Error(CellError::Num);
    }
    Value::Number(result)
}

// ---------------------------------------------------------------------------
// CEILING / FLOOR
// ---------------------------------------------------------------------------

/// `CEILING(x, significance)` — round up to nearest multiple of significance.
///
/// - `significance = 0` returns 0.
/// - `x > 0 && significance < 0` returns `#NUM!`.
pub(crate) fn builtin_ceiling(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let sig = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if sig == 0.0 {
        return Value::Number(0.0);
    }
    if x > 0.0 && sig < 0.0 {
        return Value::Error(CellError::Num);
    }
    Value::Number((x / sig).ceil() * sig)
}

/// `FLOOR(x, significance)` — round down to nearest multiple of significance.
///
/// - `significance = 0` returns 0.
/// - `x < 0 && significance > 0` returns `#NUM!`.
pub(crate) fn builtin_floor(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let sig = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if sig == 0.0 {
        return Value::Number(0.0);
    }
    Value::Number((x / sig).floor() * sig)
}

// ---------------------------------------------------------------------------
// PI
// ---------------------------------------------------------------------------

/// `PI()` — returns the constant pi. Any arguments return `#VALUE!`.
pub(crate) fn builtin_pi(args: &[Value]) -> Value {
    if !args.is_empty() {
        return Value::Error(CellError::Value);
    }
    Value::Number(std::f64::consts::PI)
}

// ---------------------------------------------------------------------------
// LN / LOG / LOG10 / EXP
// ---------------------------------------------------------------------------

/// `LN(x)` — natural logarithm. `x <= 0` returns `#NUM!`.
pub(crate) fn builtin_ln(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if x <= 0.0 {
        return Value::Error(CellError::Num);
    }
    Value::Number(x.ln())
}

/// `LOG(x, base?)` — logarithm. Base defaults to 10.
///
/// - `x <= 0` returns `#NUM!`.
/// - `base <= 0` or `base == 1` returns `#NUM!`.
#[allow(clippy::float_cmp)]
pub(crate) fn builtin_log(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if x <= 0.0 {
        return Value::Error(CellError::Num);
    }
    let base = if args.len() == 2 {
        match num_arg(args, 1) {
            Ok(n) => n,
            Err(e) => return e,
        }
    } else {
        10.0
    };
    if base <= 0.0 || base == 1.0 {
        return Value::Error(CellError::Num);
    }
    Value::Number(x.log(base))
}

/// `LOG10(x)` — base-10 logarithm. `x <= 0` returns `#NUM!`.
pub(crate) fn builtin_log10(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if x <= 0.0 {
        return Value::Error(CellError::Num);
    }
    Value::Number(x.log10())
}

/// `EXP(x)` — e raised to the power of x.
pub(crate) fn builtin_exp(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let result = x.exp();
    if result.is_infinite() || result.is_nan() {
        return Value::Error(CellError::Num);
    }
    Value::Number(result)
}

// ---------------------------------------------------------------------------
// SIN / COS / TAN / ASIN / ACOS / ATAN / ATAN2
// ---------------------------------------------------------------------------

/// `SIN(x)` — sine of x in radians.
pub(crate) fn builtin_sin(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    Value::Number(x.sin())
}

/// `COS(x)` — cosine of x in radians.
pub(crate) fn builtin_cos(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    Value::Number(x.cos())
}

/// `TAN(x)` — tangent of x in radians.
pub(crate) fn builtin_tan(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    Value::Number(x.tan())
}

/// `ASIN(x)` — arcsine. Out-of-range `[-1, 1]` returns `#NUM!`.
pub(crate) fn builtin_asin(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let result = x.asin();
    if result.is_nan() {
        return Value::Error(CellError::Num);
    }
    Value::Number(result)
}

/// `ACOS(x)` — arccosine. Out-of-range `[-1, 1]` returns `#NUM!`.
pub(crate) fn builtin_acos(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let result = x.acos();
    if result.is_nan() {
        return Value::Error(CellError::Num);
    }
    Value::Number(result)
}

/// `ATAN(x)` — arctangent, returns radians.
pub(crate) fn builtin_atan(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    Value::Number(x.atan())
}

/// `ATAN2(x_num, y_num)` — four-quadrant arctangent.
///
/// Excel arg order: first arg is x, second is y. Computes `y.atan2(x)`.
/// Both zero returns `#DIV/0!`.
pub(crate) fn builtin_atan2(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let y = match num_arg(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if x == 0.0 && y == 0.0 {
        return Value::Error(CellError::Div0);
    }
    Value::Number(y.atan2(x))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    use xlstream_core::{CellError, Value};

    use super::*;

    // Helper for approximate equality
    fn assert_approx(actual: Value, expected: f64, tol: f64) {
        match actual {
            Value::Number(n) => {
                assert!(
                    (n - expected).abs() < tol,
                    "expected {expected}, got {n} (diff {})",
                    (n - expected).abs()
                );
            }
            other => panic!("expected Number, got {other:?}"),
        }
    }

    // ===== ROUND =====

    #[test]
    fn round_half_away_from_zero_positive() {
        assert_eq!(builtin_round(&[Value::Number(0.5), Value::Number(0.0)]), Value::Number(1.0));
    }

    #[test]
    fn round_half_away_from_zero_negative() {
        assert_eq!(builtin_round(&[Value::Number(-0.5), Value::Number(0.0)]), Value::Number(-1.0));
    }

    #[test]
    fn round_two_decimal_places() {
        assert_eq!(builtin_round(&[Value::Number(2.345), Value::Number(2.0)]), Value::Number(2.35));
    }

    #[test]
    fn round_1_005_to_two_decimal_places() {
        assert_eq!(builtin_round(&[Value::Number(1.005), Value::Number(2.0)]), Value::Number(1.01));
    }

    #[test]
    fn round_negative_digits() {
        assert_eq!(
            builtin_round(&[Value::Number(1234.0), Value::Number(-2.0)]),
            Value::Number(1200.0)
        );
    }

    #[test]
    fn round_zero() {
        assert_eq!(builtin_round(&[Value::Number(0.0), Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn round_error_propagation() {
        assert_eq!(
            builtin_round(&[Value::Error(CellError::Na), Value::Number(0.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn round_coercion_from_text() {
        assert_eq!(
            builtin_round(&[Value::Text("2.5".into()), Value::Number(0.0)]),
            Value::Number(3.0)
        );
    }

    #[test]
    fn round_wrong_arg_count() {
        assert_eq!(builtin_round(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn round_type_mismatch() {
        assert_eq!(
            builtin_round(&[Value::Text("abc".into()), Value::Number(0.0)]),
            Value::Error(CellError::Value)
        );
    }

    // ===== ROUNDUP =====

    #[test]
    fn roundup_positive() {
        assert_eq!(builtin_roundup(&[Value::Number(2.1), Value::Number(0.0)]), Value::Number(3.0));
    }

    #[test]
    fn roundup_negative() {
        assert_eq!(
            builtin_roundup(&[Value::Number(-2.1), Value::Number(0.0)]),
            Value::Number(-3.0)
        );
    }

    #[test]
    fn roundup_already_integer() {
        assert_eq!(builtin_roundup(&[Value::Number(3.0), Value::Number(0.0)]), Value::Number(3.0));
    }

    #[test]
    fn roundup_decimal_digits() {
        assert_eq!(
            builtin_roundup(&[Value::Number(2.121), Value::Number(2.0)]),
            Value::Number(2.13)
        );
    }

    #[test]
    fn roundup_error_propagation() {
        assert_eq!(
            builtin_roundup(&[Value::Number(1.0), Value::Error(CellError::Ref)]),
            Value::Error(CellError::Ref)
        );
    }

    #[test]
    fn roundup_wrong_arg_count() {
        assert_eq!(builtin_roundup(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    // ===== ROUNDDOWN =====

    #[test]
    fn rounddown_positive() {
        assert_eq!(
            builtin_rounddown(&[Value::Number(2.9), Value::Number(0.0)]),
            Value::Number(2.0)
        );
    }

    #[test]
    fn rounddown_negative() {
        assert_eq!(
            builtin_rounddown(&[Value::Number(-2.9), Value::Number(0.0)]),
            Value::Number(-2.0)
        );
    }

    #[test]
    fn rounddown_already_integer() {
        assert_eq!(
            builtin_rounddown(&[Value::Number(3.0), Value::Number(0.0)]),
            Value::Number(3.0)
        );
    }

    #[test]
    fn rounddown_decimal_digits() {
        assert_eq!(
            builtin_rounddown(&[Value::Number(2.129), Value::Number(2.0)]),
            Value::Number(2.12)
        );
    }

    #[test]
    fn rounddown_error_propagation() {
        assert_eq!(
            builtin_rounddown(&[Value::Error(CellError::Div0), Value::Number(0.0)]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn rounddown_wrong_arg_count() {
        assert_eq!(builtin_rounddown(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    // ===== INT =====

    #[test]
    fn int_positive_fraction() {
        assert_eq!(builtin_int(&[Value::Number(1.9)]), Value::Number(1.0));
    }

    #[test]
    fn int_negative_floors_toward_negative_infinity() {
        assert_eq!(builtin_int(&[Value::Number(-1.5)]), Value::Number(-2.0));
    }

    #[test]
    fn int_already_integer() {
        assert_eq!(builtin_int(&[Value::Number(5.0)]), Value::Number(5.0));
    }

    #[test]
    fn int_zero() {
        assert_eq!(builtin_int(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn int_error_propagation() {
        assert_eq!(builtin_int(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn int_coercion_from_text() {
        assert_eq!(builtin_int(&[Value::Text("3.7".into())]), Value::Number(3.0));
    }

    #[test]
    fn int_type_mismatch() {
        assert_eq!(builtin_int(&[Value::Text("abc".into())]), Value::Error(CellError::Value));
    }

    #[test]
    fn int_wrong_arg_count() {
        assert_eq!(builtin_int(&[]), Value::Error(CellError::Value));
    }

    // ===== MOD =====

    #[test]
    fn mod_sign_of_divisor_positive() {
        assert_eq!(builtin_mod(&[Value::Number(-3.0), Value::Number(2.0)]), Value::Number(1.0));
    }

    #[test]
    fn mod_sign_of_divisor_negative() {
        assert_eq!(builtin_mod(&[Value::Number(3.0), Value::Number(-2.0)]), Value::Number(-1.0));
    }

    #[test]
    fn mod_positive_args() {
        assert_eq!(builtin_mod(&[Value::Number(7.0), Value::Number(3.0)]), Value::Number(1.0));
    }

    #[test]
    fn mod_zero_divisor_returns_div0() {
        assert_eq!(
            builtin_mod(&[Value::Number(5.0), Value::Number(0.0)]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn mod_error_propagation() {
        assert_eq!(
            builtin_mod(&[Value::Error(CellError::Na), Value::Number(2.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn mod_coercion_from_text() {
        assert_eq!(
            builtin_mod(&[Value::Text("10".into()), Value::Number(3.0)]),
            Value::Number(1.0)
        );
    }

    #[test]
    fn mod_type_mismatch() {
        assert_eq!(
            builtin_mod(&[Value::Text("abc".into()), Value::Number(2.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn mod_wrong_arg_count() {
        assert_eq!(builtin_mod(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    // ===== ABS =====

    #[test]
    fn abs_positive() {
        assert_eq!(builtin_abs(&[Value::Number(5.0)]), Value::Number(5.0));
    }

    #[test]
    fn abs_negative() {
        assert_eq!(builtin_abs(&[Value::Number(-5.0)]), Value::Number(5.0));
    }

    #[test]
    fn abs_zero() {
        assert_eq!(builtin_abs(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn abs_error_propagation() {
        assert_eq!(builtin_abs(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn abs_coercion_from_text() {
        assert_eq!(builtin_abs(&[Value::Text("-7".into())]), Value::Number(7.0));
    }

    #[test]
    fn abs_wrong_arg_count() {
        assert_eq!(builtin_abs(&[]), Value::Error(CellError::Value));
    }

    // ===== SIGN =====

    #[test]
    fn sign_positive() {
        assert_eq!(builtin_sign(&[Value::Number(42.0)]), Value::Number(1.0));
    }

    #[test]
    fn sign_negative() {
        assert_eq!(builtin_sign(&[Value::Number(-42.0)]), Value::Number(-1.0));
    }

    #[test]
    fn sign_zero() {
        assert_eq!(builtin_sign(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn sign_error_propagation() {
        assert_eq!(builtin_sign(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn sign_coercion_from_bool() {
        assert_eq!(builtin_sign(&[Value::Bool(true)]), Value::Number(1.0));
    }

    #[test]
    fn sign_wrong_arg_count() {
        assert_eq!(builtin_sign(&[]), Value::Error(CellError::Value));
    }

    // ===== SQRT =====

    #[test]
    fn sqrt_perfect_square() {
        assert_eq!(builtin_sqrt(&[Value::Number(9.0)]), Value::Number(3.0));
    }

    #[test]
    fn sqrt_zero() {
        assert_eq!(builtin_sqrt(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn sqrt_fractional() {
        assert_eq!(builtin_sqrt(&[Value::Number(2.0)]), Value::Number(2.0_f64.sqrt()));
    }

    #[test]
    fn sqrt_negative_returns_num() {
        assert_eq!(builtin_sqrt(&[Value::Number(-1.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn sqrt_error_propagation() {
        assert_eq!(builtin_sqrt(&[Value::Error(CellError::Ref)]), Value::Error(CellError::Ref));
    }

    #[test]
    fn sqrt_wrong_arg_count() {
        assert_eq!(builtin_sqrt(&[]), Value::Error(CellError::Value));
    }

    // ===== POWER =====

    #[test]
    fn power_basic() {
        assert_eq!(builtin_power(&[Value::Number(2.0), Value::Number(3.0)]), Value::Number(8.0));
    }

    #[test]
    fn power_fractional_exponent() {
        assert_eq!(builtin_power(&[Value::Number(4.0), Value::Number(0.5)]), Value::Number(2.0));
    }

    #[test]
    fn power_zero_exponent() {
        assert_eq!(builtin_power(&[Value::Number(5.0), Value::Number(0.0)]), Value::Number(1.0));
    }

    #[test]
    fn power_negative_base_fractional_exp_returns_num() {
        assert_eq!(
            builtin_power(&[Value::Number(-2.0), Value::Number(0.5)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn power_error_propagation() {
        assert_eq!(
            builtin_power(&[Value::Error(CellError::Na), Value::Number(2.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn power_wrong_arg_count() {
        assert_eq!(builtin_power(&[Value::Number(2.0)]), Value::Error(CellError::Value));
    }

    // ===== CEILING =====

    #[test]
    fn ceiling_round_up_positive() {
        assert_eq!(builtin_ceiling(&[Value::Number(2.5), Value::Number(1.0)]), Value::Number(3.0));
    }

    #[test]
    fn ceiling_already_multiple() {
        assert_eq!(builtin_ceiling(&[Value::Number(6.0), Value::Number(3.0)]), Value::Number(6.0));
    }

    #[test]
    fn ceiling_sig_zero_returns_zero() {
        assert_eq!(builtin_ceiling(&[Value::Number(5.0), Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn ceiling_positive_x_negative_sig_returns_num() {
        assert_eq!(
            builtin_ceiling(&[Value::Number(5.0), Value::Number(-1.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn ceiling_error_propagation() {
        assert_eq!(
            builtin_ceiling(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn ceiling_wrong_arg_count() {
        assert_eq!(builtin_ceiling(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    // ===== FLOOR =====

    #[test]
    fn floor_round_down_positive() {
        assert_eq!(builtin_floor(&[Value::Number(2.5), Value::Number(1.0)]), Value::Number(2.0));
    }

    #[test]
    fn floor_already_multiple() {
        assert_eq!(builtin_floor(&[Value::Number(6.0), Value::Number(3.0)]), Value::Number(6.0));
    }

    #[test]
    fn floor_sig_zero_returns_zero() {
        assert_eq!(builtin_floor(&[Value::Number(5.0), Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn floor_negative_x_positive_sig_rounds_toward_neg_infinity() {
        assert_eq!(builtin_floor(&[Value::Number(-5.0), Value::Number(1.0)]), Value::Number(-5.0));
        assert_eq!(builtin_floor(&[Value::Number(-2.3), Value::Number(1.0)]), Value::Number(-3.0));
    }

    #[test]
    fn floor_error_propagation() {
        assert_eq!(
            builtin_floor(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn floor_wrong_arg_count() {
        assert_eq!(builtin_floor(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    // ===== PI =====

    #[test]
    fn pi_returns_constant() {
        assert_eq!(builtin_pi(&[]), Value::Number(PI));
    }

    #[test]
    fn pi_with_args_returns_value_error() {
        assert_eq!(builtin_pi(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn pi_approximate_value() {
        assert_approx(builtin_pi(&[]), PI, 1e-15);
    }

    #[test]
    fn pi_is_positive() {
        match builtin_pi(&[]) {
            Value::Number(n) => assert!(n > 3.0 && n < 4.0),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pi_two_args_returns_value_error() {
        assert_eq!(
            builtin_pi(&[Value::Number(1.0), Value::Number(2.0)]),
            Value::Error(CellError::Value)
        );
    }

    // ===== LN =====

    #[test]
    fn ln_of_one() {
        assert_eq!(builtin_ln(&[Value::Number(1.0)]), Value::Number(0.0));
    }

    #[test]
    fn ln_of_e() {
        assert_approx(builtin_ln(&[Value::Number(std::f64::consts::E)]), 1.0, 1e-15);
    }

    #[test]
    fn ln_zero_returns_num() {
        assert_eq!(builtin_ln(&[Value::Number(0.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn ln_negative_returns_num() {
        assert_eq!(builtin_ln(&[Value::Number(-1.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn ln_error_propagation() {
        assert_eq!(builtin_ln(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn ln_wrong_arg_count() {
        assert_eq!(builtin_ln(&[]), Value::Error(CellError::Value));
    }

    // ===== LOG =====

    #[test]
    fn log_default_base_10() {
        assert_eq!(builtin_log(&[Value::Number(100.0)]), Value::Number(2.0));
    }

    #[test]
    fn log_explicit_base() {
        assert_eq!(builtin_log(&[Value::Number(8.0), Value::Number(2.0)]), Value::Number(3.0));
    }

    #[test]
    fn log_x_zero_returns_num() {
        assert_eq!(builtin_log(&[Value::Number(0.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn log_x_negative_returns_num() {
        assert_eq!(builtin_log(&[Value::Number(-5.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn log_base_one_returns_num() {
        assert_eq!(
            builtin_log(&[Value::Number(10.0), Value::Number(1.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn log_base_zero_returns_num() {
        assert_eq!(
            builtin_log(&[Value::Number(10.0), Value::Number(0.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn log_base_negative_returns_num() {
        assert_eq!(
            builtin_log(&[Value::Number(10.0), Value::Number(-2.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn log_error_propagation() {
        assert_eq!(builtin_log(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn log_wrong_arg_count() {
        assert_eq!(builtin_log(&[]), Value::Error(CellError::Value));
    }

    // ===== LOG10 =====

    #[test]
    fn log10_basic() {
        assert_eq!(builtin_log10(&[Value::Number(1000.0)]), Value::Number(3.0));
    }

    #[test]
    fn log10_one() {
        assert_eq!(builtin_log10(&[Value::Number(1.0)]), Value::Number(0.0));
    }

    #[test]
    fn log10_zero_returns_num() {
        assert_eq!(builtin_log10(&[Value::Number(0.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn log10_negative_returns_num() {
        assert_eq!(builtin_log10(&[Value::Number(-10.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn log10_error_propagation() {
        assert_eq!(builtin_log10(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn log10_wrong_arg_count() {
        assert_eq!(builtin_log10(&[]), Value::Error(CellError::Value));
    }

    // ===== EXP =====

    #[test]
    fn exp_zero() {
        assert_eq!(builtin_exp(&[Value::Number(0.0)]), Value::Number(1.0));
    }

    #[test]
    fn exp_one() {
        assert_approx(builtin_exp(&[Value::Number(1.0)]), std::f64::consts::E, 1e-15);
    }

    #[test]
    fn exp_negative() {
        assert_approx(builtin_exp(&[Value::Number(-1.0)]), 1.0 / std::f64::consts::E, 1e-15);
    }

    #[test]
    fn exp_error_propagation() {
        assert_eq!(builtin_exp(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn exp_coercion_from_text() {
        assert_eq!(builtin_exp(&[Value::Text("0".into())]), Value::Number(1.0));
    }

    #[test]
    fn exp_wrong_arg_count() {
        assert_eq!(builtin_exp(&[]), Value::Error(CellError::Value));
    }

    // ===== SIN =====

    #[test]
    fn sin_zero() {
        assert_eq!(builtin_sin(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn sin_pi_half() {
        assert_approx(builtin_sin(&[Value::Number(FRAC_PI_2)]), 1.0, 1e-15);
    }

    #[test]
    fn sin_negative() {
        assert_approx(builtin_sin(&[Value::Number(-FRAC_PI_2)]), -1.0, 1e-15);
    }

    #[test]
    fn sin_error_propagation() {
        assert_eq!(builtin_sin(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn sin_wrong_arg_count() {
        assert_eq!(builtin_sin(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn sin_coercion_from_bool() {
        // sin(1) ~= 0.8414
        assert_approx(builtin_sin(&[Value::Bool(true)]), 1.0_f64.sin(), 1e-15);
    }

    // ===== COS =====

    #[test]
    fn cos_zero() {
        assert_eq!(builtin_cos(&[Value::Number(0.0)]), Value::Number(1.0));
    }

    #[test]
    fn cos_pi() {
        assert_approx(builtin_cos(&[Value::Number(PI)]), -1.0, 1e-15);
    }

    #[test]
    fn cos_negative() {
        assert_approx(builtin_cos(&[Value::Number(-PI)]), -1.0, 1e-15);
    }

    #[test]
    fn cos_error_propagation() {
        assert_eq!(builtin_cos(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn cos_wrong_arg_count() {
        assert_eq!(builtin_cos(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn cos_coercion_from_text() {
        assert_eq!(builtin_cos(&[Value::Text("0".into())]), Value::Number(1.0));
    }

    // ===== TAN =====

    #[test]
    fn tan_zero() {
        assert_eq!(builtin_tan(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn tan_pi_over_four() {
        assert_approx(builtin_tan(&[Value::Number(FRAC_PI_4)]), 1.0, 1e-15);
    }

    #[test]
    fn tan_negative() {
        assert_approx(builtin_tan(&[Value::Number(-FRAC_PI_4)]), -1.0, 1e-15);
    }

    #[test]
    fn tan_error_propagation() {
        assert_eq!(builtin_tan(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn tan_wrong_arg_count() {
        assert_eq!(builtin_tan(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn tan_coercion_from_bool() {
        assert_approx(builtin_tan(&[Value::Bool(true)]), 1.0_f64.tan(), 1e-15);
    }

    // ===== ASIN =====

    #[test]
    fn asin_zero() {
        assert_eq!(builtin_asin(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn asin_one() {
        assert_approx(builtin_asin(&[Value::Number(1.0)]), FRAC_PI_2, 1e-15);
    }

    #[test]
    fn asin_negative_one() {
        assert_approx(builtin_asin(&[Value::Number(-1.0)]), -FRAC_PI_2, 1e-15);
    }

    #[test]
    fn asin_out_of_range_returns_num() {
        assert_eq!(builtin_asin(&[Value::Number(2.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn asin_error_propagation() {
        assert_eq!(builtin_asin(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn asin_wrong_arg_count() {
        assert_eq!(builtin_asin(&[]), Value::Error(CellError::Value));
    }

    // ===== ACOS =====

    #[test]
    fn acos_one() {
        assert_eq!(builtin_acos(&[Value::Number(1.0)]), Value::Number(0.0));
    }

    #[test]
    fn acos_zero() {
        assert_approx(builtin_acos(&[Value::Number(0.0)]), FRAC_PI_2, 1e-15);
    }

    #[test]
    fn acos_negative_one() {
        assert_approx(builtin_acos(&[Value::Number(-1.0)]), PI, 1e-15);
    }

    #[test]
    fn acos_out_of_range_returns_num() {
        assert_eq!(builtin_acos(&[Value::Number(2.0)]), Value::Error(CellError::Num));
    }

    #[test]
    fn acos_error_propagation() {
        assert_eq!(builtin_acos(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn acos_wrong_arg_count() {
        assert_eq!(builtin_acos(&[]), Value::Error(CellError::Value));
    }

    // ===== ATAN =====

    #[test]
    fn atan_zero() {
        assert_eq!(builtin_atan(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn atan_one() {
        assert_approx(builtin_atan(&[Value::Number(1.0)]), FRAC_PI_4, 1e-15);
    }

    #[test]
    fn atan_negative() {
        assert_approx(builtin_atan(&[Value::Number(-1.0)]), -FRAC_PI_4, 1e-15);
    }

    #[test]
    fn atan_error_propagation() {
        assert_eq!(builtin_atan(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn atan_wrong_arg_count() {
        assert_eq!(builtin_atan(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn atan_coercion_from_text() {
        assert_eq!(builtin_atan(&[Value::Text("0".into())]), Value::Number(0.0));
    }

    // ===== ATAN2 =====

    #[test]
    fn atan2_quadrant_one() {
        assert_approx(builtin_atan2(&[Value::Number(1.0), Value::Number(1.0)]), FRAC_PI_4, 1e-15);
    }

    #[test]
    fn atan2_both_zero_returns_div0() {
        assert_eq!(
            builtin_atan2(&[Value::Number(0.0), Value::Number(0.0)]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn atan2_x_zero_y_positive() {
        assert_approx(builtin_atan2(&[Value::Number(0.0), Value::Number(1.0)]), FRAC_PI_2, 1e-15);
    }

    #[test]
    fn atan2_negative_args() {
        assert_approx(
            builtin_atan2(&[Value::Number(-1.0), Value::Number(-1.0)]),
            -3.0 * FRAC_PI_4,
            1e-15,
        );
    }

    #[test]
    fn atan2_error_propagation() {
        assert_eq!(
            builtin_atan2(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn atan2_wrong_arg_count() {
        assert_eq!(builtin_atan2(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }
}
