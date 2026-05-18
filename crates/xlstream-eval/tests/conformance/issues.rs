// Issue-specific conformance fixtures go here.
// Each test points at a fixture in fixtures/issues/.

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

#[test]
fn issue_76_self_referential_formulas() {
    super::conformance::run_conformance("issues/issue-76-self-referential-formulas.xlsx");
}

/// Proves iterative calc transforms values: seed=500 -> -500 after one negation.
/// Can't use conformance for this — oscillating formulas phase-shift when
/// re-seeded from their own output.
#[test]
fn issue_76_negation_max_iter_1() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "Flag").unwrap();
    ws.write_string(1, 0, "Risk_KC").unwrap();
    ws.write_formula(1, 1, Formula::new("IF(A2=\"Risk_KC\",B2*-1,B2)").set_result("500")).unwrap();
    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), max_iterations: 1, ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    let b2 = &rows[1][1];
    assert!(matches!(b2, Data::Float(v) if (*v + 500.0).abs() < 1e-6), "expected -500, got {b2:?}");
}

#[test]
fn issue_136_cross_sheet_cell_ref() {
    super::conformance::run_conformance("issues/issue-136-cross-sheet-cell-ref.xlsx");
}

#[test]
fn issue_136_cross_sheet_bare_cell_ref() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    // Sheet2: data sheet with known values
    let data = wb.add_worksheet();
    data.set_name("Sheet2").unwrap();
    data.write_number(0, 0, 100.0).unwrap(); // A1 = 100
    data.write_number(0, 1, 200.0).unwrap(); // B1 = 200
    data.write_number(1, 0, 42.0).unwrap(); // A2 = 42
    data.write_number(1, 2, 7.0).unwrap(); // C2 = 7

    // Sheet1: main sheet with formulas referencing Sheet2
    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();

    // A1: data value (not a formula)
    main.write_number(0, 0, 1.0).unwrap();
    // B1: bare cross-sheet ref
    main.write_formula(0, 1, Formula::new("Sheet2!A1").set_result("100")).unwrap();
    // C1: cross-sheet ref inside math
    main.write_formula(0, 2, Formula::new("A1+Sheet2!B1").set_result("201")).unwrap();
    // A2: data
    main.write_number(1, 0, 5.0).unwrap();
    // B2: cross-sheet ref inside function
    main.write_formula(1, 1, Formula::new("EVEN(Sheet2!A2)").set_result("42")).unwrap();
    // C2: cross-sheet ref inside IF
    main.write_formula(1, 2, Formula::new("IF(A2>0,Sheet2!C2,0)").set_result("7")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = Sheet2!A1 = 100
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 100.0).abs() < 1e-6),
        "B1: expected 100, got {:?}",
        rows[0][1]
    );
    // C1 = A1 + Sheet2!B1 = 1 + 200 = 201
    assert!(
        matches!(&rows[0][2], Data::Float(v) if (*v - 201.0).abs() < 1e-6),
        "C1: expected 201, got {:?}",
        rows[0][2]
    );
    // B2 = EVEN(Sheet2!A2) = EVEN(42) = 42
    assert!(
        matches!(&rows[1][1], Data::Float(v) if (*v - 42.0).abs() < 1e-6),
        "B2: expected 42, got {:?}",
        rows[1][1]
    );
    // C2 = IF(5>0, Sheet2!C2, 0) = 7
    assert!(
        matches!(&rows[1][2], Data::Float(v) if (*v - 7.0).abs() < 1e-6),
        "C2: expected 7, got {:?}",
        rows[1][2]
    );
}

/// Self-ref guard: Sheet1 formulas referencing Sheet1!A (self) must resolve
/// from the streaming row, not a lookup snapshot. Cross-sheet refs to Sheet2
/// must still resolve via lookup.
#[test]
fn issue_136_self_ref_not_loaded_as_lookup() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let data = wb.add_worksheet();
    data.set_name("Sheet2").unwrap();
    data.write_number(0, 0, 500.0).unwrap(); // A1 = 500

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();

    // A1-A2: data column
    main.write_number(0, 0, 10.0).unwrap();
    main.write_number(1, 0, 20.0).unwrap();
    // B: formula referencing Sheet1!A (self) — must use streaming row value
    main.write_formula(0, 1, Formula::new("Sheet1!A1*2").set_result("20")).unwrap();
    main.write_formula(1, 1, Formula::new("Sheet1!A2*2").set_result("40")).unwrap();
    // C: formula referencing Sheet2!A1 (cross-sheet) + Sheet1!A (self)
    main.write_formula(0, 2, Formula::new("Sheet2!A1+Sheet1!A1").set_result("510")).unwrap();
    main.write_formula(1, 2, Formula::new("Sheet2!A1+Sheet1!A2").set_result("520")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = Sheet1!A1*2 = 10*2 = 20 (streaming value, not static lookup)
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 20.0).abs() < 1e-6),
        "B1: expected 20, got {:?}",
        rows[0][1]
    );
    // B2 = Sheet1!A2*2 = 20*2 = 40
    assert!(
        matches!(&rows[1][1], Data::Float(v) if (*v - 40.0).abs() < 1e-6),
        "B2: expected 40, got {:?}",
        rows[1][1]
    );
    // C1 = Sheet2!A1 + Sheet1!A1 = 500 + 10 = 510
    assert!(
        matches!(&rows[0][2], Data::Float(v) if (*v - 510.0).abs() < 1e-6),
        "C1: expected 510, got {:?}",
        rows[0][2]
    );
    // C2 = Sheet2!A1 + Sheet1!A2 = 500 + 20 = 520
    assert!(
        matches!(&rows[1][2], Data::Float(v) if (*v - 520.0).abs() < 1e-6),
        "C2: expected 520, got {:?}",
        rows[1][2]
    );
}

