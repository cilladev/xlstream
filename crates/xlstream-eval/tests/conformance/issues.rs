use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

#[test]
fn issue_76_self_referential_formulas() {
    super::conformance::run_conformance("issues/issue-76-self-referential-formulas.xlsx");
}

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

fn make_formula_workbook() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "Double").unwrap();
    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_formula(1, 1, Formula::new("A2*2").set_result("20")).unwrap();
    ws.write_number(2, 0, 30.0).unwrap();
    ws.write_formula(2, 1, Formula::new("A3*2").set_result("60")).unwrap();
    wb.save(tmp.path()).unwrap();
    tmp
}

#[test]
fn keep_formulas_preserves_formula_elements() {
    let input = make_formula_workbook();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions::default();
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let formula_range = wb.worksheet_formula("Sheet1").unwrap();
    let formulas: Vec<_> =
        formula_range.rows().flat_map(|row| row.iter()).filter(|s| !s.is_empty()).collect();

    assert!(!formulas.is_empty(), "output must contain formulas in default mode");
}

#[test]
fn keep_formulas_produces_correct_values() {
    let input = make_formula_workbook();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions::default();
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    assert_eq!(rows[1][0], Data::Float(10.0));
    assert_eq!(rows[1][1], Data::Float(20.0));
    assert_eq!(rows[2][0], Data::Float(30.0));
    assert_eq!(rows[2][1], Data::Float(60.0));
}

#[test]
fn values_only_strips_formula_elements() {
    let input = make_formula_workbook();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions { values_only: true, ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let formula_range = wb.worksheet_formula("Sheet1").unwrap();
    let formulas: Vec<_> =
        formula_range.rows().flat_map(|row| row.iter()).filter(|s| !s.is_empty()).collect();

    assert!(formulas.is_empty(), "values_only output must not contain formulas");
}
