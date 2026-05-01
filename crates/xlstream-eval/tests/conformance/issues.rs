// Issue-specific conformance fixtures go here.
// Each test points at a fixture in fixtures/issues/.

use calamine::{open_workbook, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

#[test]
fn issue_76_self_referential_formulas() {
    super::conformance::run_conformance("issues/issue-76-self-referential-formulas.xlsx");
}

#[test]
fn issue_76_negation_with_max_iter_1() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    // Build fixture: =IF(A2="Risk_KC",C2*-1,C2) in C2 with cached seed 500
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "Flag").unwrap();
    ws.write_string(0, 1, "Input").unwrap();
    ws.write_string(0, 2, "SelfRef").unwrap();
    // Row 2: Risk_KC trigger, seed=500 -> after 1 iter -> -500
    ws.write_string(1, 0, "Risk_KC").unwrap();
    ws.write_number(1, 1, 500.0).unwrap();
    ws.write_formula(1, 2, Formula::new("IF(A2=\"Risk_KC\",C2*-1,C2)").set_result("500")).unwrap();
    // Row 3: Normal, seed=300 -> stays 300
    ws.write_string(2, 0, "Normal").unwrap();
    ws.write_number(2, 1, 300.0).unwrap();
    ws.write_formula(2, 2, Formula::new("IF(A3=\"Risk_KC\",C3*-1,C3)").set_result("300")).unwrap();
    wb.save(input.path()).unwrap();

    let options =
        EvaluateOptions { workers: Some(1), max_iterations: 1, ..EvaluateOptions::default() };
    evaluate(input.path(), output.path(), &options).unwrap();

    let mut actual: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = actual.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // C2: seed=500, one iteration of IF(TRUE, 500*-1, 500) = -500
    let c2 = &rows[1][2];
    assert!(
        matches!(c2, calamine::Data::Float(v) if (*v - -500.0).abs() < 1e-6),
        "C2: expected -500, got {c2:?}"
    );
    // C3: seed=300, IF(FALSE, ..., 300) = 300
    let c3 = &rows[2][2];
    assert!(
        matches!(c3, calamine::Data::Float(v) if (*v - 300.0).abs() < 1e-6),
        "C3: expected 300, got {c3:?}"
    );
}
