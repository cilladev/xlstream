//! Evaluator-level integration tests for the & (concatenation) operator.

#![allow(clippy::unwrap_used)]

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
fn concat_two_text_literals() {
    assert_eq!(eval_formula("\"a\"&\"b\"", &[]), Value::Text("ab".into()));
}

#[test]
fn concat_three_texts() {
    assert_eq!(eval_formula("\"a\"&\"b\"&\"c\"", &[]), Value::Text("abc".into()));
}

#[test]
fn concat_number_and_text() {
    assert_eq!(eval_formula("10&\"5\"", &[]), Value::Text("105".into()));
}

#[test]
fn concat_with_cell_ref() {
    let row = vec![Value::Text("hello".into()), Value::Text(" world".into())];
    assert_eq!(eval_formula("A1&B1", &row), Value::Text("hello world".into()));
}

#[test]
fn concat_error_propagation() {
    let row = vec![Value::Error(CellError::Ref)];
    assert_eq!(eval_formula("A1&\"x\"", &row), Value::Error(CellError::Ref));
}

#[test]
fn concat_empty_cells() {
    let row = vec![Value::Empty, Value::Empty];
    assert_eq!(eval_formula("A1&B1", &row), Value::Text("".into()));
}
