//! Integration tests: formulas that classify as `RowLocal`.

use xlstream_parse::{classify, parse, Classification, ClassificationContext};

#[test]
fn literal_arithmetic_is_row_local() {
    let ast = parse("1+2").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn current_row_cell_arithmetic_is_row_local() {
    let ast = parse("A2*B2").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn if_with_row_local_branches_is_row_local() {
    let ast = parse("IF(A2>0, \"Y\", \"N\")").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 3);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn upper_left_year_text_concat_is_row_local() {
    let ast = parse("UPPER(LEFT(A2,3))&\"-\"&YEAR(B2)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn today_volatile_streaming_ok_is_row_local() {
    let ast = parse("TODAY()").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 1, 1);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn round_is_row_local() {
    let ast = parse("ROUND(A2, 2)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}

#[test]
fn lowercase_function_name_classifies_correctly() {
    let ast = parse("sum(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}
