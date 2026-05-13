//! Engineering builtin functions.

use xlstream_core::{coerce, CellError, Value};

use crate::builtins::math::num_arg_ce;

const MAX_BASE_STR_LEN: usize = 10;

const BIN_BIT_WIDTH: u32 = 10;
const BIN_TWO_POW: i64 = 1 << BIN_BIT_WIDTH;
const BIN_MAX_POSITIVE: i64 = BIN_TWO_POW / 2 - 1;
const BIN_MIN_NEGATIVE: i64 = -(BIN_TWO_POW / 2);

const OCT_BIT_WIDTH: u32 = 30;
const OCT_TWO_POW: i64 = 1 << OCT_BIT_WIDTH;
const OCT_MAX_POSITIVE: i64 = OCT_TWO_POW / 2 - 1;
const OCT_MIN_NEGATIVE: i64 = -(OCT_TWO_POW / 2);

const HEX_BIT_WIDTH: u32 = 40;
const HEX_TWO_POW: i64 = 1 << HEX_BIT_WIDTH;
const HEX_MAX_POSITIVE: i64 = HEX_TWO_POW / 2 - 1;
const HEX_MIN_NEGATIVE: i64 = -(HEX_TWO_POW / 2);

const BASE_MAX_VALUE: f64 = 9_007_199_254_740_991.0;
const BASE_MIN_RADIX: i64 = 2;
const BASE_MAX_RADIX: i64 = 36;
const BASE_MAX_MIN_LENGTH: i64 = 255;
const BASE_DIGITS: &[u8; 36] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Parse a base-N string to decimal, applying two's complement for 10-digit
/// values whose unsigned interpretation exceeds the half-range.
///
/// Coerces numeric inputs to text (Excel: `BIN2DEC(101)` == `BIN2DEC("101")`).
fn parse_base_str(
    v: &Value,
    radix: u32,
    is_valid: fn(u8) -> bool,
    bit_width: u32,
) -> Result<i64, CellError> {
    if let Value::Error(e) = v {
        return Err(*e);
    }
    let s = coerce::to_text(v);
    let s = s.as_ref();
    if s.is_empty() || s.len() > MAX_BASE_STR_LEN {
        return Err(CellError::Num);
    }
    if !s.bytes().all(is_valid) {
        return Err(CellError::Num);
    }
    let unsigned = i64::from_str_radix(s, radix).map_err(|_| CellError::Num)?;
    let two_pow: i64 = 1 << bit_width;
    let max_positive = two_pow / 2 - 1;
    if s.len() == MAX_BASE_STR_LEN && unsigned > max_positive {
        Ok(unsigned - two_pow)
    } else {
        Ok(unsigned)
    }
}

/// Extract optional `places` argument at index 1. Valid range: 1..=10.
fn extract_places(args: &[Value]) -> Result<Option<usize>, CellError> {
    if args.len() < 2 {
        return Ok(None);
    }
    let pf = coerce::to_number(&args[1])?;
    if !pf.is_finite() || !(1.0..=10.0).contains(&pf) {
        return Err(CellError::Num);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let p = pf.trunc() as usize;
    if p > MAX_BASE_STR_LEN {
        return Err(CellError::Num);
    }
    Ok(Some(p))
}

/// Format a decimal integer as a base-N string with optional zero-padding.
///
/// Negative values use two's complement (always produces the correct digit
/// count for the given bit width). Hex output is uppercase.
fn format_base_str(
    int_val: i64,
    radix: u32,
    bit_width: u32,
    min_neg: i64,
    max_pos: i64,
    places: Option<usize>,
) -> Result<Value, CellError> {
    if !(min_neg..=max_pos).contains(&int_val) {
        return Err(CellError::Num);
    }
    let s = if int_val < 0 {
        #[allow(clippy::cast_sign_loss)]
        let unsigned = (int_val + (1i64 << bit_width)) as u64;
        match radix {
            2 => format!("{unsigned:b}"),
            8 => format!("{unsigned:o}"),
            _ => format!("{unsigned:X}"),
        }
    } else {
        #[allow(clippy::cast_sign_loss)]
        let unsigned = int_val as u64;
        match radix {
            2 => format!("{unsigned:b}"),
            8 => format!("{unsigned:o}"),
            _ => format!("{unsigned:X}"),
        }
    };
    if let Some(p) = places {
        if s.len() > p {
            return Err(CellError::Num);
        }
        Ok(Value::Text(format!("{s:0>p$}").into()))
    } else {
        Ok(Value::Text(s.into()))
    }
}

fn is_binary_digit(b: u8) -> bool {
    b == b'0' || b == b'1'
}

fn is_octal_digit(b: u8) -> bool {
    (b'0'..=b'7').contains(&b)
}

const MAX_BITWISE: u64 = (1_u64 << 48) - 1;
const MAX_SHIFT: i32 = 53;
// ---------------------------------------------------------------------------
// Complex number helpers (pub(crate) for reuse by future IM* functions)
// ---------------------------------------------------------------------------

/// Format a complex number as an Excel-canonical text string.
///
/// Omits zero parts, omits coefficient 1/-1 on the imaginary suffix.
/// Used by `COMPLEX` and future IM* output functions.
pub(crate) fn format_complex(real: f64, imag: f64, suffix: char) -> String {
    if imag == 0.0 && real == 0.0 {
        return "0".to_string();
    }
    if imag == 0.0 {
        return format_number(real);
    }
    let imag_part = format_imag_part(imag, suffix);
    if real == 0.0 {
        return imag_part;
    }
    let real_str = format_number(real);
    if imag > 0.0 {
        format!("{real_str}+{imag_part}")
    } else {
        format!("{real_str}{imag_part}")
    }
}

/// Format a number with no trailing ".0" for integers.
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.is_finite() && n.abs() < 1e15 {
        #[allow(clippy::cast_possible_truncation)]
        let i = n as i64;
        format!("{i}")
    } else {
        format!("{n}")
    }
}

/// Format the imaginary part with suffix, handling coefficient 1/-1.
#[allow(clippy::float_cmp)] // Exact 1.0/-1.0 check is intentional (Excel convention)
fn format_imag_part(imag: f64, suffix: char) -> String {
    if imag == 1.0 {
        return suffix.to_string();
    }
    if imag == -1.0 {
        return format!("-{suffix}");
    }
    let coeff = format_number(imag);
    format!("{coeff}{suffix}")
}

/// Parse an Excel complex number text string.
///
/// Returns `(real, imag, suffix)`. Suffix is `'i'` or `'j'`.
/// For pure-real inputs, suffix defaults to `'i'`.
///
/// Handles: `"a+bi"`, `"a-bi"`, `"a"`, `"bi"`, `"i"`, `"-i"`,
/// and the same with `'j'` suffix. Scientific notation is supported
/// in number parts.
///
/// # Errors
///
/// Returns `CellError::Num` for invalid complex number format.
pub(crate) fn parse_complex(s: &str) -> Result<(f64, f64, char), CellError> {
    if s.is_empty() {
        return Err(CellError::Num);
    }

    let last = s.as_bytes()[s.len() - 1];
    let suffix = if last == b'i' || last == b'j' {
        last as char
    } else {
        // Pure real number — no suffix
        let real: f64 = s.parse().map_err(|_| CellError::Num)?;
        return Ok((real, 0.0, 'i'));
    };

    // Strip suffix
    let body = &s[..s.len() - 1];

    // Bare suffix: "i" or "j"
    if body.is_empty() {
        return Ok((0.0, 1.0, suffix));
    }

    // "-i" or "+i"
    if body == "-" {
        return Ok((0.0, -1.0, suffix));
    }
    if body == "+" {
        return Ok((0.0, 1.0, suffix));
    }

    // Try pure imaginary: "4i", "-4i", "3.5i", "1E2i"
    if let Ok(imag) = body.parse::<f64>() {
        return Ok((0.0, imag, suffix));
    }

    // Full form: find the split point between real and imaginary parts.
    // The imaginary part starts at the last '+' or '-' that is NOT
    // inside a scientific notation exponent (i.e., not preceded by 'e'/'E').
    let split = find_imag_split(body)?;
    let real_str = &body[..split];
    let imag_str = &body[split..];

    let real: f64 = real_str.parse().map_err(|_| CellError::Num)?;
    let imag: f64 = match imag_str {
        "+" => 1.0,
        "-" => -1.0,
        _ => imag_str.parse().map_err(|_| CellError::Num)?,
    };

    Ok((real, imag, suffix))
}

/// Find the byte index where the imaginary part begins (at its sign).
///
/// Scans right-to-left for '+' or '-' that isn't part of a scientific
/// notation exponent (preceded by 'e'/'E').
fn find_imag_split(body: &str) -> Result<usize, CellError> {
    let bytes = body.as_bytes();
    let mut i = bytes.len();
    while i > 0 {
        i -= 1;
        // i > 0: leading sign belongs to real part, also guards bytes[i-1] access
        if (bytes[i] == b'+' || bytes[i] == b'-') && i > 0 {
            let prev = bytes[i - 1];
            if prev == b'e' || prev == b'E' {
                // Part of scientific notation — skip
                continue;
            }
            return Ok(i);
        }
    }
    Err(CellError::Num)
}

// ---------------------------------------------------------------------------
// COMPLEX / IMREAL / IMAGINARY builtins
// ---------------------------------------------------------------------------

/// `COMPLEX(real_num, i_num, [suffix])` — create complex number text.
///
/// Returns a text string like `"3+4i"` or `"3+4j"`.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity (not 2-3 args), invalid suffix,
/// or non-numeric arguments. Propagates errors from arguments.
pub(crate) fn builtin_complex(args: &[Value]) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }

    let real = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let imag = match coerce::to_number(&args[1]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };

    if !real.is_finite() || !imag.is_finite() {
        return Value::Error(CellError::Num);
    }

    let suffix = if args.len() == 3 {
        let v = &args[2];
        if let Value::Error(e) = v {
            return Value::Error(*e);
        }
        let s = coerce::to_text(v);
        match s.as_ref() {
            "i" => 'i',
            "j" => 'j',
            _ => return Value::Error(CellError::Value),
        }
    } else {
        'i'
    };

    Value::Text(format_complex(real, imag, suffix).into())
}

