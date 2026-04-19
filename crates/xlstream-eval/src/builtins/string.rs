//! String builtin functions (Phase 9, Chunk 0).
//!
//! Pure functions: each takes `&[Value]` and returns `Value`.
//! Error propagation is explicit — `Value::Error` inputs are checked
//! before calling `coerce::to_text` (which does NOT propagate errors).

use std::borrow::Cow;
use std::fmt::Write as _;

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

// ---------------------------------------------------------------------------
// CONCAT / CONCATENATE / TEXTJOIN
// ---------------------------------------------------------------------------

/// `CONCAT(a, b, ...)` — join arguments as text. Zero args returns `#VALUE!`.
pub(crate) fn builtin_concat(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }
    let mut result = String::new();
    for v in args {
        if let Value::Error(e) = v {
            return Value::Error(*e);
        }
        result.push_str(&coerce::to_text(v));
    }
    Value::Text(result.into())
}

/// `TEXTJOIN(delim, ignore_empty, val1, val2, ...)` — join with delimiter.
///
/// When `ignore_empty` is true, skips both `Value::Empty` and empty
/// strings `""`.
pub(crate) fn builtin_textjoin(args: &[Value]) -> Value {
    if args.len() < 3 {
        return Value::Error(CellError::Value);
    }
    let delim = match text_arg(args, 0) {
        Ok(s) => s.into_owned(),
        Err(e) => return e,
    };
    let ignore_empty = match coerce::to_bool(&args[1]) {
        Ok(b) => b,
        Err(e) => return Value::Error(e),
    };

    let mut parts: Vec<String> = Vec::new();
    for v in &args[2..] {
        if let Value::Error(e) = v {
            return Value::Error(*e);
        }
        let is_empty_val = matches!(v, Value::Empty) || matches!(v, Value::Text(s) if s.is_empty());
        if ignore_empty && is_empty_val {
            continue;
        }
        parts.push(coerce::to_text(v).into_owned());
    }
    Value::Text(parts.join(&delim).into())
}

// ---------------------------------------------------------------------------
// FIND / SEARCH
// ---------------------------------------------------------------------------

/// `FIND(needle, haystack, start?)` — 1-based position, case-sensitive.
///
/// Returns `#VALUE!` if not found. Empty needle returns start position.
pub(crate) fn builtin_find(args: &[Value]) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let needle = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let haystack = match text_arg(args, 1) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let start = match opt_positive_int(args, 2, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if start == 0 {
        return Value::Error(CellError::Value);
    }

    // Empty needle: Excel returns start position (if in range)
    if needle.is_empty() {
        let len = haystack.chars().count();
        if start <= len + 1 {
            #[allow(clippy::cast_precision_loss)]
            return Value::Number(start as f64);
        }
        return Value::Error(CellError::Value);
    }

    // Convert to char-based search from start position
    let hay_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let search_start = start.saturating_sub(1);

    for i in search_start..hay_chars.len() {
        if i + needle_chars.len() > hay_chars.len() {
            break;
        }
        if hay_chars[i..i + needle_chars.len()] == needle_chars[..] {
            #[allow(clippy::cast_precision_loss)]
            return Value::Number((i + 1) as f64);
        }
    }
    Value::Error(CellError::Value)
}

/// `SEARCH(needle, haystack, start?)` — like FIND but case-insensitive,
/// supports `?` (single char) and `*` (any chars) wildcards.
pub(crate) fn builtin_search(args: &[Value]) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let needle = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let haystack = match text_arg(args, 1) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let start = match opt_positive_int(args, 2, 1) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if start == 0 {
        return Value::Error(CellError::Value);
    }

    // Empty needle: return start position
    if needle.is_empty() {
        let len = haystack.chars().count();
        if start <= len + 1 {
            #[allow(clippy::cast_precision_loss)]
            return Value::Number(start as f64);
        }
        return Value::Error(CellError::Value);
    }

    let hay_lower: Vec<char> = haystack.to_ascii_lowercase().chars().collect();
    let needle_lower: Vec<char> = needle.to_ascii_lowercase().chars().collect();
    let search_start = start.saturating_sub(1);

    for i in search_start..hay_lower.len() {
        if wildcard_match(&needle_lower, &hay_lower[i..]) {
            #[allow(clippy::cast_precision_loss)]
            return Value::Number((i + 1) as f64);
        }
    }
    Value::Error(CellError::Value)
}

