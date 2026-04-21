//! The [`Reader`] — calamine-backed xlsx reader that yields streaming
//! cell and formula iterators.

use std::io::BufReader;
use std::path::Path;

use calamine::Reader as _;
use xlstream_core::{Value, XlStreamError};

use crate::convert::convert_data_ref;
use crate::stream::{CellSource, CellStream};

/// Streaming xlsx reader backed by [`calamine::Xlsx`]. Wraps the workbook
/// and exposes sheet enumeration, cell streaming, and formula extraction.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_io::Reader;
///
/// let reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
/// let names = reader.sheet_names();
/// assert!(!names.is_empty());
/// ```
pub struct Reader {
    workbook: calamine::Xlsx<BufReader<std::fs::File>>,
}

impl std::fmt::Debug for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader").finish_non_exhaustive()
    }
}

/// A [`CellSource`] implementation backed by a closure. This allows us to
/// erase calamine's `XlsxCellReader` type (which is not publicly re-exported)
/// without naming it.
struct ClosureCellSource<F>(F);

impl<F> CellSource for ClosureCellSource<F>
where
    F: FnMut() -> Result<Option<(u32, u32, Value)>, XlStreamError>,
{
    fn next_cell(&mut self) -> Result<Option<(u32, u32, Value)>, XlStreamError> {
        (self.0)()
    }
}

impl Reader {
    /// Open an xlsx file for streaming reads.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::Xlsx`] if calamine cannot parse the file
    /// (corrupt archive, missing internal XML, etc.).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Reader;
    ///
    /// let reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
    /// ```
    pub fn open(path: &Path) -> Result<Self, XlStreamError> {
        let workbook = calamine::open_workbook::<calamine::Xlsx<BufReader<std::fs::File>>, _>(path)
            .map_err(|e| XlStreamError::Xlsx(e.to_string()))?;
        Ok(Self { workbook })
    }

    /// Return the names of all sheets in workbook order.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Reader;
    ///
    /// let reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
    /// for name in reader.sheet_names() {
    ///     println!("{name}");
    /// }
    /// ```
    #[must_use]
    pub fn sheet_names(&self) -> Vec<String> {
        self.workbook.sheet_names().clone()
    }

    /// Return all workbook-level defined names as `(name, value)` pairs.
    ///
    /// Values are raw strings from the xlsx `<definedNames>` element,
    /// e.g. `"Sheet1!$A$1:$A$100"` or `"0.15"`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Reader;
    ///
    /// let reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
    /// for (name, value) in reader.defined_names() {
    ///     println!("{name} = {value}");
    /// }
    /// ```
    #[must_use]
    pub fn defined_names(&self) -> Vec<(String, String)> {
        self.workbook.defined_names().to_vec()
    }

    /// Open a streaming cell reader for the named sheet. Cells are yielded
    /// row-by-row via [`CellStream::next_row`]. Each call opens a fresh
    /// read from the start of the sheet.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::Xlsx`] if the sheet does not exist or
    /// calamine encounters a parse error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Reader;
    ///
    /// let mut reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
    /// let mut stream = reader.cells("Sheet1").unwrap();
    /// while let Some(row) = stream.next_row().unwrap() {
    ///     println!("{row:?}");
    /// }
    /// ```
    pub fn cells(&mut self, sheet: &str) -> Result<CellStream<'_>, XlStreamError> {
        let mut cell_reader = self
            .workbook
            .worksheet_cells_reader(sheet)
            .map_err(|e| XlStreamError::Xlsx(e.to_string()))?;
        let dims = cell_reader.dimensions();
        let capacity_hint = (dims.end.1 as usize) + 1;

        // Capture the cell reader in a closure. This erases the concrete
        // XlsxCellReader type (which calamine does not publicly export)
        // behind the CellSource trait.
        let source = ClosureCellSource(move || match cell_reader.next_cell() {
            Ok(Some(cell)) => {
                let (row, col) = cell.get_position();
                let value = convert_data_ref(cell.get_value());
                Ok(Some((row, col, value)))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(XlStreamError::Xlsx(e.to_string())),
        });

        Ok(CellStream::new(Box::new(source), capacity_hint))
    }

    /// Collect all formula cells for the named sheet. Returns a vec of
    /// `(row, col, formula_text)` tuples. Only cells that contain a
    /// non-empty formula are included. Each call opens a fresh read
    /// from the start of the sheet.
    ///
    /// Formulas are typically sparse, so eagerly collecting them is
    /// acceptable for memory.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::Xlsx`] if the sheet does not exist or
    /// calamine encounters a parse error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Reader;
    ///
    /// let mut reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
    /// let formulas = reader.formulas("Sheet1").unwrap();
    /// for (row, col, text) in &formulas {
    ///     println!("({row}, {col}): {text}");
    /// }
    /// ```
    pub fn formulas(&mut self, sheet: &str) -> Result<Vec<(u32, u32, String)>, XlStreamError> {
        let mut cell_reader = self
            .workbook
            .worksheet_cells_reader(sheet)
            .map_err(|e| XlStreamError::Xlsx(e.to_string()))?;
        let mut out = Vec::new();
        loop {
            match cell_reader.next_formula() {
                Ok(Some(cell)) => {
                    let (row, col) = cell.get_position();
                    let text = cell.get_value().clone();
                    if !text.is_empty() {
                        out.push((row, col, text));
                    }
                }
                Ok(None) => break,
                Err(e) => return Err(XlStreamError::Xlsx(e.to_string())),
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::path::Path;

    use super::*;

    #[test]
    fn open_nonexistent_file_returns_xlsx_error() {
        let err = Reader::open(Path::new("doesnt-exist.xlsx")).unwrap_err();
        assert!(matches!(err, XlStreamError::Xlsx(_)), "expected Xlsx error, got {err:?}",);
    }

    #[test]
    fn defined_names_returns_empty_for_plain_workbook() {
        let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
        let mut wb = rust_xlsxwriter::Workbook::new();
        wb.save(tmp.path()).unwrap();

        let reader = Reader::open(tmp.path()).unwrap();
        assert!(reader.defined_names().is_empty());
    }
}
