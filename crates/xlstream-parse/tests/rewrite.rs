//! Integration tests: AST rewrite golden tests.

use xlstream_parse::{
    classify, collect_lookup_keys, parse, rewrite, AggKind, AggregateKey, Classification,
    ClassificationContext, LookupKey, LookupKind, PreludeKey,
};

/// Extract the `root: ...` portion of the `Ast` debug output, which is the
/// rewritten mirror tree. The `upstream` field retains the original parse
/// tree and must be ignored in rewrite assertions.
fn root_dbg(ast: &xlstream_parse::Ast) -> String {
    let full = format!("{ast:?}");
    // Find "root: " and take everything after it (minus the trailing " }").
    if let Some(idx) = full.find("root: ") {
        let tail = &full[idx..];
        // Strip outer trailing " }"
        tail.strip_suffix(" }").unwrap_or(tail).to_owned()
    } else {
        full
    }
}

#[test]
fn sum_whole_column_collapses_to_prelude_ref() {
    let ast = parse("SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("PreludeRef"), "expected PreludeRef: {dbg}");
    assert!(dbg.contains("Sum"), "expected Sum: {dbg}");
    assert!(!dbg.contains("Function"), "expected no Function node in root: {dbg}");
}

#[test]
fn deal_value_over_sum_collapses_only_the_aggregate() {
    // A2/SUM(A:A) — BinaryOp preserved, inner becomes PreludeRef, Cell ref preserved.
    let ast = parse("A2/SUM(A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    assert_eq!(verdict, Classification::Mixed);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("BinaryOp"), "expected BinaryOp: {dbg}");
    assert!(dbg.contains("PreludeRef"), "expected PreludeRef: {dbg}");
    assert!(dbg.contains("Cell"), "expected Cell ref: {dbg}");
}

#[test]
fn unsupported_classifications_pass_through_untouched() {
    let ast = parse("OFFSET(A1, 1, 0)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    assert!(matches!(verdict, Classification::Unsupported(_)));
    let original_dbg = format!("{ast:?}");
    let rewritten = rewrite(ast, &ctx, &verdict);
    let rewritten_dbg = format!("{rewritten:?}");
    assert_eq!(original_dbg, rewritten_dbg);
}

#[test]
fn row_local_classifications_pass_through_untouched() {
    let ast = parse("1+2").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    assert_eq!(verdict, Classification::RowLocal);
    let original_dbg = format!("{ast:?}");
    let rewritten = rewrite(ast, &ctx, &verdict);
    let rewritten_dbg = format!("{rewritten:?}");
    assert_eq!(original_dbg, rewritten_dbg);
}

#[test]
fn vlookup_stays_as_function_node() {
    let ast = parse("VLOOKUP(A2, 'Region Info'!A:C, 2, FALSE)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5).with_lookup_sheet("Region Info");
    let verdict = classify(&ast, &ctx);
    assert_eq!(verdict, Classification::LookupOnly);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("Function"), "expected Function node preserved: {dbg}");
    assert!(dbg.contains("VLOOKUP"), "expected VLOOKUP name: {dbg}");
}

#[test]
fn multi_arg_aggregate_rewrites_only_range_children() {
    // SUM(1, 2, A:A) — Function node preserved, PreludeRef for the range,
    // literals preserved as-is.
    let ast = parse("SUM(1, 2, A:A)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("Function"), "expected Function preserved: {dbg}");
    assert!(dbg.contains("PreludeRef"), "expected PreludeRef for range: {dbg}");
    assert!(dbg.contains("Number(1.0)"), "expected literal 1: {dbg}");
    assert!(dbg.contains("Number(2.0)"), "expected literal 2: {dbg}");
}

#[test]
fn count_whole_column_collapses_to_count_prelude_ref() {
    let ast = parse("COUNT(B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("PreludeRef"), "expected PreludeRef: {dbg}");
    assert!(dbg.contains("Count"), "expected Count kind: {dbg}");
}

#[test]
fn average_whole_column_collapses_to_average_prelude_ref() {
    let ast = parse("AVERAGE(C:C)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("PreludeRef"), "expected PreludeRef: {dbg}");
    assert!(dbg.contains("Average"), "expected Average kind: {dbg}");
}

#[test]
fn nested_aggregates_both_collapse() {
    let ast = parse("SUM(A:A)+COUNT(B:B)").unwrap();
    let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
    let verdict = classify(&ast, &ctx);
    let rewritten = rewrite(ast, &ctx, &verdict);
    let dbg = root_dbg(&rewritten);
    assert!(dbg.contains("BinaryOp"), "expected BinaryOp: {dbg}");
    assert!(dbg.contains("Sum"), "expected Sum: {dbg}");
    assert!(dbg.contains("Count"), "expected Count: {dbg}");
    assert!(!dbg.contains("Function"), "expected no Function nodes in root: {dbg}");
}

#[test]
fn collect_vlookup_key_extracts_indices() {
    let ast = parse("VLOOKUP(A2, 'Lookup'!A:D, 3, FALSE)").unwrap();
    let keys = collect_lookup_keys(&ast);
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].kind, LookupKind::VLookup);
    assert_eq!(keys[0].sheet, "Lookup");
    assert_eq!(keys[0].key_index, 1);
    assert_eq!(keys[0].value_index, 3);
}

#[test]
fn collect_xlookup_key() {
    let ast = parse("XLOOKUP(A1, 'Data'!B:B, 'Data'!D:D)").unwrap();
    let keys = collect_lookup_keys(&ast);
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].kind, LookupKind::XLookup);
    assert_eq!(keys[0].sheet, "Data");
    assert_eq!(keys[0].key_index, 2);
    assert_eq!(keys[0].value_index, 4);
}

#[test]
fn collect_no_lookups_returns_empty() {
    let ast = parse("A1+B1*2").unwrap();
    let keys = collect_lookup_keys(&ast);
    assert!(keys.is_empty());
}

#[test]
fn collect_match_registers_sheet() {
    let ast = parse("MATCH(A1, 'Lookup'!B:B, 0)").unwrap();
    let keys = collect_lookup_keys(&ast);
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].sheet, "Lookup");
    assert_eq!(keys[0].key_index, 2);
}

#[test]
fn collect_index_registers_sheet() {
    let ast = parse("INDEX('Data'!A:C, 2, 1)").unwrap();
    let keys = collect_lookup_keys(&ast);
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].sheet, "Data");
}

#[test]
fn prelude_key_aggregate_equality() {
    let a = PreludeKey::Aggregate(AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 });
    let b = PreludeKey::Aggregate(AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 });
    assert_eq!(a, b);
}

#[test]
fn prelude_key_lookup_equality() {
    let a = PreludeKey::Lookup(LookupKey {
        kind: LookupKind::VLookup,
        sheet: "S".into(),
        key_index: 1,
        value_index: 2,
    });
    let b = PreludeKey::Lookup(LookupKey {
        kind: LookupKind::VLookup,
        sheet: "S".into(),
        key_index: 1,
        value_index: 2,
    });
    assert_eq!(a, b);
}