/// Recursive wildcard matcher. `pattern` may contain `?` (single char)
/// and `*` (zero or more chars). Matches a prefix of `text` — succeeds
/// when the entire pattern is consumed.
fn wildcard_match(pattern: &[char], text: &[char]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    match pattern[0] {
        '?' => {
            if text.is_empty() {
                return false;
            }
            wildcard_match(&pattern[1..], &text[1..])
        }
        '*' => {
            // Try matching zero chars, then one, two, ...
            for skip in 0..=text.len() {
                if wildcard_match(&pattern[1..], &text[skip..]) {
                    return true;
                }
            }
            false
        }
        ch => {
            if text.is_empty() || text[0] != ch {
                return false;
            }
            wildcard_match(&pattern[1..], &text[1..])
        }
    }
}

// ---------------------------------------------------------------------------
// SUBSTITUTE / REPLACE / TEXT / VALUE / EXACT
// ---------------------------------------------------------------------------

/// `SUBSTITUTE(text, old, new, which?)` — replace occurrences.
///
/// If `which` is given, replace only the Nth occurrence. Case-sensitive.
pub(crate) fn builtin_substitute(args: &[Value]) -> Value {
    if args.len() < 3 || args.len() > 4 {
        return Value::Error(CellError::Value);
    }
    let text = match text_arg(args, 0) {
        Ok(s) => s.into_owned(),
        Err(e) => return e,
    };
    let old = match text_arg(args, 1) {
        Ok(s) => s.into_owned(),
        Err(e) => return e,
    };
    let new_str = match text_arg(args, 2) {
        Ok(s) => s.into_owned(),
        Err(e) => return e,
    };

    if old.is_empty() {
        return Value::Text(text.into());
    }

    if args.len() == 4 {
        // Replace only the Nth occurrence
        let which = match req_positive_int(args, 3) {
            Ok(n) => n,
            Err(e) => return e,
        };
        if which == 0 {
            return Value::Error(CellError::Value);
        }
        let mut result = String::with_capacity(text.len());
        let mut count = 0usize;
        let mut remaining = text.as_str();
        while let Some(pos) = remaining.find(&old) {
            count += 1;
            if count == which {
                result.push_str(&remaining[..pos]);
                result.push_str(&new_str);
                result.push_str(&remaining[pos + old.len()..]);
                return Value::Text(result.into());
            }
            result.push_str(&remaining[..pos + old.len()]);
            remaining = &remaining[pos + old.len()..];
        }
        // Nth occurrence not found — return original
        result.push_str(remaining);
        Value::Text(result.into())
    } else {
        // Replace all occurrences
        Value::Text(text.replace(&old, &new_str).into())
    }
}

/// `REPLACE(text, start, n, new)` — replace by 1-based position.
pub(crate) fn builtin_replace(args: &[Value]) -> Value {
    if args.len() != 4 {
        return Value::Error(CellError::Value);
    }
    let text = match text_arg(args, 0) {
        Ok(s) => s.into_owned(),
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
    let new_str = match text_arg(args, 3) {
        Ok(s) => s.into_owned(),
        Err(e) => return e,
    };

    let chars: Vec<char> = text.chars().collect();
    let start_idx = start.saturating_sub(1);
    let mut result = String::with_capacity(chars.len());
    for ch in &chars[..start_idx.min(chars.len())] {
        result.push(*ch);
    }
    result.push_str(&new_str);
    let end_idx = (start_idx + n).min(chars.len());
    for ch in &chars[end_idx..] {
        result.push(*ch);
    }
    Value::Text(result.into())
}

/// Format a number with thousands separators and the given decimal places.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn format_with_thousands(n: f64, decimals: usize) -> String {
    let abs = n.abs();
    let negative = n < 0.0;

    // Round to requested decimal places. `decimals` is at most 3 in practice.
    #[allow(clippy::cast_possible_wrap)]
    let factor = 10_f64.powi(decimals as i32);
    let rounded = (abs * factor + 0.5) as u64;
    let factor_u = factor as u64;
    let int_part = rounded / factor_u;
    let frac_part = rounded % factor_u;

    // Format integer part with commas
    let int_str = int_part.to_string();
    let mut with_commas = String::new();
    for (i, ch) in int_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            with_commas.push(',');
        }
        with_commas.push(ch);
    }
    let with_commas: String = with_commas.chars().rev().collect();

    let mut result = String::new();
    if negative {
        result.push('-');
    }
    result.push_str(&with_commas);
    if decimals > 0 {
        result.push('.');
        let _ = write!(result, "{frac_part:0>decimals$}");
    }
    result
}

