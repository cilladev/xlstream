//! Reference types and the [`extract_references`] walker.

use smallvec::SmallVec;

use crate::ast::Ast;

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

/// Walk `ast` and collect every reference, sheet name, and function name.
///
/// Uses upstream's zero-alloc `visit_refs` walker for references; walks
/// the upstream tree once more for function names. Both passes are
/// stack-based.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{extract_references, parse};
/// let ast = parse("SUM(A:A)").unwrap();
/// let refs = extract_references(&ast);
/// assert_eq!(refs.ranges.len(), 1);
/// ```
#[must_use]
pub fn extract_references(ast: &Ast) -> References {
    let mut out = References::default();
    ast.as_upstream().visit_refs(|rv| collect_view(rv, &mut out));
    walk_functions(ast.as_upstream(), &mut out);
    out
}

fn collect_view(rv: formualizer_parse::parser::RefView<'_>, out: &mut References) {
    use formualizer_parse::parser::RefView as V;
    match rv {
        V::Cell { sheet, row, col, .. } => {
            push_sheet(sheet, out);
            out.cells.push(Reference::Cell { sheet: sheet.map(str::to_owned), row, col });
        }
        V::Range { sheet, start_row, end_row, start_col, end_col, .. } => {
            push_sheet(sheet, out);
            out.ranges.push(Reference::Range {
                sheet: sheet.map(str::to_owned),
                start_row,
                end_row,
                start_col,
                end_col,
            });
        }
        V::External { raw, book, sheet, .. } => {
            push_sheet(Some(sheet), out);
            out.ranges.push(Reference::External {
                raw: raw.to_owned(),
                book: book.to_owned(),
                sheet: sheet.to_owned(),
            });
        }
        V::Table { name, specifier } => {
            out.ranges.push(Reference::Table {
                name: name.to_owned(),
                specifier: specifier.map(|s| format!("{s}")),
            });
        }
        V::NamedRange { name } => {
            out.ranges.push(Reference::Named(name.to_owned()));
        }
    }
}

fn push_sheet(sheet: Option<&str>, out: &mut References) {
    if let Some(s) = sheet {
        if !out.sheets.iter().any(|existing| existing == s) {
            out.sheets.push(s.to_owned());
        }
    }
}

fn walk_functions(node: &formualizer_parse::ASTNode, out: &mut References) {
    use formualizer_parse::ASTNodeType as T;
    match &node.node_type {
        T::Literal(_) | T::Reference { .. } => {}
        T::UnaryOp { expr, .. } => walk_functions(expr, out),
        T::BinaryOp { left, right, .. } => {
            walk_functions(left, out);
            walk_functions(right, out);
        }
        T::Function { name, args } => {
            if !out.functions.iter().any(|n| n.eq_ignore_ascii_case(name)) {
                out.functions.push(name.clone());
            }
            for a in args {
                walk_functions(a, out);
            }
        }
        T::Array(rows) => {
            for row in rows {
                for cell in row {
                    walk_functions(cell, out);
                }
            }
        }
    }
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
