//! Integration tests for `xlstream_parse::parse`. Drive the public API
//! exactly as a downstream consumer would.

use xlstream_parse::parse;

#[test]
fn simple_arithmetic_parses() {
    let ast = parse("1+2").expect("parse failed");
    let dbg = format!("{ast:?}");
    assert!(dbg.contains("BinaryOp"), "expected BinaryOp in debug: {dbg}");
    assert!(dbg.contains('1'), "expected literal 1: {dbg}");
    assert!(dbg.contains('2'), "expected literal 2: {dbg}");
}

#[test]
fn function_call_parses() {
    let ast = parse("SUM(A1, B2)").expect("parse failed");
    let dbg = format!("{ast:?}");
    assert!(dbg.contains("Function"), "expected Function: {dbg}");
    assert!(dbg.contains("SUM"), "expected SUM name: {dbg}");
}

#[test]
fn whole_column_range_parses() {
    let ast = parse("SUM(A:A)").expect("parse failed");
    assert!(format!("{ast:?}").contains("Range"));
}

#[test]
fn cross_sheet_reference_parses() {
    let ast = parse("'Tax Rates'!A1").expect("parse failed");
    assert!(format!("{ast:?}").contains("Tax Rates"));
}

#[test]
fn excel_error_literal_lowers_to_node_error() {
    let ast = parse("#REF!").expect("parse failed");
    assert!(format!("{ast:?}").contains("Error"));
}

#[test]
fn malformed_input_returns_formula_parse_error() {
    let err = parse("SUM(").unwrap_err();
    match err {
        xlstream_core::XlStreamError::FormulaParse { ref formula, position, .. } => {
            assert_eq!(formula, "SUM(");
            let _ = position;
        }
        other => panic!("expected FormulaParse, got {other:?}"),
    }
}

#[test]
fn malformed_input_carries_upstream_position_when_available() {
    let err = parse("1+SUM(A,").unwrap_err();
    match err {
        xlstream_core::XlStreamError::FormulaParse { position, .. } => {
            assert!(position.is_some(), "expected upstream to report position");
        }
        other => panic!("expected FormulaParse, got {other:?}"),
    }
}