/// Extract the complex-number text argument for IMREAL/IMAGINARY.
///
/// Numbers are coerced to text (e.g. `5` -> `"5"` -> `5+0i`).
/// Booleans return `#VALUE!` (Excel rejects `TRUE`/`FALSE`).
/// Errors propagate.
fn complex_text_arg(args: &[Value]) -> Result<std::borrow::Cow<'_, str>, Value> {
    if args.len() != 1 {
        return Err(Value::Error(CellError::Value));
    }
    let v = &args[0];
    match v {
        Value::Error(e) => Err(Value::Error(*e)),
        Value::Bool(_) => Err(Value::Error(CellError::Value)),
        _ => Ok(coerce::to_text(v)),
    }
}

/// `IMREAL(inumber)` — extract real part from complex number text.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or boolean input. Returns `#NUM!`
/// for invalid complex number format. Propagates errors from the argument.
pub(crate) fn builtin_imreal(args: &[Value]) -> Value {
    let text = match complex_text_arg(args) {
        Ok(t) => t,
        Err(e) => return e,
    };
    match parse_complex(text.as_ref()) {
        Ok((real, _, _)) => Value::Number(real),
        Err(e) => Value::Error(e),
    }
}

/// `IMAGINARY(inumber)` — extract imaginary part from complex number text.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or boolean input. Returns `#NUM!`
/// for invalid complex number format. Propagates errors from the argument.
pub(crate) fn builtin_imaginary(args: &[Value]) -> Value {
    let text = match complex_text_arg(args) {
        Ok(t) => t,
        Err(e) => return e,
    };
    match parse_complex(text.as_ref()) {
        Ok((_, imag, _)) => Value::Number(imag),
        Err(e) => Value::Error(e),
    }
}

// ---------------------------------------------------------------------------
// HEX2DEC / DEC2HEX
// ---------------------------------------------------------------------------

/// `HEX2DEC(number)` — convert hex string to decimal number.
///
/// Accepts 1-10 hex digit strings. 10-digit values starting with
/// digit >= 8 are treated as 40-bit two's complement negatives.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity. Returns `#NUM!` for invalid
/// hex characters, empty input, or strings longer than 10 digits.
pub(crate) fn builtin_hex2dec(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    match parse_base_str(&args[0], 16, |b| b.is_ascii_hexdigit(), HEX_BIT_WIDTH) {
        Ok(v) =>
        {
            #[allow(clippy::cast_precision_loss)]
            Value::Number(v as f64)
        }
        Err(e) => Value::Error(e),
    }
}

