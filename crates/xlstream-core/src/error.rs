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
    #[error("formula parse error at {address}: {message}\n  formula: {formula}")]
    FormulaParse {
        /// The cell address that held the formula.
        address: String,
        /// The original formula text.
        formula: String,
        /// Parser-supplied diagnostic.
        message: String,
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
}
