//! Integration tests: formulas that classify as `Unsupported`.

use xlstream_parse::{classify, parse, Classification, ClassificationContext, UnsupportedReason};

#[test]
fn offset_is_refused_with_unsupported_function() {
    let ast = parse("OFFSET(A1,1,0)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnsupportedFunction("OFFSET".into()))
    );
}

#[test]
fn indirect_is_refused() {
    let ast = parse("INDIRECT(\"A1\")").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnsupportedFunction("INDIRECT".into()))
    );
}

#[test]
fn non_current_row_ref_is_refused() {
    let ast = parse("A1+B1").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::NonCurrentRowRef)
    );
}

#[test]
fn circular_self_reference_is_refused() {
    // Cell E2 references E2
    let ast = parse("E2+1").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::Unsupported(UnsupportedReason::CircularRef));
}

#[test]
fn filter_dynamic_array_is_refused() {
    let ast = parse("FILTER(A:A, B:B>0)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnsupportedFunction("FILTER".into()))
    );
}

#[test]
fn unique_dynamic_array_is_refused() {
    let ast = parse("UNIQUE(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnsupportedFunction("UNIQUE".into()))
    );
}

#[test]
fn rand_volatile_is_refused() {
    let ast = parse("RAND()").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnsupportedFunction("RAND".into()))
    );
}

#[test]
fn bare_whole_column_is_refused() {
    let ast = parse("A:A*2").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnboundedRange)
    );
}

#[test]
fn nested_unsupported_propagates() {
    let ast = parse("IF(A2>0, OFFSET(A1,1,0), 0)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::UnsupportedFunction("OFFSET".into()))
    );
}

#[test]
fn external_reference_is_refused() {
    let ast = parse("[OtherBook.xlsx]Sheet1!A1").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::ExternalReference)
    );
}

#[test]
fn table_reference_is_refused() {
    let ast = parse("MyTable[Col1]").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::TableReference)
    );
}

#[test]
fn named_range_unknown_passes_classification_as_row_local() {
    let ast = parse("MyRange+1").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(classify(&ast, &ctx), Classification::RowLocal);
}
