//! The [`SheetHandle`] — a mutable reference to a `rust_xlsxwriter::Worksheet`
//! with row-order enforcement for constant-memory mode.

use std::collections::HashMap;

use rust_xlsxwriter::{Format, Formula, Worksheet};
use xlstream_core::{Value, XlStreamError};

use crate::convert::{cell_error_to_excel_string, value_to_result_string};

/// Wraps a mutable reference to a [`Worksheet`] and enforces strictly-increasing
/// row indices, which is required by `rust_xlsxwriter`'s constant-memory mode.
///
/// Obtain a handle via [`Writer::add_sheet`](crate::Writer::add_sheet).
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_core::Value;
/// use xlstream_io::Writer;
///
/// let mut w = Writer::create(Path::new("out.xlsx")).unwrap();
/// let mut sh = w.add_sheet("Data").unwrap();
/// sh.write_row(0, &[Value::Number(1.0), Value::Text("hi".into())]).unwrap();
/// ```
pub struct SheetHandle<'a> {
    worksheet: &'a mut Worksheet,
    last_row: Option<u32>,
    date_format: Format,
}

impl<'a> SheetHandle<'a> {
    /// Create a new handle wrapping the given worksheet.
    pub(crate) fn new(worksheet: &'a mut Worksheet) -> Self {
        let date_format = Format::new().set_num_format("yyyy-mm-dd");
        Self { worksheet, last_row: None, date_format }
    }

    /// Write a row of values at the given `row_idx`.
    ///
    /// Row indices must be strictly increasing — writing row 3 after row 5
    /// returns [`XlStreamError::Internal`]. This pre-check prevents
    /// `rust_xlsxwriter`'s `RowColumnOrderError` from firing in
    /// constant-memory mode.
    ///
    /// # Errors
    ///
    /// - [`XlStreamError::Internal`] if `row_idx` is not strictly greater than
    ///   the previous row written.
    /// - [`XlStreamError::XlsxWrite`] if the underlying write fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_core::Value;
    /// use xlstream_io::Writer;
    ///
    /// let mut w = Writer::create(Path::new("out.xlsx")).unwrap();
    /// let mut sh = w.add_sheet("Sheet1").unwrap();
    /// sh.write_row(0, &[Value::Number(42.0)]).unwrap();
    /// sh.write_row(1, &[Value::Text("ok".into())]).unwrap();
    /// ```
    pub fn write_row(&mut self, row_idx: u32, values: &[Value]) -> Result<(), XlStreamError> {
        self.enforce_row_order(row_idx)?;

        for (col_offset, val) in values.iter().enumerate() {
            let col = u16::try_from(col_offset).map_err(|_| {
                XlStreamError::Internal(format!("column index {col_offset} exceeds u16::MAX"))
            })?;
            self.write_value(row_idx, col, val)?;
        }
        Ok(())
    }

    /// Write a formula cell with a cached result value.
    ///
    /// The cached result is embedded so that applications that don't
    /// recalculate (e.g. preview tools) show the correct value.
    ///
    /// # Errors
    ///
    /// - [`XlStreamError::XlsxWrite`] if the underlying write fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_core::Value;
    /// use xlstream_io::Writer;
    ///
    /// let mut w = Writer::create(Path::new("out.xlsx")).unwrap();
    /// let mut sh = w.add_sheet("Sheet1").unwrap();
    /// sh.write_row(0, &[Value::Number(1.0), Value::Number(2.0)]).unwrap();
    /// sh.write_formula(0, 2, "=A1+B1", &Value::Number(3.0)).unwrap();
    /// ```
    pub fn write_formula(
        &mut self,
        row: u32,
        col: u16,
        formula: &str,
        cached: &Value,
    ) -> Result<(), XlStreamError> {
        let cached_str = value_to_result_string(cached);
        let f = Formula::new(formula).set_result(&cached_str);
        self.worksheet
            .write_formula(row, col, f)
            .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
        Ok(())
    }

