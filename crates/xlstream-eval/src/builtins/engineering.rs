//! Engineering builtin functions.

use xlstream_core::{coerce, CellError, Value};

const MAX_HEX_LEN: usize = 10;
const TWO_POW_40: i64 = 1 << 40;
const MAX_POSITIVE: i64 = TWO_POW_40 / 2 - 1;
const MIN_NEGATIVE: i64 = -(TWO_POW_40 / 2);

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
    let v = &args[0];
    if let Value::Error(e) = v {
        return Value::Error(*e);
    }
    let hex_str = coerce::to_text(v);
    let hex_str = hex_str.as_ref();

    if hex_str.is_empty() || hex_str.len() > MAX_HEX_LEN {
        return Value::Error(CellError::Num);
    }
    if !hex_str.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Value::Error(CellError::Num);
    }

    let Ok(unsigned) = i64::from_str_radix(hex_str, 16) else {
        return Value::Error(CellError::Num);
    };

    // 10-digit hex with leading digit >= 8 → negative (two's complement)
    let result = if hex_str.len() == MAX_HEX_LEN && unsigned > MAX_POSITIVE {
        unsigned - TWO_POW_40
    } else {
        unsigned
    };

    #[allow(clippy::cast_precision_loss)]
    Value::Number(result as f64)
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

    if !num.is_finite() {
        return Value::Error(CellError::Num);
    }

    #[allow(clippy::cast_possible_truncation)]
    let int_val = num.trunc() as i64;

    if !(MIN_NEGATIVE..=MAX_POSITIVE).contains(&int_val) {
        return Value::Error(CellError::Num);
    }

    let hex = if int_val < 0 {
        #[allow(clippy::cast_sign_loss)]
        let unsigned = (int_val + TWO_POW_40) as u64;
        format!("{unsigned:010X}")
    } else {
        #[allow(clippy::cast_sign_loss)]
        let unsigned = int_val as u64;
        format!("{unsigned:X}")
    };

    if args.len() == 2 {
        let places_f = match coerce::to_number(&args[1]) {
            Ok(n) => n,
            Err(e) => return Value::Error(e),
        };
        if !places_f.is_finite() || places_f < 1.0 {
            return Value::Error(CellError::Num);
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let places = places_f.trunc() as usize;
        if places > MAX_HEX_LEN {
            return Value::Error(CellError::Num);
        }
        if hex.len() > places {
            return Value::Error(CellError::Num);
        }
        return Value::Text(format!("{hex:0>places$}").into());
    }

    Value::Text(hex.into())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use xlstream_core::{CellError, Value};

    use super::*;

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
    fn hex2dec_numeric_input_coerced_to_text() {
        assert_eq!(builtin_hex2dec(&[Value::Number(1.0)]), Value::Number(1.0));
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
    fn dec2hex_wrong_arity() {
        assert_eq!(builtin_dec2hex(&[]), Value::Error(CellError::Value));
        assert_eq!(
            builtin_dec2hex(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
            Value::Error(CellError::Value),
        );
    }
}
