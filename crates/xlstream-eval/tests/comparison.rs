//! Evaluator-level integration tests for comparison operators.

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
fn eq_numbers() {
    assert_eq!(eval_formula("1=1", &[]), Value::Bool(true));
}

#[test]
fn eq_different_numbers() {
    assert_eq!(eval_formula("1=2", &[]), Value::Bool(false));
}

#[test]
fn ne_numbers() {
    assert_eq!(eval_formula("1<>2", &[]), Value::Bool(true));
}

#[test]
fn lt_numbers() {
    assert_eq!(eval_formula("1<2", &[]), Value::Bool(true));
}

#[test]
fn gt_numbers() {
    assert_eq!(eval_formula("3>2", &[]), Value::Bool(true));
}

#[test]
fn le_equal() {
    assert_eq!(eval_formula("5<=5", &[]), Value::Bool(true));
}

#[test]
fn ge_equal() {
    assert_eq!(eval_formula("5>=5", &[]), Value::Bool(true));
}

#[test]
fn text_case_insensitive_eq() {
    assert_eq!(eval_formula("\"a\"=\"A\"", &[]), Value::Bool(true));
}

#[test]
fn ieee_rounding_zero_point_one_plus_zero_point_two() {
    assert_eq!(eval_formula("(0.1+0.2)=0.3", &[]), Value::Bool(true));
}

#[test]
fn ieee_rounding_one_third_times_three() {
    assert_eq!(eval_formula("(1/3*3)=1", &[]), Value::Bool(true));
}

#[test]
fn text_lexicographic_ten_vs_nine() {
    assert_eq!(eval_formula("\"10\">\"9\"", &[]), Value::Bool(false));
}

#[test]
fn number_less_than_text_by_tier() {
    // Number < Text always in Excel (type ordering, not coercion)
    assert_eq!(eval_formula("1<\"2\"", &[]), Value::Bool(true));
}

#[test]
fn number_not_equal_to_numeric_text() {
    // 1 = "1" is FALSE — different type tiers
    assert_eq!(eval_formula("1=\"1\"", &[]), Value::Bool(false));
}

#[test]
fn bool_outranks_number() {
    // TRUE > 1000 — boolean tier > number tier
    assert_eq!(eval_formula("TRUE>1000", &[]), Value::Bool(true));
}

#[test]
fn bool_not_equal_to_one() {
    // TRUE = 1 is FALSE — different type tiers
    assert_eq!(eval_formula("TRUE=1", &[]), Value::Bool(false));
}

#[test]
fn comparison_error_propagation() {
    let row = vec![Value::Error(CellError::Div0)];
    assert_eq!(eval_formula("A1=1", &row), Value::Error(CellError::Div0));
}

#[test]
fn comparison_with_cell_refs() {
    let row = vec![Value::Number(10.0), Value::Number(20.0)];
    assert_eq!(eval_formula("A1<B1", &row), Value::Bool(true));
}
