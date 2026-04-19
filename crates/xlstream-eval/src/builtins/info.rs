//! Info/type-checking builtin functions (Phase 9, Chunk 3).
//!
//! Pure functions: each takes `&[Value]` and returns `Value`.
//! All type-checking functions are simple pattern matches on the
//! [`Value`] enum — no coercion needed.

use xlstream_core::{CellError, Value};

// ---------------------------------------------------------------------------
// ISBLANK
// ---------------------------------------------------------------------------

/// `ISBLANK(x)` — true iff `x` is `Value::Empty`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_isblank;
/// assert_eq!(builtin_isblank(&[Value::Empty]), Value::Bool(true));
/// assert_eq!(builtin_isblank(&[Value::Number(0.0)]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_isblank(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(matches!(args[0], Value::Empty))
}

// ---------------------------------------------------------------------------
// ISNUMBER
// ---------------------------------------------------------------------------

/// `ISNUMBER(x)` — true for `Number`, `Integer`, and `Date`.
///
/// Excel treats dates as numeric, so `ISNUMBER(date)` returns `TRUE`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_isnumber;
/// assert_eq!(builtin_isnumber(&[Value::Number(42.0)]), Value::Bool(true));
/// assert_eq!(builtin_isnumber(&[Value::Text("42".into())]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_isnumber(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(matches!(args[0], Value::Number(_) | Value::Integer(_) | Value::Date(_)))
}

// ---------------------------------------------------------------------------
// ISTEXT
// ---------------------------------------------------------------------------

/// `ISTEXT(x)` — true for `Text` only.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_istext;
/// assert_eq!(builtin_istext(&[Value::Text("hi".into())]), Value::Bool(true));
/// assert_eq!(builtin_istext(&[Value::Number(1.0)]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_istext(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(matches!(args[0], Value::Text(_)))
}

// ---------------------------------------------------------------------------
// ISERROR
// ---------------------------------------------------------------------------

/// `ISERROR(x)` — true for any `Error` variant.
///
/// # Examples
///
/// ```
/// use xlstream_core::{CellError, Value};
/// use xlstream_eval::builtins::info::builtin_iserror;
/// assert_eq!(builtin_iserror(&[Value::Error(CellError::Div0)]), Value::Bool(true));
/// assert_eq!(builtin_iserror(&[Value::Number(1.0)]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_iserror(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(matches!(args[0], Value::Error(_)))
}

// ---------------------------------------------------------------------------
// ISNA
// ---------------------------------------------------------------------------

/// `ISNA(x)` — true only for `Error(CellError::Na)`.
///
/// # Examples
///
/// ```
/// use xlstream_core::{CellError, Value};
/// use xlstream_eval::builtins::info::builtin_isna;
/// assert_eq!(builtin_isna(&[Value::Error(CellError::Na)]), Value::Bool(true));
/// assert_eq!(builtin_isna(&[Value::Error(CellError::Div0)]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_isna(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(matches!(args[0], Value::Error(CellError::Na)))
}

// ---------------------------------------------------------------------------
// ISLOGICAL
// ---------------------------------------------------------------------------

/// `ISLOGICAL(x)` — true for `Bool` only.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_islogical;
/// assert_eq!(builtin_islogical(&[Value::Bool(true)]), Value::Bool(true));
/// assert_eq!(builtin_islogical(&[Value::Number(1.0)]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_islogical(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(matches!(args[0], Value::Bool(_)))
}

// ---------------------------------------------------------------------------
// ISNONTEXT
// ---------------------------------------------------------------------------

/// `ISNONTEXT(x)` — true for anything that is NOT `Text`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_isnontext;
/// assert_eq!(builtin_isnontext(&[Value::Number(1.0)]), Value::Bool(true));
/// assert_eq!(builtin_isnontext(&[Value::Text("hi".into())]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_isnontext(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(!matches!(args[0], Value::Text(_)))
}

// ---------------------------------------------------------------------------
// ISREF
// ---------------------------------------------------------------------------

/// `ISREF(x)` — always `false`.
///
/// By the time builtins run, all references have already been resolved
/// to concrete values. A proper implementation would require the parser
/// to preserve ref-vs-value distinction, which is out of scope for v0.1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_isref;
/// assert_eq!(builtin_isref(&[Value::Number(1.0)]), Value::Bool(false));
/// ```
#[must_use]
pub fn builtin_isref(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    Value::Bool(false)
}