    /// Write a row of values, substituting formula cells where specified.
    ///
    /// `formulas` maps column indices to formula text. Columns present in
    /// `formulas` are written as formula cells with the corresponding value
    /// from `values` as the cached result. All other columns are written as
    /// plain values.
    ///
    /// Row indices must be strictly increasing — same constraint as
    /// [`write_row`].
    ///
    /// # Errors
    ///
    /// - [`XlStreamError::Internal`] if `row_idx` violates row order or a
    ///   column index exceeds `u16::MAX`.
    /// - [`XlStreamError::XlsxWrite`] if the underlying write fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::collections::HashMap;
    /// use std::path::Path;
    /// use xlstream_core::Value;
    /// use xlstream_io::Writer;
    ///
    /// let mut w = Writer::create(Path::new("out.xlsx")).unwrap();
    /// let mut sh = w.add_sheet("Sheet1").unwrap();
    /// let values = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
    /// let mut formulas = HashMap::new();
    /// formulas.insert(2u16, "A1+B1");
    /// sh.write_row_with_formulas(0, &values, &formulas).unwrap();
    /// ```
    pub fn write_row_with_formulas(
        &mut self,
        row_idx: u32,
        values: &[Value],
        formulas: &HashMap<u16, &str>,
    ) -> Result<(), XlStreamError> {
        self.enforce_row_order(row_idx)?;

        for (col_offset, val) in values.iter().enumerate() {
            let col = u16::try_from(col_offset).map_err(|_| {
                XlStreamError::Internal(format!("column index {col_offset} exceeds u16::MAX"))
            })?;
            if let Some(formula_text) = formulas.get(&col) {
                if can_round_trip_as_formula(val) {
                    let cached_str = value_to_result_string(val);
                    let f = Formula::new(formula_text).set_result(&cached_str);
                    self.worksheet
                        .write_formula(row_idx, col, f)
                        .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
                } else {
                    tracing::warn!(
                        row = row_idx,
                        col = col,
                        "formula discarded: cached value type ({val:?}) cannot round-trip through set_result()"
                    );
                    self.write_value(row_idx, col, val)?;
                }
            } else {
                self.write_value(row_idx, col, val)?;
            }
        }
        Ok(())
    }

    /// Enforce strictly-increasing row order.
    fn enforce_row_order(&mut self, row_idx: u32) -> Result<(), XlStreamError> {
        if let Some(last) = self.last_row {
            if row_idx <= last {
                return Err(XlStreamError::Internal(format!(
                    "row index {row_idx} is not strictly greater than last written row {last}"
                )));
            }
        }
        self.last_row = Some(row_idx);
        Ok(())
    }

    /// Dispatch a single [`Value`] to the appropriate `rust_xlsxwriter` write
    /// method.
    fn write_value(&mut self, row: u32, col: u16, val: &Value) -> Result<(), XlStreamError> {
        match val {
            Value::Number(n) => {
                self.worksheet
                    .write_number(row, col, *n)
                    .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
            }
            Value::Integer(i) => {
                #[allow(clippy::cast_precision_loss)]
                let n = *i as f64;
                self.worksheet
                    .write_number(row, col, n)
                    .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
            }
            Value::Text(s) => {
                self.worksheet
                    .write_string(row, col, &**s)
                    .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
            }
            Value::Bool(b) => {
                self.worksheet
                    .write_boolean(row, col, *b)
                    .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
            }
            Value::Date(d) => {
                self.worksheet
                    .write_number_with_format(row, col, d.serial, &self.date_format)
                    .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
            }
            Value::Error(e) => {
                self.worksheet
                    .write_string(row, col, cell_error_to_excel_string(*e))
                    .map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
            }
            Value::Empty => { /* leave cell default */ }
        }
        Ok(())
    }
}

/// `Formula::set_result()` stores a raw string — calamine will parse it back
/// as a number for numeric-looking strings. Only write a formula cell when
/// the cached value type survives the round-trip.
fn can_round_trip_as_formula(val: &Value) -> bool {
    matches!(
        val,
        Value::Number(_) | Value::Integer(_) | Value::Bool(_) | Value::Date(_) | Value::Empty
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use xlstream_core::{CellError, ExcelDate, Value};

    use super::can_round_trip_as_formula;

    #[test]
    fn can_round_trip_accepts_numeric_types() {
        assert!(can_round_trip_as_formula(&Value::Number(42.0)));
        assert!(can_round_trip_as_formula(&Value::Integer(7)));
        assert!(can_round_trip_as_formula(&Value::Bool(true)));
        assert!(can_round_trip_as_formula(&Value::Date(ExcelDate { serial: 45000.0 })));
        assert!(can_round_trip_as_formula(&Value::Empty));
    }

    #[test]
    fn can_round_trip_rejects_text_and_error() {
        assert!(!can_round_trip_as_formula(&Value::Text("hello".into())));
        assert!(!can_round_trip_as_formula(&Value::Error(CellError::Na)));
    }
}