/// Referencing a nonexistent sheet currently aborts evaluation at
/// load_lookup_sheets. Graceful per-cell #REF! is a follow-up.
#[test]
fn issue_136_nonexistent_sheet_aborts_evaluation() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();
    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_number(0, 0, 1.0).unwrap();
    main.write_formula(0, 1, Formula::new("NoSuchSheet!A1").set_result("0")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    let result = evaluate(input.path(), output.path(), &opts);
    assert!(result.is_err(), "expected Err for nonexistent sheet, got Ok");
    // If result is Err, that's also acceptable — the formula references a
    // nonexistent sheet and the loader may legitimately reject it.
}

/// Multiple lookup sheets (Sheet2 + Sheet3) referenced in the same formula.
#[test]
fn issue_136_multi_sheet_refs() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let s2 = wb.add_worksheet();
    s2.set_name("Sheet2").unwrap();
    s2.write_number(0, 0, 10.0).unwrap();

    let s3 = wb.add_worksheet();
    s3.set_name("Sheet3").unwrap();
    s3.write_number(0, 0, 20.0).unwrap();

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_number(0, 0, 1.0).unwrap();
    // IF referencing two different sheets
    main.write_formula(0, 1, Formula::new("IF(A1>0,Sheet2!A1,Sheet3!A1)").set_result("10"))
        .unwrap();
    // Both sheets in one expression
    main.write_formula(0, 2, Formula::new("Sheet2!A1+Sheet3!A1").set_result("30")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = IF(1>0, Sheet2!A1, Sheet3!A1) = 10
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 10.0).abs() < 1e-6),
        "B1: expected 10, got {:?}",
        rows[0][1]
    );
    // C1 = Sheet2!A1 + Sheet3!A1 = 30
    assert!(
        matches!(&rows[0][2], Data::Float(v) if (*v - 30.0).abs() < 1e-6),
        "C1: expected 30, got {:?}",
        rows[0][2]
    );
}

/// CONCATENATE with a cross-sheet string reference.
#[test]
fn issue_136_concatenate_cross_sheet() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let data = wb.add_worksheet();
    data.set_name("Sheet2").unwrap();
    data.write_string(0, 0, "world").unwrap();

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_formula(
        0,
        0,
        Formula::new("CONCATENATE(\"hello \",Sheet2!A1)").set_result("hello world"),
    )
    .unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    assert!(
        matches!(&rows[0][0], Data::String(s) if s == "hello world"),
        "A1: expected 'hello world', got {:?}",
        rows[0][0]
    );
}

/// Quoted sheet name with spaces in a cross-sheet ref.
#[test]
fn issue_136_quoted_sheet_name_with_spaces() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let data = wb.add_worksheet();
    data.set_name("Tax Rates").unwrap();
    data.write_number(0, 0, 0.15).unwrap();

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_number(0, 0, 100.0).unwrap();
    main.write_formula(0, 1, Formula::new("A1*'Tax Rates'!A1").set_result("15")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = 100 * 0.15 = 15
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 15.0).abs() < 1e-6),
        "B1: expected 15, got {:?}",
        rows[0][1]
    );
}

#[test]
fn issue_136_cross_sheet_row_override() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let data = wb.add_worksheet();
    data.set_name("Rates").unwrap();
    data.write_number(0, 0, 0.05).unwrap(); // A1 = 0.05
    data.write_number(1, 0, 0.10).unwrap(); // A2 = 0.10

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();

    // Column B: row 1 has template formula, row 2 has a row override referencing Rates sheet
    main.write_number(0, 0, 10.0).unwrap();
    main.write_formula(0, 1, Formula::new("A1*2").set_result("20")).unwrap();
    main.write_number(1, 0, 20.0).unwrap();
    main.write_formula(1, 1, Formula::new("A2*Rates!A2").set_result("2")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = A1*2 = 20
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 20.0).abs() < 1e-6),
        "B1: expected 20, got {:?}",
        rows[0][1]
    );
    // B2 = A2 * Rates!A2 = 20 * 0.10 = 2
    assert!(
        matches!(&rows[1][1], Data::Float(v) if (*v - 2.0).abs() < 1e-6),
        "B2: expected 2, got {:?}",
        rows[1][1]
    );
}
