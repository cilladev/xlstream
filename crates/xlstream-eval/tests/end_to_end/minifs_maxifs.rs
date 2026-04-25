//! End-to-end: MINIFS/MAXIFS with row-local criteria.

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
fn minifs_row_local_criteria() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Amount").unwrap();
    ws.write_string(0, 2, "MinByRegion").unwrap();

    let data: &[(&str, f64)] =
        &[("EMEA", 100.0), ("APAC", 200.0), ("EMEA", 300.0), ("AMER", 400.0), ("APAC", 150.0)];

    for (i, &(region, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let r = row + 1;
        ws.write_string(row, 0, region).unwrap();
        ws.write_number(row, 1, amount).unwrap();
        ws.write_formula(row, 2, Formula::new(format!("=MINIFS(B:B,A:A,A{r})"))).unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 5);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    assert_eq!(rows[1].1[2], Value::Number(100.0), "EMEA min = 100");
    assert_eq!(rows[2].1[2], Value::Number(150.0), "APAC min = 150");
    assert_eq!(rows[3].1[2], Value::Number(100.0), "EMEA again");
    assert_eq!(rows[4].1[2], Value::Number(400.0), "AMER min = 400");
    assert_eq!(rows[5].1[2], Value::Number(150.0), "APAC again");
}

#[test]
fn maxifs_row_local_criteria() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Amount").unwrap();
    ws.write_string(0, 2, "MaxByRegion").unwrap();

    let data: &[(&str, f64)] =
        &[("EMEA", 100.0), ("APAC", 200.0), ("EMEA", 300.0), ("AMER", 400.0), ("APAC", 150.0)];

    for (i, &(region, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let r = row + 1;
        ws.write_string(row, 0, region).unwrap();
        ws.write_number(row, 1, amount).unwrap();
        ws.write_formula(row, 2, Formula::new(format!("=MAXIFS(B:B,A:A,A{r})"))).unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 5);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    assert_eq!(rows[1].1[2], Value::Number(300.0), "EMEA max = 300");
    assert_eq!(rows[2].1[2], Value::Number(200.0), "APAC max = 200");
    assert_eq!(rows[3].1[2], Value::Number(300.0), "EMEA again");
    assert_eq!(rows[4].1[2], Value::Number(400.0), "AMER max = 400");
    assert_eq!(rows[5].1[2], Value::Number(200.0), "APAC again");
}

#[test]
fn minifs_two_criteria() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Dept").unwrap();
    ws.write_string(0, 2, "Amount").unwrap();
    ws.write_string(0, 3, "MinByBoth").unwrap();

    let data: &[(&str, &str, f64)] = &[
        ("EMEA", "Sales", 100.0),
        ("EMEA", "Ops", 200.0),
        ("EMEA", "Sales", 50.0),
        ("APAC", "Sales", 300.0),
    ];

    for (i, &(region, dept, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let r = row + 1;
        ws.write_string(row, 0, region).unwrap();
        ws.write_string(row, 1, dept).unwrap();
        ws.write_number(row, 2, amount).unwrap();
        ws.write_formula(row, 3, Formula::new(format!("=MINIFS(C:C,A:A,A{r},B:B,B{r})"))).unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 4);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    assert_eq!(rows[1].1[3], Value::Number(50.0), "EMEA+Sales min = 50");
    assert_eq!(rows[2].1[3], Value::Number(200.0), "EMEA+Ops min = 200");
    assert_eq!(rows[3].1[3], Value::Number(50.0), "EMEA+Sales again");
    assert_eq!(rows[4].1[3], Value::Number(300.0), "APAC+Sales min = 300");
}

#[test]
fn mixed_minifs_sumifs_same_workbook() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Amount").unwrap();
    ws.write_string(0, 2, "Sum").unwrap();
    ws.write_string(0, 3, "Min").unwrap();
    ws.write_string(0, 4, "Max").unwrap();

    let data: &[(&str, f64)] = &[("EMEA", 100.0), ("APAC", 200.0), ("EMEA", 300.0)];

    for (i, &(region, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let r = row + 1;
        ws.write_string(row, 0, region).unwrap();
        ws.write_number(row, 1, amount).unwrap();
        ws.write_formula(row, 2, Formula::new(format!("=SUMIFS(B:B,A:A,A{r})"))).unwrap();
        ws.write_formula(row, 3, Formula::new(format!("=MINIFS(B:B,A:A,A{r})"))).unwrap();
        ws.write_formula(row, 4, Formula::new(format!("=MAXIFS(B:B,A:A,A{r})"))).unwrap();
    }

    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 9);

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    // EMEA: sum=400, min=100, max=300
    assert_eq!(rows[1].1[2], Value::Number(400.0));
    assert_eq!(rows[1].1[3], Value::Number(100.0));
    assert_eq!(rows[1].1[4], Value::Number(300.0));
    // APAC: sum=200, min=200, max=200
    assert_eq!(rows[2].1[2], Value::Number(200.0));
    assert_eq!(rows[2].1[3], Value::Number(200.0));
    assert_eq!(rows[2].1[4], Value::Number(200.0));
}
