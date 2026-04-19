//! Evaluator-level integration tests for aggregate `PreludeRef` resolution.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

use std::collections::HashMap;

use xlstream_core::{CellError, Value};
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::{classify, parse, rewrite, AggKind, AggregateKey, ClassificationContext};

fn eval_aggregate(formula: &str, aggs: HashMap<AggregateKey, Value>) -> Value {
    let prelude = Prelude::with_aggregates(aggs);
    let interp = Interpreter::new(&prelude);
    let ast = parse(formula).unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 10);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let scope = RowScope::new(&[], 0);
    interp.eval(rewritten.root(), &scope)
}

#[test]
fn sum_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 }, Value::Number(1000.0));
    assert_eq!(eval_aggregate("SUM(A:A)", aggs), Value::Number(1000.0));
}

#[test]
fn count_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(AggregateKey { kind: AggKind::Count, sheet: None, column: 1 }, Value::Number(50.0));
    assert_eq!(eval_aggregate("COUNT(A:A)", aggs), Value::Number(50.0));
}

#[test]
fn average_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(
        AggregateKey { kind: AggKind::Average, sheet: None, column: 1 },
        Value::Number(20.0),
    );
    assert_eq!(eval_aggregate("AVERAGE(A:A)", aggs), Value::Number(20.0));
}

#[test]
fn min_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(AggregateKey { kind: AggKind::Min, sheet: None, column: 1 }, Value::Number(1.0));
    assert_eq!(eval_aggregate("MIN(A:A)", aggs), Value::Number(1.0));
}

#[test]
fn max_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(AggregateKey { kind: AggKind::Max, sheet: None, column: 1 }, Value::Number(99.0));
    assert_eq!(eval_aggregate("MAX(A:A)", aggs), Value::Number(99.0));
}

#[test]
fn product_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(
        AggregateKey { kind: AggKind::Product, sheet: None, column: 1 },
        Value::Number(120.0),
    );
    assert_eq!(eval_aggregate("PRODUCT(A:A)", aggs), Value::Number(120.0));
}

#[test]
fn median_whole_column_returns_precomputed_scalar() {
    let mut aggs = HashMap::new();
    aggs.insert(AggregateKey { kind: AggKind::Median, sheet: None, column: 1 }, Value::Number(5.0));
    assert_eq!(eval_aggregate("MEDIAN(A:A)", aggs), Value::Number(5.0));
}

#[test]
fn mixed_formula_cell_div_sum_times_hundred() {
    let mut aggs = HashMap::new();
    aggs.insert(AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 }, Value::Number(1000.0));
    let prelude = Prelude::with_aggregates(aggs);
    let interp = Interpreter::new(&prelude);

    let ast = parse("A2/SUM(A:A)*100").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);

    let row = vec![Value::Number(50.0)];
    let scope = RowScope::new(&row, 1);
    let result = interp.eval(rewritten.root(), &scope);
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn missing_aggregate_prelude_ref_returns_value_error() {
    let key = AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 };
    let prelude = Prelude::empty();
    assert_eq!(prelude.get_aggregate(&key), Value::Error(CellError::Value));
}
