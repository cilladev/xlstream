//! Classification — the verdict a formula gets before evaluation. Tells the
//! evaluator whether a formula can be streamed, needs prelude-only data, or
//! must be refused.

use std::collections::{HashMap, HashSet};

use crate::ast::{Ast, Node};
use crate::references::Reference;
use crate::sets;

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
    /// Volatile function not in `crate::sets::VOLATILE_STREAMING_OK`.
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
    /// `Sheet1:Sheet3!A1`-style 3D sheet reference.
    ThreeDimensionalRef,
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
                write!(f, "table reference could not be resolved (unknown table or column)")
            }
            Self::NamedRange => write!(f, "named range not found in workbook"),
            Self::NestedUnsupported => write!(f, "contains an unsupported sub-expression"),
            Self::LookupSheetNotPrepared => {
                write!(f, "lookup range points at a sheet the prelude has not indexed")
            }
            Self::LookupIntoStreamingSheet => {
                write!(f, "lookup range points at the main streaming sheet")
            }
            Self::ThreeDimensionalRef => {
                write!(f, "3D sheet references are incompatible with streaming")
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
            Self::NonCurrentRowRef
            | Self::CircularRef
            | Self::LookupIntoStreamingSheet
            | Self::ExternalReference => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#classification-algorithm",
            Self::UnsupportedFunction(_) | Self::DynamicArray | Self::VolatileUnsupported => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#why-offset-and-indirect-are-unsupported",
            Self::UnboundedRange => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#aggregate-of-a-column",
            Self::NonStaticCriteria => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#aggregate-pre-pass",
            Self::LookupSheetNotPrepared => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md#lookup-index-pre-pass",
            Self::TableReference | Self::NamedRange => "https://github.com/cilladev/xlstream/blob/main/docs/roadmap/v0.2/README.md",
            Self::NestedUnsupported
            | Self::ThreeDimensionalRef => "https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md",
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
    /// Formula cannot be streamed. Use [`UnsupportedReason::doc_link`] for
    /// the stable documentation URL.
    Unsupported(UnsupportedReason),
}

/// Context passed to [`classify`]. Carries the cell address and workbook
/// metadata needed for streaming classification.
///
/// # Examples
///
/// ```
/// use xlstream_parse::ClassificationContext;
/// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
/// assert_eq!(ctx.current_sheet(), "Sheet1");
/// assert_eq!(ctx.current_row(), 1);
/// assert_eq!(ctx.current_col(), 1);
/// ```
#[derive(Debug)]
pub struct ClassificationContext {
    current_sheet: String,
    current_row: u32,
    current_col: u32,
    lookup_sheets: HashSet<String>,
    column_headers: HashMap<String, u32>,
}

impl ClassificationContext {
    /// Build a context positioned at `(sheet, row, col)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
    /// assert_eq!(ctx.current_sheet(), "Sheet1");
    /// ```
    #[must_use]
    pub fn for_cell(sheet: &str, row: u32, col: u32) -> Self {
        Self {
            current_sheet: sheet.to_owned(),
            current_row: row,
            current_col: col,
            lookup_sheets: HashSet::new(),
            column_headers: HashMap::new(),
        }
    }

    /// Register a sheet as pre-loaded for lookups.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1)
    ///     .with_lookup_sheet("Region Info");
    /// assert!(ctx.is_lookup_sheet("Region Info"));
    /// ```
    #[must_use]
    pub fn with_lookup_sheet(mut self, sheet: &str) -> Self {
        self.lookup_sheets.insert(sheet.to_ascii_lowercase());
        self
    }

    /// Register a column header mapping (reserved for v0.2).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1)
    ///     .with_header("Amount", 3);
    /// assert_eq!(ctx.headers().get("Amount"), Some(&3));
    /// ```
    #[must_use]
    pub fn with_header(mut self, header: &str, col: u32) -> Self {
        self.column_headers.insert(header.to_owned(), col);
        self
    }

    /// Name of the sheet being streamed.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Data", 1, 1);
    /// assert_eq!(ctx.current_sheet(), "Data");
    /// ```
    #[must_use]
    pub fn current_sheet(&self) -> &str {
        &self.current_sheet
    }

