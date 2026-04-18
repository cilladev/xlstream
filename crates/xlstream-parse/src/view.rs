//! Public read-only view API for the AST.
//!
//! [`NodeRef`] wraps a `&Node` and provides typed child accessors.
//! [`NodeView`] is the `Copy`-able enum the evaluator matches on for
//! dispatch — it borrows into the tree without exposing internal types.

use xlstream_core::CellError;

use crate::ast::{Node, NumLiteral};
use crate::references::Reference;
use crate::rewrite::PreludeKey;

/// Borrowed view of a node's content. Match on this in the evaluator.
/// Does not expose children — use [`NodeRef`] methods for those.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{parse, NodeView};
/// let ast = parse("42").unwrap();
/// assert!(matches!(ast.root().view(), NodeView::Number(_)));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum NodeView<'a> {
    /// `f64` numeric literal.
    Number(f64),
    /// Boolean literal.
    Bool(bool),
    /// Text literal.
    Text(&'a str),
    /// Error literal.
    Error(CellError),
    /// Cell reference (1-based row and col).
    CellRef {
        /// Sheet name, or `None` for the active sheet.
        sheet: Option<&'a str>,
        /// 1-based row.
        row: u32,
        /// 1-based column.
        col: u32,
    },
    /// Range reference.
    RangeRef {
        /// Sheet name, or `None` for the active sheet.
        sheet: Option<&'a str>,
        /// 1-based start row, or `None` for whole-column refs.
        start_row: Option<u32>,
        /// 1-based end row, or `None` for whole-column refs.
        end_row: Option<u32>,
        /// 1-based start column, or `None` for whole-row refs.
        start_col: Option<u32>,
        /// 1-based end column, or `None` for whole-row refs.
        end_col: Option<u32>,
    },
    /// Named range reference.
    NamedRef(&'a str),
    /// External workbook reference.
    ExternalRef {
        /// Original source text.
        raw: &'a str,
        /// Workbook token.
        book: &'a str,
        /// Sheet name within the external book.
        sheet: &'a str,
    },
    /// Table reference.
    TableRef {
        /// Table name.
        name: &'a str,
        /// Column/row/item specifier, or `None` for the whole table.
        specifier: Option<&'a str>,
    },
    /// Binary operator. Children via [`NodeRef::left`] / [`NodeRef::right`].
    BinaryOp {
        /// Operator token (e.g. `"+"`, `"*"`).
        op: &'a str,
    },
    /// Unary operator. Child via [`NodeRef::operand`].
    UnaryOp {
        /// Operator token (e.g. `"-"`, `"%"`).
        op: &'a str,
    },
    /// Function call. Children via [`NodeRef::args`].
    Function {
        /// Function name, case-preserved (e.g. `"SUM"`).
        name: &'a str,
    },
    /// Array literal. Children via [`NodeRef::array_cells`].
    Array {
        /// Number of rows.
        rows: usize,
        /// Number of columns.
        cols: usize,
    },
    /// Prelude-computed reference.
    PreludeRef(&'a PreludeKey),
}

/// Borrowed handle to an AST node. Call [`view()`](NodeRef::view) to inspect
/// the node's content, and child accessors to traverse the tree.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{parse, NodeRef, NodeView};
/// let ast = parse("1+2").unwrap();
/// let root: NodeRef<'_> = ast.root();
/// assert!(matches!(root.view(), NodeView::BinaryOp { op: "+" }));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NodeRef<'a>(pub(crate) &'a Node);

impl<'a> NodeRef<'a> {
    /// Inspect this node's content.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::{parse, NodeView};
    /// let ast = parse("42").unwrap();
    /// assert!(matches!(ast.root().view(), NodeView::Number(n) if (n - 42.0).abs() < f64::EPSILON));
    /// ```
    #[must_use]
    pub fn view(&self) -> NodeView<'a> {
        match self.0 {
            Node::Literal(NumLiteral::Number(n)) => NodeView::Number(*n),
            Node::Literal(NumLiteral::Bool(b)) => NodeView::Bool(*b),
            Node::Text(s) => NodeView::Text(s.as_str()),
            Node::Error(e) => NodeView::Error(*e),
            Node::Reference(Reference::Cell { sheet, row, col }) => {
                NodeView::CellRef { sheet: sheet.as_deref(), row: *row, col: *col }
            }
            Node::Reference(Reference::Range { sheet, start_row, end_row, start_col, end_col }) => {
                NodeView::RangeRef {
                    sheet: sheet.as_deref(),
                    start_row: *start_row,
                    end_row: *end_row,
                    start_col: *start_col,
                    end_col: *end_col,
                }
            }
            Node::Reference(Reference::Named(name)) => NodeView::NamedRef(name.as_str()),
            Node::Reference(Reference::External { raw, book, sheet }) => NodeView::ExternalRef {
                raw: raw.as_str(),
                book: book.as_str(),
                sheet: sheet.as_str(),
            },
            Node::Reference(Reference::Table { name, specifier }) => {
                NodeView::TableRef { name: name.as_str(), specifier: specifier.as_deref() }
            }
            Node::BinaryOp { op, .. } => NodeView::BinaryOp { op: op.as_str() },
            Node::UnaryOp { op, .. } => NodeView::UnaryOp { op: op.as_str() },
            Node::Function { name, .. } => NodeView::Function { name: name.as_str() },
            Node::Array(rows) => {
                NodeView::Array { rows: rows.len(), cols: rows.first().map_or(0, Vec::len) }
            }
            Node::PreludeRef(key) => NodeView::PreludeRef(key),
        }
    }

    /// Left child of a `BinaryOp`. `None` for other node types.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::{parse, NodeView};
    /// let ast = parse("1+2").unwrap();
    /// let left = ast.root().left().unwrap();
    /// assert!(matches!(left.view(), NodeView::Number(_)));
    /// ```
    #[must_use]
    pub fn left(&self) -> Option<NodeRef<'a>> {
        match self.0 {
            Node::BinaryOp { left, .. } => Some(NodeRef(left)),
            _ => None,
        }
    }

    /// Right child of a `BinaryOp`. `None` for other node types.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::{parse, NodeView};
    /// let ast = parse("1+2").unwrap();
    /// let right = ast.root().right().unwrap();
    /// assert!(matches!(right.view(), NodeView::Number(_)));
    /// ```
    #[must_use]
    pub fn right(&self) -> Option<NodeRef<'a>> {
        match self.0 {
            Node::BinaryOp { right, .. } => Some(NodeRef(right)),
            _ => None,
        }
    }

    /// Operand of a `UnaryOp`. `None` for other node types.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::{parse, NodeView};
    /// let ast = parse("-5").unwrap();
    /// let operand = ast.root().operand().unwrap();
    /// assert!(matches!(operand.view(), NodeView::Number(_)));
    /// ```
    #[must_use]
    pub fn operand(&self) -> Option<NodeRef<'a>> {
        match self.0 {
            Node::UnaryOp { expr, .. } => Some(NodeRef(expr)),
            _ => None,
        }
    }

    /// Arguments of a `Function` call. Empty vec for other node types.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::{parse, NodeView};
    /// let ast = parse("SUM(A1, B1)").unwrap();
    /// let args = ast.root().args();
    /// assert_eq!(args.len(), 2);
    /// ```
    #[must_use]
    pub fn args(&self) -> Vec<NodeRef<'a>> {
        match self.0 {
            Node::Function { args, .. } => args.iter().map(NodeRef).collect(),
            _ => Vec::new(),
        }
    }

    /// Cells of an `Array` literal as rows x cols. Empty for other types.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::{parse, NodeView};
    /// let ast = parse("{1,2;3,4}").unwrap();
    /// let cells = ast.root().array_cells();
    /// assert_eq!(cells.len(), 2);
    /// assert_eq!(cells[0].len(), 2);
    /// ```
    #[must_use]
    pub fn array_cells(&self) -> Vec<Vec<NodeRef<'a>>> {
        match self.0 {
            Node::Array(rows) => rows.iter().map(|r| r.iter().map(NodeRef).collect()).collect(),
            _ => Vec::new(),
        }
    }
}
