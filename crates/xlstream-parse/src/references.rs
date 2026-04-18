//! Reference types and (in Chunk 1) the [`extract_references`] walker.

use smallvec::SmallVec;

/// One Excel reference: cell, range, named range, external workbook ref,
/// or table ref.
///
/// External and Table variants preserve enough upstream context for an
/// actionable refusal message (Chunk 3) without structurally mirroring
/// upstream's `ExternalRefKind` / `TableSpecifier` (drift risk).
///
/// # Examples
///
/// ```
/// use xlstream_parse::Reference;
/// let r = Reference::Cell { sheet: Some("Sheet1".into()), row: 2, col: 3 };
/// assert_eq!(r.sheet(), Some("Sheet1"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    /// `Sheet1!$A$2` — fully-resolved cell address.
    Cell {
        /// Sheet name. `None` for an unqualified ref on the active sheet.
        sheet: Option<String>,
        /// 1-based row.
        row: u32,
        /// 1-based column.
        col: u32,
    },
    /// `A:A`, `A1:B10`, `Sheet2!A:C`. Whole-column refs use
    /// `start_row = None`, `end_row = None`. Whole-row refs use
    /// `start_col = None`, `end_col = None`.
    Range {
        /// Sheet name. `None` for an unqualified range on the active sheet.
        sheet: Option<String>,
        /// 1-based start row, or `None` for whole-column refs.
        start_row: Option<u32>,
        /// 1-based end row, or `None` for whole-column refs.
        end_row: Option<u32>,
        /// 1-based start column, or `None` for whole-row refs.
        start_col: Option<u32>,
        /// 1-based end column, or `None` for whole-row refs.
        end_col: Option<u32>,
    },
    /// `MyRange` — workbook-level named range.
    Named(String),
    /// `[OtherBook.xlsx]Sheet1!A1` — reference into another workbook.
    /// Refused at classification with
    /// [`crate::UnsupportedReason::ExternalReference`].
    External {
        /// Original source text (preserved for diagnostics).
        raw: String,
        /// Book token from upstream (e.g. `OtherBook.xlsx` or `[1]`).
        book: String,
        /// Sheet name within the external book.
        sheet: String,
    },
    /// `Table[Column]` — structured table reference. Refused at
    /// classification with [`crate::UnsupportedReason::TableReference`].
    Table {
        /// Table name.
        name: String,
        /// Column / row / item specifier rendered via upstream's `Display`.
        /// `None` = whole table.
        specifier: Option<String>,
    },
}

/// References surfaced by the (Chunk 1) `extract_references` walker.
/// Sized by P99 expectation in `docs/architecture/parse-reuse.md`.
///
/// # Examples
///
/// ```
/// use xlstream_parse::References;
/// let refs = References::default();
/// assert!(refs.cells.is_empty());
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct References {
    /// Every cell reference reachable from the AST root.
    pub cells: SmallVec<[Reference; 8]>,
    /// Every range / named-range / external / table reference.
    pub ranges: SmallVec<[Reference; 4]>,
    /// Every distinct sheet name mentioned (de-duplicated).
    pub sheets: SmallVec<[String; 2]>,
    /// Every function name called (case-preserved, case-insensitively
    /// de-duplicated).
    pub functions: SmallVec<[String; 8]>,
}

impl Reference {
    /// Sheet name this reference points to, if explicit.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::Reference;
    /// let r = Reference::Cell { sheet: Some("Sheet1".into()), row: 2, col: 3 };
    /// assert_eq!(r.sheet(), Some("Sheet1"));
    /// ```
    #[must_use]
    pub fn sheet(&self) -> Option<&str> {
        match self {
            Self::Cell { sheet, .. } | Self::Range { sheet, .. } => sheet.as_deref(),
            Self::External { sheet, .. } => Some(sheet.as_str()),
            Self::Named(_) | Self::Table { .. } => None,
        }
    }

    /// `true` for `A:A`-style refs (no row bounds; column bounds present).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::Reference;
    /// let r = Reference::Range { sheet: None,
    ///     start_row: None, end_row: None,
    ///     start_col: Some(1), end_col: Some(1) };
    /// assert!(r.is_whole_column());
    /// ```
    #[must_use]
    pub fn is_whole_column(&self) -> bool {
        matches!(
            self,
            Self::Range {
                start_row: None,
                end_row: None,
                start_col: Some(_),
                end_col: Some(_),
                ..
            }
        )
    }

    /// `true` for `1:1`-style whole-row refs.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::Reference;
    /// let r = Reference::Range { sheet: None,
    ///     start_row: Some(1), end_row: Some(1),
    ///     start_col: None, end_col: None };
    /// assert!(r.is_whole_row());
    /// ```
    #[must_use]
    pub fn is_whole_row(&self) -> bool {
        matches!(
            self,
            Self::Range {
                start_col: None,
                end_col: None,
                start_row: Some(_),
                end_row: Some(_),
                ..
            }
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn cell_reference_reports_its_sheet() {
        let r = Reference::Cell { sheet: Some("Sheet1".into()), row: 2, col: 3 };
        assert_eq!(r.sheet(), Some("Sheet1"));
    }

    #[test]
    fn unqualified_cell_has_no_sheet() {
        let r = Reference::Cell { sheet: None, row: 1, col: 1 };
        assert_eq!(r.sheet(), None);
    }

    #[test]
    fn external_reference_reports_its_sheet() {
        let r = Reference::External {
            raw: "[Book2]Sheet1!A1".into(),
            book: "Book2".into(),
            sheet: "Sheet1".into(),
        };
        assert_eq!(r.sheet(), Some("Sheet1"));
    }

    #[test]
    fn whole_column_range_is_detected() {
        let r = Reference::Range {
            sheet: None,
            start_row: None,
            end_row: None,
            start_col: Some(1),
            end_col: Some(1),
        };
        assert!(r.is_whole_column());
        assert!(!r.is_whole_row());
    }

    #[test]
    fn whole_row_range_is_detected() {
        let r = Reference::Range {
            sheet: None,
            start_row: Some(1),
            end_row: Some(1),
            start_col: None,
            end_col: None,
        };
        assert!(r.is_whole_row());
        assert!(!r.is_whole_column());
    }

    #[test]
    fn bounded_range_is_neither_whole_column_nor_whole_row() {
        let r = Reference::Range {
            sheet: None,
            start_row: Some(1),
            end_row: Some(10),
            start_col: Some(1),
            end_col: Some(2),
        };
        assert!(!r.is_whole_column());
        assert!(!r.is_whole_row());
    }
}
