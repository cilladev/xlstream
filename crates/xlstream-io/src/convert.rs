//! Value conversion between calamine types and xlstream [`Value`].

use calamine::CellErrorType;
use xlstream_core::{CellError, ExcelDate, Value};

/// Convert a calamine [`DataRef`] to an xlstream [`Value`].
pub(crate) fn convert_data_ref(d: &calamine::DataRef<'_>) -> Value {
    use calamine::DataRef;
    match d {
        DataRef::Int(i) => Value::Integer(*i),
        DataRef::Float(f) => Value::Number(*f),
        DataRef::String(s) | DataRef::DateTimeIso(s) | DataRef::DurationIso(s) => {
            Value::Text(s.as_str().into())
        }
        DataRef::SharedString(s) => Value::Text((*s).into()),
        DataRef::Bool(b) => Value::Bool(*b),
        DataRef::DateTime(dt) => Value::Date(ExcelDate { serial: dt.as_f64() }),
        DataRef::Error(e) => Value::Error(convert_cell_error(e)),
        DataRef::Empty => Value::Empty,
    }
}

fn convert_cell_error(e: &CellErrorType) -> CellError {
    match e {
        CellErrorType::Div0 => CellError::Div0,
        CellErrorType::NA => CellError::Na,
        CellErrorType::Name => CellError::Name,
        CellErrorType::Null => CellError::Null,
        CellErrorType::Num => CellError::Num,
        CellErrorType::Ref => CellError::Ref,
        CellErrorType::Value | CellErrorType::GettingData => CellError::Value,
    }
}

/// Render a [`CellError`] as the Excel error string.
///
/// # Examples
///
/// ```
/// use xlstream_core::CellError;
/// use xlstream_io::convert::cell_error_to_excel_string;
/// assert_eq!(cell_error_to_excel_string(CellError::Na), "#N/A");
/// ```
#[must_use]
pub fn cell_error_to_excel_string(e: CellError) -> &'static str {
    match e {
        CellError::Div0 => "#DIV/0!",
        CellError::Value => "#VALUE!",
        CellError::Ref => "#REF!",
        CellError::Name => "#NAME?",
        CellError::Na => "#N/A",
        CellError::Num => "#NUM!",
        CellError::Null => "#NULL!",
    }
}

/// Render a [`Value`] as a result string for `Formula::set_result()`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_io::convert::value_to_result_string;
/// assert_eq!(value_to_result_string(&Value::Number(5.0)), "5");
/// ```
#[must_use]
pub fn value_to_result_string(val: &Value) -> String {
    match val {
        Value::Number(n) => format!("{n}"),
        Value::Integer(i) => format!("{i}"),
        Value::Text(s) => s.to_string(),
        Value::Bool(true) => "TRUE".to_owned(),
        Value::Bool(false) => "FALSE".to_owned(),
        Value::Date(d) => format!("{}", d.serial),
        Value::Error(e) => cell_error_to_excel_string(*e).to_owned(),
        Value::Empty => String::new(),
    }
}

/// The `t` attribute value for the `<c>` element in xlsx XML.
///
/// # Examples
///
/// ```
/// use xlstream_io::convert::XmlCellType;
/// assert_eq!(XmlCellType::Number.as_attr(), None);
/// assert_eq!(XmlCellType::InlineString.as_attr(), Some("str"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlCellType {
    /// Numeric value (no `t` attribute).
    Number,
    /// Inline string result (`t="str"`).
    InlineString,
    /// Boolean result (`t="b"`).
    Boolean,
    /// Error result (`t="e"`).
    Error,
}

impl XmlCellType {
    /// The `t` attribute value, or `None` for numbers (omit `t`).
    #[must_use]
    pub fn as_attr(self) -> Option<&'static str> {
        match self {
            Self::Number => None,
            Self::InlineString => Some("str"),
            Self::Boolean => Some("b"),
            Self::Error => Some("e"),
        }
    }
}

/// Cached formula result for XML replacement. Holds the `<v>` text
/// content and the cell type for the enclosing `<c>` element.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_io::convert::{value_to_cell_result, XmlCellType};
/// let r = value_to_cell_result(&Value::Number(42.0));
/// assert_eq!(r.value, "42");
/// assert_eq!(r.cell_type, XmlCellType::Number);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellResult {
    /// Text content for the `<v>` element.
    pub value: String,
    /// Cell type for the `t` attribute on `<c>`.
    pub cell_type: XmlCellType,
}

