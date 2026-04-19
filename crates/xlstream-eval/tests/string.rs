//! Evaluator-level integration tests for string builtins.

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

// ===== LEFT =====

#[test]
fn left_with_cell_ref() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(eval_formula("LEFT(A1, 3)", &row), Value::Text("Hel".into()));
}

#[test]
fn left_default_on_cell_ref() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(eval_formula("LEFT(A1)", &row), Value::Text("H".into()));
}

#[test]
fn left_with_literal() {
    assert_eq!(eval_formula("LEFT(\"World\", 2)", &[]), Value::Text("Wo".into()));
}

// ===== RIGHT =====

#[test]
fn right_with_cell_ref() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(eval_formula("RIGHT(A1, 3)", &row), Value::Text("llo".into()));
}

#[test]
fn right_default_on_cell_ref() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(eval_formula("RIGHT(A1)", &row), Value::Text("o".into()));
}

// ===== MID =====

#[test]
fn mid_with_cell_ref() {
    let row = vec![Value::Text("Hello World".into())];
    assert_eq!(eval_formula("MID(A1, 7, 5)", &row), Value::Text("World".into()));
}

#[test]
fn mid_with_literal() {
    assert_eq!(eval_formula("MID(\"abcdef\", 2, 3)", &[]), Value::Text("bcd".into()));
}

// ===== LEN =====

#[test]
fn len_with_cell_ref() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(eval_formula("LEN(A1)", &row), Value::Number(5.0));
}

#[test]
fn len_empty_cell() {
    assert_eq!(eval_formula("LEN(A1)", &[Value::Empty]), Value::Number(0.0));
}

// ===== UPPER =====

#[test]
fn upper_with_cell_ref() {
    let row = vec![Value::Text("hello world".into())];
    assert_eq!(eval_formula("UPPER(A1)", &row), Value::Text("HELLO WORLD".into()));
}

// ===== CONCATENATE =====

#[test]
fn concatenate_two_cells() {
    let row = vec![Value::Text("Hello".into()), Value::Text(" World".into())];
    assert_eq!(eval_formula("CONCATENATE(A1, B1)", &row), Value::Text("Hello World".into()));
}

#[test]
fn concatenate_mixed_types() {
    let row = vec![Value::Text("Score: ".into()), Value::Number(42.0)];
    assert_eq!(eval_formula("CONCATENATE(A1, B1)", &row), Value::Text("Score: 42".into()));
}

#[test]
fn concat_alias_works() {
    assert_eq!(eval_formula("CONCAT(\"a\", \"b\", \"c\")", &[]), Value::Text("abc".into()));
}

// ===== FIND =====

#[test]
fn find_in_cell_ref() {
    let row = vec![Value::Text("Hello World".into())];
    assert_eq!(eval_formula("FIND(\"World\", A1)", &row), Value::Number(7.0));
}

#[test]
fn find_not_found_returns_value_error() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(eval_formula("FIND(\"xyz\", A1)", &row), Value::Error(CellError::Value));
}

// ===== SUBSTITUTE =====

#[test]
fn substitute_all_occurrences() {
    let row = vec![Value::Text("aabaa".into())];
    assert_eq!(eval_formula("SUBSTITUTE(A1, \"a\", \"X\")", &row), Value::Text("XXbXX".into()));
}

#[test]
fn substitute_nth_occurrence() {
    let row = vec![Value::Text("aabaa".into())];
    assert_eq!(eval_formula("SUBSTITUTE(A1, \"a\", \"X\", 3)", &row), Value::Text("aabXa".into()));
}

// ===== TRIM =====

#[test]
fn trim_with_cell_ref() {
    let row = vec![Value::Text("  hello   world  ".into())];
    assert_eq!(eval_formula("TRIM(A1)", &row), Value::Text("hello world".into()));
}

// ===== VALUE =====

#[test]
fn value_parses_numeric_text() {
    let row = vec![Value::Text("42.5".into())];
    assert_eq!(eval_formula("VALUE(A1)", &row), Value::Number(42.5));
}

#[test]
fn value_non_numeric_returns_error() {
    let row = vec![Value::Text("abc".into())];
    assert_eq!(eval_formula("VALUE(A1)", &row), Value::Error(CellError::Value));
}

// ===== Nested / combined =====

#[test]
fn upper_of_left() {
    let row = vec![Value::Text("hello world".into())];
    assert_eq!(eval_formula("UPPER(LEFT(A1, 5))", &row), Value::Text("HELLO".into()));
}

#[test]
fn len_of_concatenate() {
    assert_eq!(
        eval_formula("LEN(CONCATENATE(\"hello\", \" \", \"world\"))", &[]),
        Value::Number(11.0)
    );
}

#[test]
fn if_with_len() {
    let row = vec![Value::Text("Hi".into())];
    assert_eq!(
        eval_formula("IF(LEN(A1)>3, \"long\", \"short\")", &row),
        Value::Text("short".into())
    );
}

#[test]
fn iferror_with_find() {
    let row = vec![Value::Text("Hello".into())];
    assert_eq!(
        eval_formula("IFERROR(FIND(\"xyz\", A1), \"not found\")", &row),
        Value::Text("not found".into())
    );
}

#[test]
fn case_insensitive_function_names() {
    assert_eq!(eval_formula("left(\"Hello\", 2)", &[]), Value::Text("He".into()));
    assert_eq!(eval_formula("UPPER(\"hello\")", &[]), Value::Text("HELLO".into()));
}

// ===== TEXT date format =====

#[test]
fn text_date_format_through_evaluator() {
    let serial = xlstream_core::ExcelDate::from_ymd(2026, 4, 15).serial;
    let row = vec![Value::Number(serial)];
    assert_eq!(eval_formula("TEXT(A1, \"yyyy-mm-dd\")", &row), Value::Text("2026-04-15".into()));
}
