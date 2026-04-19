//! Evaluator-level integration tests for math builtins.

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

fn assert_approx(actual: Value, expected: f64, tol: f64) {
    match actual {
        Value::Number(n) => {
            assert!(
                (n - expected).abs() < tol,
                "expected {expected}, got {n} (diff {})",
                (n - expected).abs()
            );
        }
        other => panic!("expected Number, got {other:?}"),
    }
}

// ===== ROUND =====

#[test]
fn round_half_away_from_zero_through_evaluator() {
    assert_eq!(eval_formula("ROUND(0.5, 0)", &[]), Value::Number(1.0));
}

#[test]
fn round_negative_half_through_evaluator() {
    assert_eq!(eval_formula("ROUND(-0.5, 0)", &[]), Value::Number(-1.0));
}

#[test]
fn round_negative_digits_through_evaluator() {
    assert_eq!(eval_formula("ROUND(1234, -2)", &[]), Value::Number(1200.0));
}

#[test]
fn round_cell_ref() {
    let row = vec![Value::Number(2.5)];
    assert_eq!(eval_formula("ROUND(A1, 0)", &row), Value::Number(3.0));
}

// ===== INT =====

#[test]
fn int_negative_floors_through_evaluator() {
    assert_eq!(eval_formula("INT(-1.5)", &[]), Value::Number(-2.0));
}

#[test]
fn int_positive_through_evaluator() {
    assert_eq!(eval_formula("INT(1.9)", &[]), Value::Number(1.0));
}

// ===== MOD =====

#[test]
fn mod_sign_of_divisor_through_evaluator() {
    assert_eq!(eval_formula("MOD(-3, 2)", &[]), Value::Number(1.0));
}

#[test]
fn mod_negative_divisor_through_evaluator() {
    assert_eq!(eval_formula("MOD(3, -2)", &[]), Value::Number(-1.0));
}

#[test]
fn mod_zero_divisor_through_evaluator() {
    assert_eq!(eval_formula("MOD(5, 0)", &[]), Value::Error(CellError::Div0));
}

// ===== PI =====

#[test]
fn pi_through_evaluator() {
    assert_approx(eval_formula("PI()", &[]), std::f64::consts::PI, 1e-15);
}

#[test]
fn pi_in_expression() {
    // PI()*2 should be approximately 2*pi
    assert_approx(eval_formula("PI()*2", &[]), 2.0 * std::f64::consts::PI, 1e-14);
}

// ===== POWER =====

#[test]
fn power_through_evaluator() {
    assert_eq!(eval_formula("POWER(2, 10)", &[]), Value::Number(1024.0));
}

#[test]
fn power_fractional_exp() {
    assert_eq!(eval_formula("POWER(4, 0.5)", &[]), Value::Number(2.0));
}

// ===== SQRT =====

#[test]
fn sqrt_through_evaluator() {
    assert_eq!(eval_formula("SQRT(16)", &[]), Value::Number(4.0));
}

#[test]
fn sqrt_negative_through_evaluator() {
    assert_eq!(eval_formula("SQRT(-1)", &[]), Value::Error(CellError::Num));
}

// ===== ABS =====

#[test]
fn abs_through_evaluator() {
    assert_eq!(eval_formula("ABS(-42)", &[]), Value::Number(42.0));
}

// ===== CEILING / FLOOR =====

#[test]
fn ceiling_through_evaluator() {
    assert_eq!(eval_formula("CEILING(2.5, 1)", &[]), Value::Number(3.0));
}

#[test]
fn floor_through_evaluator() {
    assert_eq!(eval_formula("FLOOR(2.5, 1)", &[]), Value::Number(2.0));
}

// ===== LOG / LN =====

#[test]
fn log10_through_evaluator() {
    assert_eq!(eval_formula("LOG(100)", &[]), Value::Number(2.0));
}

#[test]
fn ln_through_evaluator() {
    assert_approx(eval_formula("LN(1)", &[]), 0.0, 1e-15);
}

// ===== TRIG =====

#[test]
fn sin_pi_half_through_evaluator() {
    assert_approx(eval_formula("SIN(PI()/2)", &[]), 1.0, 1e-14);
}

#[test]
fn cos_pi_through_evaluator() {
    assert_approx(eval_formula("COS(PI())", &[]), -1.0, 1e-14);
}

// ===== Compound expressions =====

#[test]
fn round_with_cell_and_arithmetic() {
    let row = vec![Value::Number(2.567)];
    assert_eq!(eval_formula("ROUND(A1, 2)", &row), Value::Number(2.57));
}

#[test]
fn nested_math_functions() {
    // SQRT(POWER(3, 2) + POWER(4, 2)) = SQRT(25) = 5
    assert_eq!(eval_formula("SQRT(POWER(3, 2) + POWER(4, 2))", &[]), Value::Number(5.0),);
}
