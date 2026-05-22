//! Integration tests: formulas that classify as `AggregateOnly`.

use xlstream_parse::{classify, parse, Classification, ClassificationContext};

fn real_meta(name: &str) -> Option<&xlstream_parse::FunctionMeta> {
    xlstream_eval::registry::lookup_meta(name)
}

#[test]
fn sum_whole_column_is_aggregate_only() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}

#[test]
fn sumif_with_static_criterion_is_aggregate_only() {
    let ast = parse("SUMIF(A:A, \"EMEA\", B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}

#[test]
fn count_whole_column_is_aggregate_only() {
    let ast = parse("COUNT(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}

#[test]
fn nested_aggregates_are_still_aggregate_only() {
    let ast = parse("SUM(A:A)+COUNT(B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}

#[test]
fn aggregate_functions_have_aggregate_category_in_registry() {
    use xlstream_parse::FnCategory;
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
        let meta = real_meta(name).unwrap_or_else(|| panic!("{name} missing from registry"));
        assert_eq!(meta.category, FnCategory::Aggregate, "{name} should be Aggregate");
    }
}

#[test]
fn aggregate_with_range_on_main_streaming_sheet_is_aggregate_only() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 10, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}

#[test]
fn sumif_with_row_varying_criterion_classifies_as_mixed() {
    let ast = parse("SUMIF(A:A, B2, C:C)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::Mixed);
}

#[test]
fn countif_with_row_varying_criterion_classifies_as_mixed() {
    let ast = parse("COUNTIF(A:A, A2)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::Mixed);
}

#[test]
fn sumifs_with_row_varying_criterion_classifies_as_mixed() {
    let ast = parse("SUMIFS(C:C, A:A, D2, B:B, E2)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 6);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::Mixed);
}