    /// 1-based row of the cell being classified.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 5, 1);
    /// assert_eq!(ctx.current_row(), 5);
    /// ```
    #[must_use]
    pub fn current_row(&self) -> u32 {
        self.current_row
    }

    /// 1-based column of the cell being classified.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 1, 7);
    /// assert_eq!(ctx.current_col(), 7);
    /// ```
    #[must_use]
    pub fn current_col(&self) -> u32 {
        self.current_col
    }

    /// `true` if `sheet` was registered via [`Self::with_lookup_sheet`].
    /// Case-insensitive comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1)
    ///     .with_lookup_sheet("Tax Rates");
    /// assert!(ctx.is_lookup_sheet("Tax Rates"));
    /// assert!(ctx.is_lookup_sheet("tax rates"));
    /// assert!(!ctx.is_lookup_sheet("Other"));
    /// ```
    #[must_use]
    pub fn is_lookup_sheet(&self, sheet: &str) -> bool {
        self.lookup_sheets.contains(&sheet.to_ascii_lowercase())
    }

    /// The set of registered lookup sheet names.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
    /// assert!(ctx.lookup_sheets().is_empty());
    /// ```
    #[must_use]
    pub fn lookup_sheets(&self) -> &HashSet<String> {
        &self.lookup_sheets
    }

    /// Column header mappings (reserved for v0.2).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::ClassificationContext;
    /// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
    /// assert!(ctx.headers().is_empty());
    /// ```
    #[must_use]
    pub fn headers(&self) -> &HashMap<String, u32> {
        &self.column_headers
    }
}

// ---------------------------------------------------------------------------
// Internal disposition types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Disposition {
    RowLocal,
    Aggregate,
    Lookup,
    Mixed,
    Unsupported(UnsupportedReason),
}

#[derive(Debug, Clone, Copy)]
enum FnKind {
    Aggregate,
    Lookup,
    RangeExpanding,
}

/// Classify a parsed formula for streaming evaluation.
///
/// Walks the AST to determine whether each sub-expression is row-local,
/// aggregate-only, lookup-only, mixed, or unsupported.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{classify, parse, Classification, ClassificationContext};
/// let ast = parse("1+2").unwrap();
/// let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
/// assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
/// ```
#[must_use]
pub fn classify(ast: &Ast, ctx: &ClassificationContext) -> Classification {
    let d = disposition(&ast.root, ctx, None);
    match d {
        Disposition::RowLocal => Classification::RowLocal,
        Disposition::Aggregate => Classification::AggregateOnly,
        Disposition::Lookup => Classification::LookupOnly,
        Disposition::Mixed => Classification::Mixed,
        Disposition::Unsupported(r) => Classification::Unsupported(r),
    }
}

fn disposition(node: &Node, ctx: &ClassificationContext, parent: Option<FnKind>) -> Disposition {
    match node {
        Node::Literal(_) | Node::Text(_) | Node::Error(_) | Node::PreludeRef(_) => {
            Disposition::RowLocal
        }
        Node::Reference(r) => classify_reference(r, ctx, parent),
        Node::UnaryOp { expr, .. } => disposition(expr, ctx, parent),
        Node::BinaryOp { left, right, .. } => {
            fold(disposition(left, ctx, parent), disposition(right, ctx, parent))
        }
        Node::Function { name, args } => classify_function(name, args, ctx),
        Node::Array(rows) => {
            let mut acc = Disposition::RowLocal;
            for row in rows {
                for cell in row {
                    acc = fold(acc, disposition(cell, ctx, parent));
                    if matches!(acc, Disposition::Unsupported(_)) {
                        return acc;
                    }
                }
            }
            acc
        }
    }
}

