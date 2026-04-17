//! Classification — the verdict a formula gets before evaluation. Tells the
//! evaluator whether a formula can be streamed, needs prelude-only data, or
//! must be refused.

use crate::ast::Ast;

/// The verdict returned by [`classify`] for a single formula.
///
/// Phase 1 ships this enum shape; Phase 2 replaces `Unsupported(String)`
/// with a structured reason type (see `docs/phases/phase-02-parser.md`).
///
/// # Examples
///
/// ```
/// use xlstream_parse::Classification;
/// let c = Classification::RowLocal;
/// assert_eq!(c, Classification::RowLocal);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Classification {
    /// Formula reads only the current row; streamable without prelude.
    RowLocal,
    /// Formula reads only prelude-computed aggregates.
    AggregateOnly,
    /// Formula reads only prelude-loaded lookup sheets.
    LookupOnly,
    /// Formula mixes row-local, aggregate, and/or lookup reads — still
    /// streamable via prelude + row data.
    Mixed,
    /// Formula cannot be streamed. The string explains why.
    Unsupported(String),
}

/// Context passed to [`classify`]. Phase 1 is a placeholder; Phase 2 fills
/// it with workbook metadata (sheet names, header maps, prelude plans).
///
/// # Examples
///
/// ```
/// use xlstream_parse::ClassificationContext;
/// let _ctx = ClassificationContext::default();
/// ```
#[derive(Debug, Default)]
pub struct ClassificationContext {
    _private: (),
}

/// Classify a parsed formula. Phase 1 always returns
/// [`Classification::Unsupported`] with a note that real classification
/// lands in Phase 2.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{classify, Ast, Classification, ClassificationContext};
/// let ast = Ast::default();
/// let ctx = ClassificationContext::default();
/// matches!(classify(&ast, &ctx), Classification::Unsupported(_));
/// ```
#[must_use]
pub fn classify(_ast: &Ast, _ctx: &ClassificationContext) -> Classification {
    Classification::Unsupported("classification not implemented until Phase 2".into())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::Ast;

    #[test]
    fn classify_returns_unsupported_stub() {
        let ast = Ast::default();
        let ctx = ClassificationContext::default();
        match classify(&ast, &ctx) {
            Classification::Unsupported(msg) => {
                assert!(msg.contains("Phase 2"), "expected Phase 2 note, got: {msg}");
            }
            other => panic!("expected Unsupported, got {other:?}"),
        }
    }

    #[test]
    fn classification_variants_compare_equal() {
        assert_eq!(Classification::RowLocal, Classification::RowLocal);
        assert_ne!(Classification::RowLocal, Classification::AggregateOnly);
    }
}