/// `DEC2HEX(number, [places])` — convert decimal number to hex string.
///
/// Accepts values in `[-549755813888, 549755813887]`. Negative values
/// produce 10-digit two's complement output. Optional `places` (1-10)
/// zero-pads the result.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity. Returns `#NUM!` for out-of-range
/// input, invalid places, or result longer than places.
pub(crate) fn builtin_dec2hex(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let num = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let int_val = match super::math::to_int(num) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(int_val, 16, HEX_BIT_WIDTH, HEX_MIN_NEGATIVE, HEX_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

/// Validate a bitwise argument: must be finite, non-negative, integer,
/// and within [0, 2^48 - 1].
fn validate_bit_arg(v: f64) -> Result<u64, CellError> {
    if !v.is_finite() {
        return Err(CellError::Num);
    }
    if v < 0.0 {
        return Err(CellError::Num);
    }
    if v.fract() != 0.0 {
        return Err(CellError::Num);
    }
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let n = v as u64;
    if n > MAX_BITWISE {
        return Err(CellError::Num);
    }
    Ok(n)
}

/// Validate a shift amount: must be finite, integer, and in [-53, 53].
fn validate_shift(v: f64) -> Result<i32, CellError> {
    if !v.is_finite() {
        return Err(CellError::Num);
    }
    if v.fract() != 0.0 {
        return Err(CellError::Num);
    }
    if v < f64::from(-MAX_SHIFT) || v > f64::from(MAX_SHIFT) {
        return Err(CellError::Num);
    }
    #[allow(clippy::cast_possible_truncation)]
    let s = v as i32;
    Ok(s)
}

/// Apply a bitwise shift. Positive `shift` is left, negative is right.
/// Returns `#NUM!` if the result exceeds 2^48 - 1.
fn do_shift(number: u64, shift: i32) -> Result<u64, CellError> {
    let result = if shift >= 0 {
        #[allow(clippy::cast_sign_loss)]
        let s = shift as u32;
        number.checked_shl(s).unwrap_or(u64::MAX)
    } else {
        #[allow(clippy::cast_sign_loss)]
        let s = shift.unsigned_abs();
        number >> s
    };
    if result > MAX_BITWISE {
        return Err(CellError::Num);
    }
    Ok(result)
}

/// `BITAND(number1, number2)` — bitwise AND of two non-negative integers.
///
/// Both arguments must be non-negative integers in [0, 2^48 - 1].
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or non-numeric input.
/// Returns `#NUM!` for negative, non-integer, or out-of-range arguments.
pub(crate) fn builtin_bitand(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let a = match num_arg_ce(args, 0) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    let b = match num_arg_ce(args, 1) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    #[allow(clippy::cast_precision_loss)]
    Value::Number((a & b) as f64)
}

/// `BITOR(number1, number2)` — bitwise OR of two non-negative integers.
///
/// Both arguments must be non-negative integers in [0, 2^48 - 1].
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or non-numeric input.
/// Returns `#NUM!` for negative, non-integer, or out-of-range arguments.
pub(crate) fn builtin_bitor(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let a = match num_arg_ce(args, 0) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    let b = match num_arg_ce(args, 1) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    #[allow(clippy::cast_precision_loss)]
    Value::Number((a | b) as f64)
}

/// `BITXOR(number1, number2)` — bitwise XOR of two non-negative integers.
///
/// Both arguments must be non-negative integers in [0, 2^48 - 1].
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or non-numeric input.
/// Returns `#NUM!` for negative, non-integer, or out-of-range arguments.
pub(crate) fn builtin_bitxor(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let a = match num_arg_ce(args, 0) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    let b = match num_arg_ce(args, 1) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    #[allow(clippy::cast_precision_loss)]
    Value::Number((a ^ b) as f64)
}

/// `BITLSHIFT(number, shift_amount)` — bitwise left shift.
///
/// `number` must be a non-negative integer in [0, 2^48 - 1].
/// `shift_amount` must be an integer in [-53, 53]. Negative shift = right shift.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or non-numeric input.
/// Returns `#NUM!` for invalid arguments or if the result exceeds 2^48 - 1.
pub(crate) fn builtin_bitlshift(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let number = match num_arg_ce(args, 0) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    let shift = match num_arg_ce(args, 1) {
        Ok(n) => match validate_shift(n) {
            Ok(s) => s,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    match do_shift(number, shift) {
        Ok(result) =>
        {
            #[allow(clippy::cast_precision_loss)]
            Value::Number(result as f64)
        }
        Err(e) => Value::Error(e),
    }
}

/// `BITRSHIFT(number, shift_amount)` — bitwise right shift.
///
/// `number` must be a non-negative integer in [0, 2^48 - 1].
/// `shift_amount` must be an integer in [-53, 53]. Negative shift = left shift.
/// Equivalent to `BITLSHIFT(number, -shift_amount)`.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or non-numeric input.
/// Returns `#NUM!` for invalid arguments or if the result exceeds 2^48 - 1.
pub(crate) fn builtin_bitrshift(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let number = match num_arg_ce(args, 0) {
        Ok(n) => match validate_bit_arg(n) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    let shift = match num_arg_ce(args, 1) {
        Ok(n) => match validate_shift(n) {
            Ok(s) => s,
            Err(e) => return Value::Error(e),
        },
        Err(e) => return Value::Error(e),
    };
    match do_shift(number, -shift) {
        Ok(result) =>
        {
            #[allow(clippy::cast_precision_loss)]
            Value::Number(result as f64)
        }
        Err(e) => Value::Error(e),
    }
}

// ---------------------------------------------------------------------------
// BIN2DEC / DEC2BIN
// ---------------------------------------------------------------------------

/// `BIN2DEC(number)` — binary string to decimal number.
///
/// 10-bit two's complement. Range: -512 to 511.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for invalid binary input.
pub(crate) fn builtin_bin2dec(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    match parse_base_str(&args[0], 2, is_binary_digit, BIN_BIT_WIDTH) {
        Ok(v) =>
        {
            #[allow(clippy::cast_precision_loss)]
            Value::Number(v as f64)
        }
        Err(e) => Value::Error(e),
    }
}

/// `DEC2BIN(number, [places])` — decimal number to binary string.
///
/// 10-bit two's complement. Range: -512 to 511. Optional places (1-10).
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for out-of-range, invalid places.
pub(crate) fn builtin_dec2bin(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let num = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let int_val = match super::math::to_int(num) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(int_val, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

// ---------------------------------------------------------------------------
// OCT2DEC / DEC2OCT
// ---------------------------------------------------------------------------

/// `OCT2DEC(number)` — octal string to decimal number.
///
/// 30-bit two's complement. Range: -536870912 to 536870911.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for invalid octal input.
pub(crate) fn builtin_oct2dec(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    match parse_base_str(&args[0], 8, is_octal_digit, OCT_BIT_WIDTH) {
        Ok(v) =>
        {
            #[allow(clippy::cast_precision_loss)]
            Value::Number(v as f64)
        }
        Err(e) => Value::Error(e),
    }
}

/// `DEC2OCT(number, [places])` — decimal number to octal string.
///
/// 30-bit two's complement. Range: -536870912 to 536870911.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for out-of-range, invalid places.
pub(crate) fn builtin_dec2oct(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let num = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let int_val = match super::math::to_int(num) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(int_val, 8, OCT_BIT_WIDTH, OCT_MIN_NEGATIVE, OCT_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

// ---------------------------------------------------------------------------
// Cross-base: HEX2BIN, BIN2HEX, HEX2OCT, OCT2HEX, BIN2OCT, OCT2BIN
// ---------------------------------------------------------------------------

/// `HEX2BIN(number, [places])` — hex string to binary string.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` if hex value outside binary domain.
pub(crate) fn builtin_hex2bin(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let dec = match parse_base_str(&args[0], 16, |b| b.is_ascii_hexdigit(), HEX_BIT_WIDTH) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(dec, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

/// `BIN2HEX(number, [places])` — binary string to hex string.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for invalid binary input.
pub(crate) fn builtin_bin2hex(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let dec = match parse_base_str(&args[0], 2, is_binary_digit, BIN_BIT_WIDTH) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(dec, 16, HEX_BIT_WIDTH, HEX_MIN_NEGATIVE, HEX_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

/// `HEX2OCT(number, [places])` — hex string to octal string.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` if hex value outside octal domain.
pub(crate) fn builtin_hex2oct(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let dec = match parse_base_str(&args[0], 16, |b| b.is_ascii_hexdigit(), HEX_BIT_WIDTH) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(dec, 8, OCT_BIT_WIDTH, OCT_MIN_NEGATIVE, OCT_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

/// `OCT2HEX(number, [places])` — octal string to hex string.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for invalid octal input.
pub(crate) fn builtin_oct2hex(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let dec = match parse_base_str(&args[0], 8, is_octal_digit, OCT_BIT_WIDTH) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(dec, 16, HEX_BIT_WIDTH, HEX_MIN_NEGATIVE, HEX_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

/// `BIN2OCT(number, [places])` — binary string to octal string.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for invalid binary input.
pub(crate) fn builtin_bin2oct(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let dec = match parse_base_str(&args[0], 2, is_binary_digit, BIN_BIT_WIDTH) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(dec, 8, OCT_BIT_WIDTH, OCT_MIN_NEGATIVE, OCT_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

/// `OCT2BIN(number, [places])` — octal string to binary string.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` if octal value outside binary domain.
pub(crate) fn builtin_oct2bin(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let dec = match parse_base_str(&args[0], 8, is_octal_digit, OCT_BIT_WIDTH) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let places = match extract_places(args) {
        Ok(p) => p,
        Err(e) => return Value::Error(e),
    };
    match format_base_str(dec, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, places) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}

// ---------------------------------------------------------------------------
// BASE
// ---------------------------------------------------------------------------

/// `BASE(number, radix, [min_length])` — convert non-negative integer to
/// text in the given base (2-36). Uses digits `0-9A-Z`.
///
/// Unlike DEC2HEX/DEC2BIN/DEC2OCT, BASE does **not** use two's complement.
/// Negative input returns `#NUM!`.
///
/// # Errors
///
/// `#VALUE!` for wrong arity. `#NUM!` for negative number, radix outside
/// 2-36, `min_length` outside 0-255, or number >= 2^53.
pub(crate) fn builtin_base(args: &[Value]) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }

    let num = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    if !num.is_finite() || !(0.0..=BASE_MAX_VALUE).contains(&num) {
        return Value::Error(CellError::Num);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let int_val = num.trunc() as u64;

    let radix_f = match coerce::to_number(&args[1]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let radix = match super::math::to_int(radix_f) {
        Ok(r) => r,
        Err(e) => return Value::Error(e),
    };
    if !(BASE_MIN_RADIX..=BASE_MAX_RADIX).contains(&radix) {
        return Value::Error(CellError::Num);
    }

    let min_len = if args.len() == 3 {
        let ml_f = match coerce::to_number(&args[2]) {
            Ok(n) => n,
            Err(e) => return Value::Error(e),
        };
        let ml = match super::math::to_int(ml_f) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        };
        if !(0..=BASE_MAX_MIN_LENGTH).contains(&ml) {
            return Value::Error(CellError::Num);
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let ml_usize = ml as usize;
        ml_usize
    } else {
        0
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let radix_u = radix as u32;
    let s = if int_val == 0 {
        "0".to_string()
    } else {
        let mut digits = Vec::new();
        let mut n = int_val;
        while n > 0 {
            #[allow(clippy::cast_possible_truncation)]
            let d = (n % u64::from(radix_u)) as usize;
            digits.push(BASE_DIGITS[d]);
            n /= u64::from(radix_u);
        }
        digits.reverse();
        // SAFETY (logical): all bytes are from BASE_DIGITS which is pure ASCII
        String::from_utf8(digits).unwrap_or_default()
    };

    if s.len() < min_len {
        Value::Text(format!("{s:0>min_len$}").into())
    } else {
        Value::Text(s.into())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use xlstream_core::{CellError, Value};

    use super::*;

    // -- parse_base_str --

    #[test]
    fn parse_base_str_binary_positive() {
        assert_eq!(
            parse_base_str(&Value::Text("1100100".into()), 2, is_binary_digit, BIN_BIT_WIDTH),
            Ok(100),
        );
    }

    #[test]
    fn parse_base_str_binary_negative_twos_complement() {
        assert_eq!(
            parse_base_str(&Value::Text("1111111111".into()), 2, is_binary_digit, BIN_BIT_WIDTH),
            Ok(-1),
        );
    }

    #[test]
    fn parse_base_str_empty_returns_num() {
        assert_eq!(
            parse_base_str(&Value::Text("".into()), 2, is_binary_digit, BIN_BIT_WIDTH),
            Err(CellError::Num),
        );
    }

    #[test]
    fn parse_base_str_too_long_returns_num() {
        assert_eq!(
            parse_base_str(&Value::Text("10000000001".into()), 2, is_binary_digit, BIN_BIT_WIDTH),
            Err(CellError::Num),
        );
    }

    #[test]
    fn parse_base_str_invalid_char_returns_num() {
        assert_eq!(
            parse_base_str(&Value::Text("2".into()), 2, is_binary_digit, BIN_BIT_WIDTH),
            Err(CellError::Num),
        );
    }

    #[test]
    fn parse_base_str_propagates_error() {
        assert_eq!(
            parse_base_str(&Value::Error(CellError::Na), 2, is_binary_digit, BIN_BIT_WIDTH),
            Err(CellError::Na),
        );
    }

    #[test]
    fn parse_base_str_coerces_number_to_text() {
        assert_eq!(parse_base_str(&Value::Number(101.0), 2, is_binary_digit, BIN_BIT_WIDTH), Ok(5),);
    }

    #[test]
    fn parse_base_str_octal_positive() {
        assert_eq!(
            parse_base_str(&Value::Text("144".into()), 8, is_octal_digit, OCT_BIT_WIDTH),
            Ok(100),
        );
    }

    #[test]
    fn parse_base_str_octal_negative() {
        assert_eq!(
            parse_base_str(&Value::Text("7777777777".into()), 8, is_octal_digit, OCT_BIT_WIDTH),
            Ok(-1),
        );
    }

    // -- extract_places --

    #[test]
    fn extract_places_present() {
        let args = [Value::Number(100.0), Value::Number(4.0)];
        assert_eq!(extract_places(&args), Ok(Some(4)));
    }

    #[test]
    fn extract_places_absent() {
        let args = [Value::Number(100.0)];
        assert_eq!(extract_places(&args), Ok(None));
    }

    #[test]
    fn extract_places_truncates_fractional() {
        let args = [Value::Number(100.0), Value::Number(4.9)];
        assert_eq!(extract_places(&args), Ok(Some(4)));
    }

    #[test]
    fn extract_places_zero_returns_num() {
        let args = [Value::Number(100.0), Value::Number(0.0)];
        assert_eq!(extract_places(&args), Err(CellError::Num));
    }

    #[test]
    fn extract_places_eleven_returns_num() {
        let args = [Value::Number(100.0), Value::Number(11.0)];
        assert_eq!(extract_places(&args), Err(CellError::Num));
    }

    #[test]
    fn extract_places_error_propagates() {
        let args = [Value::Number(100.0), Value::Error(CellError::Na)];
        assert_eq!(extract_places(&args), Err(CellError::Na));
    }

    // -- format_base_str --

    #[test]
    fn format_base_str_binary_positive() {
        assert_eq!(
            format_base_str(100, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, None),
            Ok(Value::Text("1100100".into())),
        );
    }

    #[test]
    fn format_base_str_binary_zero() {
        assert_eq!(
            format_base_str(0, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, None),
            Ok(Value::Text("0".into())),
        );
    }

    #[test]
    fn format_base_str_binary_negative() {
        assert_eq!(
            format_base_str(-1, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, None),
            Ok(Value::Text("1111111111".into())),
        );
    }

    #[test]
    fn format_base_str_binary_min_negative() {
        assert_eq!(
            format_base_str(-512, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, None),
            Ok(Value::Text("1000000000".into())),
        );
    }

    #[test]
    fn format_base_str_binary_with_places() {
        assert_eq!(
            format_base_str(100, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, Some(10)),
            Ok(Value::Text("0001100100".into())),
        );
    }

    #[test]
    fn format_base_str_binary_places_too_small() {
        assert_eq!(
            format_base_str(100, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, Some(6)),
            Err(CellError::Num),
        );
    }

    #[test]
    fn format_base_str_binary_out_of_range_high() {
        assert_eq!(
            format_base_str(512, 2, BIN_BIT_WIDTH, BIN_MIN_NEGATIVE, BIN_MAX_POSITIVE, None),
            Err(CellError::Num),
        );
    }

    #[test]
    fn format_base_str_octal_positive() {
        assert_eq!(
            format_base_str(100, 8, OCT_BIT_WIDTH, OCT_MIN_NEGATIVE, OCT_MAX_POSITIVE, None),
            Ok(Value::Text("144".into())),
        );
    }

    #[test]
    fn format_base_str_octal_negative() {
        assert_eq!(
            format_base_str(-1, 8, OCT_BIT_WIDTH, OCT_MIN_NEGATIVE, OCT_MAX_POSITIVE, None),
            Ok(Value::Text("7777777777".into())),
        );
    }

    #[test]
    fn format_base_str_hex_positive() {
        assert_eq!(
            format_base_str(255, 16, HEX_BIT_WIDTH, HEX_MIN_NEGATIVE, HEX_MAX_POSITIVE, None),
            Ok(Value::Text("FF".into())),
        );
    }

    #[test]
    fn format_base_str_hex_negative() {
        assert_eq!(
            format_base_str(-1, 16, HEX_BIT_WIDTH, HEX_MIN_NEGATIVE, HEX_MAX_POSITIVE, None),
            Ok(Value::Text("FFFFFFFFFF".into())),
        );
    }

    // -- HEX2DEC --

    #[test]
    fn hex2dec_basic() {
        assert_eq!(builtin_hex2dec(&[Value::Text("FF".into())]), Value::Number(255.0));
        assert_eq!(builtin_hex2dec(&[Value::Text("A5".into())]), Value::Number(165.0));
        assert_eq!(builtin_hex2dec(&[Value::Text("0".into())]), Value::Number(0.0));
        assert_eq!(builtin_hex2dec(&[Value::Text("1".into())]), Value::Number(1.0));
    }

    #[test]
    fn hex2dec_max_positive() {
        assert_eq!(
            builtin_hex2dec(&[Value::Text("7FFFFFFFFF".into())]),
            Value::Number(549_755_813_887.0),
        );
    }

    #[test]
    fn hex2dec_negative_twos_complement() {
        assert_eq!(builtin_hex2dec(&[Value::Text("FFFFFFFFFF".into())]), Value::Number(-1.0));
        assert_eq!(
            builtin_hex2dec(&[Value::Text("8000000000".into())]),
            Value::Number(-549_755_813_888.0),
        );
        assert_eq!(builtin_hex2dec(&[Value::Text("FFFFFFFF00".into())]), Value::Number(-256.0));
    }

    #[test]
    fn hex2dec_case_insensitive() {
        assert_eq!(builtin_hex2dec(&[Value::Text("ff".into())]), Value::Number(255.0));
    }

    #[test]
    fn hex2dec_leading_zeros() {
        assert_eq!(builtin_hex2dec(&[Value::Text("0000000001".into())]), Value::Number(1.0));
    }

    #[test]
    fn hex2dec_nine_digit_high_value_stays_positive() {
        // 9 F's = 68,719,476,735 — must NOT trigger two's complement (requires exactly 10 digits)
        assert_eq!(
            builtin_hex2dec(&[Value::Text("FFFFFFFFF".into())]),
            Value::Number(68_719_476_735.0),
        );
    }

    #[test]
    fn hex2dec_numeric_input_coerced_to_text() {
        // 10.0 → text "10" → hex 0x10 → 16 (proves hex interpretation, not decimal passthrough)
        assert_eq!(builtin_hex2dec(&[Value::Number(10.0)]), Value::Number(16.0));
    }

    #[test]
    fn hex2dec_empty_string_returns_num_error() {
        assert_eq!(builtin_hex2dec(&[Value::Text("".into())]), Value::Error(CellError::Num));
    }

    #[test]
    fn hex2dec_invalid_hex_char_returns_num_error() {
        assert_eq!(builtin_hex2dec(&[Value::Text("G1".into())]), Value::Error(CellError::Num));
    }

    #[test]
    fn hex2dec_too_long_returns_num_error() {
        assert_eq!(
            builtin_hex2dec(&[Value::Text("1FFFFFFFFFF".into())]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn hex2dec_error_propagation() {
        assert_eq!(builtin_hex2dec(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn hex2dec_wrong_arity() {
        assert_eq!(builtin_hex2dec(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_hex2dec(&[Value::Text("FF".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- DEC2HEX --

    #[test]
    fn dec2hex_basic() {
        assert_eq!(builtin_dec2hex(&[Value::Number(255.0)]), Value::Text("FF".into()));
        assert_eq!(builtin_dec2hex(&[Value::Number(0.0)]), Value::Text("0".into()));
        assert_eq!(builtin_dec2hex(&[Value::Number(165.0)]), Value::Text("A5".into()));
    }

    #[test]
    fn dec2hex_max_positive() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(549_755_813_887.0)]),
            Value::Text("7FFFFFFFFF".into()),
        );
    }

    #[test]
    fn dec2hex_with_places() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Number(4.0)]),
            Value::Text("00FF".into()),
        );
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Number(2.0)]),
            Value::Text("FF".into()),
        );
        assert_eq!(
            builtin_dec2hex(&[Value::Number(0.0), Value::Number(4.0)]),
            Value::Text("0000".into()),
        );
        assert_eq!(
            builtin_dec2hex(&[Value::Number(1.0), Value::Number(10.0)]),
            Value::Text("0000000001".into()),
        );
    }

    #[test]
    fn dec2hex_negative_twos_complement() {
        assert_eq!(builtin_dec2hex(&[Value::Number(-1.0)]), Value::Text("FFFFFFFFFF".into()),);
        assert_eq!(
            builtin_dec2hex(&[Value::Number(-549_755_813_888.0)]),
            Value::Text("8000000000".into()),
        );
        assert_eq!(builtin_dec2hex(&[Value::Number(-256.0)]), Value::Text("FFFFFFFF00".into()),);
    }

    #[test]
    fn dec2hex_truncates_decimal() {
        assert_eq!(builtin_dec2hex(&[Value::Number(255.7)]), Value::Text("FF".into()));
    }

    #[test]
    fn dec2hex_text_coercion() {
        assert_eq!(builtin_dec2hex(&[Value::Text("255".into())]), Value::Text("FF".into()));
    }

    #[test]
    fn dec2hex_bool_coercion() {
        assert_eq!(builtin_dec2hex(&[Value::Bool(true)]), Value::Text("1".into()));
    }

    #[test]
    fn dec2hex_zero_with_single_place() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(0.0), Value::Number(1.0)]),
            Value::Text("0".into()),
        );
    }

    #[test]
    fn dec2hex_fractional_places_truncated() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Number(4.9)]),
            Value::Text("00FF".into()),
        );
    }

    #[test]
    fn dec2hex_nan_returns_num_error() {
        assert_eq!(builtin_dec2hex(&[Value::Number(f64::NAN)]), Value::Error(CellError::Num),);
        assert_eq!(builtin_dec2hex(&[Value::Number(f64::INFINITY)]), Value::Error(CellError::Num),);
    }

    #[test]
    fn dec2hex_out_of_range() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(549_755_813_888.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_dec2hex(&[Value::Number(-549_755_813_889.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2hex_places_too_small() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Number(1.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2hex_places_out_of_bounds() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Number(11.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2hex_negative_with_places_too_small() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(-1.0), Value::Number(2.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2hex_error_propagation() {
        assert_eq!(builtin_dec2hex(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn dec2hex_error_propagation_in_places() {
        assert_eq!(
            builtin_dec2hex(&[Value::Number(255.0), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn dec2hex_non_numeric_text_returns_value_error() {
        assert_eq!(builtin_dec2hex(&[Value::Text("abc".into())]), Value::Error(CellError::Value),);
    }

    #[test]
    fn dec2hex_wrong_arity() {
        assert_eq!(builtin_dec2hex(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_dec2hex(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- format_complex --

    #[test]
    fn format_complex_full_form() {
        assert_eq!(format_complex(3.0, 4.0, 'i'), "3+4i");
        assert_eq!(format_complex(3.0, -4.0, 'i'), "3-4i");
        assert_eq!(format_complex(-3.0, 4.0, 'i'), "-3+4i");
        assert_eq!(format_complex(-3.0, -4.0, 'i'), "-3-4i");
    }

    #[test]
    fn format_complex_unit_coefficients() {
        assert_eq!(format_complex(1.0, 1.0, 'i'), "1+i");
        assert_eq!(format_complex(1.0, -1.0, 'i'), "1-i");
        assert_eq!(format_complex(-1.0, 1.0, 'i'), "-1+i");
        assert_eq!(format_complex(-1.0, -1.0, 'i'), "-1-i");
    }

    #[test]
    fn format_complex_zero_parts() {
        assert_eq!(format_complex(0.0, 0.0, 'i'), "0");
        assert_eq!(format_complex(3.0, 0.0, 'i'), "3");
        assert_eq!(format_complex(-3.0, 0.0, 'i'), "-3");
        assert_eq!(format_complex(0.0, 4.0, 'i'), "4i");
        assert_eq!(format_complex(0.0, -4.0, 'i'), "-4i");
        assert_eq!(format_complex(0.0, 1.0, 'i'), "i");
        assert_eq!(format_complex(0.0, -1.0, 'i'), "-i");
    }

    #[test]
    fn format_complex_decimals() {
        assert_eq!(format_complex(3.5, 2.7, 'i'), "3.5+2.7i");
        assert_eq!(format_complex(0.5, 0.5, 'i'), "0.5+0.5i");
    }

    #[test]
    fn format_complex_j_suffix() {
        assert_eq!(format_complex(3.0, 4.0, 'j'), "3+4j");
        assert_eq!(format_complex(0.0, 1.0, 'j'), "j");
        assert_eq!(format_complex(0.0, -1.0, 'j'), "-j");
    }

    // -- parse_complex --

    #[test]
    fn parse_complex_full_form() {
        let (r, i, s) = parse_complex("3+4i").unwrap();
        assert!((r - 3.0).abs() < 1e-12);
        assert!((i - 4.0).abs() < 1e-12);
        assert_eq!(s, 'i');
    }

    #[test]
    fn parse_complex_negative_imag() {
        let (r, i, _) = parse_complex("3-4i").unwrap();
        assert!((r - 3.0).abs() < 1e-12);
        assert!((i - (-4.0)).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_pure_real() {
        let (r, i, s) = parse_complex("3").unwrap();
        assert!((r - 3.0).abs() < 1e-12);
        assert!((i).abs() < 1e-12);
        assert_eq!(s, 'i');
    }

    #[test]
    fn parse_complex_pure_imaginary() {
        let (r, i, _) = parse_complex("4i").unwrap();
        assert!((r).abs() < 1e-12);
        assert!((i - 4.0).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_bare_suffix() {
        let (r, i, _) = parse_complex("i").unwrap();
        assert!((r).abs() < 1e-12);
        assert!((i - 1.0).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_negative_bare_suffix() {
        let (r, i, _) = parse_complex("-i").unwrap();
        assert!((r).abs() < 1e-12);
        assert!((i - (-1.0)).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_j_suffix() {
        let (r, i, s) = parse_complex("3+4j").unwrap();
        assert!((r - 3.0).abs() < 1e-12);
        assert!((i - 4.0).abs() < 1e-12);
        assert_eq!(s, 'j');
    }

    #[test]
    fn parse_complex_negative_real_and_imag() {
        let (r, i, _) = parse_complex("-2-3i").unwrap();
        assert!((r - (-2.0)).abs() < 1e-12);
        assert!((i - (-3.0)).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_decimal_parts() {
        let (r, i, _) = parse_complex("-3.5+2.7i").unwrap();
        assert!((r - (-3.5)).abs() < 1e-12);
        assert!((i - 2.7).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_scientific_notation() {
        let (r, i, _) = parse_complex("1E2+3i").unwrap();
        assert!((r - 100.0).abs() < 1e-12);
        assert!((i - 3.0).abs() < 1e-12);

        let (r2, i2, _) = parse_complex("1.5E-2+3i").unwrap();
        assert!((r2 - 0.015).abs() < 1e-12);
        assert!((i2 - 3.0).abs() < 1e-12);

        let (r3, i3, _) = parse_complex("3+1E2i").unwrap();
        assert!((r3 - 3.0).abs() < 1e-12);
        assert!((i3 - 100.0).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_leading_plus() {
        let (r, i, _) = parse_complex("+3+4i").unwrap();
        assert!((r - 3.0).abs() < 1e-12);
        assert!((i - 4.0).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_zero_forms() {
        let (r, i, _) = parse_complex("3+0i").unwrap();
        assert!((r - 3.0).abs() < 1e-12);
        assert!((i).abs() < 1e-12);

        let (r2, i2, _) = parse_complex("0+0i").unwrap();
        assert!((r2).abs() < 1e-12);
        assert!((i2).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_negative_pure_imaginary() {
        let (r, i, _) = parse_complex("-3i").unwrap();
        assert!((r).abs() < 1e-12);
        assert!((i - (-3.0)).abs() < 1e-12);
    }

    #[test]
    fn parse_complex_invalid_format() {
        assert_eq!(parse_complex("abc"), Err(CellError::Num));
        assert_eq!(parse_complex(""), Err(CellError::Num));
        assert_eq!(parse_complex("3+4i+5"), Err(CellError::Num));
        assert_eq!(parse_complex("3+4ij"), Err(CellError::Num));
    }

    // -- COMPLEX --

    #[test]
    fn complex_happy_path() {
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0)]),
            Value::Text("3+4i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(-4.0)]),
            Value::Text("3-4i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(1.0), Value::Number(1.0)]),
            Value::Text("1+i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(1.0), Value::Number(-1.0)]),
            Value::Text("1-i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(-3.0), Value::Number(4.0)]),
            Value::Text("-3+4i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(-3.0), Value::Number(-4.0)]),
            Value::Text("-3-4i".into()),
        );
    }

    #[test]
    fn complex_zero_handling() {
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(0.0)]),
            Value::Text("0".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(0.0)]),
            Value::Text("3".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(-3.0), Value::Number(0.0)]),
            Value::Text("-3".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(4.0)]),
            Value::Text("4i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(-4.0)]),
            Value::Text("-4i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(1.0)]),
            Value::Text("i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(-1.0)]),
            Value::Text("-i".into()),
        );
    }

    #[test]
    fn complex_with_suffix() {
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Text("j".into())]),
            Value::Text("3+4j".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(1.0), Value::Text("j".into())]),
            Value::Text("j".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.0), Value::Number(-1.0), Value::Text("j".into())]),
            Value::Text("-j".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Text("i".into())]),
            Value::Text("3+4i".into()),
        );
    }

    #[test]
    fn complex_decimals() {
        assert_eq!(
            builtin_complex(&[Value::Number(3.5), Value::Number(2.7)]),
            Value::Text("3.5+2.7i".into()),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(0.5), Value::Number(0.5)]),
            Value::Text("0.5+0.5i".into()),
        );
    }

    #[test]
    fn complex_coercion() {
        // text "3" -> number 3
        assert_eq!(
            builtin_complex(&[Value::Text("3".into()), Value::Number(4.0)]),
            Value::Text("3+4i".into()),
        );
        // bool true -> 1
        assert_eq!(
            builtin_complex(&[Value::Bool(true), Value::Number(4.0)]),
            Value::Text("1+4i".into()),
        );
    }

    #[test]
    fn complex_error_invalid_suffix() {
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Text("k".into())]),
            Value::Error(CellError::Value),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Text("I".into())]),
            Value::Error(CellError::Value),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Text("J".into())]),
            Value::Error(CellError::Value),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Text("".into())]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn complex_error_propagation() {
        assert_eq!(
            builtin_complex(&[Value::Error(CellError::Na), Value::Number(4.0)]),
            Value::Error(CellError::Na),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(4.0), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn complex_non_finite_returns_num_error() {
        assert_eq!(
            builtin_complex(&[Value::Number(f64::NAN), Value::Number(4.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_complex(&[Value::Number(3.0), Value::Number(f64::INFINITY)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn complex_wrong_arity() {
        assert_eq!(builtin_complex(&[]), Value::Error(CellError::Value));
        assert_eq!(builtin_complex(&[Value::Number(3.0)]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_complex(&[
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Text("i".into()),
                Value::Number(4.0),
            ]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn complex_type_mismatch() {
        assert_eq!(
            builtin_complex(&[Value::Text("abc".into()), Value::Number(4.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- IMREAL --

    #[test]
    fn imreal_happy_path() {
        assert_eq!(builtin_imreal(&[Value::Text("3+4i".into())]), Value::Number(3.0));
        assert_eq!(builtin_imreal(&[Value::Text("3-4i".into())]), Value::Number(3.0));
        assert_eq!(builtin_imreal(&[Value::Text("3".into())]), Value::Number(3.0));
        assert_eq!(builtin_imreal(&[Value::Text("4i".into())]), Value::Number(0.0));
        assert_eq!(builtin_imreal(&[Value::Text("i".into())]), Value::Number(0.0));
        assert_eq!(builtin_imreal(&[Value::Text("-i".into())]), Value::Number(0.0));
        assert_eq!(builtin_imreal(&[Value::Text("0".into())]), Value::Number(0.0));
        assert_eq!(builtin_imreal(&[Value::Text("-3.5+2.7i".into())]), Value::Number(-3.5));
        assert_eq!(builtin_imreal(&[Value::Text("3+4j".into())]), Value::Number(3.0));
    }

    #[test]
    fn imreal_coercion() {
        assert_eq!(builtin_imreal(&[Value::Number(5.0)]), Value::Number(5.0));
        assert_eq!(builtin_imreal(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn imreal_errors() {
        assert_eq!(builtin_imreal(&[Value::Text("abc".into())]), Value::Error(CellError::Num));
        assert_eq!(builtin_imreal(&[Value::Text("".into())]), Value::Error(CellError::Num));
        assert_eq!(builtin_imreal(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn imreal_wrong_arity() {
        assert_eq!(builtin_imreal(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_imreal(&[Value::Text("3+4i".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn imreal_bool_returns_value_error() {
        assert_eq!(builtin_imreal(&[Value::Bool(true)]), Value::Error(CellError::Value));
        assert_eq!(builtin_imreal(&[Value::Bool(false)]), Value::Error(CellError::Value));
    }

    // -- IMAGINARY --

    #[test]
    fn imaginary_happy_path() {
        assert_eq!(builtin_imaginary(&[Value::Text("3+4i".into())]), Value::Number(4.0));
        assert_eq!(builtin_imaginary(&[Value::Text("3-4i".into())]), Value::Number(-4.0));
        assert_eq!(builtin_imaginary(&[Value::Text("3".into())]), Value::Number(0.0));
        assert_eq!(builtin_imaginary(&[Value::Text("4i".into())]), Value::Number(4.0));
        assert_eq!(builtin_imaginary(&[Value::Text("-4i".into())]), Value::Number(-4.0));
        assert_eq!(builtin_imaginary(&[Value::Text("i".into())]), Value::Number(1.0));
        assert_eq!(builtin_imaginary(&[Value::Text("-i".into())]), Value::Number(-1.0));
        assert_eq!(builtin_imaginary(&[Value::Text("0".into())]), Value::Number(0.0));
        assert_eq!(builtin_imaginary(&[Value::Text("3.5+2.7j".into())]), Value::Number(2.7));
    }

    #[test]
    fn imaginary_coercion() {
        assert_eq!(builtin_imaginary(&[Value::Number(5.0)]), Value::Number(0.0));
        assert_eq!(builtin_imaginary(&[Value::Number(0.0)]), Value::Number(0.0));
    }

    #[test]
    fn imaginary_errors() {
        assert_eq!(builtin_imaginary(&[Value::Text("abc".into())]), Value::Error(CellError::Num),);
        assert_eq!(builtin_imaginary(&[Value::Text("".into())]), Value::Error(CellError::Num));
        assert_eq!(builtin_imaginary(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn imaginary_wrong_arity() {
        assert_eq!(builtin_imaginary(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_imaginary(&[Value::Text("3+4i".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn imaginary_bool_returns_value_error() {
        assert_eq!(builtin_imaginary(&[Value::Bool(true)]), Value::Error(CellError::Value));
        assert_eq!(builtin_imaginary(&[Value::Bool(false)]), Value::Error(CellError::Value));
    }

    // -- BITAND --

    #[test]
    fn bitand_happy_path() {
        assert_eq!(builtin_bitand(&[Value::Number(13.0), Value::Number(25.0)]), Value::Number(9.0));
        assert_eq!(builtin_bitand(&[Value::Number(1.0), Value::Number(1.0)]), Value::Number(1.0));
        assert_eq!(builtin_bitand(&[Value::Number(0.0), Value::Number(0.0)]), Value::Number(0.0));
        assert_eq!(
            builtin_bitand(&[Value::Number(255.0), Value::Number(15.0)]),
            Value::Number(15.0),
        );
        assert_eq!(
            builtin_bitand(&[
                Value::Number(281_474_976_710_655.0),
                Value::Number(281_474_976_710_655.0),
            ]),
            Value::Number(281_474_976_710_655.0),
        );
    }

    #[test]
    fn bitand_negative_returns_num_error() {
        assert_eq!(
            builtin_bitand(&[Value::Number(-1.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_bitand(&[Value::Number(0.0), Value::Number(-1.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitand_non_integer_returns_num_error() {
        assert_eq!(
            builtin_bitand(&[Value::Number(13.5), Value::Number(25.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_bitand(&[Value::Number(13.0), Value::Number(25.1)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitand_exceeds_max_returns_num_error() {
        assert_eq!(
            builtin_bitand(&[Value::Number(281_474_976_710_656.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitand_wrong_arity() {
        assert_eq!(builtin_bitand(&[Value::Number(1.0)]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_bitand(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitand_type_error() {
        assert_eq!(
            builtin_bitand(&[Value::Text("abc".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitand_error_propagation() {
        assert_eq!(
            builtin_bitand(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn bitand_coercion() {
        assert_eq!(builtin_bitand(&[Value::Bool(true), Value::Number(1.0)]), Value::Number(1.0));
        assert_eq!(
            builtin_bitand(&[Value::Text("13".into()), Value::Number(25.0)]),
            Value::Number(9.0),
        );
    }

    // -- BITOR --

    #[test]
    fn bitor_happy_path() {
        assert_eq!(builtin_bitor(&[Value::Number(13.0), Value::Number(25.0)]), Value::Number(29.0));
        assert_eq!(builtin_bitor(&[Value::Number(0.0), Value::Number(0.0)]), Value::Number(0.0));
        assert_eq!(
            builtin_bitor(&[Value::Number(255.0), Value::Number(256.0)]),
            Value::Number(511.0),
        );
        assert_eq!(
            builtin_bitor(&[Value::Number(281_474_976_710_655.0), Value::Number(0.0)]),
            Value::Number(281_474_976_710_655.0),
        );
    }

    #[test]
    fn bitor_negative_returns_num_error() {
        assert_eq!(
            builtin_bitor(&[Value::Number(-1.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitor_exceeds_max_returns_num_error() {
        assert_eq!(
            builtin_bitor(&[Value::Number(281_474_976_710_656.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitor_wrong_arity() {
        assert_eq!(builtin_bitor(&[Value::Number(1.0)]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_bitor(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitor_type_error() {
        assert_eq!(
            builtin_bitor(&[Value::Text("abc".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitor_error_propagation() {
        assert_eq!(
            builtin_bitor(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn bitor_coercion() {
        assert_eq!(builtin_bitor(&[Value::Bool(true), Value::Number(0.0)]), Value::Number(1.0));
        assert_eq!(
            builtin_bitor(&[Value::Text("13".into()), Value::Number(25.0)]),
            Value::Number(29.0),
        );
    }

    // -- BITXOR --

    #[test]
    fn bitxor_happy_path() {
        assert_eq!(
            builtin_bitxor(&[Value::Number(13.0), Value::Number(25.0)]),
            Value::Number(20.0),
        );
        assert_eq!(builtin_bitxor(&[Value::Number(0.0), Value::Number(0.0)]), Value::Number(0.0));
        assert_eq!(
            builtin_bitxor(&[Value::Number(255.0), Value::Number(255.0)]),
            Value::Number(0.0),
        );
        assert_eq!(
            builtin_bitxor(&[Value::Number(255.0), Value::Number(0.0)]),
            Value::Number(255.0),
        );
    }

    #[test]
    fn bitxor_non_integer_returns_num_error() {
        assert_eq!(
            builtin_bitxor(&[Value::Number(13.0), Value::Number(25.1)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitxor_wrong_arity() {
        assert_eq!(builtin_bitxor(&[Value::Number(1.0)]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_bitxor(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitxor_type_error() {
        assert_eq!(
            builtin_bitxor(&[Value::Text("abc".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitxor_error_propagation() {
        assert_eq!(
            builtin_bitxor(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn bitxor_coercion() {
        assert_eq!(builtin_bitxor(&[Value::Bool(true), Value::Number(1.0)]), Value::Number(0.0));
        assert_eq!(
            builtin_bitxor(&[Value::Text("13".into()), Value::Number(25.0)]),
            Value::Number(20.0),
        );
    }

    // -- BITLSHIFT --

    #[test]
    fn bitlshift_happy_path() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(4.0), Value::Number(2.0)]),
            Value::Number(16.0),
        );
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.0), Value::Number(0.0)]),
            Value::Number(1.0),
        );
        assert_eq!(
            builtin_bitlshift(&[Value::Number(0.0), Value::Number(10.0)]),
            Value::Number(0.0),
        );
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.0), Value::Number(47.0)]),
            Value::Number(140_737_488_355_328.0),
        );
    }

    #[test]
    fn bitlshift_negative_shift_is_right_shift() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(16.0), Value::Number(-2.0)]),
            Value::Number(4.0),
        );
        assert_eq!(
            builtin_bitlshift(&[Value::Number(4.0), Value::Number(-2.0)]),
            Value::Number(1.0),
        );
    }

    #[test]
    fn bitlshift_overflow_returns_num_error() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.0), Value::Number(48.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitlshift_shift_out_of_range_returns_num_error() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.0), Value::Number(54.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.0), Value::Number(-54.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitlshift_negative_number_returns_num_error() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(-1.0), Value::Number(1.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitlshift_non_integer_number_returns_num_error() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.5), Value::Number(2.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitlshift_non_integer_shift_returns_num_error() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(1.0), Value::Number(2.5)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitlshift_wrong_arity() {
        assert_eq!(builtin_bitlshift(&[Value::Number(1.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn bitlshift_error_propagation() {
        assert_eq!(
            builtin_bitlshift(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn bitlshift_zero_shifted_max_is_zero() {
        assert_eq!(
            builtin_bitlshift(&[Value::Number(0.0), Value::Number(53.0)]),
            Value::Number(0.0),
        );
    }

    #[test]
    fn bitlshift_type_error() {
        assert_eq!(
            builtin_bitlshift(&[Value::Text("abc".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitlshift_coercion() {
        assert_eq!(builtin_bitlshift(&[Value::Bool(true), Value::Number(3.0)]), Value::Number(8.0),);
        assert_eq!(
            builtin_bitlshift(&[Value::Text("4".into()), Value::Number(2.0)]),
            Value::Number(16.0),
        );
    }

    // -- BITRSHIFT --

    #[test]
    fn bitrshift_happy_path() {
        assert_eq!(
            builtin_bitrshift(&[Value::Number(16.0), Value::Number(2.0)]),
            Value::Number(4.0),
        );
        assert_eq!(
            builtin_bitrshift(&[Value::Number(1.0), Value::Number(0.0)]),
            Value::Number(1.0),
        );
        assert_eq!(
            builtin_bitrshift(&[Value::Number(1.0), Value::Number(1.0)]),
            Value::Number(0.0),
        );
    }

    #[test]
    fn bitrshift_negative_shift_is_left_shift() {
        assert_eq!(
            builtin_bitrshift(&[Value::Number(4.0), Value::Number(-2.0)]),
            Value::Number(16.0),
        );
        assert_eq!(
            builtin_bitrshift(&[Value::Number(1.0), Value::Number(-3.0)]),
            Value::Number(8.0),
        );
    }

    #[test]
    fn bitrshift_negative_number_returns_num_error() {
        assert_eq!(
            builtin_bitrshift(&[Value::Number(-1.0), Value::Number(1.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bitrshift_wrong_arity() {
        assert_eq!(builtin_bitrshift(&[Value::Number(1.0)]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_bitrshift(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitrshift_type_error() {
        assert_eq!(
            builtin_bitrshift(&[Value::Text("abc".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn bitrshift_error_propagation() {
        assert_eq!(
            builtin_bitrshift(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn bitrshift_coercion() {
        assert_eq!(builtin_bitrshift(&[Value::Bool(true), Value::Number(0.0)]), Value::Number(1.0),);
        assert_eq!(
            builtin_bitrshift(&[Value::Text("16".into()), Value::Number(2.0)]),
            Value::Number(4.0),
        );
    }

    // -- BIN2DEC --

    #[test]
    fn bin2dec_basic() {
        assert_eq!(builtin_bin2dec(&[Value::Text("1100100".into())]), Value::Number(100.0));
        assert_eq!(builtin_bin2dec(&[Value::Text("1010".into())]), Value::Number(10.0));
        assert_eq!(builtin_bin2dec(&[Value::Text("0".into())]), Value::Number(0.0));
        assert_eq!(builtin_bin2dec(&[Value::Text("1".into())]), Value::Number(1.0));
        assert_eq!(builtin_bin2dec(&[Value::Text("11".into())]), Value::Number(3.0));
    }

    #[test]
    fn bin2dec_max_positive() {
        assert_eq!(builtin_bin2dec(&[Value::Text("0111111111".into())]), Value::Number(511.0),);
    }

    #[test]
    fn bin2dec_negative_twos_complement() {
        assert_eq!(builtin_bin2dec(&[Value::Text("1111111111".into())]), Value::Number(-1.0),);
        assert_eq!(builtin_bin2dec(&[Value::Text("1000000000".into())]), Value::Number(-512.0),);
        assert_eq!(builtin_bin2dec(&[Value::Text("1111111110".into())]), Value::Number(-2.0),);
        assert_eq!(builtin_bin2dec(&[Value::Text("1111111100".into())]), Value::Number(-4.0),);
        assert_eq!(builtin_bin2dec(&[Value::Text("1110000000".into())]), Value::Number(-128.0),);
    }

    #[test]
    fn bin2dec_leading_zeros() {
        assert_eq!(builtin_bin2dec(&[Value::Text("0000000001".into())]), Value::Number(1.0),);
        assert_eq!(builtin_bin2dec(&[Value::Text("0000000000".into())]), Value::Number(0.0),);
    }

    #[test]
    fn bin2dec_numeric_coercion() {
        assert_eq!(builtin_bin2dec(&[Value::Number(101.0)]), Value::Number(5.0));
    }

    #[test]
    fn bin2dec_empty_string() {
        assert_eq!(builtin_bin2dec(&[Value::Text("".into())]), Value::Error(CellError::Num),);
    }

    #[test]
    fn bin2dec_invalid_chars() {
        assert_eq!(builtin_bin2dec(&[Value::Text("2".into())]), Value::Error(CellError::Num),);
        assert_eq!(builtin_bin2dec(&[Value::Text("10201".into())]), Value::Error(CellError::Num),);
        assert_eq!(builtin_bin2dec(&[Value::Text("1A".into())]), Value::Error(CellError::Num),);
    }

    #[test]
    fn bin2dec_too_long() {
        assert_eq!(
            builtin_bin2dec(&[Value::Text("10000000001".into())]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn bin2dec_error_propagation() {
        assert_eq!(builtin_bin2dec(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn bin2dec_wrong_arity() {
        assert_eq!(builtin_bin2dec(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_bin2dec(&[Value::Text("1".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- DEC2BIN --

    #[test]
    fn dec2bin_basic() {
        assert_eq!(builtin_dec2bin(&[Value::Number(100.0)]), Value::Text("1100100".into()),);
        assert_eq!(builtin_dec2bin(&[Value::Number(0.0)]), Value::Text("0".into()));
        assert_eq!(builtin_dec2bin(&[Value::Number(10.0)]), Value::Text("1010".into()));
        assert_eq!(builtin_dec2bin(&[Value::Number(511.0)]), Value::Text("111111111".into()),);
        assert_eq!(builtin_dec2bin(&[Value::Number(1.0)]), Value::Text("1".into()));
    }

    #[test]
    fn dec2bin_with_places() {
        assert_eq!(
            builtin_dec2bin(&[Value::Number(100.0), Value::Number(10.0)]),
            Value::Text("0001100100".into()),
        );
        assert_eq!(
            builtin_dec2bin(&[Value::Number(100.0), Value::Number(7.0)]),
            Value::Text("1100100".into()),
        );
        assert_eq!(
            builtin_dec2bin(&[Value::Number(0.0), Value::Number(4.0)]),
            Value::Text("0000".into()),
        );
        assert_eq!(
            builtin_dec2bin(&[Value::Number(1.0), Value::Number(10.0)]),
            Value::Text("0000000001".into()),
        );
    }

    #[test]
    fn dec2bin_negative_twos_complement() {
        assert_eq!(builtin_dec2bin(&[Value::Number(-1.0)]), Value::Text("1111111111".into()),);
        assert_eq!(builtin_dec2bin(&[Value::Number(-512.0)]), Value::Text("1000000000".into()),);
        assert_eq!(builtin_dec2bin(&[Value::Number(-2.0)]), Value::Text("1111111110".into()),);
        assert_eq!(builtin_dec2bin(&[Value::Number(-128.0)]), Value::Text("1110000000".into()),);
    }

    #[test]
    fn dec2bin_out_of_range() {
        assert_eq!(builtin_dec2bin(&[Value::Number(512.0)]), Value::Error(CellError::Num),);
        assert_eq!(builtin_dec2bin(&[Value::Number(-513.0)]), Value::Error(CellError::Num),);
    }

    #[test]
    fn dec2bin_places_too_small() {
        assert_eq!(
            builtin_dec2bin(&[Value::Number(100.0), Value::Number(6.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2bin_places_out_of_bounds() {
        assert_eq!(
            builtin_dec2bin(&[Value::Number(100.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_dec2bin(&[Value::Number(100.0), Value::Number(11.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2bin_negative_with_places_too_small() {
        assert_eq!(
            builtin_dec2bin(&[Value::Number(-1.0), Value::Number(2.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2bin_truncates_decimal() {
        assert_eq!(builtin_dec2bin(&[Value::Number(100.7)]), Value::Text("1100100".into()),);
    }

    #[test]
    fn dec2bin_text_coercion() {
        assert_eq!(builtin_dec2bin(&[Value::Text("100".into())]), Value::Text("1100100".into()),);
    }

    #[test]
    fn dec2bin_bool_coercion() {
        assert_eq!(builtin_dec2bin(&[Value::Bool(true)]), Value::Text("1".into()));
    }

    #[test]
    fn dec2bin_nan_returns_num() {
        assert_eq!(builtin_dec2bin(&[Value::Number(f64::NAN)]), Value::Error(CellError::Num),);
        assert_eq!(builtin_dec2bin(&[Value::Number(f64::INFINITY)]), Value::Error(CellError::Num),);
    }

    #[test]
    fn dec2bin_error_propagation() {
        assert_eq!(builtin_dec2bin(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn dec2bin_wrong_arity() {
        assert_eq!(builtin_dec2bin(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_dec2bin(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- OCT2DEC --

    #[test]
    fn oct2dec_basic() {
        assert_eq!(builtin_oct2dec(&[Value::Text("144".into())]), Value::Number(100.0));
        assert_eq!(builtin_oct2dec(&[Value::Text("52".into())]), Value::Number(42.0));
        assert_eq!(builtin_oct2dec(&[Value::Text("0".into())]), Value::Number(0.0));
        assert_eq!(builtin_oct2dec(&[Value::Text("1".into())]), Value::Number(1.0));
        assert_eq!(builtin_oct2dec(&[Value::Text("7".into())]), Value::Number(7.0));
        assert_eq!(builtin_oct2dec(&[Value::Text("10".into())]), Value::Number(8.0));
    }

    #[test]
    fn oct2dec_max_positive() {
        assert_eq!(
            builtin_oct2dec(&[Value::Text("3777777777".into())]),
            Value::Number(536_870_911.0),
        );
    }

    #[test]
    fn oct2dec_negative_twos_complement() {
        assert_eq!(builtin_oct2dec(&[Value::Text("7777777777".into())]), Value::Number(-1.0),);
        assert_eq!(
            builtin_oct2dec(&[Value::Text("4000000000".into())]),
            Value::Number(-536_870_912.0),
        );
        assert_eq!(builtin_oct2dec(&[Value::Text("7777777770".into())]), Value::Number(-8.0),);
        assert_eq!(builtin_oct2dec(&[Value::Text("7777777600".into())]), Value::Number(-128.0),);
    }

    #[test]
    fn oct2dec_leading_zeros() {
        assert_eq!(builtin_oct2dec(&[Value::Text("0000000001".into())]), Value::Number(1.0),);
        assert_eq!(builtin_oct2dec(&[Value::Text("0000000000".into())]), Value::Number(0.0),);
    }

    #[test]
    fn oct2dec_numeric_coercion() {
        assert_eq!(builtin_oct2dec(&[Value::Number(77.0)]), Value::Number(63.0));
    }

    #[test]
    fn oct2dec_invalid_chars() {
        assert_eq!(builtin_oct2dec(&[Value::Text("8".into())]), Value::Error(CellError::Num),);
        assert_eq!(builtin_oct2dec(&[Value::Text("9".into())]), Value::Error(CellError::Num),);
        assert_eq!(builtin_oct2dec(&[Value::Text("1A".into())]), Value::Error(CellError::Num),);
    }

    #[test]
    fn oct2dec_empty_string() {
        assert_eq!(builtin_oct2dec(&[Value::Text("".into())]), Value::Error(CellError::Num),);
    }

    #[test]
    fn oct2dec_too_long() {
        assert_eq!(
            builtin_oct2dec(&[Value::Text("10000000001".into())]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn oct2dec_error_propagation() {
        assert_eq!(builtin_oct2dec(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn oct2dec_wrong_arity() {
        assert_eq!(builtin_oct2dec(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_oct2dec(&[Value::Text("1".into()), Value::Number(1.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- DEC2OCT --

    #[test]
    fn dec2oct_basic() {
        assert_eq!(builtin_dec2oct(&[Value::Number(100.0)]), Value::Text("144".into()));
        assert_eq!(builtin_dec2oct(&[Value::Number(0.0)]), Value::Text("0".into()));
        assert_eq!(builtin_dec2oct(&[Value::Number(42.0)]), Value::Text("52".into()));
        assert_eq!(builtin_dec2oct(&[Value::Number(8.0)]), Value::Text("10".into()));
        assert_eq!(
            builtin_dec2oct(&[Value::Number(536_870_911.0)]),
            Value::Text("3777777777".into()),
        );
    }

    #[test]
    fn dec2oct_with_places() {
        assert_eq!(
            builtin_dec2oct(&[Value::Number(100.0), Value::Number(6.0)]),
            Value::Text("000144".into()),
        );
        assert_eq!(
            builtin_dec2oct(&[Value::Number(100.0), Value::Number(3.0)]),
            Value::Text("144".into()),
        );
        assert_eq!(
            builtin_dec2oct(&[Value::Number(0.0), Value::Number(4.0)]),
            Value::Text("0000".into()),
        );
        assert_eq!(
            builtin_dec2oct(&[Value::Number(1.0), Value::Number(10.0)]),
            Value::Text("0000000001".into()),
        );
    }

    #[test]
    fn dec2oct_negative_twos_complement() {
        assert_eq!(builtin_dec2oct(&[Value::Number(-1.0)]), Value::Text("7777777777".into()),);
        assert_eq!(
            builtin_dec2oct(&[Value::Number(-536_870_912.0)]),
            Value::Text("4000000000".into()),
        );
        assert_eq!(builtin_dec2oct(&[Value::Number(-8.0)]), Value::Text("7777777770".into()),);
        assert_eq!(builtin_dec2oct(&[Value::Number(-128.0)]), Value::Text("7777777600".into()),);
    }

    #[test]
    fn dec2oct_out_of_range() {
        assert_eq!(builtin_dec2oct(&[Value::Number(536_870_912.0)]), Value::Error(CellError::Num),);
        assert_eq!(builtin_dec2oct(&[Value::Number(-536_870_913.0)]), Value::Error(CellError::Num),);
    }

    #[test]
    fn dec2oct_places_too_small() {
        assert_eq!(
            builtin_dec2oct(&[Value::Number(100.0), Value::Number(2.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2oct_places_out_of_bounds() {
        assert_eq!(
            builtin_dec2oct(&[Value::Number(100.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_dec2oct(&[Value::Number(100.0), Value::Number(11.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2oct_negative_with_places_too_small() {
        assert_eq!(
            builtin_dec2oct(&[Value::Number(-1.0), Value::Number(2.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn dec2oct_truncates_decimal() {
        assert_eq!(builtin_dec2oct(&[Value::Number(100.7)]), Value::Text("144".into()),);
    }

    #[test]
    fn dec2oct_text_coercion() {
        assert_eq!(builtin_dec2oct(&[Value::Text("100".into())]), Value::Text("144".into()),);
    }

    #[test]
    fn dec2oct_bool_coercion() {
        assert_eq!(builtin_dec2oct(&[Value::Bool(true)]), Value::Text("1".into()));
    }

    #[test]
    fn dec2oct_nan_returns_num() {
        assert_eq!(builtin_dec2oct(&[Value::Number(f64::NAN)]), Value::Error(CellError::Num),);
    }

    #[test]
    fn dec2oct_error_propagation() {
        assert_eq!(builtin_dec2oct(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn dec2oct_wrong_arity() {
        assert_eq!(builtin_dec2oct(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_dec2oct(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }

    // -- HEX2BIN --

    #[test]
    fn hex2bin_basic() {
        assert_eq!(builtin_hex2bin(&[Value::Text("F".into())]), Value::Text("1111".into()),);
        assert_eq!(builtin_hex2bin(&[Value::Text("A".into())]), Value::Text("1010".into()),);
        assert_eq!(builtin_hex2bin(&[Value::Text("0".into())]), Value::Text("0".into()));
        assert_eq!(builtin_hex2bin(&[Value::Text("1FF".into())]), Value::Text("111111111".into()),);
    }

    #[test]
    fn hex2bin_with_places() {
        assert_eq!(
            builtin_hex2bin(&[Value::Text("F".into()), Value::Number(8.0)]),
            Value::Text("00001111".into()),
        );
        assert_eq!(
            builtin_hex2bin(&[Value::Text("F".into()), Value::Number(4.0)]),
            Value::Text("1111".into()),
        );
    }

    #[test]
    fn hex2bin_negative() {
        assert_eq!(
            builtin_hex2bin(&[Value::Text("FFFFFFFFFE".into())]),
            Value::Text("1111111110".into()),
        );
        assert_eq!(
            builtin_hex2bin(&[Value::Text("FFFFFFFE00".into())]),
            Value::Text("1000000000".into()),
        );
    }

    #[test]
    fn hex2bin_out_of_binary_range() {
        assert_eq!(builtin_hex2bin(&[Value::Text("200".into())]), Value::Error(CellError::Num),);
        assert_eq!(
            builtin_hex2bin(&[Value::Text("FFFFFFFDFF".into())]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn hex2bin_wrong_arity() {
        assert_eq!(builtin_hex2bin(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn hex2bin_error_propagation() {
        assert_eq!(builtin_hex2bin(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn hex2bin_lowercase_input() {
        assert_eq!(builtin_hex2bin(&[Value::Text("f".into())]), Value::Text("1111".into()),);
    }

    // -- BIN2HEX --

    #[test]
    fn bin2hex_basic() {
        assert_eq!(builtin_bin2hex(&[Value::Text("1111".into())]), Value::Text("F".into()),);
        assert_eq!(builtin_bin2hex(&[Value::Text("0".into())]), Value::Text("0".into()));
        assert_eq!(builtin_bin2hex(&[Value::Text("0111111111".into())]), Value::Text("1FF".into()),);
    }

    #[test]
    fn bin2hex_with_places() {
        assert_eq!(
            builtin_bin2hex(&[Value::Text("1111".into()), Value::Number(4.0)]),
            Value::Text("000F".into()),
        );
    }

    #[test]
    fn bin2hex_negative() {
        assert_eq!(
            builtin_bin2hex(&[Value::Text("1111111111".into())]),
            Value::Text("FFFFFFFFFF".into()),
        );
        assert_eq!(
            builtin_bin2hex(&[Value::Text("1000000000".into())]),
            Value::Text("FFFFFFFE00".into()),
        );
    }

    #[test]
    fn bin2hex_error_propagation() {
        assert_eq!(builtin_bin2hex(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na),);
    }

    #[test]
    fn bin2hex_wrong_arity() {
        assert_eq!(builtin_bin2hex(&[]), Value::Error(CellError::Value));
    }

    // -- HEX2OCT --

    #[test]
    fn hex2oct_basic() {
        assert_eq!(builtin_hex2oct(&[Value::Text("F".into())]), Value::Text("17".into()),);
        assert_eq!(builtin_hex2oct(&[Value::Text("0".into())]), Value::Text("0".into()));
        assert_eq!(
            builtin_hex2oct(&[Value::Text("1FFFFFFF".into())]),
            Value::Text("3777777777".into()),
        );
    }

    #[test]
    fn hex2oct_with_places() {
        assert_eq!(
            builtin_hex2oct(&[Value::Text("F".into()), Value::Number(4.0)]),
            Value::Text("0017".into()),
        );
    }

    #[test]
    fn hex2oct_negative() {
        assert_eq!(
            builtin_hex2oct(&[Value::Text("FFFFFFFFFF".into())]),
            Value::Text("7777777777".into()),
        );
    }

    #[test]
    fn hex2oct_out_of_octal_range() {
        assert_eq!(
            builtin_hex2oct(&[Value::Text("2000000000".into())]),
            Value::Error(CellError::Num),
        );
    }

    // -- OCT2HEX --

    #[test]
    fn oct2hex_basic() {
        assert_eq!(builtin_oct2hex(&[Value::Text("17".into())]), Value::Text("F".into()),);
        assert_eq!(builtin_oct2hex(&[Value::Text("0".into())]), Value::Text("0".into()));
        assert_eq!(
            builtin_oct2hex(&[Value::Text("3777777777".into())]),
            Value::Text("1FFFFFFF".into()),
        );
    }

    #[test]
    fn oct2hex_with_places() {
        assert_eq!(
            builtin_oct2hex(&[Value::Text("17".into()), Value::Number(4.0)]),
            Value::Text("000F".into()),
        );
    }

    #[test]
    fn oct2hex_negative() {
        assert_eq!(
            builtin_oct2hex(&[Value::Text("7777777777".into())]),
            Value::Text("FFFFFFFFFF".into()),
        );
    }

    // -- BIN2OCT --

    #[test]
    fn bin2oct_basic() {
        assert_eq!(builtin_bin2oct(&[Value::Text("1111".into())]), Value::Text("17".into()),);
        assert_eq!(builtin_bin2oct(&[Value::Text("0".into())]), Value::Text("0".into()));
        assert_eq!(builtin_bin2oct(&[Value::Text("0111111111".into())]), Value::Text("777".into()),);
    }

    #[test]
    fn bin2oct_with_places() {
        assert_eq!(
            builtin_bin2oct(&[Value::Text("1111".into()), Value::Number(4.0)]),
            Value::Text("0017".into()),
        );
    }

    #[test]
    fn bin2oct_negative() {
        assert_eq!(
            builtin_bin2oct(&[Value::Text("1111111111".into())]),
            Value::Text("7777777777".into()),
        );
        assert_eq!(
            builtin_bin2oct(&[Value::Text("1000000000".into())]),
            Value::Text("7777777000".into()),
        );
    }

    // -- OCT2BIN --

    #[test]
    fn oct2bin_basic() {
        assert_eq!(builtin_oct2bin(&[Value::Text("17".into())]), Value::Text("1111".into()),);
        assert_eq!(builtin_oct2bin(&[Value::Text("0".into())]), Value::Text("0".into()));
        assert_eq!(builtin_oct2bin(&[Value::Text("777".into())]), Value::Text("111111111".into()),);
    }

    #[test]
    fn oct2bin_with_places() {
        assert_eq!(
            builtin_oct2bin(&[Value::Text("17".into()), Value::Number(8.0)]),
            Value::Text("00001111".into()),
        );
    }

    #[test]
    fn oct2bin_negative() {
        assert_eq!(
            builtin_oct2bin(&[Value::Text("7777777776".into())]),
            Value::Text("1111111110".into()),
        );
        assert_eq!(
            builtin_oct2bin(&[Value::Text("7777777000".into())]),
            Value::Text("1000000000".into()),
        );
    }

    #[test]
    fn oct2bin_out_of_binary_range() {
        assert_eq!(builtin_oct2bin(&[Value::Text("1000".into())]), Value::Error(CellError::Num),);
    }

    // -- BASE --

    #[test]
    fn base_basic() {
        assert_eq!(
            builtin_base(&[Value::Number(255.0), Value::Number(16.0)]),
            Value::Text("FF".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(2.0)]),
            Value::Text("1010".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(100.0), Value::Number(8.0)]),
            Value::Text("144".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(0.0), Value::Number(16.0)]),
            Value::Text("0".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(0.0), Value::Number(2.0)]),
            Value::Text("0".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(255.0), Value::Number(10.0)]),
            Value::Text("255".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(35.0), Value::Number(36.0)]),
            Value::Text("Z".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(36.0), Value::Number(36.0)]),
            Value::Text("10".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(1295.0), Value::Number(36.0)]),
            Value::Text("ZZ".into()),
        );
    }

    #[test]
    fn base_with_min_length() {
        assert_eq!(
            builtin_base(&[Value::Number(255.0), Value::Number(16.0), Value::Number(4.0)]),
            Value::Text("00FF".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(7.0), Value::Number(2.0), Value::Number(8.0)]),
            Value::Text("00000111".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(255.0), Value::Number(16.0), Value::Number(2.0)]),
            Value::Text("FF".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(255.0), Value::Number(16.0), Value::Number(1.0)]),
            Value::Text("FF".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(0.0), Value::Number(2.0), Value::Number(4.0)]),
            Value::Text("0000".into()),
        );
    }

    #[test]
    fn base_large_values() {
        assert_eq!(
            builtin_base(&[Value::Number(9_007_199_254_740_991.0), Value::Number(16.0)]),
            Value::Text("1FFFFFFFFFFFFF".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(4_294_967_295.0), Value::Number(16.0)]),
            Value::Text("FFFFFFFF".into()),
        );
    }

    #[test]
    fn base_various_radixes() {
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(3.0)]),
            Value::Text("101".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(10.0)]),
            Value::Text("10".into()),
        );
    }

    #[test]
    fn base_type_coercion() {
        assert_eq!(
            builtin_base(&[Value::Number(255.9), Value::Number(16.0)]),
            Value::Text("FF".into()),
        );
        assert_eq!(
            builtin_base(&[Value::Text("255".into()), Value::Number(16.0)]),
            Value::Text("FF".into()),
        );
        assert_eq!(builtin_base(&[Value::Bool(true), Value::Number(2.0)]), Value::Text("1".into()),);
    }

    #[test]
    fn base_negative_returns_num() {
        assert_eq!(
            builtin_base(&[Value::Number(-1.0), Value::Number(16.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn base_too_large_returns_num() {
        assert_eq!(
            builtin_base(&[Value::Number(9_007_199_254_740_992.0), Value::Number(16.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn base_radix_out_of_range() {
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(1.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(37.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(0.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn base_min_length_out_of_range() {
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(2.0), Value::Number(256.0)]),
            Value::Error(CellError::Num),
        );
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Number(2.0), Value::Number(-1.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn base_error_propagation() {
        assert_eq!(
            builtin_base(&[Value::Error(CellError::Na), Value::Number(16.0)]),
            Value::Error(CellError::Na),
        );
        assert_eq!(
            builtin_base(&[Value::Number(10.0), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn base_wrong_arity() {
        assert_eq!(builtin_base(&[]), Value::Error(CellError::Value));
        assert_eq!(builtin_base(&[Value::Number(10.0)]), Value::Error(CellError::Value),);
        assert_eq!(
            builtin_base(&[
                Value::Number(10.0),
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(1.0)
            ]),
            Value::Error(CellError::Value),
        );
    }

    #[test]
    fn base_nan_returns_num() {
        assert_eq!(
            builtin_base(&[Value::Number(f64::NAN), Value::Number(16.0)]),
            Value::Error(CellError::Num),
        );
    }
}