fn classify_reference(
    r: &Reference,
    ctx: &ClassificationContext,
    parent: Option<FnKind>,
) -> Disposition {
    match r {
        Reference::Cell { sheet, row, col } => {
            let resolved = sheet.as_deref().unwrap_or(ctx.current_sheet());
            if resolved.eq_ignore_ascii_case(ctx.current_sheet())
                && *row == ctx.current_row()
                && *col == ctx.current_col()
            {
                return Disposition::RowLocal;
            }
            if resolved.eq_ignore_ascii_case(ctx.current_sheet()) && *row == ctx.current_row() {
                Disposition::RowLocal
            } else if sheet.is_some() && ctx.is_lookup_sheet(resolved) {
                Disposition::Lookup
            } else {
                Disposition::Unsupported(UnsupportedReason::NonCurrentRowRef)
            }
        }

        Reference::Range { sheet, .. } => {
            let resolved = sheet.as_deref().unwrap_or(ctx.current_sheet());
            match parent {
                Some(FnKind::Aggregate) => Disposition::Aggregate,
                Some(FnKind::Lookup) => {
                    if resolved.eq_ignore_ascii_case(ctx.current_sheet()) {
                        Disposition::Unsupported(UnsupportedReason::LookupIntoStreamingSheet)
                    } else if ctx.is_lookup_sheet(resolved) {
                        Disposition::Lookup
                    } else {
                        Disposition::Unsupported(UnsupportedReason::LookupSheetNotPrepared)
                    }
                }
                Some(FnKind::RangeExpanding) => {
                    // Only bounded single-column ranges supported
                    match r {
                        Reference::Range {
                            start_row: Some(_),
                            end_row: Some(_),
                            start_col: Some(sc),
                            end_col: Some(ec),
                            ..
                        } if sc == ec => {
                            if resolved.eq_ignore_ascii_case(ctx.current_sheet()) {
                                Disposition::Mixed
                            } else if ctx.is_lookup_sheet(resolved) {
                                Disposition::Lookup
                            } else {
                                Disposition::Unsupported(UnsupportedReason::LookupSheetNotPrepared)
                            }
                        }
                        _ => Disposition::Unsupported(UnsupportedReason::UnboundedRange),
                    }
                }
                None => Disposition::Unsupported(UnsupportedReason::UnboundedRange),
            }
        }

        Reference::Named(_) => Disposition::RowLocal,
        Reference::External { .. } => {
            Disposition::Unsupported(UnsupportedReason::ExternalReference)
        }
        Reference::Table { .. } => Disposition::Unsupported(UnsupportedReason::TableReference),
        Reference::ThreeDimensional { .. } => {
            Disposition::Unsupported(UnsupportedReason::ThreeDimensionalRef)
        }
    }
}

fn classify_function(name: &str, args: &[Node], ctx: &ClassificationContext) -> Disposition {
    if sets::is_unsupported(name) {
        return Disposition::Unsupported(UnsupportedReason::UnsupportedFunction(
            name.to_uppercase(),
        ));
    }

    if sets::is_volatile_streaming_ok(name) {
        return Disposition::RowLocal;
    }

    if sets::is_aggregate(name) {
        return classify_aggregate(name, args, ctx);
    }

    if sets::is_lookup(name) {
        return fold_fn_args(args, ctx, FnKind::Lookup);
    }

    if sets::is_range_expanding(name) {
        return fold_args(args, ctx, Some(FnKind::RangeExpanding));
    }

    fold_args(args, ctx, None)
}

fn classify_aggregate(name: &str, args: &[Node], ctx: &ClassificationContext) -> Disposition {
    let upper = name.to_uppercase();
    let mut has_row_local_criteria = false;
    let mut has_aggregate_arg = false;
    for (i, arg) in args.iter().enumerate() {
        if is_criteria_arg(&upper, i) && contains_row_local_ref(arg, ctx) {
            has_row_local_criteria = true;
            let d = disposition(arg, ctx, None);
            if matches!(d, Disposition::Unsupported(_)) {
                return d;
            }
            continue;
        }
        let d = disposition(arg, ctx, Some(FnKind::Aggregate));
        if matches!(d, Disposition::Unsupported(_)) {
            return d;
        }
        if matches!(d, Disposition::Aggregate) {
            has_aggregate_arg = true;
        }
    }
    if has_row_local_criteria {
        Disposition::Mixed
    } else if has_aggregate_arg {
        Disposition::Aggregate
    } else {
        Disposition::RowLocal
    }
}

fn is_criteria_arg(fn_upper: &str, index: usize) -> bool {
    match fn_upper {
        "SUMIF" | "COUNTIF" | "AVERAGEIF" => index == 1,
        "SUMIFS" | "COUNTIFS" | "AVERAGEIFS" | "MINIFS" | "MAXIFS" => index >= 2 && index % 2 == 0,
        _ => false,
    }
}

