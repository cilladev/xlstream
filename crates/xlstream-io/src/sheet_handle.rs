//! The [`SheetHandle`] — a mutable reference to a `rust_xlsxwriter::Worksheet`
//! with row-order enforcement for constant-memory mode.

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