/// `TEXT(value, format)` — format number as text.
///
/// v0.1 supported formats: `"0"`, `"0.0"`, `"0.00"`, `"0.000"`,
/// `"#,##0"`, `"#,##0.00"`, `"0%"`, `"0.00%"`.
/// Unknown format emits `tracing::warn!` and returns `#VALUE!`.
pub(crate) fn builtin_text(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    if let Value::Error(e) = &args[0] {
        return Value::Error(*e);
    }
    let n = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let fmt = match text_arg(args, 1) {
        Ok(s) => s.into_owned(),
        Err(e) => return e,
    };

    match fmt.as_str() {
        "0" => Value::Text(format!("{n:.0}").into()),
        "0.0" => Value::Text(format!("{n:.1}").into()),
        "0.00" => Value::Text(format!("{n:.2}").into()),
        "0.000" => Value::Text(format!("{n:.3}").into()),
        "#,##0" => Value::Text(format_with_thousands(n, 0).into()),
        "#,##0.00" => Value::Text(format_with_thousands(n, 2).into()),
        "0%" => {
            let pct = n * 100.0;
            Value::Text(format!("{pct:.0}%").into())
        }
        "0.00%" => {
            let pct = n * 100.0;
            Value::Text(format!("{pct:.2}%").into())
        }
        _ => {
            tracing::warn!(format = %fmt, "TEXT: unsupported format string");
            Value::Error(CellError::Value)
        }
    }
}

/// `VALUE(text)` — text to number (delegates to `coerce::to_number`).
pub(crate) fn builtin_value(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    if let Value::Error(e) = &args[0] {
        return Value::Error(*e);
    }
    match coerce::to_number(&args[0]) {
        Ok(n) => Value::Number(n),
        Err(e) => Value::Error(e),
    }
}

