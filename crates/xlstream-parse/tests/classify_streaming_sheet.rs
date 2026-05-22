//! Integration tests: lookup/aggregate behaviour against the streaming sheet.

use xlstream_parse::{classify, parse, Classification, ClassificationContext, UnsupportedReason};

fn real_meta(name: &str) -> Option<&xlstream_parse::FunctionMeta> {
    xlstream_eval::registry::lookup_meta(name)
}

#[test]
fn vlookup_with_unqualified_range_resolves_to_streaming_sheet_and_is_refused() {
    let ast = parse("VLOOKUP(A2, A:C, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx, &real_meta),
        Classification::Unsupported(UnsupportedReason::LookupIntoStreamingSheet)
    );
}

#[test]
fn vlookup_explicitly_into_main_sheet_is_refused() {
    let ast = parse("VLOOKUP(A2, Sheet1!A:C, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx, &real_meta),
        Classification::Unsupported(UnsupportedReason::LookupIntoStreamingSheet)
    );
}

#[test]
fn aggregate_with_unqualified_range_on_main_sheet_is_aggregate_only() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}

#[test]
fn aggregate_explicitly_into_main_sheet_is_aggregate_only() {
    let ast = parse("SUM(Sheet1!A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx, &real_meta), Classification::AggregateOnly);
}
