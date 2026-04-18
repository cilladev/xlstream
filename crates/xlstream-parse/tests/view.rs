#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use xlstream_parse::{parse, NodeView};

#[test]
fn number_literal_view() {
    let ast = parse("42").unwrap();
    assert!(matches!(ast.root().view(), NodeView::Number(n) if (n - 42.0).abs() < f64::EPSILON));
}

#[test]
fn text_literal_view() {
    let ast = parse("\"hello\"").unwrap();
    assert!(matches!(ast.root().view(), NodeView::Text("hello")));
}

#[test]
fn bool_literal_view() {
    let ast = parse("TRUE").unwrap();
    assert!(matches!(ast.root().view(), NodeView::Bool(true)));
}

#[test]
fn cell_ref_view() {
    let ast = parse("A2").unwrap();
    match ast.root().view() {
        NodeView::CellRef { col: 1, row: 2, .. } => {}
        other => panic!("expected CellRef(A2), got {other:?}"),
    }
}

#[test]
fn binary_op_with_children() {
    let ast = parse("1+2").unwrap();
    let root = ast.root();
    assert!(matches!(root.view(), NodeView::BinaryOp { op: "+" }));
    assert!(root.left().is_some());
    assert!(root.right().is_some());
    assert!(matches!(root.left().unwrap().view(), NodeView::Number(_)));
    assert!(matches!(root.right().unwrap().view(), NodeView::Number(_)));
}

#[test]
fn unary_op_with_operand() {
    let ast = parse("-5").unwrap();
    let root = ast.root();
    assert!(matches!(root.view(), NodeView::UnaryOp { op: "-" }));
    assert!(root.operand().is_some());
}

#[test]
fn function_with_args() {
    let ast = parse("SUM(A1, B1)").unwrap();
    let root = ast.root();
    assert!(matches!(root.view(), NodeView::Function { name: "SUM" }));
    let args = root.args();
    assert_eq!(args.len(), 2);
}

#[test]
fn leaf_nodes_have_no_children() {
    let ast = parse("42").unwrap();
    let root = ast.root();
    assert!(root.left().is_none());
    assert!(root.right().is_none());
    assert!(root.operand().is_none());
    assert!(root.args().is_empty());
    assert!(root.array_cells().is_empty());
}

#[test]
fn range_ref_view() {
    let ast = parse("A1:B10").unwrap();
    match ast.root().view() {
        NodeView::RangeRef { start_col: Some(1), end_col: Some(2), .. } => {}
        other => panic!("expected RangeRef, got {other:?}"),
    }
}

#[test]
fn error_literal_view() {
    let ast = parse("#REF!").unwrap();
    assert!(matches!(ast.root().view(), NodeView::Error(xlstream_core::CellError::Ref)));
}
