//! Excel-compatible type coercion helpers.
//!
//! Every operator and builtin function uses these helpers — no ad-hoc
//! conversions. The three functions mirror Excel's three coercion
//! contexts: numeric, text, and boolean.

use std::borrow::Cow;

use crate::{CellError, Value};

/// Coerce a [`Value`] to `f64` using Excel's numeric coercion rules.
///
/// | Variant | Result |
/// |---------|--------|
/// | `Number(n)` | `Ok(n)` |
/// | `Integer(i)` | `Ok(i as f64)` |
/// | `Bool(true)` | `Ok(1.0)` |
/// | `Bool(false)` | `Ok(0.0)` |
/// | `Text(s)` | parsed, or `Err(CellError::Value)` |
/// | `Empty` | `Ok(0.0)` |
/// | `Date(d)` | `Ok(d.serial)` |
/// | `Error(e)` | `Err(e)` — propagated |
///
/// # Errors
///
/// Returns `Err(CellError::Value)` for non-numeric text.
/// Returns `Err(e)` for `Value::Error(e)` (propagated).
///
/// # Examples
///
/// ```
/// use xlstream_core::{Value, CellError, coerce};
/// assert_eq!(coerce::to_number(&Value::Number(1.5)), Ok(1.5));
/// assert_eq!(coerce::to_number(&Value::Bool(true)), Ok(1.0));
/// assert_eq!(coerce::to_number(&Value::Text("abc".into())), Err(CellError::Value));
/// ```
pub fn to_number(v: &Value) -> Result<f64, CellError> {
    match v {
        Value::Number(n) => Ok(*n),
        #[allow(clippy::cast_precision_loss)]
        Value::Integer(i) => Ok(*i as f64),
        Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
        Value::Text(s) => {
            let n = s.trim().parse::<f64>().map_err(|_| CellError::Value)?;
            if n.is_finite() {
                Ok(n)
            } else {
                Err(CellError::Value)
            }
        }
        Value::Empty => Ok(0.0),
        Value::Date(d) => Ok(d.serial),
        Value::Error(e) => Err(*e),
    }
}

/// Coerce a [`Value`] to text using Excel's text coercion rules.
///
/// Returns `Cow::Borrowed` for zero-allocation paths (booleans, text,
/// empty). Returns `Cow::Owned` for formatted numbers and dates.
///
/// Callers must check for `Error` variants before calling — this
/// function returns an error display string as a safety net, but
/// correct code propagates errors before reaching `to_text`.
///
/// # Examples
///
/// ```
/// use xlstream_core::{Value, coerce};
/// assert_eq!(coerce::to_text(&Value::Number(1.0)).as_ref(), "1");
/// assert_eq!(coerce::to_text(&Value::Bool(true)).as_ref(), "TRUE");
/// assert_eq!(coerce::to_text(&Value::Empty).as_ref(), "");
/// ```
#[must_use]
pub fn to_text(v: &Value) -> Cow<'_, str> {
    match v {
        Value::Number(n) => Cow::Owned(format_number(*n)),
        Value::Integer(i) => Cow::Owned(format!("{i}")),
        Value::Bool(true) => Cow::Borrowed("TRUE"),
        Value::Bool(false) => Cow::Borrowed("FALSE"),
        Value::Text(s) => Cow::Borrowed(s),
        Value::Empty => Cow::Borrowed(""),
        Value::Date(d) => Cow::Owned(format_number(d.serial)),
        Value::Error(e) => Cow::Owned(format_cell_error(*e)),
    }
}

