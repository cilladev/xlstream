//! Evaluator-level integration tests for unary operators.

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

#[test]
fn negate_number() {
    assert_eq!(eval_formula("-5", &[]), Value::Number(-5.0));
}

#[test]
fn double_negate() {
    assert_eq!(eval_formula("-(-1)", &[]), Value::Number(1.0));
}

#[test]
fn negate_cell_ref() {
    let row = vec![Value::Number(7.0)];
    assert_eq!(eval_formula("-A1", &row), Value::Number(-7.0));
}

#[test]
fn unary_plus_identity() {
    assert_eq!(eval_formula("+5", &[]), Value::Number(5.0));
}

#[test]
fn percent() {
    assert_eq!(eval_formula("50%", &[]), Value::Number(0.5));
}

#[test]
fn negate_error_propagation() {
    let row = vec![Value::Error(CellError::Div0)];
    assert_eq!(eval_formula("-A1", &row), Value::Error(CellError::Div0));
}

#[test]
fn negate_with_arithmetic() {
    assert_eq!(eval_formula("-3+5", &[]), Value::Number(2.0));
}
