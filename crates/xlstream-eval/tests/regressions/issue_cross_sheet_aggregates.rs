//! Regression: conditional aggregates on non-streaming sheets return #VALUE!.
//!
//! SUMIF/COUNTIF/AVERAGEIF/AVERAGEIFS with ranges on a prelude-loaded
//! (non-streaming) sheet produce #VALUE! instead of the correct result.
//! Plain aggregates (SUM, COUNT, etc.) on the streaming sheet work fine.
//!
//! See: issue.md section 2.
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

fn build_cross_sheet_fixture(input: &std::path::Path) {
    let mut wb = Workbook::new();

    // Non-streaming sheet with data
    let ws_ref = wb.add_worksheet();
    ws_ref.set_name("RefData").unwrap();
    let data: &[(&str, f64)] =
        &[("EMEA", 500.0), ("APAC", 300.0), ("AMER", 700.0), ("EMEA", 200.0), ("APAC", 400.0)];
    for (i, &(region, amount)) in data.iter().enumerate() {
        let r = i as u32;
        ws_ref.write_string(r, 0, region).unwrap();
        ws_ref.write_number(r, 1, amount).unwrap();
    }

    // Streaming sheet with formulas referencing RefData
    let ws_main = wb.add_worksheet();
    ws_main.set_name("Main").unwrap();
    ws_main.write_string(0, 0, "Label").unwrap();
    ws_main.write_string(0, 1, "SumifResult").unwrap();
    ws_main.write_string(0, 2, "CountifResult").unwrap();
    ws_main.write_string(0, 3, "AvgifResult").unwrap();

    // Row 1: SUMIF EMEA = 500+200 = 700
    ws_main.write_string(1, 0, "test").unwrap();
    ws_main.write_formula(1, 1, Formula::new("=SUMIF(RefData!A:A,\"EMEA\",RefData!B:B)")).unwrap();
    ws_main.write_formula(1, 2, Formula::new("=COUNTIF(RefData!A:A,\"EMEA\")")).unwrap();
    ws_main
        .write_formula(1, 3, Formula::new("=AVERAGEIF(RefData!A:A,\"APAC\",RefData!B:B)"))
        .unwrap();

    wb.save(input).unwrap();
}

#[test]
#[ignore = "bug: cross-sheet conditional aggregates return #VALUE! (see issue.md section 2)"]
fn sumif_on_non_streaming_sheet() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    build_cross_sheet_fixture(&input);
    evaluate(&input, &output, None).unwrap();

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Main");

    // SUMIF(RefData!A:A, "EMEA", RefData!B:B) = 500 + 200 = 700
    assert_eq!(rows[1].1[1], Value::Number(700.0), "cross-sheet SUMIF");
}

#[test]
#[ignore = "bug: cross-sheet conditional aggregates return #VALUE! (see issue.md section 2)"]
fn countif_on_non_streaming_sheet() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    build_cross_sheet_fixture(&input);
    evaluate(&input, &output, None).unwrap();

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Main");

    // COUNTIF(RefData!A:A, "EMEA") = 2
    assert_eq!(rows[1].1[2], Value::Number(2.0), "cross-sheet COUNTIF");
}

#[test]
#[ignore = "bug: cross-sheet conditional aggregates return #VALUE! (see issue.md section 2)"]
fn averageif_on_non_streaming_sheet() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    build_cross_sheet_fixture(&input);
    evaluate(&input, &output, None).unwrap();

    let mut reader = Reader::open(&output).unwrap();
    let rows = read_all_rows(&mut reader, "Main");

    // AVERAGEIF(RefData!A:A, "APAC", RefData!B:B) = (300+400)/2 = 350
    assert_eq!(rows[1].1[3], Value::Number(350.0), "cross-sheet AVERAGEIF");
}