/// Coerce a [`Value`] to `bool` using Excel's boolean coercion rules.
///
/// | Variant | Result |
/// |---------|--------|
/// | `Bool(b)` | `Ok(b)` |
/// | `Number(0.0)` | `Ok(false)` |
/// | `Number(_)` | `Ok(true)` |
/// | `Integer(0)` | `Ok(false)` |
/// | `Integer(_)` | `Ok(true)` |
/// | `Text("TRUE")` | `Ok(true)` (case-insensitive) |
/// | `Text("FALSE")` | `Ok(false)` (case-insensitive) |
/// | Other `Text` | `Err(CellError::Value)` |
/// | `Empty` | `Ok(false)` |
/// | `Date(_)` | `Ok(true)` |
/// | `Error(e)` | `Err(e)` — propagated |
///
/// # Errors
///
/// Returns `Err(CellError::Value)` for non-boolean text.
/// Returns `Err(e)` for `Value::Error(e)` (propagated).
///
/// # Examples
///
/// ```
/// use xlstream_core::{Value, CellError, coerce};
/// assert_eq!(coerce::to_bool(&Value::Bool(true)), Ok(true));
/// assert_eq!(coerce::to_bool(&Value::Number(0.0)), Ok(false));
/// assert_eq!(coerce::to_bool(&Value::Text("abc".into())), Err(CellError::Value));
/// ```
pub fn to_bool(v: &Value) -> Result<bool, CellError> {
    match v {
        Value::Bool(b) => Ok(*b),
        Value::Number(n) => Ok(*n != 0.0),
        Value::Integer(i) => Ok(*i != 0),
        Value::Text(s) => {
            if s.eq_ignore_ascii_case("true") {
                Ok(true)
            } else if s.eq_ignore_ascii_case("false") {
                Ok(false)
            } else {
                Err(CellError::Value)
            }
        }
        Value::Empty => Ok(false),
        Value::Date(_) => Ok(true),
        Value::Error(e) => Err(*e),
    }
}

/// Format an `f64` the way Excel displays numbers in text context.
///
/// Integers display without a decimal point (`1.0` -> `"1"`).
/// Non-integers keep significant decimal digits (`1.5` -> `"1.5"`).
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.is_finite() && n.abs() < 1e15 {
        #[allow(clippy::cast_possible_truncation)]
        let i = n as i64;
        format!("{i}")
    } else {
        format!("{n}")
    }
}

