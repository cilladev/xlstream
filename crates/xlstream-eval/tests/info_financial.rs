//! Evaluator-level integration tests for info and financial builtins.

#![allow(clippy::unwrap_used, clippy::float_cmp)]

use xlstream_core::{CellError, Value};
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn eval_formula(formula: &str, row: &[Value]) -> Value {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let ast = parse(formula).unwrap();
    let scope = RowScope::new(row, 0);
    interp.eval(ast.root(), &scope)
}

// ===== Info builtins through evaluator =====

#[test]
fn isblank_empty_cell() {
    let row = vec![Value::Empty];
    assert_eq!(eval_formula("ISBLANK(A1)", &row), Value::Bool(true));
}

#[test]
fn isblank_number_cell() {
    let row = vec![Value::Number(0.0)];
    assert_eq!(eval_formula("ISBLANK(A1)", &row), Value::Bool(false));
}

#[test]
fn isnumber_through_evaluator() {
    let row = vec![Value::Number(42.0)];
    assert_eq!(eval_formula("ISNUMBER(A1)", &row), Value::Bool(true));
}

#[test]
fn istext_through_evaluator() {
    let row = vec![Value::Text("hello".into())];
    assert_eq!(eval_formula("ISTEXT(A1)", &row), Value::Bool(true));
}

#[test]
fn na_through_evaluator() {
    assert_eq!(eval_formula("NA()", &[]), Value::Error(CellError::Na));
}

#[test]
fn iferror_catches_na() {
    assert_eq!(eval_formula("IFERROR(NA(), 0)", &[]), Value::Number(0.0));
}

#[test]
fn iserror_on_division_by_zero() {
    let row = vec![Value::Number(0.0)];
    assert_eq!(eval_formula("ISERROR(1/A1)", &row), Value::Bool(true));
}

#[test]
fn isna_on_na() {
    assert_eq!(eval_formula("ISNA(NA())", &[]), Value::Bool(true));
}

#[test]
fn type_on_number() {
    let row = vec![Value::Number(42.0)];
    assert_eq!(eval_formula("TYPE(A1)", &row), Value::Number(1.0));
}

#[test]
fn type_on_text() {
    let row = vec![Value::Text("hello".into())];
    assert_eq!(eval_formula("TYPE(A1)", &row), Value::Number(2.0));
}

#[test]
fn type_on_bool() {
    assert_eq!(eval_formula("TYPE(TRUE)", &[]), Value::Number(4.0));
}

#[test]
fn islogical_through_evaluator() {
    assert_eq!(eval_formula("ISLOGICAL(TRUE)", &[]), Value::Bool(true));
}

#[test]
fn isnontext_through_evaluator() {
    let row = vec![Value::Number(1.0)];
    assert_eq!(eval_formula("ISNONTEXT(A1)", &row), Value::Bool(true));
}

// ===== Financial builtins through evaluator =====

#[test]
fn pmt_zero_rate_through_evaluator() {
    let result = eval_formula("PMT(0, 12, 1200)", &[]);
    match result {
        Value::Number(n) => assert!((n - (-100.0)).abs() < 0.01),
        other => panic!("expected Number, got {other:?}"),
    }
}

#[test]
fn pmt_standard_loan_through_evaluator() {
    let result = eval_formula("PMT(0.05/12, 360, 200000)", &[]);
    match result {
        Value::Number(n) => assert!((n - (-1073.64)).abs() < 0.01),
        other => panic!("expected Number, got {other:?}"),
    }
}

#[test]
fn pv_through_evaluator() {
    let result = eval_formula("PV(0, 12, -100)", &[]);
    match result {
        Value::Number(n) => assert!((n - 1200.0).abs() < 0.01),
        other => panic!("expected Number, got {other:?}"),
    }
}

#[test]
fn fv_through_evaluator() {
    let result = eval_formula("FV(0, 12, -100)", &[]);
    match result {
        Value::Number(n) => assert!((n - 1200.0).abs() < 0.01),
        other => panic!("expected Number, got {other:?}"),
    }
}

#[test]
fn npv_through_evaluator() {
    let result = eval_formula("NPV(0.10, -1000, 300, 400, 500)", &[]);
    match result {
        Value::Number(n) => assert!((n - (-19.12)).abs() < 1.0),
        other => panic!("expected Number, got {other:?}"),
    }
}
