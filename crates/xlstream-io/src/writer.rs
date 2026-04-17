//! The [`Writer`] stub — `rust_xlsxwriter`-backed xlsx writer. Real logic
//! lands in Phase 3.

use std::path::Path;

use xlstream_core::XlStreamError;

/// Streaming xlsx writer. Phase 1 ships a unit-struct stub; Phase 3 wires
/// it to a `rust_xlsxwriter::Workbook` in constant-memory mode.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use xlstream_io::Writer;
/// let err = Writer::create(Path::new("out.xlsx")).unwrap_err();
/// assert!(err.to_string().contains("unimplemented"));
/// ```
#[derive(Debug)]
pub struct Writer {
    _private: (),
}

impl Writer {
    /// Create a new xlsx file ready to accept streamed rows.
    ///
    /// # Errors
    ///
    /// Phase 1: always [`XlStreamError::Internal`]. Phase 3 onward: real
    /// errors surfaced as [`XlStreamError::Io`] / `XlStreamError::XlsxWrite`
    /// (variant added in Phase 3).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use xlstream_io::Writer;
    /// assert!(Writer::create(Path::new("x.xlsx")).is_err());
    /// ```
    pub fn create(_path: &Path) -> Result<Self, XlStreamError> {
        Err(XlStreamError::Internal("unimplemented: Writer::create — lands in Phase 3".into()))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::path::Path;

    use super::*;

    #[test]
    fn create_returns_unimplemented_internal_error() {
        let err = Writer::create(Path::new("out.xlsx")).unwrap_err();
        assert!(
            matches!(err, xlstream_core::XlStreamError::Internal(ref msg) if msg.contains("unimplemented")),
            "expected Internal(unimplemented...), got {err:?}",
        );
    }
}
