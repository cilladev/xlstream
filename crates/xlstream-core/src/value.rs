//! The [`Value`] enum — the cell-value sum type used throughout xlstream.

use crate::cell_error::CellError;
use crate::date::ExcelDate;

/// A cell value. Every evaluated formula result, every loaded cell, and every
/// function argument flows through this enum.
///
/// Variants match Excel's type model: `Empty` for blanks, `Number` for `f64`,
/// `Integer` for Excel's rare integer-semantics path, `Text` for strings,
/// `Bool` for boolean literals, `Date` for the 1900-serial date type, and
/// `Error` for Excel's in-cell error values.
///
/// # Examples
///
/// ```
/// use xlstream_core::{CellError, Value};
/// let v = Value::Number(42.0);
/// assert_eq!(v, Value::Number(42.0));
///
/// let err = Value::Error(CellError::Div0);
/// assert_ne!(err, Value::Empty);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A blank cell.
    Empty,
    /// An `f64` numeric value (Excel's default numeric type).
    Number(f64),
    /// An Excel integer value — reserved for functions whose semantics
    /// explicitly require integer arithmetic.
    Integer(i64),
    /// A text value. `Box<str>` because cell text is immutable after load.
    Text(Box<str>),
    /// A boolean literal (`TRUE` / `FALSE`).
    Bool(bool),
    /// An Excel date — see [`ExcelDate`].
    Date(ExcelDate),
    /// An Excel cell-level error — see [`CellError`].
    Error(CellError),
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn empty_equals_empty() {
        assert_eq!(Value::Empty, Value::Empty);
    }

    #[test]
    fn number_roundtrips() {
        let v = Value::Number(2.5);
        match v {
            Value::Number(n) => assert!((n - 2.5).abs() < f64::EPSILON),
            _ => panic!("expected Number"),
        }
    }

    #[test]
    fn text_stores_owned_box_str() {
        let v = Value::Text("hello".into());
        assert_eq!(v, Value::Text("hello".into()));
    }

    #[test]
    fn error_wraps_cell_error() {
        assert_eq!(Value::Error(CellError::Div0), Value::Error(CellError::Div0));
        assert_ne!(Value::Error(CellError::Div0), Value::Error(CellError::Na));
    }
}
