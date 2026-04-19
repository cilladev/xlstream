//! Operator precedence and interaction tests.

#![allow(clippy::unwrap_used, clippy::float_cmp)]

use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn eval_formula(formula: &str, row: &[Value]) -> Value {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let ast = parse(formula).unwrap();
    let scope = RowScope::new(row, 0);
    interp.eval(ast.root(), &scope)
}

// -- Arithmetic precedence --

#[test]
fn mul_before_add() {
    assert_eq!(eval_formula("1+2*3", &[]), Value::Number(7.0));
}

#[test]
fn parens_override_precedence() {
    assert_eq!(eval_formula("(1+2)*3", &[]), Value::Number(9.0));
}

#[test]
fn div_before_sub() {
    assert_eq!(eval_formula("10-6/3", &[]), Value::Number(8.0));
}

#[test]
fn power_before_mul() {
    assert_eq!(eval_formula("2*3^2", &[]), Value::Number(18.0));
}

// -- The -2^2 precedence --
// NOTE: Excel evaluates -2^2 as (-2)^2 = 4. However, formualizer-parse
// gives ^ higher precedence than unary minus (standard math), so
// -2^2 parses as -(2^2) = -4. This is a known parser divergence from
// Excel; fixing it is a Phase 2 concern, not Phase 5.

#[test]
fn negative_two_squared_follows_parser_precedence() {
    // Parser: -2^2 = -(2^2) = -4 (^ binds tighter than unary -)
    assert_eq!(eval_formula("-2^2", &[]), Value::Number(-4.0));
}

#[test]
fn explicit_parens_negative_two_squared() {
    // Explicit parens: (-2)^2 = 4
    assert_eq!(eval_formula("(-2)^2", &[]), Value::Number(4.0));
}

#[test]
fn double_negate_is_identity() {
    assert_eq!(eval_formula("-(-1)", &[]), Value::Number(1.0));
}

// -- Mixed operator types --

#[test]
fn comparison_lowest_precedence() {
    assert_eq!(eval_formula("1+2=3", &[]), Value::Bool(true));
}

#[test]
fn percent_high_precedence() {
    assert_eq!(eval_formula("50%*200", &[]), Value::Number(100.0));
}

// -- Complex expressions --

#[test]
fn complex_arithmetic_with_cells() {
    let row = vec![Value::Number(3.0), Value::Number(4.0), Value::Number(5.0)];
    assert_eq!(eval_formula("A1*B1+C1", &row), Value::Number(17.0));
}

#[test]
fn revenue_minus_cost_div_quantity() {
    let row = vec![Value::Number(100.0), Value::Number(60.0), Value::Number(8.0)];
    assert_eq!(eval_formula("(A1-B1)/C1", &row), Value::Number(5.0));
}

#[test]
fn concat_with_computed_values() {
    let row = vec![Value::Number(5.0)];
    assert_eq!(eval_formula("(A1*2)&\" units\"", &row), Value::Text("10 units".into()));
}

// -- IEEE rounding in comparison --

#[test]
fn ieee_rounding_point_one_plus_point_two_equals_point_three() {
    assert_eq!(eval_formula("(0.1+0.2)=0.3", &[]), Value::Bool(true));
}

#[test]
fn ieee_rounding_one_over_three_times_three_equals_one() {
    assert_eq!(eval_formula("(1/3*3)=1", &[]), Value::Bool(true));
}

// -- Edge cases --

#[test]
fn zero_power_zero() {
    assert_eq!(eval_formula("0^0", &[]), Value::Number(1.0));
}

#[test]
fn empty_concat_empty() {
    let row = vec![Value::Empty, Value::Empty];
    assert_eq!(eval_formula("A1&B1", &row), Value::Text("".into()));
}

#[test]
fn text_coercion_in_arithmetic() {
    assert_eq!(eval_formula("\"5\"+\"3\"", &[]), Value::Number(8.0));
}

#[test]
fn text_and_number_concat() {
    assert_eq!(eval_formula("10&\"5\"", &[]), Value::Text("105".into()));
}

#[test]
fn bool_in_arithmetic() {
    assert_eq!(eval_formula("TRUE+TRUE", &[]), Value::Number(2.0));
}

// -- Parser precedence verification --

#[test]
fn negative_two_squared_parser_gives_caret_higher_precedence() {
    use xlstream_parse::NodeView;
    let ast = parse("-2^2").unwrap();
    // Parser: root is UnaryOp(-) wrapping BinaryOp(^)
    // This means ^ binds tighter than unary - (standard math, not Excel)
    match ast.root().view() {
        NodeView::UnaryOp { op: "-" } => {
            let operand = ast.root().operand().unwrap();
            assert!(
                matches!(operand.view(), NodeView::BinaryOp { op: "^" }),
                "expected BinaryOp(^), got {:?}",
                operand.view()
            );
        }
        other => panic!("expected UnaryOp(-), got {other:?}"),
    }
}
