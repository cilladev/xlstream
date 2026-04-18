//! Integration tests: formulas that classify as `AggregateOnly`.

use xlstream_parse::{classify, parse, Classification, ClassificationContext};

#[test]
fn sum_whole_column_is_aggregate_only() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn sumif_with_static_criterion_is_aggregate_only() {
    let ast = parse("SUMIF(A:A, \"EMEA\", B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn count_whole_column_is_aggregate_only() {
    let ast = parse("COUNT(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn nested_aggregates_are_still_aggregate_only() {
    let ast = parse("SUM(A:A)+COUNT(B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}

#[test]
fn aggregate_set_recognises_each_listed_function() {
    use xlstream_parse::sets::is_aggregate;
    for name in [
        "SUM",
        "COUNT",
        "COUNTA",
        "AVERAGE",
        "MIN",
        "MAX",
        "PRODUCT",
        "SUMIF",
        "COUNTIF",
        "AVERAGEIF",
        "SUMIFS",
        "COUNTIFS",
        "AVERAGEIFS",
        "MINIFS",
        "MAXIFS",
        "MEDIAN",
    ] {
        assert!(is_aggregate(name), "{name} should be aggregate");
    }
}

#[test]
fn aggregate_with_range_on_main_streaming_sheet_is_aggregate_only() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 10, 5);
    assert_eq!(classify(&ast, &ctx), Classification::AggregateOnly);
}
