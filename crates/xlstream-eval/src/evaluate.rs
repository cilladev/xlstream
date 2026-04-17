//! The [`evaluate`] entry point and [`EvaluateSummary`] return type. Phase
//! 1 stubs only; real streaming logic lands in Phase 4.

use std::path::Path;

use xlstream_core::XlStreamError;

/// Summary of a completed evaluation. Returned by [`evaluate`] once the
/// whole workbook has been streamed through.
///
/// # Examples
///
/// ```
/// use xlstream_eval::EvaluateSummary;
/// let s = EvaluateSummary::default();
/// assert_eq!(s.rows_processed, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvaluateSummary {
    /// Number of rows processed across every sheet (sum).
    pub rows_processed: u32,
    /// Wall-clock duration of the evaluation, in milliseconds.
    pub duration_ms: u64,
    /// Peak resident-set size observed during the run, in bytes.
    pub peak_rss_bytes: u64,
}

/// Evaluate every formula in `input`, write the results to `output`, and
/// return an [`EvaluateSummary`].
///
/// `workers` selects the level of row-level parallelism: `None` lets the
/// engine auto-choose (typically the rayon global pool); `Some(n)` pins
/// to `n` workers. Phase 1 ignores the argument; Phase 10 wires it up.
///
/// # Errors
///
/// Phase 1: always [`XlStreamError::Internal`]. Phase 4 onward: any of the
/// library-level error variants.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use xlstream_eval::evaluate;
/// let err = evaluate(Path::new("in.xlsx"), Path::new("out.xlsx"), None).unwrap_err();
/// assert!(err.to_string().contains("unimplemented"));
/// ```
pub fn evaluate(
    _input: &Path,
    _output: &Path,
    _workers: Option<usize>,
) -> Result<EvaluateSummary, XlStreamError> {
    tracing::trace!("xlstream-eval::evaluate stub called");
    Err(XlStreamError::Internal("unimplemented: evaluate — lands in Phase 4".into()))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::path::Path;

    use super::*;

    #[test]
    fn evaluate_returns_unimplemented_internal_error() {
        let err = evaluate(Path::new("in.xlsx"), Path::new("out.xlsx"), None).unwrap_err();
        assert!(
            matches!(err, xlstream_core::XlStreamError::Internal(ref msg) if msg.contains("unimplemented")),
            "expected Internal(unimplemented...), got {err:?}",
        );
    }

    #[test]
    fn summary_fields_default_to_zero() {
        let s = EvaluateSummary::default();
        assert_eq!(s.rows_processed, 0);
        assert_eq!(s.duration_ms, 0);
        assert_eq!(s.peak_rss_bytes, 0);
    }
}