fn contains_row_local_ref(node: &Node, ctx: &ClassificationContext) -> bool {
    match node {
        Node::Reference(Reference::Cell { sheet, row, .. }) => {
            let resolved = sheet.as_deref().unwrap_or(ctx.current_sheet());
            resolved.eq_ignore_ascii_case(ctx.current_sheet()) && *row == ctx.current_row()
        }
        Node::BinaryOp { left, right, .. } => {
            contains_row_local_ref(left, ctx) || contains_row_local_ref(right, ctx)
        }
        Node::UnaryOp { expr, .. } => contains_row_local_ref(expr, ctx),
        Node::Function { args, .. } => args.iter().any(|a| contains_row_local_ref(a, ctx)),
        _ => false,
    }
}

/// For lookup functions, the function's own kind determines the
/// disposition. `RowLocal` args (keys, column indices, match type) are
/// absorbed — only `Unsupported` propagates.
fn fold_fn_args(args: &[Node], ctx: &ClassificationContext, kind: FnKind) -> Disposition {
    let target = match kind {
        FnKind::Aggregate => Disposition::Aggregate,
        FnKind::Lookup => Disposition::Lookup,
        FnKind::RangeExpanding => Disposition::RowLocal,
    };
    for arg in args {
        let d = disposition(arg, ctx, Some(kind));
        if let Disposition::Unsupported(_) = d {
            return d;
        }
    }
    target
}

fn fold_args(args: &[Node], ctx: &ClassificationContext, parent: Option<FnKind>) -> Disposition {
    let mut iter = args.iter();
    let mut acc = match iter.next() {
        Some(first) => disposition(first, ctx, parent),
        None => return Disposition::RowLocal,
    };
    for arg in iter {
        if matches!(acc, Disposition::Unsupported(_)) {
            return acc;
        }
        acc = fold(acc, disposition(arg, ctx, parent));
    }
    acc
}

