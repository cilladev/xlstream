//! The [`parse`] entry point.

use xlstream_core::XlStreamError;

use crate::ast::Ast;

/// Parse an Excel formula expression into an [`Ast`].
///
/// The input must **not** include a leading `=`; that's an I/O concern,
/// stripped before the parser sees the text.
///
/// Phase 1 returns [`XlStreamError::Internal`] so callers do not
/// accidentally build on a pre-integration parser. The real implementation
/// lands in Phase 2.
///
/// # Errors
///
/// - Phase 1: always [`XlStreamError::Internal`].
/// - Phase 2 onward: [`XlStreamError::FormulaParse`] for malformed input.
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// let err = parse("SUM(A1:A10)").unwrap_err();
/// assert!(err.to_string().contains("unimplemented"));
/// ```
pub fn parse(_expr: &str) -> Result<Ast, XlStreamError> {
    Err(XlStreamError::Internal("unimplemented: parse — lands in Phase 2".into()))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn parse_returns_unimplemented_internal_error() {
        let err = parse("SUM(A1:A10)").unwrap_err();
        assert!(
            matches!(err, xlstream_core::XlStreamError::Internal(ref msg) if msg.contains("unimplemented")),
            "expected Internal(unimplemented...), got {err:?}",
        );
    }
}
