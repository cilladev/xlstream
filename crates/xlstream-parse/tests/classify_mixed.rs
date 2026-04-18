//! Integration tests: formulas that classify as `Mixed`.

use xlstream_parse::{classify, parse, Classification, ClassificationContext};

#[test]
fn deal_value_over_sum_is_mixed() {
    let ast = parse("A2/SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}

#[test]
fn lookup_plus_row_local_is_mixed() {
    let ast = parse("VLOOKUP(A2, 'Rates'!A:B, 2, FALSE) + B2").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Rates");
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}

#[test]
fn aggregate_plus_lookup_is_mixed() {
    let ast = parse("SUM(A:A) + VLOOKUP(B2, 'Rates'!A:B, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Rates");
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}

#[test]
fn nested_if_with_aggregate_branch_is_mixed() {
    let ast = parse("IF(A2>0, SUM(B:B), 0)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::Mixed);
}