// ---------------------------------------------------------------------------
// NA
// ---------------------------------------------------------------------------

/// `NA()` — returns `#N/A`. With any arguments, returns `#VALUE!`.
///
/// # Examples
///
/// ```
/// use xlstream_core::{CellError, Value};
/// use xlstream_eval::builtins::info::builtin_na;
/// assert_eq!(builtin_na(&[]), Value::Error(CellError::Na));
/// ```
#[must_use]
pub fn builtin_na(args: &[Value]) -> Value {
    if args.is_empty() {
        Value::Error(CellError::Na)
    } else {
        Value::Error(CellError::Value)
    }
}

// ---------------------------------------------------------------------------
// TYPE
// ---------------------------------------------------------------------------

/// `TYPE(x)` — Excel type code.
///
/// | Value variant        | Code |
/// |----------------------|------|
/// | Number/Integer/Empty/Date | 1.0  |
/// | Text                 | 2.0  |
/// | Bool                 | 4.0  |
/// | Error                | 16.0 |
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::info::builtin_type;
/// assert_eq!(builtin_type(&[Value::Number(1.0)]), Value::Number(1.0));
/// assert_eq!(builtin_type(&[Value::Text("hi".into())]), Value::Number(2.0));
/// assert_eq!(builtin_type(&[Value::Bool(true)]), Value::Number(4.0));
/// ```
#[must_use]
pub fn builtin_type(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let code = match &args[0] {
        Value::Number(_) | Value::Integer(_) | Value::Empty | Value::Date(_) => 1.0,
        Value::Text(_) => 2.0,
        Value::Bool(_) => 4.0,
        Value::Error(_) => 16.0,
    };
    Value::Number(code)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, ExcelDate, Value};

    use super::*;

    // ===== ISBLANK =====

    #[test]
    fn isblank_empty_is_true() {
        assert_eq!(builtin_isblank(&[Value::Empty]), Value::Bool(true));
    }

    #[test]
    fn isblank_zero_is_false() {
        assert_eq!(builtin_isblank(&[Value::Number(0.0)]), Value::Bool(false));
    }

    #[test]
    fn isblank_empty_string_is_false() {
        assert_eq!(builtin_isblank(&[Value::Text("".into())]), Value::Bool(false));
    }

    #[test]
    fn isblank_number_is_false() {
        assert_eq!(builtin_isblank(&[Value::Number(42.0)]), Value::Bool(false));
    }

    #[test]
    fn isblank_error_is_false() {
        assert_eq!(builtin_isblank(&[Value::Error(CellError::Na)]), Value::Bool(false));
    }

    #[test]
    fn isblank_bool_is_false() {
        assert_eq!(builtin_isblank(&[Value::Bool(false)]), Value::Bool(false));
    }

    #[test]
    fn isblank_wrong_arg_count() {
        assert_eq!(builtin_isblank(&[]), Value::Error(CellError::Value));
    }

    // ===== ISNUMBER =====

    #[test]
    fn isnumber_number_is_true() {
        assert_eq!(builtin_isnumber(&[Value::Number(42.0)]), Value::Bool(true));
    }

    #[test]
    fn isnumber_integer_is_true() {
        assert_eq!(builtin_isnumber(&[Value::Integer(7)]), Value::Bool(true));
    }

    #[test]
    fn isnumber_date_is_true() {
        assert_eq!(
            builtin_isnumber(&[Value::Date(ExcelDate { serial: 44927.0 })]),
            Value::Bool(true)
        );
    }

    #[test]
    fn isnumber_text_is_false() {
        assert_eq!(builtin_isnumber(&[Value::Text("42".into())]), Value::Bool(false));
    }

    #[test]
    fn isnumber_bool_is_false() {
        assert_eq!(builtin_isnumber(&[Value::Bool(true)]), Value::Bool(false));
    }

    #[test]
    fn isnumber_empty_is_false() {
        assert_eq!(builtin_isnumber(&[Value::Empty]), Value::Bool(false));
    }

    #[test]
    fn isnumber_error_is_false() {
        assert_eq!(builtin_isnumber(&[Value::Error(CellError::Div0)]), Value::Bool(false));
    }

    #[test]
    fn isnumber_wrong_arg_count() {
        assert_eq!(builtin_isnumber(&[]), Value::Error(CellError::Value));
    }

    // ===== ISTEXT =====

    #[test]
    fn istext_text_is_true() {
        assert_eq!(builtin_istext(&[Value::Text("hello".into())]), Value::Bool(true));
    }

    #[test]
    fn istext_empty_text_is_true() {
        assert_eq!(builtin_istext(&[Value::Text("".into())]), Value::Bool(true));
    }

    #[test]
    fn istext_number_is_false() {
        assert_eq!(builtin_istext(&[Value::Number(1.0)]), Value::Bool(false));
    }

    #[test]
    fn istext_bool_is_false() {
        assert_eq!(builtin_istext(&[Value::Bool(true)]), Value::Bool(false));
    }

    #[test]
    fn istext_empty_is_false() {
        assert_eq!(builtin_istext(&[Value::Empty]), Value::Bool(false));
    }

    #[test]
    fn istext_error_is_false() {
        assert_eq!(builtin_istext(&[Value::Error(CellError::Na)]), Value::Bool(false));
    }

    #[test]
    fn istext_wrong_arg_count() {
        assert_eq!(
            builtin_istext(&[Value::Text("a".into()), Value::Text("b".into())]),
            Value::Error(CellError::Value)
        );
    }

    // ===== ISERROR =====

    #[test]
    fn iserror_div0_is_true() {
        assert_eq!(builtin_iserror(&[Value::Error(CellError::Div0)]), Value::Bool(true));
    }

    #[test]
    fn iserror_na_is_true() {
        assert_eq!(builtin_iserror(&[Value::Error(CellError::Na)]), Value::Bool(true));
    }

    #[test]
    fn iserror_value_error_is_true() {
        assert_eq!(builtin_iserror(&[Value::Error(CellError::Value)]), Value::Bool(true));
    }

    #[test]
    fn iserror_ref_is_true() {
        assert_eq!(builtin_iserror(&[Value::Error(CellError::Ref)]), Value::Bool(true));
    }

    #[test]
    fn iserror_num_is_true() {
        assert_eq!(builtin_iserror(&[Value::Error(CellError::Num)]), Value::Bool(true));
    }

    #[test]
    fn iserror_number_is_false() {
        assert_eq!(builtin_iserror(&[Value::Number(1.0)]), Value::Bool(false));
    }

    #[test]
    fn iserror_text_is_false() {
        assert_eq!(builtin_iserror(&[Value::Text("err".into())]), Value::Bool(false));
    }

    #[test]
    fn iserror_wrong_arg_count() {
        assert_eq!(builtin_iserror(&[]), Value::Error(CellError::Value));
    }

    // ===== ISNA =====

    #[test]
    fn isna_na_is_true() {
        assert_eq!(builtin_isna(&[Value::Error(CellError::Na)]), Value::Bool(true));
    }

    #[test]
    fn isna_div0_is_false() {
        assert_eq!(builtin_isna(&[Value::Error(CellError::Div0)]), Value::Bool(false));
    }

    #[test]
    fn isna_value_error_is_false() {
        assert_eq!(builtin_isna(&[Value::Error(CellError::Value)]), Value::Bool(false));
    }

    #[test]
    fn isna_number_is_false() {
        assert_eq!(builtin_isna(&[Value::Number(1.0)]), Value::Bool(false));
    }

    #[test]
    fn isna_empty_is_false() {
        assert_eq!(builtin_isna(&[Value::Empty]), Value::Bool(false));
    }

    #[test]
    fn isna_wrong_arg_count() {
        assert_eq!(builtin_isna(&[]), Value::Error(CellError::Value));
    }

    // ===== ISLOGICAL =====

    #[test]
    fn islogical_true_is_true() {
        assert_eq!(builtin_islogical(&[Value::Bool(true)]), Value::Bool(true));
    }

    #[test]
    fn islogical_false_is_true() {
        assert_eq!(builtin_islogical(&[Value::Bool(false)]), Value::Bool(true));
    }

    #[test]
    fn islogical_number_is_false() {
        assert_eq!(builtin_islogical(&[Value::Number(1.0)]), Value::Bool(false));
    }

    #[test]
    fn islogical_text_is_false() {
        assert_eq!(builtin_islogical(&[Value::Text("TRUE".into())]), Value::Bool(false));
    }

    #[test]
    fn islogical_empty_is_false() {
        assert_eq!(builtin_islogical(&[Value::Empty]), Value::Bool(false));
    }

    #[test]
    fn islogical_wrong_arg_count() {
        assert_eq!(builtin_islogical(&[]), Value::Error(CellError::Value));
    }

    // ===== ISNONTEXT =====

    #[test]
    fn isnontext_number_is_true() {
        assert_eq!(builtin_isnontext(&[Value::Number(1.0)]), Value::Bool(true));
    }

    #[test]
    fn isnontext_empty_is_true() {
        assert_eq!(builtin_isnontext(&[Value::Empty]), Value::Bool(true));
    }

    #[test]
    fn isnontext_bool_is_true() {
        assert_eq!(builtin_isnontext(&[Value::Bool(true)]), Value::Bool(true));
    }

    #[test]
    fn isnontext_error_is_true() {
        assert_eq!(builtin_isnontext(&[Value::Error(CellError::Na)]), Value::Bool(true));
    }

    #[test]
    fn isnontext_text_is_false() {
        assert_eq!(builtin_isnontext(&[Value::Text("hello".into())]), Value::Bool(false));
    }

    #[test]
    fn isnontext_wrong_arg_count() {
        assert_eq!(builtin_isnontext(&[]), Value::Error(CellError::Value));
    }

    // ===== ISREF =====

    #[test]
    fn isref_number_is_false() {
        assert_eq!(builtin_isref(&[Value::Number(1.0)]), Value::Bool(false));
    }

    #[test]
    fn isref_text_is_false() {
        assert_eq!(builtin_isref(&[Value::Text("A1".into())]), Value::Bool(false));
    }

    #[test]
    fn isref_empty_is_false() {
        assert_eq!(builtin_isref(&[Value::Empty]), Value::Bool(false));
    }

    #[test]
    fn isref_error_is_false() {
        assert_eq!(builtin_isref(&[Value::Error(CellError::Ref)]), Value::Bool(false));
    }

    #[test]
    fn isref_bool_is_false() {
        assert_eq!(builtin_isref(&[Value::Bool(true)]), Value::Bool(false));
    }

    #[test]
    fn isref_wrong_arg_count() {
        assert_eq!(builtin_isref(&[]), Value::Error(CellError::Value));
    }

    // ===== NA =====

    #[test]
    fn na_no_args_returns_na() {
        assert_eq!(builtin_na(&[]), Value::Error(CellError::Na));
    }

    #[test]
    fn na_with_arg_returns_value_error() {
        assert_eq!(builtin_na(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn na_with_two_args_returns_value_error() {
        assert_eq!(
            builtin_na(&[Value::Number(1.0), Value::Number(2.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn na_with_empty_arg_returns_value_error() {
        assert_eq!(builtin_na(&[Value::Empty]), Value::Error(CellError::Value));
    }

    #[test]
    fn na_with_error_arg_returns_value_error() {
        assert_eq!(builtin_na(&[Value::Error(CellError::Na)]), Value::Error(CellError::Value));
    }

    // ===== TYPE =====

    #[test]
    fn type_number_returns_1() {
        assert_eq!(builtin_type(&[Value::Number(42.0)]), Value::Number(1.0));
    }

    #[test]
    fn type_integer_returns_1() {
        assert_eq!(builtin_type(&[Value::Integer(7)]), Value::Number(1.0));
    }

    #[test]
    fn type_empty_returns_1() {
        assert_eq!(builtin_type(&[Value::Empty]), Value::Number(1.0));
    }

    #[test]
    fn type_date_returns_1() {
        assert_eq!(builtin_type(&[Value::Date(ExcelDate { serial: 44927.0 })]), Value::Number(1.0));
    }

    #[test]
    fn type_text_returns_2() {
        assert_eq!(builtin_type(&[Value::Text("hello".into())]), Value::Number(2.0));
    }

    #[test]
    fn type_bool_returns_4() {
        assert_eq!(builtin_type(&[Value::Bool(true)]), Value::Number(4.0));
    }

    #[test]
    fn type_error_returns_16() {
        assert_eq!(builtin_type(&[Value::Error(CellError::Na)]), Value::Number(16.0));
    }

    #[test]
    fn type_wrong_arg_count() {
        assert_eq!(builtin_type(&[]), Value::Error(CellError::Value));
    }
}
