//! The [`XlStreamError`] enum — the library-level error type returned by
//! every fallible public API in the xlstream workspace.

use std::path::PathBuf;

/// Library-level errors. Cell-level errors (`#DIV/0!`, `#VALUE!`, ...) live
/// on [`crate::CellError`]; this type is for failures that stop evaluation
/// rather than becoming a cell value.
///
/// See [`docs/architecture/errors.md`] for the full taxonomy and the
/// panic / logging policy.
///
/// [`docs/architecture/errors.md`]: https://github.com/cilladev/xlstream/blob/main/docs/architecture/errors.md
///
/// # Examples
///
/// ```
/// use xlstream_core::XlStreamError;
/// let e = XlStreamError::Internal("stub".into());
/// assert!(e.to_string().contains("stub"));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum XlStreamError {
    /// Filesystem I/O failure (missing file, permission denied, etc.).
    #[error("I/O error reading {path}: {source}")]
    Io {
        /// The path that failed to open or read.
        path: PathBuf,
        /// The underlying [`std::io::Error`].
        #[source]
        source: std::io::Error,
    },

    /// An xlsx parse/read error from `calamine` (stringified at conversion
    /// site to keep xlstream-core free of I/O library deps).
    #[error("xlsx error: {0}")]
    Xlsx(String),

    /// An xlsx write error from `rust_xlsxwriter` (stringified at conversion
    /// site).
    #[error("xlsx write error: {0}")]
    XlsxWrite(String),

    /// A parsed formula could not be interpreted as a supported shape.
    #[error("unsupported formula at {address}: {reason}\n  formula: {formula}\n  see: {doc_link}")]
    Unsupported {
        /// The cell address that held the formula (e.g. `Sheet1!A1`).
        address: String,
        /// The original formula text.
        formula: String,
        /// Human-readable reason the formula cannot be streamed.
        reason: String,
        /// Documentation URL covering the refusal.
        doc_link: &'static str,
    },

    /// The formula parser rejected the input as malformed.
    #[error("formula parse error at {address}{}: {message}\n  formula: {formula}",
        position.map_or(String::new(), |p| format!(" (position {p})")))]
    FormulaParse {
        /// The cell address that held the formula.
        address: String,
        /// The original formula text.
        formula: String,
        /// Parser-supplied diagnostic.
        message: String,
        /// Byte offset into `formula` where the parser failed, if upstream
        /// reported one.
        position: Option<usize>,
    },

    /// The classifier could not assign a [`Classification`] to a formula.
    ///
    /// [`Classification`]: https://docs.rs/xlstream-parse/latest/xlstream_parse/enum.Classification.html
    #[error("classification error at {address}: {message}")]
    Classification {
        /// The cell address that held the formula.
        address: String,
        /// Human-readable diagnostic.
        message: String,
    },

    /// A set of cells forms a dependency cycle.
    #[error("circular reference involving {cells:?}")]
    CircularReference {
        /// The cells involved in the cycle.
        cells: Vec<String>,
    },

    /// An invariant the library itself was supposed to uphold has been
    /// violated. Always a bug; never caused by bad user input.
    #[error("internal invariant violation: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn internal_variant_formats_with_message() {
        let e = XlStreamError::Internal("oops".into());
        assert_eq!(e.to_string(), "internal invariant violation: oops");
    }

    #[test]
    fn formula_parse_includes_position_in_message_when_present() {
        let e = XlStreamError::FormulaParse {
            address: "Sheet1!A1".into(),
            formula: "SUM(A1:".into(),
            message: "expected closing paren".into(),
            position: Some(7),
        };
        let msg = e.to_string();
        assert!(msg.contains("position 7"), "expected position in message: {msg}");
    }

    #[test]
    fn formula_parse_omits_position_when_absent() {
        let e = XlStreamError::FormulaParse {
            address: "Sheet1!A1".into(),
            formula: "SUM(".into(),
            message: "unexpected end of input".into(),
            position: None,
        };
        let msg = e.to_string();
        assert!(!msg.contains("position"), "did not expect 'position' in: {msg}");
    }

    #[test]
    fn unsupported_variant_mentions_address_and_formula() {
        let e = XlStreamError::Unsupported {
            address: "Sheet1!A1".into(),
            formula: "OFFSET(A1,1,0)".into(),
            reason: "OFFSET not streamable".into(),
            doc_link: "https://example.invalid/unsupported",
        };
        let msg = e.to_string();
        assert!(msg.contains("Sheet1!A1"), "address missing: {msg}");
        assert!(msg.contains("OFFSET(A1,1,0)"), "formula missing: {msg}");
        assert!(msg.contains("OFFSET not streamable"), "reason missing: {msg}");
    }

    #[test]
    fn xlsx_error_formats_with_source_message() {
        let e = XlStreamError::Xlsx("file not found: Sheet1".into());
        let msg = e.to_string();
        assert!(msg.contains("xlsx error"), "expected xlsx error prefix: {msg}");
        assert!(msg.contains("Sheet1"), "expected source message: {msg}");
    }

    #[test]
    fn xlsx_write_error_formats_with_source_message() {
        let e = XlStreamError::XlsxWrite("row order violated".into());
        let msg = e.to_string();
        assert!(msg.contains("xlsx write error"), "expected write prefix: {msg}");
        assert!(msg.contains("row order"), "expected source message: {msg}");
    }
}