fn fold(a: Disposition, b: Disposition) -> Disposition {
    match (a, b) {
        (Disposition::Unsupported(r), _) | (_, Disposition::Unsupported(r)) => {
            Disposition::Unsupported(r)
        }
        (Disposition::RowLocal, Disposition::RowLocal) => Disposition::RowLocal,
        (Disposition::Aggregate, Disposition::Aggregate) => Disposition::Aggregate,
        (Disposition::Lookup, Disposition::Lookup) => Disposition::Lookup,
        _ => Disposition::Mixed,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn classify_literal_is_row_local() {
        let ast = crate::parse("1+2").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
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

    #[test]
    fn context_builder_round_trips() {
        let ctx = ClassificationContext::for_cell("Sheet1", 5, 3)
            .with_lookup_sheet("Lookup1")
            .with_header("Amount", 4);
        assert_eq!(ctx.current_sheet(), "Sheet1");
        assert_eq!(ctx.current_row(), 5);
        assert_eq!(ctx.current_col(), 3);
        assert!(ctx.is_lookup_sheet("Lookup1"));
        assert!(ctx.is_lookup_sheet("lookup1"));
        assert!(!ctx.is_lookup_sheet("Other"));
        assert_eq!(ctx.headers().get("Amount"), Some(&4));
    }

    #[test]
    fn cross_sheet_cell_ref_to_lookup_sheet_is_lookup() {
        let ast = crate::parse("'Tax Rates'!A1").unwrap();
        let ctx = ClassificationContext::for_cell("Main", 2, 1).with_lookup_sheet("Tax Rates");
        assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
    }

    #[test]
    fn cross_sheet_cell_ref_to_non_lookup_sheet_is_unsupported() {
        let ast = crate::parse("'Other'!A1").unwrap();
        let ctx = ClassificationContext::for_cell("Main", 2, 1).with_lookup_sheet("Tax Rates");
        assert_eq!(
            classify(&ast, &ctx),
            Classification::Unsupported(UnsupportedReason::NonCurrentRowRef)
        );
    }

    #[test]
    fn cross_sheet_cell_ref_mixed_with_row_local() {
        let ast = crate::parse("A2+'Tax Rates'!B1").unwrap();
        let ctx = ClassificationContext::for_cell("Main", 2, 3).with_lookup_sheet("Tax Rates");
        assert_eq!(classify(&ast, &ctx), Classification::Mixed);
    }

    #[test]
    fn fold_same_kinds_stay_same() {
        assert_eq!(fold(Disposition::RowLocal, Disposition::RowLocal), Disposition::RowLocal);
        assert_eq!(fold(Disposition::Aggregate, Disposition::Aggregate), Disposition::Aggregate);
        assert_eq!(fold(Disposition::Lookup, Disposition::Lookup), Disposition::Lookup);
    }

    #[test]
    fn fold_different_supported_kinds_become_mixed() {
        assert_eq!(fold(Disposition::RowLocal, Disposition::Aggregate), Disposition::Mixed);
        assert_eq!(fold(Disposition::RowLocal, Disposition::Lookup), Disposition::Mixed);
        assert_eq!(fold(Disposition::Aggregate, Disposition::Lookup), Disposition::Mixed);
    }

    #[test]
    fn fold_unsupported_absorbs_all() {
        let u = Disposition::Unsupported(UnsupportedReason::CircularRef);
        assert!(matches!(fold(u.clone(), Disposition::RowLocal), Disposition::Unsupported(_)));
        assert!(matches!(fold(Disposition::Aggregate, u), Disposition::Unsupported(_)));
    }

    #[test]
    fn irr_with_bounded_range_classifies_as_mixed() {
        let ast = crate::parse("IRR(B2:B10)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(matches!(result, Classification::Mixed), "expected Mixed, got {result:?}");
    }

    #[test]
    fn irr_with_unbounded_range_is_unsupported() {
        let ast = crate::parse("IRR(B:B)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert!(matches!(classify(&ast, &ctx), Classification::Unsupported(_)));
    }

    #[test]
    fn concat_with_bounded_range_classifies() {
        let ast = crate::parse("CONCAT(A2:A5)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(
            !matches!(result, Classification::Unsupported(_)),
            "expected non-unsupported, got {result:?}"
        );
    }

    #[test]
    fn concat_multi_column_range_is_unsupported() {
        let ast = crate::parse("CONCAT(A2:C5)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert!(matches!(classify(&ast, &ctx), Classification::Unsupported(_)));
    }

    #[test]
    fn sum_whole_column_still_classifies_as_aggregate() {
        let ast = crate::parse("SUM(A:A)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
    }

    #[test]
    fn networkdays_without_holidays_classifies() {
        let ast = crate::parse("NETWORKDAYS(A2, B2)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert!(!matches!(classify(&ast, &ctx), Classification::Unsupported(_)));
    }

    #[test]
    fn sumif_with_row_local_criteria_classifies_as_mixed() {
        let ast = crate::parse("SUMIF(A:A,A2,B:B)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::Mixed);
    }

    #[test]
    fn countif_with_row_local_criteria_classifies_as_mixed() {
        let ast = crate::parse("COUNTIF(A:A,A2)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::Mixed);
    }

    #[test]
    fn sumif_with_static_criteria_still_classifies_as_aggregate() {
        let ast = crate::parse("SUMIF(A:A,\"EMEA\",B:B)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
    }

    #[test]
    fn aggregate_with_literal_args_is_row_local() {
        let ast = crate::parse("PRODUCT(2,3,4)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    #[test]
    fn aggregate_with_cell_ref_args_is_row_local() {
        let ast = crate::parse("PRODUCT(A2,B2)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    #[test]
    fn aggregate_with_range_arg_stays_aggregate() {
        let ast = crate::parse("PRODUCT(A:A)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
        assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
    }

    #[test]
    fn sum_with_literal_args_is_row_local() {
        let ast = crate::parse("SUM(1,2,3)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    // -- Named range classification (v0.2) --

    fn resolve_and_classify(
        formula: &str,
        name_pairs: &[(&str, &str)],
        sheet: &str,
        row: u32,
        col: u32,
        lookup_sheets: &[&str],
    ) -> Classification {
        let ast = crate::parse(formula).unwrap();
        let names: std::collections::HashMap<String, String> =
            name_pairs.iter().map(|(k, v)| (k.to_ascii_lowercase(), (*v).to_string())).collect();
        let resolved = crate::resolve::resolve_named_ranges(ast, &names);
        let mut ctx = ClassificationContext::for_cell(sheet, row, col);
        for ls in lookup_sheets {
            ctx = ctx.with_lookup_sheet(ls);
        }
        classify(&resolved, &ctx)
    }

    #[test]
    fn named_range_sum_whole_column_is_aggregate() {
        let c = resolve_and_classify(
            "SUM(SalesData)",
            &[("SalesData", "Sheet1!$A:$A")],
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn named_range_constant_times_cell_is_row_local() {
        let c = resolve_and_classify("TaxRate*B2", &[("TaxRate", "0.15")], "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn named_range_vlookup_table_array_is_lookup() {
        let c = resolve_and_classify(
            "VLOOKUP(A2, Rates, 2, FALSE)",
            &[("Rates", "Lookup!$A:$D")],
            "Main",
            2,
            3,
            &["Lookup"],
        );
        assert_eq!(c, Classification::LookupOnly);
    }

    #[test]
    fn named_range_cross_sheet_aggregate_is_aggregate() {
        let c = resolve_and_classify(
            "SUM(RegionData)",
            &[("RegionData", "Lookup!$B:$B")],
            "Main",
            2,
            3,
            &["Lookup"],
        );
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn named_range_unknown_classifies_as_row_local() {
        let c = resolve_and_classify("SUM(MissingName)", &[], "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn named_range_case_insensitive_resolves() {
        let c = resolve_and_classify(
            "SUM(salesdata)",
            &[("SalesData", "Sheet1!$A:$A")],
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn named_range_sumif_both_ranges_resolve() {
        let c = resolve_and_classify(
            "SUMIF(RegionCol, \"EMEA\", AmountCol)",
            &[("RegionCol", "Sheet1!$A:$A"), ("AmountCol", "Sheet1!$B:$B")],
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn named_range_sumif_with_row_local_criteria_is_mixed() {
        let c = resolve_and_classify(
            "SUMIF(RegionCol, A2, AmountCol)",
            &[("RegionCol", "Sheet1!$A:$A"), ("AmountCol", "Sheet1!$B:$B")],
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::Mixed);
    }

    // -- SUMPRODUCT classification --

    #[test]
    fn sumproduct_two_bounded_ranges_classifies_as_streamable() {
        let ast = crate::parse("SUMPRODUCT(A1:A3, B1:B3)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(
            !matches!(result, Classification::Unsupported(_)),
            "expected streamable, got {result:?}"
        );
    }

    #[test]
    fn sumproduct_single_bounded_range_classifies_as_streamable() {
        let ast = crate::parse("SUMPRODUCT(A1:A5)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(
            !matches!(result, Classification::Unsupported(_)),
            "expected streamable, got {result:?}"
        );
    }

    #[test]
    fn sumproduct_three_bounded_ranges_classifies_as_streamable() {
        let ast = crate::parse("SUMPRODUCT(A1:A3, B1:B3, C1:C3)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(
            !matches!(result, Classification::Unsupported(_)),
            "expected streamable, got {result:?}"
        );
    }

    #[test]
    fn sumproduct_case_insensitive() {
        let ast = crate::parse("sumproduct(A1:A5, B1:B5)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(
            !matches!(result, Classification::Unsupported(_)),
            "expected streamable, got {result:?}"
        );
    }

    #[test]
    fn sumproduct_nested_inside_if() {
        let ast = crate::parse("IF(D2, SUMPRODUCT(A1:A3, B1:B3), 0)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        let result = classify(&ast, &ctx);
        assert!(
            !matches!(result, Classification::Unsupported(_)),
            "expected streamable, got {result:?}"
        );
    }

    #[test]
    fn self_reference_classifies_as_row_local() {
        let ast = crate::parse("E2+1").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    #[test]
    fn self_reference_in_if_classifies_as_row_local() {
        let ast = crate::parse("IF(A2=\"Risk_KC\",E2*-1,E2)").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    #[test]
    fn self_reference_mixed_with_other_deps_classifies_as_row_local() {
        let ast = crate::parse("E2+D2").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    #[test]
    fn cross_column_same_row_ref_is_row_local_individually() {
        let ast = crate::parse("B2+1").unwrap();
        let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
        assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
    }

    // -- Table reference classification (v0.2) --

    fn resolve_tables_and_classify(
        formula: &str,
        sheet: &str,
        row: u32,
        col: u32,
        lookup_sheets: &[&str],
    ) -> Classification {
        let ast = crate::parse(formula).unwrap();
        let mut tables = std::collections::HashMap::new();
        tables.insert(
            "table1".to_string(),
            crate::TableInfo {
                sheet_name: "Sheet1".into(),
                columns: vec![
                    "Region".into(),
                    "Amount".into(),
                    "Price".into(),
                    "Qty".into(),
                    "Status".into(),
                ],
                header_row: 0,
                data_start_row: 1,
                data_end_row: 10,
                start_col: 0,
            },
        );
        tables.insert(
            "table2".to_string(),
            crate::TableInfo {
                sheet_name: "Sheet2".into(),
                columns: vec!["Key".into(), "Value".into()],
                header_row: 0,
                data_start_row: 1,
                data_end_row: 5,
                start_col: 0,
            },
        );
        let resolved = crate::resolve_table_references(ast, &tables, Some(sheet), row);
        let names: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let resolved = crate::resolve_named_ranges(resolved, &names);
        let mut ctx = ClassificationContext::for_cell(sheet, row, col);
        for ls in lookup_sheets {
            ctx = ctx.with_lookup_sheet(ls);
        }
        classify(&resolved, &ctx)
    }

    #[test]
    fn table_whole_column_classifies_as_aggregate() {
        let c = resolve_tables_and_classify("SUM(Table1[Amount])", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn table_this_row_classifies_as_row_local() {
        let c = resolve_tables_and_classify("[@Price]*1.1", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn table_this_row_mixed_with_aggregate() {
        let c = resolve_tables_and_classify("[@Amount]/SUM(Table1[Amount])", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::Mixed);
    }

    #[test]
    fn table_countif_on_column_classifies_as_aggregate() {
        let c =
            resolve_tables_and_classify("COUNTIF(Table1[Region],\"EMEA\")", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn table_if_with_this_row_classifies_as_row_local() {
        let c = resolve_tables_and_classify(
            "IF([@Status]=\"Active\",[@Amount],0)",
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn table_unknown_table_classifies_as_unsupported() {
        let c = resolve_tables_and_classify("SUM(UnknownTable[Col])", "Sheet1", 2, 5, &[]);
        assert!(
            matches!(c, Classification::Unsupported(UnsupportedReason::TableReference)),
            "expected Unsupported(TableReference), got {c:?}"
        );
    }

    #[test]
    fn table_case_insensitive_resolves_and_classifies() {
        let c = resolve_tables_and_classify("SUM(table1[amount])", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn table_sumif_both_columns_classifies_as_aggregate() {
        let c = resolve_tables_and_classify(
            "SUMIF(Table1[Region],\"EMEA\",Table1[Amount])",
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::AggregateOnly);
    }

    #[test]
    fn table_iferror_with_this_row_classifies_as_row_local() {
        let c = resolve_tables_and_classify("IFERROR([@Price]/[@Qty],0)", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn table_this_row_mixed_with_cell_ref_is_row_local() {
        let c = resolve_tables_and_classify("[@Price]+B2", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn table_unknown_column_classifies_as_unsupported() {
        let c = resolve_tables_and_classify("SUM(Table1[NonexistentCol])", "Sheet1", 2, 5, &[]);
        assert!(
            matches!(c, Classification::Unsupported(UnsupportedReason::TableReference)),
            "expected Unsupported(TableReference), got {c:?}"
        );
    }

    #[test]
    fn table_nested_if_with_this_row_classifies_as_row_local() {
        let c = resolve_tables_and_classify(
            "IF([@Amount]>100,[@Price]*2,[@Price])",
            "Sheet1",
            2,
            5,
            &[],
        );
        assert_eq!(c, Classification::RowLocal);
    }

    #[test]
    fn table_average_whole_column_classifies_as_aggregate() {
        let c = resolve_tables_and_classify("AVERAGE(Table1[Price])", "Sheet1", 2, 5, &[]);
        assert_eq!(c, Classification::AggregateOnly);
    }
}