/// `EXACT(a, b)` — case-sensitive text comparison.
pub(crate) fn builtin_exact(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let a = match text_arg(args, 0) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let b = match text_arg(args, 1) {
        Ok(s) => s,
        Err(e) => return e,
    };
    Value::Bool(a == b)
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

    // ===== CONCAT =====

    #[test]
    fn concat_two_strings() {
        assert_eq!(
            builtin_concat(&[Value::Text("Hello".into()), Value::Text(" World".into())]),
            Value::Text("Hello World".into())
        );
    }

    #[test]
    fn concat_mixed_types() {
        assert_eq!(
            builtin_concat(&[Value::Text("n=".into()), Value::Number(42.0)]),
            Value::Text("n=42".into())
        );
    }

    #[test]
    fn concat_single_arg() {
        assert_eq!(builtin_concat(&[Value::Text("solo".into())]), Value::Text("solo".into()));
    }

    #[test]
    fn concat_zero_args_returns_value_error() {
        assert_eq!(builtin_concat(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn concat_error_propagation() {
        assert_eq!(
            builtin_concat(&[Value::Text("ok".into()), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn concat_empty_value_becomes_empty_string() {
        assert_eq!(
            builtin_concat(&[Value::Text("a".into()), Value::Empty, Value::Text("b".into())]),
            Value::Text("ab".into())
        );
    }

    #[test]
    fn concat_bool_coerced() {
        assert_eq!(
            builtin_concat(&[Value::Bool(true), Value::Bool(false)]),
            Value::Text("TRUEFALSE".into())
        );
    }

    // ===== TEXTJOIN =====

    #[test]
    fn textjoin_basic() {
        assert_eq!(
            builtin_textjoin(&[
                Value::Text(", ".into()),
                Value::Bool(false),
                Value::Text("a".into()),
                Value::Text("b".into()),
                Value::Text("c".into()),
            ]),
            Value::Text("a, b, c".into())
        );
    }

    #[test]
    fn textjoin_ignore_empty_true() {
        assert_eq!(
            builtin_textjoin(&[
                Value::Text(",".into()),
                Value::Bool(true),
                Value::Text("a".into()),
                Value::Empty,
                Value::Text("".into()),
                Value::Text("b".into()),
            ]),
            Value::Text("a,b".into())
        );
    }

    #[test]
    fn textjoin_ignore_empty_false_preserves_blanks() {
        assert_eq!(
            builtin_textjoin(&[
                Value::Text(",".into()),
                Value::Bool(false),
                Value::Text("a".into()),
                Value::Empty,
                Value::Text("b".into()),
            ]),
            Value::Text("a,,b".into())
        );
    }

    #[test]
    fn textjoin_error_propagation_in_delim() {
        assert_eq!(
            builtin_textjoin(&[
                Value::Error(CellError::Div0),
                Value::Bool(false),
                Value::Text("a".into()),
            ]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn textjoin_error_propagation_in_values() {
        assert_eq!(
            builtin_textjoin(&[
                Value::Text(",".into()),
                Value::Bool(false),
                Value::Text("a".into()),
                Value::Error(CellError::Na),
            ]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn textjoin_too_few_args() {
        assert_eq!(
            builtin_textjoin(&[Value::Text(",".into()), Value::Bool(false)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn textjoin_empty_delimiter() {
        assert_eq!(
            builtin_textjoin(&[
                Value::Text("".into()),
                Value::Bool(false),
                Value::Text("a".into()),
                Value::Text("b".into()),
            ]),
            Value::Text("ab".into())
        );
    }

    // ===== FIND =====

    #[test]
    fn find_basic() {
        assert_eq!(
            builtin_find(&[Value::Text("lo".into()), Value::Text("Hello".into())]),
            Value::Number(4.0)
        );
    }

    #[test]
    fn find_case_sensitive() {
        assert_eq!(
            builtin_find(&[Value::Text("LO".into()), Value::Text("Hello".into())]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn find_not_found() {
        assert_eq!(
            builtin_find(&[Value::Text("xyz".into()), Value::Text("Hello".into())]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn find_with_start_position() {
        assert_eq!(
            builtin_find(&[
                Value::Text("l".into()),
                Value::Text("Hello".into()),
                Value::Number(4.0)
            ]),
            Value::Number(4.0)
        );
    }

    #[test]
    fn find_empty_needle_returns_start() {
        assert_eq!(
            builtin_find(&[Value::Text("".into()), Value::Text("Hello".into())]),
            Value::Number(1.0)
        );
    }

    #[test]
    fn find_error_propagation() {
        assert_eq!(
            builtin_find(&[Value::Error(CellError::Na), Value::Text("Hello".into())]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn find_at_start() {
        assert_eq!(
            builtin_find(&[Value::Text("He".into()), Value::Text("Hello".into())]),
            Value::Number(1.0)
        );
    }

    // ===== SEARCH =====

    #[test]
    fn search_case_insensitive() {
        assert_eq!(
            builtin_search(&[Value::Text("LO".into()), Value::Text("Hello".into())]),
            Value::Number(4.0)
        );
    }

    #[test]
    fn search_wildcard_question_mark() {
        assert_eq!(
            builtin_search(&[Value::Text("h?llo".into()), Value::Text("Hello".into())]),
            Value::Number(1.0)
        );
    }

    #[test]
    fn search_wildcard_star() {
        assert_eq!(
            builtin_search(&[Value::Text("h*o".into()), Value::Text("Hello".into())]),
            Value::Number(1.0)
        );
    }

    #[test]
    fn search_not_found() {
        assert_eq!(
            builtin_search(&[Value::Text("xyz".into()), Value::Text("Hello".into())]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn search_error_propagation() {
        assert_eq!(
            builtin_search(&[Value::Error(CellError::Div0), Value::Text("Hello".into())]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn search_empty_needle_returns_start() {
        assert_eq!(
            builtin_search(&[Value::Text("".into()), Value::Text("Hello".into())]),
            Value::Number(1.0)
        );
    }

    #[test]
    fn search_with_start_position() {
        assert_eq!(
            builtin_search(&[
                Value::Text("l".into()),
                Value::Text("Hello".into()),
                Value::Number(4.0)
            ]),
            Value::Number(4.0)
        );
    }

    #[test]
    fn search_star_matches_zero_chars() {
        assert_eq!(
            builtin_search(&[Value::Text("hel*lo".into()), Value::Text("Hello".into())]),
            Value::Number(1.0)
        );
    }

    // ===== SUBSTITUTE =====

    #[test]
    fn substitute_replace_all() {
        assert_eq!(
            builtin_substitute(&[
                Value::Text("aabbcc".into()),
                Value::Text("b".into()),
                Value::Text("X".into()),
            ]),
            Value::Text("aaXXcc".into())
        );
    }

    #[test]
    fn substitute_replace_nth() {
        assert_eq!(
            builtin_substitute(&[
                Value::Text("aabab".into()),
                Value::Text("a".into()),
                Value::Text("X".into()),
                Value::Number(2.0),
            ]),
            Value::Text("aXbab".into())
        );
    }

    #[test]
    fn substitute_not_found() {
        assert_eq!(
            builtin_substitute(&[
                Value::Text("hello".into()),
                Value::Text("z".into()),
                Value::Text("X".into()),
            ]),
            Value::Text("hello".into())
        );
    }

    #[test]
    fn substitute_empty_old_returns_original() {
        assert_eq!(
            builtin_substitute(&[
                Value::Text("hello".into()),
                Value::Text("".into()),
                Value::Text("X".into()),
            ]),
            Value::Text("hello".into())
        );
    }

    #[test]
    fn substitute_error_propagation() {
        assert_eq!(
            builtin_substitute(&[
                Value::Error(CellError::Na),
                Value::Text("a".into()),
                Value::Text("b".into()),
            ]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn substitute_case_sensitive() {
        assert_eq!(
            builtin_substitute(&[
                Value::Text("Hello".into()),
                Value::Text("h".into()),
                Value::Text("X".into()),
            ]),
            Value::Text("Hello".into())
        );
    }

    #[test]
    fn substitute_wrong_arg_count() {
        assert_eq!(
            builtin_substitute(&[Value::Text("a".into()), Value::Text("b".into())]),
            Value::Error(CellError::Value)
        );
    }

    // ===== REPLACE =====

    #[test]
    fn replace_basic() {
        assert_eq!(
            builtin_replace(&[
                Value::Text("Hello".into()),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Text("XY".into()),
            ]),
            Value::Text("HXYo".into())
        );
    }

    #[test]
    fn replace_at_start() {
        assert_eq!(
            builtin_replace(&[
                Value::Text("Hello".into()),
                Value::Number(1.0),
                Value::Number(1.0),
                Value::Text("J".into()),
            ]),
            Value::Text("Jello".into())
        );
    }

    #[test]
    fn replace_insert_no_delete() {
        assert_eq!(
            builtin_replace(&[
                Value::Text("Hello".into()),
                Value::Number(3.0),
                Value::Number(0.0),
                Value::Text("XY".into()),
            ]),
            Value::Text("HeXYllo".into())
        );
    }

    #[test]
    fn replace_error_propagation() {
        assert_eq!(
            builtin_replace(&[
                Value::Error(CellError::Ref),
                Value::Number(1.0),
                Value::Number(1.0),
                Value::Text("X".into()),
            ]),
            Value::Error(CellError::Ref)
        );
    }

    #[test]
    fn replace_start_zero_returns_value_error() {
        assert_eq!(
            builtin_replace(&[
                Value::Text("Hello".into()),
                Value::Number(0.0),
                Value::Number(1.0),
                Value::Text("X".into()),
            ]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn replace_wrong_arg_count() {
        assert_eq!(
            builtin_replace(
                &[Value::Text("Hello".into()), Value::Number(1.0), Value::Number(1.0),]
            ),
            Value::Error(CellError::Value)
        );
    }

    // ===== TEXT =====

    #[test]
    fn text_format_integer() {
        assert_eq!(
            builtin_text(&[Value::Number(1234.0), Value::Text("0".into())]),
            Value::Text("1234".into())
        );
    }

    #[test]
    fn text_format_one_decimal() {
        assert_eq!(
            builtin_text(&[Value::Number(1234.5), Value::Text("0.0".into())]),
            Value::Text("1234.5".into())
        );
    }

    #[test]
    fn text_format_two_decimals() {
        assert_eq!(
            builtin_text(&[Value::Number(1234.5), Value::Text("0.00".into())]),
            Value::Text("1234.50".into())
        );
    }

    #[test]
    fn text_format_thousands() {
        assert_eq!(
            builtin_text(&[Value::Number(1_234_567.0), Value::Text("#,##0".into())]),
            Value::Text("1,234,567".into())
        );
    }

    #[test]
    fn text_format_thousands_with_decimals() {
        assert_eq!(
            builtin_text(&[Value::Number(1234.5), Value::Text("#,##0.00".into())]),
            Value::Text("1,234.50".into())
        );
    }

    #[test]
    fn text_format_percent() {
        assert_eq!(
            builtin_text(&[Value::Number(0.75), Value::Text("0%".into())]),
            Value::Text("75%".into())
        );
    }

    #[test]
    fn text_format_percent_with_decimals() {
        assert_eq!(
            builtin_text(&[Value::Number(0.7512), Value::Text("0.00%".into())]),
            Value::Text("75.12%".into())
        );
    }

    #[test]
    fn text_unknown_format_returns_value_error() {
        assert_eq!(
            builtin_text(&[Value::Number(1.0), Value::Text("yyyy-mm-dd".into())]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn text_error_propagation() {
        assert_eq!(
            builtin_text(&[Value::Error(CellError::Na), Value::Text("0".into())]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn text_wrong_arg_count() {
        assert_eq!(builtin_text(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    // ===== VALUE =====

    #[test]
    fn value_from_numeric_text() {
        assert_eq!(builtin_value(&[Value::Text("42.5".into())]), Value::Number(42.5));
    }

    #[test]
    fn value_from_non_numeric_text() {
        assert_eq!(builtin_value(&[Value::Text("abc".into())]), Value::Error(CellError::Value));
    }

    #[test]
    fn value_from_number_passthrough() {
        assert_eq!(builtin_value(&[Value::Number(10.0)]), Value::Number(10.0));
    }

    #[test]
    fn value_from_bool() {
        assert_eq!(builtin_value(&[Value::Bool(true)]), Value::Number(1.0));
    }

    #[test]
    fn value_error_propagation() {
        assert_eq!(builtin_value(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn value_wrong_arg_count() {
        assert_eq!(builtin_value(&[]), Value::Error(CellError::Value));
    }

    // ===== EXACT =====

    #[test]
    fn exact_same_strings() {
        assert_eq!(
            builtin_exact(&[Value::Text("Hello".into()), Value::Text("Hello".into())]),
            Value::Bool(true)
        );
    }

    #[test]
    fn exact_different_case() {
        assert_eq!(
            builtin_exact(&[Value::Text("Hello".into()), Value::Text("hello".into())]),
            Value::Bool(false)
        );
    }

    #[test]
    fn exact_different_strings() {
        assert_eq!(
            builtin_exact(&[Value::Text("Hello".into()), Value::Text("World".into())]),
            Value::Bool(false)
        );
    }

    #[test]
    fn exact_empty_strings() {
        assert_eq!(
            builtin_exact(&[Value::Text("".into()), Value::Text("".into())]),
            Value::Bool(true)
        );
    }

    #[test]
    fn exact_error_propagation() {
        assert_eq!(
            builtin_exact(&[Value::Error(CellError::Na), Value::Text("a".into())]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn exact_number_coerced() {
        assert_eq!(
            builtin_exact(&[Value::Number(42.0), Value::Text("42".into())]),
            Value::Bool(true)
        );
    }

    #[test]
    fn exact_wrong_arg_count() {
        assert_eq!(builtin_exact(&[Value::Text("a".into())]), Value::Error(CellError::Value));
    }
}
