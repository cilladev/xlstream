//! Integration tests: formulas that classify as `LookupOnly`.

use xlstream_parse::{classify, parse, Classification, ClassificationContext, UnsupportedReason};

#[test]
fn vlookup_into_lookup_sheet_is_lookup_only() {
    let ast = parse("VLOOKUP(A2, 'Region Info'!A:C, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Region Info");
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn xlookup_into_lookup_sheet_is_lookup_only() {
    let ast = parse("XLOOKUP(A2, 'Region Info'!A:A, 'Region Info'!B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Region Info");
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn vlookup_with_concat_key_is_lookup_only() {
    let ast = parse("VLOOKUP(A2&B2, 'Tax Rates'!D:E, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Tax Rates");
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn match_into_lookup_sheet_is_lookup_only() {
    let ast = parse("MATCH(A2, 'Region Info'!A:A, 0)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Region Info");
    assert_eq!(classify(&ast, &ctx), Classification::LookupOnly);
}

#[test]
fn lookup_into_unprepared_sheet_is_refused() {
    let ast = parse("VLOOKUP(A2, 'Unknown'!A:C, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    assert_eq!(
        classify(&ast, &ctx),
        Classification::Unsupported(UnsupportedReason::LookupSheetNotPrepared)
    );
}