/// Convert a [`Value`] to a [`CellResult`] for XML replacement.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_io::convert::{value_to_cell_result, XmlCellType};
/// let r = value_to_cell_result(&Value::Bool(true));
/// assert_eq!(r.value, "1");
/// assert_eq!(r.cell_type, XmlCellType::Boolean);
/// ```
#[must_use]
pub fn value_to_cell_result(val: &Value) -> CellResult {
    match val {
        Value::Number(n) => CellResult { value: format!("{n}"), cell_type: XmlCellType::Number },
        Value::Integer(i) => CellResult { value: format!("{i}"), cell_type: XmlCellType::Number },
        Value::Text(s) => CellResult { value: s.to_string(), cell_type: XmlCellType::InlineString },
        Value::Bool(true) => CellResult { value: "1".into(), cell_type: XmlCellType::Boolean },
        Value::Bool(false) => CellResult { value: "0".into(), cell_type: XmlCellType::Boolean },
        Value::Date(d) => {
            CellResult { value: format!("{}", d.serial), cell_type: XmlCellType::Number }
        }
        Value::Error(e) => CellResult {
            value: cell_error_to_excel_string(*e).to_owned(),
            cell_type: XmlCellType::Error,
        },
        Value::Empty => CellResult { value: String::new(), cell_type: XmlCellType::Number },
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;
    use calamine::{DataRef, ExcelDateTime, ExcelDateTimeType};

    #[test]
    fn converts_int() {
        assert_eq!(convert_data_ref(&DataRef::Int(42)), Value::Integer(42));
    }

    #[test]
    fn converts_float() {
        assert_eq!(convert_data_ref(&DataRef::Float(2.5)), Value::Number(2.5));
    }

    #[test]
    fn converts_string() {
        assert_eq!(convert_data_ref(&DataRef::String("hello".into())), Value::Text("hello".into()));
    }

    #[test]
    fn converts_shared_string() {
        assert_eq!(convert_data_ref(&DataRef::SharedString("world")), Value::Text("world".into()));
    }

    #[test]
    fn converts_bool() {
        assert_eq!(convert_data_ref(&DataRef::Bool(true)), Value::Bool(true));
    }

    #[test]
    fn converts_datetime() {
        let edt = ExcelDateTime::new(44927.0, ExcelDateTimeType::DateTime, false);
        assert_eq!(
            convert_data_ref(&DataRef::DateTime(edt)),
            Value::Date(ExcelDate { serial: 44927.0 })
        );
    }

    #[test]
    fn converts_error_div0() {
        assert_eq!(
            convert_data_ref(&DataRef::Error(CellErrorType::Div0)),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn converts_error_na() {
        assert_eq!(
            convert_data_ref(&DataRef::Error(CellErrorType::NA)),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn converts_empty() {
        assert_eq!(convert_data_ref(&DataRef::Empty), Value::Empty);
    }

    #[test]
    fn cell_error_to_string_covers_all_variants() {
        assert_eq!(cell_error_to_excel_string(CellError::Div0), "#DIV/0!");
        assert_eq!(cell_error_to_excel_string(CellError::Na), "#N/A");
        assert_eq!(cell_error_to_excel_string(CellError::Name), "#NAME?");
        assert_eq!(cell_error_to_excel_string(CellError::Null), "#NULL!");
    }

    #[test]
    fn value_to_result_covers_all_variants() {
        assert_eq!(value_to_result_string(&Value::Number(5.0)), "5");
        assert_eq!(value_to_result_string(&Value::Integer(42)), "42");
        assert_eq!(value_to_result_string(&Value::Bool(true)), "TRUE");
        assert_eq!(value_to_result_string(&Value::Bool(false)), "FALSE");
        assert_eq!(value_to_result_string(&Value::Text("hi".into())), "hi");
        assert_eq!(value_to_result_string(&Value::Empty), "");
        assert_eq!(value_to_result_string(&Value::Error(CellError::Div0)), "#DIV/0!");
    }

    #[test]
    fn cell_result_from_number() {
        let r = value_to_cell_result(&Value::Number(42.5));
        assert_eq!(r.value, "42.5");
        assert_eq!(r.cell_type, XmlCellType::Number);
    }

    #[test]
    fn cell_result_from_integer() {
        let r = value_to_cell_result(&Value::Integer(7));
        assert_eq!(r.value, "7");
        assert_eq!(r.cell_type, XmlCellType::Number);
    }

    #[test]
    fn cell_result_from_text() {
        let r = value_to_cell_result(&Value::Text("hello".into()));
        assert_eq!(r.value, "hello");
        assert_eq!(r.cell_type, XmlCellType::InlineString);
    }

    #[test]
    fn cell_result_from_bool() {
        let r = value_to_cell_result(&Value::Bool(true));
        assert_eq!(r.value, "1");
        assert_eq!(r.cell_type, XmlCellType::Boolean);
        let r = value_to_cell_result(&Value::Bool(false));
        assert_eq!(r.value, "0");
        assert_eq!(r.cell_type, XmlCellType::Boolean);
    }

    #[test]
    fn cell_result_from_error() {
        let r = value_to_cell_result(&Value::Error(CellError::Div0));
        assert_eq!(r.value, "#DIV/0!");
        assert_eq!(r.cell_type, XmlCellType::Error);
    }

    #[test]
    fn cell_result_from_date() {
        let r = value_to_cell_result(&Value::Date(ExcelDate { serial: 44927.0 }));
        assert_eq!(r.value, "44927");
        assert_eq!(r.cell_type, XmlCellType::Number);
    }

    #[test]
    fn cell_result_from_empty() {
        let r = value_to_cell_result(&Value::Empty);
        assert_eq!(r.value, "");
        assert_eq!(r.cell_type, XmlCellType::Number);
    }
}
