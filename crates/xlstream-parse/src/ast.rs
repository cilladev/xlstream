//! The opaque [`Ast`] — wraps the raw upstream `ASTNode` plus our internal
//! tree of [`Node`]s mirroring it.
//!
//! Why both: `extract_references` (Chunk 1) uses upstream's zero-alloc
//! `visit_refs` walker via [`Ast::as_upstream`]; classification + rewrite
//! (Chunks 3-4) use the mirror so we can splice in `Node::PreludeRef`
//! (Chunk 4) without leaking upstream types in our public API.

use xlstream_core::CellError;

use crate::references::Reference;

/// Numeric or boolean literal carried by [`Node::Literal`].
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NumLiteral {
    /// An `f64` numeric value.
    Number(f64),
    /// A boolean literal (`TRUE` / `FALSE`).
    Bool(bool),
}

/// One node in the parsed-formula tree. Mirrors upstream's
/// `formualizer_parse::ASTNodeType` so we can splice in `PreludeRef`
/// (Chunk 4) and so error literals are first-class via `Node::Error`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Node {
    /// A scalar literal.
    Literal(NumLiteral),
    /// A text literal.
    Text(String),
    /// An error literal (`=#REF!`, `=#DIV/0!`, etc.).
    Error(CellError),
    /// A reference to one or more cells.
    Reference(Reference),
    /// Unary operator (`-`, `%`).
    UnaryOp { op: String, expr: Box<Node> },
    /// Binary operator (`+ - * / ^ & = <> < > <= >= :`).
    BinaryOp { op: String, left: Box<Node>, right: Box<Node> },
    /// Function call.
    Function { name: String, args: Vec<Node> },
    /// Array constant `{1,2;3,4}`. `Vec` instead of `SmallVec` because
    /// inline `Node` storage would create a layout cycle.
    Array(Vec<Vec<Node>>),
    /// A reference to a prelude-computed value (aggregate scalar or lookup
    /// index). Inserted by [`crate::rewrite::rewrite`] after classification;
    /// never produced by the parser.
    PreludeRef(crate::rewrite::PreludeKey),
}

/// Parsed formula. Opaque by design — internals are crate-internal so we
/// can evolve the tree across phases without breaking semver.
///
/// Build one with [`crate::parse`].
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// let ast = parse("1+2").unwrap();
/// assert!(format!("{ast:?}").contains("BinaryOp"));
/// ```
#[derive(Debug, Clone)]
pub struct Ast {
    /// Original upstream tree, retained for `visit_refs` etc.
    pub(crate) upstream: formualizer_parse::ASTNode,
    /// Our mirror tree. Used by classify + rewrite.
    pub(crate) root: Node,
}

impl PartialEq for Ast {
    /// Equality compares the mirror tree only.
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Ast {
    /// Crate-internal accessor for the raw upstream tree.
    #[must_use]
    pub(crate) fn as_upstream(&self) -> &formualizer_parse::ASTNode {
        &self.upstream
    }
}

#[cfg(test)]
impl Ast {
    /// Test-only constructor that builds trees without going through
    /// [`crate::parse`].
    #[allow(clippy::unwrap_used, clippy::missing_panics_doc, private_interfaces)]
    #[must_use]
    pub fn from_root_for_tests(root: Node) -> Self {
        let upstream = formualizer_parse::parse("=0").unwrap();
        Self { upstream, root }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    #[test]
    fn ast_carries_both_upstream_and_mirror() {
        let ast = Ast::from_root_for_tests(Node::Literal(NumLiteral::Number(42.0)));
        assert!(format!("{ast:?}").contains("42"), "expected literal in debug: {ast:?}");
    }

    #[test]
    fn node_literal_compares_by_value() {
        assert_eq!(Node::Literal(NumLiteral::Bool(true)), Node::Literal(NumLiteral::Bool(true)));
        assert_ne!(Node::Literal(NumLiteral::Bool(true)), Node::Literal(NumLiteral::Bool(false)));
    }

    #[test]
    fn node_error_variant_carries_cell_error() {
        let n = Node::Error(CellError::Ref);
        match n {
            Node::Error(CellError::Ref) => (),
            other => panic!("expected Error(Ref), got {other:?}"),
        }
    }
}
