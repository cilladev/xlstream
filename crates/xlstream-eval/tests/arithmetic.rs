//! Evaluator-level integration tests for arithmetic operators.

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
fn add_two_numbers() {
    assert_eq!(eval_formula("1+2", &[]), Value::Number(3.0));
}

#[test]
fn subtract_two_numbers() {
    assert_eq!(eval_formula("10-3", &[]), Value::Number(7.0));
}

#[test]
fn multiply_two_numbers() {
    assert_eq!(eval_formula("4*5", &[]), Value::Number(20.0));
}

#[test]
fn divide_two_numbers() {
    assert_eq!(eval_formula("15/3", &[]), Value::Number(5.0));
}

#[test]
fn divide_by_zero() {
    assert_eq!(eval_formula("1/0", &[]), Value::Error(CellError::Div0));
}

#[test]
fn power() {
    assert_eq!(eval_formula("2^10", &[]), Value::Number(1024.0));
}

#[test]
fn chained_arithmetic() {
    assert_eq!(eval_formula("2+3*4", &[]), Value::Number(14.0));
}

#[test]
fn parenthesized_expression() {
    assert_eq!(eval_formula("(2+3)*4", &[]), Value::Number(20.0));
}

#[test]
fn arithmetic_with_cell_ref() {
    let row = vec![Value::Number(10.0), Value::Number(3.0)];
    assert_eq!(eval_formula("A1*B1", &row), Value::Number(30.0));
}

#[test]
fn arithmetic_with_bool_coercion() {
    assert_eq!(eval_formula("TRUE+1", &[]), Value::Number(2.0));
}

#[test]
fn arithmetic_with_text_coercion() {
    assert_eq!(eval_formula("10+\"5\"", &[]), Value::Number(15.0));
}

#[test]
fn arithmetic_type_mismatch() {
    assert_eq!(eval_formula("1+\"abc\"", &[]), Value::Error(CellError::Value));
}

#[test]
fn arithmetic_error_propagation_from_cell() {
    let row = vec![Value::Error(CellError::Div0), Value::Number(1.0)];
    assert_eq!(eval_formula("A1+B1", &row), Value::Error(CellError::Div0));
}

#[test]
fn zero_to_the_zero() {
    assert_eq!(eval_formula("0^0", &[]), Value::Number(1.0));
}