fn format_cell_error(e: CellError) -> String {
    match e {
        CellError::Div0 => "#DIV/0!".into(),
        CellError::Value => "#VALUE!".into(),
        CellError::Ref => "#REF!".into(),
        CellError::Name => "#NAME?".into(),
        CellError::Na => "#N/A".into(),
        CellError::Num => "#NUM!".into(),
        CellError::Null => "#NULL!".into(),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;
    use crate::{CellError, ExcelDate, Value};

    // -- to_number --

    #[test]
    fn to_number_from_number() {
        assert_eq!(to_number(&Value::Number(42.5)), Ok(42.5));
    }

    #[test]
    fn to_number_from_integer() {
        assert_eq!(to_number(&Value::Integer(7)), Ok(7.0));
    }

    #[test]
    fn to_number_from_bool_true() {
        assert_eq!(to_number(&Value::Bool(true)), Ok(1.0));
    }

    #[test]
    fn to_number_from_bool_false() {
        assert_eq!(to_number(&Value::Bool(false)), Ok(0.0));
    }

    #[test]
    fn to_number_from_text_numeric() {
        assert_eq!(to_number(&Value::Text("2.75".into())), Ok(2.75));
    }

    #[test]
    fn to_number_from_text_non_numeric() {
        assert_eq!(to_number(&Value::Text("abc".into())), Err(CellError::Value));
    }

    #[test]
    fn to_number_from_text_nan_rejected() {
        assert_eq!(to_number(&Value::Text("NaN".into())), Err(CellError::Value));
    }

    #[test]
    fn to_number_from_text_inf_rejected() {
        assert_eq!(to_number(&Value::Text("inf".into())), Err(CellError::Value));
    }

    #[test]
    fn to_number_from_text_negative_inf_rejected() {
        assert_eq!(to_number(&Value::Text("-inf".into())), Err(CellError::Value));
    }

    #[test]
    fn to_number_from_text_infinity_rejected() {
        assert_eq!(to_number(&Value::Text("infinity".into())), Err(CellError::Value));
    }

    #[test]
    fn to_number_from_empty() {
        assert_eq!(to_number(&Value::Empty), Ok(0.0));
    }

    #[test]
    fn to_number_from_date() {
        assert_eq!(to_number(&Value::Date(ExcelDate { serial: 44927.0 })), Ok(44927.0));
    }

    #[test]
    fn to_number_from_error_propagates() {
        assert_eq!(to_number(&Value::Error(CellError::Div0)), Err(CellError::Div0));
    }

    #[test]
    fn to_number_from_text_whitespace_trimmed() {
        assert_eq!(to_number(&Value::Text("  42  ".into())), Ok(42.0));
    }

    // -- to_text --

    #[test]
    fn to_text_from_number_integer_value() {
        assert_eq!(to_text(&Value::Number(42.0)).as_ref(), "42");
    }

    #[test]
    fn to_text_from_number_fractional() {
        assert_eq!(to_text(&Value::Number(1.5)).as_ref(), "1.5");
    }

    #[test]
    fn to_text_from_number_zero() {
        assert_eq!(to_text(&Value::Number(0.0)).as_ref(), "0");
    }

    #[test]
    fn to_text_from_integer() {
        assert_eq!(to_text(&Value::Integer(99)).as_ref(), "99");
    }

    #[test]
    fn to_text_from_bool_true() {
        assert_eq!(to_text(&Value::Bool(true)).as_ref(), "TRUE");
    }

    #[test]
    fn to_text_from_bool_false() {
        assert_eq!(to_text(&Value::Bool(false)).as_ref(), "FALSE");
    }

    #[test]
    fn to_text_from_text() {
        let v = Value::Text("hello".into());
        assert_eq!(to_text(&v).as_ref(), "hello");
    }

    #[test]
    fn to_text_from_empty() {
        assert_eq!(to_text(&Value::Empty).as_ref(), "");
    }

    #[test]
    fn to_text_from_date() {
        assert_eq!(to_text(&Value::Date(ExcelDate { serial: 44927.0 })).as_ref(), "44927");
    }

    #[test]
    fn to_text_from_error() {
        let result = to_text(&Value::Error(CellError::Div0));
        assert_eq!(result.as_ref(), "#DIV/0!");
    }

    // -- to_bool --

    #[test]
    fn to_bool_from_bool_true() {
        assert_eq!(to_bool(&Value::Bool(true)), Ok(true));
    }

    #[test]
    fn to_bool_from_bool_false() {
        assert_eq!(to_bool(&Value::Bool(false)), Ok(false));
    }

    #[test]
    fn to_bool_from_number_nonzero() {
        assert_eq!(to_bool(&Value::Number(42.0)), Ok(true));
    }

    #[test]
    fn to_bool_from_number_zero() {
        assert_eq!(to_bool(&Value::Number(0.0)), Ok(false));
    }

    #[test]
    fn to_bool_from_integer_nonzero() {
        assert_eq!(to_bool(&Value::Integer(1)), Ok(true));
    }

    #[test]
    fn to_bool_from_integer_zero() {
        assert_eq!(to_bool(&Value::Integer(0)), Ok(false));
    }

    #[test]
    fn to_bool_from_text_true_case_insensitive() {
        assert_eq!(to_bool(&Value::Text("true".into())), Ok(true));
        assert_eq!(to_bool(&Value::Text("TRUE".into())), Ok(true));
        assert_eq!(to_bool(&Value::Text("True".into())), Ok(true));
    }

    #[test]
    fn to_bool_from_text_false_case_insensitive() {
        assert_eq!(to_bool(&Value::Text("false".into())), Ok(false));
        assert_eq!(to_bool(&Value::Text("FALSE".into())), Ok(false));
    }

    #[test]
    fn to_bool_from_text_other() {
        assert_eq!(to_bool(&Value::Text("abc".into())), Err(CellError::Value));
    }

    #[test]
    fn to_bool_from_empty() {
        assert_eq!(to_bool(&Value::Empty), Ok(false));
    }

    #[test]
    fn to_bool_from_date() {
        assert_eq!(to_bool(&Value::Date(ExcelDate { serial: 44927.0 })), Ok(true));
    }

    #[test]
    fn to_bool_from_error_propagates() {
        assert_eq!(to_bool(&Value::Error(CellError::Na)), Err(CellError::Na));
    }
}
