//! Evaluator-level integration tests for conditional and logical builtins.

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

// ===== IF =====

#[test]
fn if_short_circuit_avoids_div_zero() {
    let row = vec![Value::Number(0.0)];
    assert_eq!(eval_formula("IF(A1=0, 0, 1/A1)", &row), Value::Number(0.0));
}

#[test]
fn if_nonzero_cell_evaluates_division() {
    let row = vec![Value::Number(4.0)];
    assert_eq!(eval_formula("IF(A1=0, 0, 1/A1)", &row), Value::Number(0.25));
}

#[test]
fn if_with_comparison_and_text_result() {
    let row = vec![Value::Number(15000.0)];
    assert_eq!(eval_formula("IF(A1>10000, \"Big\", \"Small\")", &row), Value::Text("Big".into()));
}

// ===== IFS =====

#[test]
fn ifs_tiered_matching() {
    let row = vec![Value::Number(75000.0)];
    assert_eq!(
        eval_formula(
            "IFS(A1>100000, \"Platinum\", A1>50000, \"Gold\", A1>10000, \"Silver\", TRUE, \"Bronze\")",
            &row,
        ),
        Value::Text("Gold".into())
    );
}

#[test]
fn ifs_tiered_matching_lowest_tier() {
    let row = vec![Value::Number(5000.0)];
    assert_eq!(
        eval_formula(
            "IFS(A1>100000, \"Platinum\", A1>50000, \"Gold\", A1>10000, \"Silver\", TRUE, \"Bronze\")",
            &row,
        ),
        Value::Text("Bronze".into())
    );
}

// ===== SWITCH =====

#[test]
fn switch_with_cell_ref_match() {
    let row = vec![Value::Text("EMEA".into())];
    assert_eq!(
        eval_formula("SWITCH(A1, \"EMEA\", \"Europe\", \"APAC\", \"Asia\", \"Unknown\")", &row,),
        Value::Text("Europe".into())
    );
}

// ===== IFERROR =====

#[test]
fn iferror_catches_division_by_zero() {
    let row = vec![Value::Number(0.0)];
    assert_eq!(eval_formula("IFERROR(1/A1, \"err\")", &row), Value::Text("err".into()));
}

// ===== IFNA =====

#[test]
fn ifna_propagates_non_na_error() {
    let row = vec![Value::Number(0.0)];
    assert_eq!(eval_formula("IFNA(1/A1, \"err\")", &row), Value::Error(CellError::Div0));
}

// ===== Logical =====

#[test]
fn and_with_comparisons() {
    let row = vec![Value::Number(50.0)];
    assert_eq!(eval_formula("AND(A1>10, A1<100)", &row), Value::Bool(true));
}

#[test]
fn or_with_comparisons() {
    let row = vec![Value::Number(5.0)];
    assert_eq!(eval_formula("OR(A1>100, A1<10)", &row), Value::Bool(true));
}

#[test]
fn not_with_comparison() {
    let row = vec![Value::Number(5.0)];
    assert_eq!(eval_formula("NOT(A1>10)", &row), Value::Bool(true));
}

#[test]
fn nested_if_and_or() {
    let row = vec![Value::Number(50.0), Value::Number(80.0)];
    assert_eq!(
        eval_formula("IF(AND(A1>40, B1>70), \"pass\", \"fail\")", &row),
        Value::Text("pass".into())
    );
}

#[test]
fn nested_iferror_with_if() {
    let row = vec![Value::Number(0.0)];
    assert_eq!(
        eval_formula("IFERROR(IF(A1=0, 1/0, A1), \"safe\")", &row),
        Value::Text("safe".into())
    );
}

#[test]
fn xor_parity_check() {
    assert_eq!(eval_formula("XOR(TRUE, FALSE, TRUE)", &[]), Value::Bool(false));
}

#[test]
fn true_and_false_functions() {
    assert_eq!(eval_formula("IF(TRUE(), \"yes\", \"no\")", &[]), Value::Text("yes".into()));
    assert_eq!(eval_formula("IF(FALSE(), \"yes\", \"no\")", &[]), Value::Text("no".into()));
}

#[test]
fn case_insensitive_function_names() {
    assert_eq!(eval_formula("if(TRUE, 1, 2)", &[]), Value::Number(1.0));
    assert_eq!(eval_formula("If(TRUE, 1, 2)", &[]), Value::Number(1.0));
    assert_eq!(eval_formula("and(TRUE, TRUE)", &[]), Value::Bool(true));
}
