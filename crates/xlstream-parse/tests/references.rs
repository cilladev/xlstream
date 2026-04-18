//! Integration tests for `extract_references`.

use xlstream_parse::{extract_references, parse, Reference};

#[test]
fn single_cell_extracts_one_cell_ref() {
    let ast = parse("A1").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.cells.len(), 1);
    assert!(matches!(refs.cells[0], Reference::Cell { row: 1, col: 1, .. }));
    assert!(refs.ranges.is_empty());
    assert!(refs.functions.is_empty());
}

#[test]
fn range_extracts_one_range_ref() {
    let ast = parse("A1:B10").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.ranges.len(), 1);
}

#[test]
fn cross_sheet_dedupes_sheet_names() {
    let ast = parse("'Tax Rates'!A1 + 'Tax Rates'!B2").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.sheets.iter().filter(|s| s.as_str() == "Tax Rates").count(), 1);
}

#[test]
fn whole_column_extracts_range() {
    let ast = parse("SUM(A:A)").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.ranges.len(), 1);
    assert!(refs.ranges[0].is_whole_column());
}

#[test]
fn nested_function_collects_all_function_names() {
    let ast = parse("IF(SUM(A1:A10) > 0, VLOOKUP(B1, X:Y, 2, FALSE), 0)").unwrap();
    let refs = extract_references(&ast);
    let names: Vec<&str> = refs.functions.iter().map(String::as_str).collect();
    assert!(names.contains(&"IF"));
    assert!(names.contains(&"SUM"));
    assert!(names.contains(&"VLOOKUP"));
}

#[test]
fn function_names_dedup_case_insensitively_keeping_first_seen() {
    let ast = parse("Sum(A1) + SUM(A2) + sum(A3)").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.functions.len(), 1);
    assert_eq!(refs.functions[0], "Sum");
}

#[test]
fn external_reference_lands_in_ranges() {
    let ast = parse("[Book2]Sheet1!A1").unwrap();
    let refs = extract_references(&ast);
    assert_eq!(refs.ranges.len(), 1);
    assert!(matches!(refs.ranges[0], Reference::External { .. }));
}
