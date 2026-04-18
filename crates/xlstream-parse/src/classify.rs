//! Classification — the verdict a formula gets before evaluation. Tells the
//! evaluator whether a formula can be streamed, needs prelude-only data, or
//! must be refused.

use crate::ast::Ast;

/// The specific reason a formula was rejected.
///
/// Each variant maps to a `&'static str` doc link via [`Self::doc_link`]
/// so the user-facing error message can deep-link to the explanation.
///
/// # Examples
///
/// ```
/// use xlstream_parse::UnsupportedReason;
/// let r = UnsupportedReason::UnsupportedFunction("OFFSET".into());
/// assert!(r.to_string().contains("OFFSET"));
/// assert!(r.doc_link().starts_with("https://"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsupportedReason {
    /// Reference to a row other than the current streaming row.
    NonCurrentRowRef,
    /// Cell references itself (directly or transitively).
    CircularRef,
    /// Function not in any of the supported sets.
    UnsupportedFunction(String),
    /// Bare `A:A` (or `1:1`) outside an aggregate.
    UnboundedRange,
    /// Aggregate criteria computed per-row are not supported.
    NonStaticCriteria,
    /// Dynamic-array spill (`FILTER`, `UNIQUE`, ...).
    DynamicArray,
    /// Volatile function not in [`crate::sets::VOLATILE_STREAMING_OK`].
    VolatileUnsupported,
    /// `[Book.xlsx]Sheet1!A1`-style external workbook reference.
    ExternalReference,
    /// `Table[Column]`-style structured table reference.
    TableReference,
    /// `MyRange`-style workbook-level named range.
    NamedRange,
    /// Sub-expression nested under another unsupported sub-expression.
    NestedUnsupported,
    /// Lookup range points at a sheet the prelude has not indexed.
    LookupSheetNotPrepared,
    /// Lookup range points at the main streaming sheet.
    LookupIntoStreamingSheet,
}

impl std::fmt::Display for UnsupportedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonCurrentRowRef => {
                write!(f, "references a row other than the current one — unstreamable")
            }
            Self::CircularRef => write!(f, "circular reference"),
            Self::UnsupportedFunction(name) => write!(f, "function {name} is not supported"),
            Self::UnboundedRange => {
                write!(f, "whole-column or whole-row reference outside an aggregate")
            }
            Self::NonStaticCriteria => {
                write!(f, "aggregate criteria computed per-row are not supported")
            }
            Self::DynamicArray => write!(f, "dynamic-array spill is not supported"),
            Self::VolatileUnsupported => {
                write!(f, "volatile function is not in the streaming-OK set")
            }
            Self::ExternalReference => {
                write!(f, "external-workbook references are not supported (single-file model)")
            }
            Self::TableReference => {
                write!(f, "structured table references are not supported in v0.1")
            }
            Self::NamedRange => write!(f, "named ranges are not supported in v0.1"),
            Self::NestedUnsupported => write!(f, "contains an unsupported sub-expression"),
            Self::LookupSheetNotPrepared => {
                write!(f, "lookup range points at a sheet the prelude has not indexed")
            }
            Self::LookupIntoStreamingSheet => {
                write!(f, "lookup range points at the main streaming sheet")
            }
        }
    }
}

impl UnsupportedReason {
    /// Stable documentation URL for this refusal.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::UnsupportedReason;
    /// assert!(UnsupportedReason::NonCurrentRowRef.doc_link().starts_with("https://"));
    /// ```
    #[must_use]
    pub fn doc_link(&self) -> &'static str {
        match self {
            Self::NonCurrentRowRef | Self::CircularRef => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#classification-algorithm",
            Self::UnsupportedFunction(_) | Self::DynamicArray | Self::VolatileUnsupported => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#why-offset-and-indirect-are-unsupported",
            Self::UnboundedRange => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#aggregate-of-a-column",
            Self::NonStaticCriteria => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#aggregate-pre-pass",
            Self::LookupSheetNotPrepared => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#lookup-index-pre-pass",
            Self::LookupIntoStreamingSheet => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#lookup-into-loaded-sheet",
            Self::ExternalReference => "https://github.com/cilladev/xlstream/blob/main/MANAGER.md#permanent-exclusions",
            Self::TableReference | Self::NamedRange => "https://github.com/cilladev/xlstream/blob/main/docs/backlog/v0.2.md",
            Self::NestedUnsupported => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md",
        }
    }
}

/// The verdict returned by [`classify`] for a single formula.
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
    /// Formula cannot be streamed.
    Unsupported {
        /// Specific reason for refusal.
        reason: UnsupportedReason,
        /// Stable documentation URL explaining the refusal class.
        doc_link: &'static str,
    },
}

/// Context passed to [`classify`]. Real fields land in Chunk 3.
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

/// Classify a parsed formula. Stub: always returns
/// [`Classification::Unsupported`]. Real implementation lands in Chunk 3.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{classify, parse, Classification, ClassificationContext};
/// let ast = parse("1+2").unwrap();
/// let ctx = ClassificationContext::default();
/// assert!(matches!(classify(&ast, &ctx), Classification::Unsupported { .. }));
/// ```
#[must_use]
pub fn classify(_ast: &Ast, _ctx: &ClassificationContext) -> Classification {
    let reason = UnsupportedReason::NestedUnsupported;
    let doc_link = reason.doc_link();
    Classification::Unsupported { reason, doc_link }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn classify_returns_unsupported_stub() {
        let ast = crate::parse("1+2").unwrap();
        let ctx = ClassificationContext::default();
        match classify(&ast, &ctx) {
            Classification::Unsupported { reason, doc_link } => {
                assert!(matches!(reason, UnsupportedReason::NestedUnsupported));
                assert!(doc_link.starts_with("https://"));
            }
            other => panic!("expected Unsupported, got {other:?}"),
        }
    }

    #[test]
    fn classification_variants_compare_equal() {
        assert_eq!(Classification::RowLocal, Classification::RowLocal);
        assert_ne!(Classification::RowLocal, Classification::AggregateOnly);
    }

    #[test]
    fn unsupported_reason_renders_human_message() {
        let r = UnsupportedReason::UnsupportedFunction("OFFSET".into());
        let s = r.to_string();
        assert!(s.contains("OFFSET"), "expected OFFSET in message: {s}");
    }

    #[test]
    fn unsupported_reason_doc_link_is_stable_url() {
        let r = UnsupportedReason::NonCurrentRowRef;
        assert!(r.doc_link().starts_with("https://"));
    }

    #[test]
    fn lookup_into_streaming_sheet_has_distinct_doc_link() {
        let a = UnsupportedReason::LookupIntoStreamingSheet;
        let b = UnsupportedReason::LookupSheetNotPrepared;
        assert_ne!(
            a.doc_link(),
            b.doc_link(),
            "distinct lookup-failure modes should deep-link to distinct sections"
        );
    }
}
