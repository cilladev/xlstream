//! The [`parse`] entry point.

use xlstream_core::XlStreamError;

use crate::ast::{Ast, Node, NumLiteral};
use crate::references::Reference;

/// Parse an Excel formula expression into an [`Ast`].
///
/// The input must **not** include a leading `=`; that's an I/O concern,
/// stripped before the parser sees the text.
///
/// # Errors
///
/// [`XlStreamError::FormulaParse`] for malformed input. The `address`
/// field is left empty; callers that have the cell address must enrich
/// with `map_err`. The `position` field carries upstream's byte offset
/// into `expr` when reported.
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// let ast = parse("SUM(A1:A10)").expect("parse failed");
/// assert!(format!("{ast:?}").contains("Function"));
/// ```
pub fn parse(expr: &str) -> Result<Ast, XlStreamError> {
    let with_eq = format!("={expr}");
    let upstream = formualizer_parse::parse(&with_eq).map_err(|e| XlStreamError::FormulaParse {
        address: String::new(),
        formula: expr.to_owned(),
        message: e.message,
        position: e.position.map(|p| p.saturating_sub(1)),
    })?;

    let root = lower(&upstream);
    Ok(Ast { upstream, root })
}

/// Lower an upstream `ASTNode` into our internal [`Node`].
pub(crate) fn lower(node: &formualizer_parse::ASTNode) -> Node {
    use formualizer_parse::ASTNodeType as T;

    match &node.node_type {
        T::Literal(lv) => lower_literal(lv),
        T::UnaryOp { op, expr } => Node::UnaryOp { op: op.clone(), expr: Box::new(lower(expr)) },
        T::BinaryOp { op, left, right } => Node::BinaryOp {
            op: op.clone(),
            left: Box::new(lower(left)),
            right: Box::new(lower(right)),
        },
        T::Function { name, args } => {
            Node::Function { name: name.clone(), args: args.iter().map(lower).collect() }
        }
        T::Array(rows) => Node::Array(rows.iter().map(|r| r.iter().map(lower).collect()).collect()),
        T::Reference { reference, .. } => Node::Reference(lower_reference(reference)),
    }
}

fn lower_reference(r: &formualizer_parse::parser::ReferenceType) -> Reference {
    use formualizer_parse::parser::ReferenceType as R;
    match r {
        R::Cell { sheet, row, col, .. } => {
            Reference::Cell { sheet: sheet.clone(), row: *row, col: *col }
        }
        R::Range { sheet, start_row, end_row, start_col, end_col, .. } => Reference::Range {
            sheet: sheet.clone(),
            start_row: *start_row,
            end_row: *end_row,
            start_col: *start_col,
            end_col: *end_col,
        },
        R::NamedRange(name) => Reference::Named(name.clone()),
        R::External(ext) => Reference::External {
            raw: ext.raw.clone(),
            book: ext.book.token().to_owned(),
            sheet: ext.sheet.clone(),
        },
        R::Table(t) => Reference::Table {
            name: t.name.clone(),
            specifier: t.specifier.as_ref().map(|s| format!("{s}")),
        },
    }
}

fn lower_literal(lv: &formualizer_common::LiteralValue) -> Node {
    use formualizer_common::LiteralValue as L;
    match lv {
        L::Number(n) => Node::Literal(NumLiteral::Number(*n)),
        #[allow(clippy::cast_precision_loss)]
        L::Int(i) => Node::Literal(NumLiteral::Number(*i as f64)),
        L::Boolean(b) => Node::Literal(NumLiteral::Bool(*b)),
        L::Text(s) => Node::Text(s.clone()),
        L::Error(e) => Node::Error(map_excel_error(e)),
        // Phase 4: first-class variants for Date / Time / DateTime /
        // Duration / Empty / Pending / Array literals.
        other => Node::Text(format!("{other:?}")),
    }
}

fn map_excel_error(e: &formualizer_common::ExcelError) -> xlstream_core::CellError {
    use formualizer_common::ExcelErrorKind as K;
    use xlstream_core::CellError;
    match e.kind {
        K::Div => CellError::Div0,
        K::Value | K::Error | K::NImpl | K::Spill | K::Calc | K::Circ | K::Cancelled => {
            CellError::Value
        }
        K::Ref => CellError::Ref,
        K::Name => CellError::Name,
        K::Na => CellError::Na,
        K::Num => CellError::Num,
        K::Null => CellError::Null,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    #[test]
    fn parse_returns_ast_for_simple_arithmetic() {
        let ast = parse("1+2").unwrap();
        assert!(format!("{ast:?}").contains("BinaryOp"));
    }

    #[test]
    fn parse_returns_formula_parse_on_garbage() {
        let err = parse("SUM(").unwrap_err();
        assert!(matches!(err, XlStreamError::FormulaParse { .. }));
    }

    #[test]
    fn lower_handles_whole_column_range() {
        let upstream = formualizer_parse::parse("=SUM(A:A)").unwrap();
        let node = lower(&upstream);
        let dbg = format!("{node:?}");
        assert!(dbg.contains("start_row: None"), "expected open range: {dbg}");
        assert!(dbg.contains("end_row: None"), "expected open range: {dbg}");
    }

    #[test]
    fn lower_handles_text_literal() {
        let upstream = formualizer_parse::parse("=\"hello\"").unwrap();
        let node = lower(&upstream);
        assert!(format!("{node:?}").contains("hello"));
    }

    #[test]
    fn lower_handles_boolean_literal() {
        let upstream = formualizer_parse::parse("=TRUE").unwrap();
        let node = lower(&upstream);
        assert!(format!("{node:?}").contains("Bool(true)"));
    }

    #[test]
    fn map_excel_error_covers_known_kinds() {
        use formualizer_common::{ExcelError, ExcelErrorKind};
        use xlstream_core::CellError;
        for (kind, expected) in [
            (ExcelErrorKind::Div, CellError::Div0),
            (ExcelErrorKind::Value, CellError::Value),
            (ExcelErrorKind::Ref, CellError::Ref),
            (ExcelErrorKind::Name, CellError::Name),
            (ExcelErrorKind::Na, CellError::Na),
            (ExcelErrorKind::Num, CellError::Num),
            (ExcelErrorKind::Null, CellError::Null),
        ] {
            let e = ExcelError::from(kind);
            assert_eq!(map_excel_error(&e), expected, "kind={kind:?}");
        }
    }
}
