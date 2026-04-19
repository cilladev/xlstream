//! String builtin functions (Phase 9, Chunk 0).
//!
//! Pure functions: each takes `&[Value]` and returns `Value`.
//! Error propagation is explicit — `Value::Error` inputs are checked
//! before calling `coerce::to_text` (which does NOT propagate errors).

use std::borrow::Cow;

use xlstream_core::{coerce, CellError, Value};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Extract a text argument at `idx`, propagating errors.
///
/// Returns `Err(Value::Error(_))` if the value at `idx` is an error.
/// Returns the text representation for all other value types.
pub(crate) fn text_arg(args: &[Value], idx: usize) -> Result<Cow<'_, str>, Value> {
    let v = args.get(idx).unwrap_or(&Value::Empty);
    if let Value::Error(e) = v {
        return Err(Value::Error(*e));
    }
    Ok(coerce::to_text(v))
}

/// Extract an optional positive integer argument at `idx`, with a default.
///
/// Returns `Err(Value::Error(_))` on error propagation, negative values,
/// or non-numeric text.
fn opt_positive_int(args: &[Value], idx: usize, default: usize) -> Result<usize, Value> {
    let Some(v) = args.get(idx) else {
        return Ok(default);
    };
    let n = coerce::to_number(v).map_err(Value::Error)?;
    if n < 0.0 {
        return Err(Value::Error(CellError::Value));
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(n as usize)
}

/// Extract a required positive integer argument at `idx`.
fn req_positive_int(args: &[Value], idx: usize) -> Result<usize, Value> {
    let v = args.get(idx).unwrap_or(&Value::Empty);
    let n = coerce::to_number(v).map_err(Value::Error)?;
    if n < 0.0 {
        return Err(Value::Error(CellError::Value));
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(n as usize)
}

// ---------------------------------------------------------------------------
// LEFT / RIGHT / MID / LEN
// ---------------------------------------------------------------------------

/// `LEFT(text, n?)` — leftmost `n` characters (default 1).
pub(crate) fn builtin_left(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let n = match opt_positive_int(args, 1, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let result: String = s.chars().take(n).collect();
    Value::Text(result.into())
}

/// `RIGHT(text, n?)` — rightmost `n` characters (default 1).
pub(crate) fn builtin_right(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let n = match opt_positive_int(args, 1, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let chars: Vec<char> = s.chars().collect();
    let start = chars.len().saturating_sub(n);
    let result: String = chars[start..].iter().collect();
    Value::Text(result.into())
}

/// `MID(text, start, n)` — substring starting at 1-based `start`, `n` chars.
pub(crate) fn builtin_mid(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let start = match req_positive_int(args, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if start == 0 {
        return Value::Error(CellError::Value);
    }
    let n = match req_positive_int(args, 2) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let result: String = s.chars().skip(start - 1).take(n).collect();
    Value::Text(result.into())
}

/// `LEN(text)` — character count.
pub(crate) fn builtin_len(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    #[allow(clippy::cast_precision_loss)]
    Value::Number(s.chars().count() as f64)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};

    use super::*;

    // ===== LEFT =====

    #[test]
    fn left_default_one_char() {
        assert_eq!(builtin_left(&[Value::Text("Hello".into())]), Value::Text("H".into()));
    }

    #[test]
    fn left_explicit_count() {
        assert_eq!(
            builtin_left(&[Value::Text("Hello".into()), Value::Number(3.0)]),
            Value::Text("Hel".into())
        );
    }

    #[test]
    fn left_count_exceeds_length() {
        assert_eq!(
            builtin_left(&[Value::Text("Hi".into()), Value::Number(10.0)]),
            Value::Text("Hi".into())
        );
    }

    #[test]
    fn left_zero_count_returns_empty() {
        assert_eq!(
            builtin_left(&[Value::Text("Hello".into()), Value::Number(0.0)]),
            Value::Text("".into())
        );
    }

    #[test]
    fn left_empty_string() {
        assert_eq!(builtin_left(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn left_error_propagation() {
        assert_eq!(builtin_left(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn left_negative_count_returns_value_error() {
        assert_eq!(
            builtin_left(&[Value::Text("Hello".into()), Value::Number(-1.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn left_number_coerced_to_text() {
        assert_eq!(
            builtin_left(&[Value::Number(12345.0), Value::Number(2.0)]),
            Value::Text("12".into())
        );
    }

    #[test]
    fn left_bool_coerced_to_text() {
        assert_eq!(
            builtin_left(&[Value::Bool(true), Value::Number(2.0)]),
            Value::Text("TR".into())
        );
    }

    #[test]
    fn left_no_args_returns_value_error() {
        assert_eq!(builtin_left(&[]), Value::Error(CellError::Value));
    }

    // ===== RIGHT =====

    #[test]
    fn right_default_one_char() {
        assert_eq!(builtin_right(&[Value::Text("Hello".into())]), Value::Text("o".into()));
    }

    #[test]
    fn right_explicit_count() {
        assert_eq!(
            builtin_right(&[Value::Text("Hello".into()), Value::Number(3.0)]),
            Value::Text("llo".into())
        );
    }

    #[test]
    fn right_count_exceeds_length() {
        assert_eq!(
            builtin_right(&[Value::Text("Hi".into()), Value::Number(10.0)]),
            Value::Text("Hi".into())
        );
    }

    #[test]
    fn right_zero_count_returns_empty() {
        assert_eq!(
            builtin_right(&[Value::Text("Hello".into()), Value::Number(0.0)]),
            Value::Text("".into())
        );
    }

    #[test]
    fn right_empty_string() {
        assert_eq!(builtin_right(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn right_error_propagation() {
        assert_eq!(builtin_right(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn right_number_coerced_to_text() {
        assert_eq!(
            builtin_right(&[Value::Number(12345.0), Value::Number(2.0)]),
            Value::Text("45".into())
        );
    }

    // ===== MID =====

    #[test]
    fn mid_basic_substring() {
        assert_eq!(
            builtin_mid(&[
                Value::Text("Hello World".into()),
                Value::Number(7.0),
                Value::Number(5.0)
            ]),
            Value::Text("World".into())
        );
    }

    #[test]
    fn mid_start_beyond_length() {
        assert_eq!(
            builtin_mid(&[Value::Text("Hi".into()), Value::Number(10.0), Value::Number(5.0)]),
            Value::Text("".into())
        );
    }

    #[test]
    fn mid_count_exceeds_remaining() {
        assert_eq!(
            builtin_mid(&[Value::Text("Hello".into()), Value::Number(4.0), Value::Number(100.0)]),
            Value::Text("lo".into())
        );
    }

    #[test]
    fn mid_start_zero_returns_value_error() {
        assert_eq!(
            builtin_mid(&[Value::Text("Hello".into()), Value::Number(0.0), Value::Number(1.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn mid_error_propagation() {
        assert_eq!(
            builtin_mid(&[Value::Error(CellError::Ref), Value::Number(1.0), Value::Number(1.0)]),
            Value::Error(CellError::Ref)
        );
    }

    #[test]
    fn mid_wrong_arg_count() {
        assert_eq!(
            builtin_mid(&[Value::Text("Hello".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn mid_number_coerced_to_text() {
        assert_eq!(
            builtin_mid(&[Value::Number(12345.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Text("234".into())
        );
    }

    // ===== LEN =====

    #[test]
    fn len_basic_string() {
        assert_eq!(builtin_len(&[Value::Text("Hello".into())]), Value::Number(5.0));
    }

    #[test]
    fn len_empty_string() {
        assert_eq!(builtin_len(&[Value::Text("".into())]), Value::Number(0.0));
    }

    #[test]
    fn len_empty_value() {
        assert_eq!(builtin_len(&[Value::Empty]), Value::Number(0.0));
    }

    #[test]
    fn len_number_coerced() {
        assert_eq!(builtin_len(&[Value::Number(12345.0)]), Value::Number(5.0));
    }

    #[test]
    fn len_bool_coerced() {
        assert_eq!(builtin_len(&[Value::Bool(true)]), Value::Number(4.0));
    }

    #[test]
    fn len_error_propagation() {
        assert_eq!(builtin_len(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn len_wrong_arg_count() {
        assert_eq!(builtin_len(&[]), Value::Error(CellError::Value));
    }
}
