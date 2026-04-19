//! End-to-end tests for lookup functions through the full evaluate pipeline.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::TempDir;
use xlstream_core::Value;
use xlstream_eval::evaluate;
use xlstream_io::Reader;

fn create_lookup_fixture(path: &Path) {
    let mut wb = Workbook::new();

    // Lookup sheet: Region Info (no formulas, pure data)
    let ws_lookup = wb.add_worksheet().set_name("Region Info").unwrap();
    ws_lookup.write_string(0, 0, "EMEA").unwrap();
    ws_lookup.write_string(0, 1, "Europe").unwrap();
    ws_lookup.write_number(0, 2, 100.0).unwrap();
    ws_lookup.write_string(1, 0, "APAC").unwrap();
    ws_lookup.write_string(1, 1, "Asia").unwrap();
    ws_lookup.write_number(1, 2, 200.0).unwrap();
    ws_lookup.write_string(2, 0, "AMER").unwrap();
    ws_lookup.write_string(2, 1, "Americas").unwrap();
    ws_lookup.write_number(2, 2, 300.0).unwrap();

    // Main sheet with VLOOKUP formulas
    let ws_main = wb.add_worksheet().set_name("Main").unwrap();
    ws_main.write_string(0, 0, "Region").unwrap();
    ws_main.write_string(0, 1, "Lookup").unwrap();

    let regions = ["EMEA", "APAC", "AMER", "NONE", "apac"];
    for (i, region) in regions.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let row = (i + 1) as u32;
        ws_main.write_string(row, 0, *region).unwrap();
        ws_main
            .write_formula(
                row,
                1,
                Formula::new(format!("VLOOKUP(A{}, 'Region Info'!A:C, 2, FALSE)", row + 1)),
            )
            .unwrap();
    }

    wb.save(path).unwrap();
}

#[test]
fn end_to_end_vlookup_exact_match() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    create_lookup_fixture(&input);
    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 5);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Main").unwrap();

    // Skip header
    let _ = stream.next_row().unwrap();

    // Row 1: EMEA -> Europe
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("Europe".into()));

    // Row 2: APAC -> Asia
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("Asia".into()));

    // Row 3: AMER -> Americas
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("Americas".into()));

    // Row 4: NONE -> #N/A (writer stores errors as text strings)
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("#N/A".into()));

    // Row 5: apac -> Asia (case-insensitive)
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("Asia".into()));
}
