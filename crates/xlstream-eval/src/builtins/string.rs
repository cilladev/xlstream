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

// ---------------------------------------------------------------------------
// UPPER / LOWER / PROPER / TRIM / CLEAN
// ---------------------------------------------------------------------------

/// `UPPER(text)` — convert to uppercase.
pub(crate) fn builtin_upper(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    Value::Text(s.to_uppercase().into())
}

/// `LOWER(text)` — convert to lowercase.
pub(crate) fn builtin_lower(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    Value::Text(s.to_lowercase().into())
}

/// `PROPER(text)` — title case. Capitalizes after any non-letter character.
pub(crate) fn builtin_proper(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut result = String::with_capacity(s.len());
    let mut cap_next = true;
    for ch in s.chars() {
        if ch.is_alphabetic() {
            if cap_next {
                result.extend(ch.to_uppercase());
            } else {
                result.extend(ch.to_lowercase());
            }
            cap_next = false;
        } else {
            result.push(ch);
            cap_next = true;
        }
    }
    Value::Text(result.into())
}

/// `TRIM(text)` — strip leading/trailing spaces, collapse internal runs
/// of ASCII space (0x20) to a single space.
pub(crate) fn builtin_trim(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut result = String::with_capacity(s.len());
    let mut prev_space = true; // treat start as "after space" to strip leading
    for ch in s.chars() {
        if ch == ' ' {
            if !prev_space {
                prev_space = true;
            }
        } else {
            if prev_space && !result.is_empty() {
                result.push(' ');
            }
            result.push(ch);
            prev_space = false;
        }
    }
    Value::Text(result.into())
}

/// `CLEAN(text)` — remove characters with ASCII code 0-31.
pub(crate) fn builtin_clean(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let s = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let result: String = s.chars().filter(|c| !c.is_ascii_control()).collect();
    Value::Text(result.into())
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

    // ===== UPPER =====

    #[test]
    fn upper_basic() {
        assert_eq!(builtin_upper(&[Value::Text("hello".into())]), Value::Text("HELLO".into()));
    }

    #[test]
    fn upper_mixed_case() {
        assert_eq!(builtin_upper(&[Value::Text("HeLLo".into())]), Value::Text("HELLO".into()));
    }

    #[test]
    fn upper_empty_string() {
        assert_eq!(builtin_upper(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn upper_error_propagation() {
        assert_eq!(builtin_upper(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn upper_number_coerced() {
        assert_eq!(builtin_upper(&[Value::Number(42.0)]), Value::Text("42".into()));
    }

    #[test]
    fn upper_wrong_arg_count() {
        assert_eq!(builtin_upper(&[]), Value::Error(CellError::Value));
    }

    // ===== LOWER =====

    #[test]
    fn lower_basic() {
        assert_eq!(builtin_lower(&[Value::Text("HELLO".into())]), Value::Text("hello".into()));
    }

    #[test]
    fn lower_mixed_case() {
        assert_eq!(builtin_lower(&[Value::Text("HeLLo".into())]), Value::Text("hello".into()));
    }

    #[test]
    fn lower_empty_string() {
        assert_eq!(builtin_lower(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn lower_error_propagation() {
        assert_eq!(builtin_lower(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn lower_number_coerced() {
        assert_eq!(builtin_lower(&[Value::Number(42.0)]), Value::Text("42".into()));
    }

    #[test]
    fn lower_wrong_arg_count() {
        assert_eq!(
            builtin_lower(&[Value::Text("a".into()), Value::Text("b".into())]),
            Value::Error(CellError::Value)
        );
    }

    // ===== PROPER =====

    #[test]
    fn proper_basic() {
        assert_eq!(
            builtin_proper(&[Value::Text("hello world".into())]),
            Value::Text("Hello World".into())
        );
    }

    #[test]
    fn proper_after_digit() {
        assert_eq!(builtin_proper(&[Value::Text("123abc".into())]), Value::Text("123Abc".into()));
    }

    #[test]
    fn proper_all_caps() {
        assert_eq!(
            builtin_proper(&[Value::Text("HELLO WORLD".into())]),
            Value::Text("Hello World".into())
        );
    }

    #[test]
    fn proper_empty_string() {
        assert_eq!(builtin_proper(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn proper_error_propagation() {
        assert_eq!(builtin_proper(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn proper_number_coerced() {
        assert_eq!(builtin_proper(&[Value::Number(42.0)]), Value::Text("42".into()));
    }

    // ===== TRIM =====

    #[test]
    fn trim_leading_trailing() {
        assert_eq!(builtin_trim(&[Value::Text("  hello  ".into())]), Value::Text("hello".into()));
    }

    #[test]
    fn trim_internal_spaces_collapsed() {
        assert_eq!(
            builtin_trim(&[Value::Text("hello   world".into())]),
            Value::Text("hello world".into())
        );
    }

    #[test]
    fn trim_all_spaces() {
        assert_eq!(builtin_trim(&[Value::Text("   ".into())]), Value::Text("".into()));
    }

    #[test]
    fn trim_empty_string() {
        assert_eq!(builtin_trim(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn trim_error_propagation() {
        assert_eq!(builtin_trim(&[Value::Error(CellError::Ref)]), Value::Error(CellError::Ref));
    }

    #[test]
    fn trim_tabs_not_affected() {
        // TRIM only targets ASCII space (0x20), not tabs
        assert_eq!(
            builtin_trim(&[Value::Text("\thello\t".into())]),
            Value::Text("\thello\t".into())
        );
    }

    // ===== CLEAN =====

    #[test]
    fn clean_removes_control_chars() {
        assert_eq!(
            builtin_clean(&[Value::Text("hello\x00world\x07".into())]),
            Value::Text("helloworld".into())
        );
    }

    #[test]
    fn clean_preserves_printable() {
        assert_eq!(
            builtin_clean(&[Value::Text("hello world".into())]),
            Value::Text("hello world".into())
        );
    }

    #[test]
    fn clean_removes_newlines_tabs() {
        assert_eq!(
            builtin_clean(&[Value::Text("line1\nline2\ttab".into())]),
            Value::Text("line1line2tab".into())
        );
    }

    #[test]
    fn clean_empty_string() {
        assert_eq!(builtin_clean(&[Value::Text("".into())]), Value::Text("".into()));
    }

    #[test]
    fn clean_error_propagation() {
        assert_eq!(builtin_clean(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn clean_wrong_arg_count() {
        assert_eq!(builtin_clean(&[]), Value::Error(CellError::Value));
    }
}
