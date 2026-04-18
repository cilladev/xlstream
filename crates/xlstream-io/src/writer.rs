//! The [`Writer`] — `rust_xlsxwriter`-backed xlsx writer in constant-memory
//! mode. Writes are streamed row-by-row; the workbook is flushed on
//! [`Writer::finish`].

use std::path::{Path, PathBuf};

use rust_xlsxwriter::Workbook;
use xlstream_core::XlStreamError;

use crate::sheet_handle::SheetHandle;

/// Streaming xlsx writer backed by [`rust_xlsxwriter::Workbook`] in
/// constant-memory mode. Rows must be written in strictly-increasing order
/// per sheet — the [`SheetHandle`] returned by [`Writer::add_sheet`]
/// enforces this at runtime.
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
/// sh.write_row(0, &[Value::Number(1.0), Value::Text("hello".into())]).unwrap();
/// drop(sh);
/// w.finish().unwrap();
/// ```
pub struct Writer {
    workbook: Workbook,
    path: PathBuf,
}

impl std::fmt::Debug for Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Writer").field("path", &self.path).finish_non_exhaustive()
    }
}

impl Writer {
    /// Create a new xlsx file ready to accept streamed rows.
    ///
    /// The file at `path` is not written until [`Writer::finish`] is called.
    ///
    /// # Errors
    ///
    /// Currently infallible, but returns `Result` for forward-compat with
    /// path validation or directory-creation logic.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Writer;
    ///
    /// let w = Writer::create(Path::new("out.xlsx")).unwrap();
    /// ```
    pub fn create(path: &Path) -> Result<Self, XlStreamError> {
        let workbook = Workbook::new();
        Ok(Self { workbook, path: path.to_path_buf() })
    }

    /// Add a new worksheet in constant-memory mode and return a
    /// [`SheetHandle`] for writing rows. The handle borrows `self` mutably,
    /// so only one sheet can be written at a time — drop or rebind the
    /// handle before adding another sheet.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::XlsxWrite`] if `rust_xlsxwriter` rejects the
    /// sheet name (too long, duplicate, invalid characters, etc.).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Writer;
    ///
    /// let mut w = Writer::create(Path::new("out.xlsx")).unwrap();
    /// let sh = w.add_sheet("Data").unwrap();
    /// drop(sh);
    /// ```
    pub fn add_sheet(&mut self, name: &str) -> Result<SheetHandle<'_>, XlStreamError> {
        let ws = self.workbook.add_worksheet_with_constant_memory();
        ws.set_name(name).map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
        Ok(SheetHandle::new(ws))
    }

    /// Flush all buffered data and write the xlsx file to disk.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::XlsxWrite`] if `rust_xlsxwriter` fails to
    /// serialise or write the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use xlstream_io::Writer;
    ///
    /// let w = Writer::create(Path::new("out.xlsx")).unwrap();
    /// w.finish().unwrap();
    /// ```
    pub fn finish(mut self) -> Result<(), XlStreamError> {
        self.workbook.save(&self.path).map_err(|e| XlStreamError::XlsxWrite(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::path::Path;

    use super::*;

    #[test]
    fn create_returns_ok() {
        let w = Writer::create(Path::new("out.xlsx"));
        assert!(w.is_ok());
    }

    #[test]
    fn debug_includes_path() {
        let w = Writer::create(Path::new("test.xlsx")).unwrap();
        let dbg = format!("{w:?}");
        assert!(dbg.contains("test.xlsx"), "debug missing path: {dbg}");
    }
}
