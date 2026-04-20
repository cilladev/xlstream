//! Regression: SUMIF/COUNTIF/AVERAGEIF with row-local criteria.
//!
//! Formula like `SUMIF(A:A, A2, B:B)` where the criteria arg is a
//! current-row cell ref. Currently rejected at classification with
//! `NonStaticCriteria`. Fix requires grouped-aggregate prelude maps.
//!
//! See: issue.md section 1.
#![allow(clippy::float_cmp)]

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::TempDir;
use xlstream_core::Value;
use xlstream_eval::evaluate;
use xlstream_io::Reader;

fn read_all_rows(reader: &mut Reader, sheet: &str) -> Vec<(u32, Vec<Value>)> {
    let mut stream = reader.cells(sheet).unwrap();
    let mut rows = Vec::new();
    while let Some(row) = stream.next_row().unwrap() {
        rows.push(row);
    }
    rows
}

#[test]
fn sumif_row_local_criteria() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Amount").unwrap();
    ws.write_string(0, 2, "RegionTotal").unwrap();

    let data: &[(&str, f64)] =
        &[("EMEA", 100.0), ("APAC", 200.0), ("EMEA", 300.0), ("AMER", 400.0), ("APAC", 150.0)];

    for (i, &(region, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        ws.write_string(row, 0, region).unwrap();
        ws.write_number(row, 1, amount).unwrap();
        ws.write_formula(row, 2, Formula::new(format!("=SUMIF(A:A,A{excel_row},B:B)"))).unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 5);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    assert_eq!(rows[1].1[2], Value::Number(400.0), "EMEA total = 100+300");
    assert_eq!(rows[2].1[2], Value::Number(350.0), "APAC total = 200+150");
    assert_eq!(rows[3].1[2], Value::Number(400.0), "EMEA again");
    assert_eq!(rows[4].1[2], Value::Number(400.0), "AMER total = 400");
    assert_eq!(rows[5].1[2], Value::Number(350.0), "APAC again");
}

#[test]
fn countif_row_local_criteria() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "RegionCount").unwrap();

    let regions = ["EMEA", "APAC", "EMEA", "AMER", "APAC", "EMEA"];
    for (i, region) in regions.iter().enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        ws.write_string(row, 0, *region).unwrap();
        ws.write_formula(row, 1, Formula::new(format!("=COUNTIF(A:A,A{excel_row})"))).unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 6);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    assert_eq!(rows[1].1[1], Value::Number(3.0), "EMEA count");
    assert_eq!(rows[2].1[1], Value::Number(2.0), "APAC count");
    assert_eq!(rows[4].1[1], Value::Number(1.0), "AMER count");
}

#[test]
fn reported_formula_if_abs_sumif_row_local() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Threshold").unwrap();
    ws.write_string(0, 2, "Amount").unwrap();
    ws.write_string(0, 3, "Result").unwrap();

    let data: &[(&str, f64, f64)] = &[
        ("EMEA", 500.0, 100.0),
        ("APAC", 200.0, 200.0),
        ("EMEA", 300.0, 300.0),
        ("AMER", 100.0, 400.0),
        ("APAC", 400.0, 150.0),
    ];

    for (i, &(region, threshold, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let r = row + 1;
        ws.write_string(row, 0, region).unwrap();
        ws.write_number(row, 1, threshold).unwrap();
        ws.write_number(row, 2, amount).unwrap();
        ws.write_formula(
            row,
            3,
            Formula::new(format!("=IF(ABS(SUMIF(A:A,A{r},C:C))>ABS(B{r}),C{r},0)")),
        )
        .unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 5);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    // EMEA SUMIF = 100+300 = 400. ABS(400)>ABS(500)? no → 0
    assert_eq!(rows[1].1[3], Value::Number(0.0), "row 1");
    // APAC SUMIF = 200+150 = 350. ABS(350)>ABS(200)? yes → 200
    assert_eq!(rows[2].1[3], Value::Number(200.0), "row 2");
    // EMEA again. ABS(400)>ABS(300)? yes → 300
    assert_eq!(rows[3].1[3], Value::Number(300.0), "row 3");
    // AMER SUMIF = 400. ABS(400)>ABS(100)? yes → 400
    assert_eq!(rows[4].1[3], Value::Number(400.0), "row 4");
    // APAC again. ABS(350)>ABS(400)? no → 0
    assert_eq!(rows[5].1[3], Value::Number(0.0), "row 5");
}
