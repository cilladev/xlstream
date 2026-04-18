//! Value conversion between calamine types and xlstream [`Value`].

use calamine::CellErrorType;
use xlstream_core::{CellError, ExcelDate, Value};

/// Convert a calamine [`DataRef`] to an xlstream [`Value`].
#[allow(dead_code)]
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
#[allow(dead_code)]
pub(crate) fn cell_error_to_excel_string(e: CellError) -> &'static str {
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
#[allow(dead_code)]
pub(crate) fn value_to_result_string(val: &Value) -> String {
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
}
