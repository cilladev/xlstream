//! Integration tests for formula preservation (keep-formulas feature).
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

/// Build a small workbook with data + formulas and return the temp file.
fn make_formula_workbook() -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_string(0, 0, "X").unwrap();
    ws.write_string(0, 1, "Y").unwrap();
    ws.write_string(0, 2, "Sum").unwrap();
    ws.write_string(0, 3, "Product").unwrap();
    ws.write_string(0, 4, "IsPositive").unwrap();
    ws.write_string(0, 5, "Label").unwrap();
    ws.write_string(0, 6, "DivError").unwrap();

    for row in 1..=10u32 {
        let x = row as f64;
        let y = (row * 10) as f64;
        ws.write_number(row, 0, x).unwrap();
        ws.write_number(row, 1, y).unwrap();

        ws.write_formula(
            row,
            2,
            Formula::new(format!("=A{}+B{}", row + 1, row + 1)).set_result(format!("{}", x + y)),
        )
        .unwrap();

        ws.write_formula(
            row,
            3,
            Formula::new(format!("=A{}*B{}", row + 1, row + 1)).set_result(format!("{}", x * y)),
        )
        .unwrap();

        let is_pos = x > 5.0;
        ws.write_formula(
            row,
            4,
            Formula::new(format!("=A{}>5", row + 1)).set_result(if is_pos {
                "TRUE"
            } else {
                "FALSE"
            }),
        )
        .unwrap();

        let label = if x > 5.0 { "big" } else { "small" };
        ws.write_formula(
            row,
            5,
            Formula::new(format!(r#"=IF(A{}>5,"big","small")"#, row + 1)).set_result(label),
        )
        .unwrap();

        ws.write_formula(
            row,
            6,
            Formula::new(format!("=1/(A{}-A{})", row + 1, row + 1)).set_result("#DIV/0!"),
        )
        .unwrap();
    }

    wb.save(file.path()).unwrap();
    file
}

fn eval_to_temp(input: &Path, opts: &EvaluateOptions) -> NamedTempFile {
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();
    evaluate(input, output.path(), opts).unwrap();
    output
}

#[test]
fn default_mode_preserves_formulas() {
    let input = make_formula_workbook();
    let output = eval_to_temp(input.path(), &EvaluateOptions::default());

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let formulas = wb.worksheet_formula("Sheet1").unwrap();
    let has_formulas = formulas.rows().any(|row| row.iter().any(|c| !c.is_empty()));
    assert!(has_formulas, "default mode should write formula elements");
}

#[test]
fn values_only_omits_formulas() {
    let input = make_formula_workbook();
    let opts = EvaluateOptions {
        output_mode: xlstream_core::OutputMode::ValuesOnly,
        ..Default::default()
    };
    let output = eval_to_temp(input.path(), &opts);

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let formulas = wb.worksheet_formula("Sheet1").unwrap();
    let has_formulas = formulas.rows().any(|row| row.iter().any(|c| !c.is_empty()));
    assert!(!has_formulas, "values-only mode should not write formula elements");
}

#[test]
fn preserved_formulas_have_correct_row_references() {
    let input = make_formula_workbook();
    let output = eval_to_temp(input.path(), &EvaluateOptions::default());

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let formulas = wb.worksheet_formula("Sheet1").unwrap();
    let start = formulas.start().unwrap();
    let start_row = start.0 as usize;

    let rows: Vec<_> = formulas.rows().collect();

    // First data row (Excel row 2) has Sum formula "A2+B2"
    let data_row_0 = &rows[0];
    let sum_col = 2 - start.1 as usize;
    let sum_formula = &data_row_0[sum_col];
    assert!(
        sum_formula.contains("A2") && sum_formula.contains("B2"),
        "Sum formula row 2: {}",
        sum_formula
    );

    // Fifth data row (Excel row 6) has Sum formula "A6+B6"
    let data_row_4 = &rows[4];
    let sum_formula = &data_row_4[sum_col];
    assert!(
        sum_formula.contains("A6") && sum_formula.contains("B6"),
        "Sum formula row 6 (start_row={}): {}",
        start_row,
        sum_formula
    );
}

#[test]
fn formula_results_cover_numeric_and_boolean_types() {
    let input = make_formula_workbook();
    let output = eval_to_temp(input.path(), &EvaluateOptions::default());

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // Row 2 (index 1): Sum = 12.0 (number)
    assert!(
        matches!(rows[1][2], Data::Float(_) | Data::Int(_)),
        "sum should be numeric: {:?}",
        rows[1][2]
    );

    // Row 2 (index 1): IsPositive = false (boolean), X=1 <= 5
    assert!(matches!(rows[1][4], Data::Bool(false)), "isPositive row 2: {:?}", rows[1][4]);

    // Row 7 (index 7): IsPositive = true (boolean), X=7 > 5
    assert!(matches!(rows[7][4], Data::Bool(true)), "isPositive row 8: {:?}", rows[7][4]);
}
