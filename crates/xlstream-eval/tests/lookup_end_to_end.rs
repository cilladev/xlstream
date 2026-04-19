//! End-to-end tests for lookup functions through the full evaluate pipeline.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

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

/// VLOOKUP with concatenated helper key: Region & Business -> Threshold.
#[test]
fn end_to_end_vlookup_concat_key() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();

    // Lookup sheet "Thresholds": A=Region, B=Business, C=Threshold,
    // D=helper (A&B concat), E=Threshold copy (the return column).
    let ws_lookup = wb.add_worksheet().set_name("Thresholds").unwrap();
    let lookup_data: &[(&str, &str, f64)] =
        &[("EMEA", "Rates", 50_000.0), ("APAC", "FX", 30_000.0)];
    for (i, &(region, biz, threshold)) in lookup_data.iter().enumerate() {
        let r = i as u32;
        ws_lookup.write_string(r, 0, region).unwrap();
        ws_lookup.write_string(r, 1, biz).unwrap();
        ws_lookup.write_number(r, 2, threshold).unwrap();
        // D = helper key (concat of region & business)
        ws_lookup.write_string(r, 3, format!("{region}{biz}")).unwrap();
        // E = threshold value (return column)
        ws_lookup.write_number(r, 4, threshold).unwrap();
    }

    // Main sheet: A=Region, B=Business, C=formula
    let ws_main = wb.add_worksheet().set_name("Main").unwrap();
    ws_main.write_string(0, 0, "Region").unwrap();
    ws_main.write_string(0, 1, "Business").unwrap();
    ws_main.write_string(0, 2, "Result").unwrap();

    let main_data: &[(&str, &str)] = &[("EMEA", "Rates"), ("APAC", "FX"), ("EMEA", "FX")];
    for (i, &(region, biz)) in main_data.iter().enumerate() {
        let r = (i + 1) as u32;
        let excel_row = r + 1;
        ws_main.write_string(r, 0, region).unwrap();
        ws_main.write_string(r, 1, biz).unwrap();
        ws_main
            .write_formula(
                r,
                2,
                Formula::new(format!(
                    "VLOOKUP(A{excel_row}&B{excel_row}, 'Thresholds'!D:E, 2, FALSE)"
                )),
            )
            .unwrap();
    }

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 3);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Main").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    // EMEA+Rates -> 50000
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[2], Value::Number(50_000.0));

    // APAC+FX -> 30000
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[2], Value::Number(30_000.0));

    // EMEA+FX -> not found -> #N/A
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[2], Value::Text("#N/A".into()));
}

/// IF + VLOOKUP combo: classify lookup result and branch.
#[test]
fn end_to_end_if_vlookup_combo() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();

    // Lookup sheet "Region Info" (same as main fixture)
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

    // Main sheet with IF(VLOOKUP(...) > 150, "High", "Low")
    let ws_main = wb.add_worksheet().set_name("Main").unwrap();
    ws_main.write_string(0, 0, "Region").unwrap();
    ws_main.write_string(0, 1, "Tier").unwrap();

    let regions = ["EMEA", "APAC", "AMER"];
    for (i, region) in regions.iter().enumerate() {
        let r = (i + 1) as u32;
        let excel_row = r + 1;
        ws_main.write_string(r, 0, *region).unwrap();
        ws_main
            .write_formula(
                r,
                1,
                Formula::new(format!(
                    "IF(VLOOKUP(A{excel_row}, 'Region Info'!A:C, 3, FALSE) > 150, \"High\", \"Low\")"
                )),
            )
            .unwrap();
    }

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 3);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Main").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    // EMEA: 100 <= 150 -> "Low"
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("Low".into()));

    // APAC: 200 > 150 -> "High"
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("High".into()));

    // AMER: 300 > 150 -> "High"
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Text("High".into()));
}

/// Perf smoke test: 10k-row lookup sheet + 10k VLOOKUP formulas.
/// Not timing-sensitive — just verifies correctness at scale.
#[test]
#[ignore = "slow: 10k-row fixture, ~2s"]
fn perf_smoke_10k_vlookup() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();

    // 10k-row lookup sheet: col A = key (1..=10000), col B = key * 10
    let ws_lookup = wb.add_worksheet().set_name("BigLookup").unwrap();
    for i in 0..10_000u32 {
        let key = f64::from(i + 1);
        ws_lookup.write_number(i, 0, key).unwrap();
        ws_lookup.write_number(i, 1, key * 10.0).unwrap();
    }

    // Main sheet: 10k rows, each doing VLOOKUP against BigLookup
    let ws_main = wb.add_worksheet().set_name("Main").unwrap();
    ws_main.write_string(0, 0, "Key").unwrap();
    ws_main.write_string(0, 1, "Result").unwrap();

    for i in 0..10_000u32 {
        let r = i + 1;
        let excel_row = r + 1;
        ws_main.write_number(r, 0, f64::from(i + 1)).unwrap();
        ws_main
            .write_formula(
                r,
                1,
                Formula::new(format!("VLOOKUP(A{excel_row}, 'BigLookup'!A:B, 2, FALSE)")),
            )
            .unwrap();
    }

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 10_000);

    // Spot-check a few results
    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Main").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    // Row 1: key=1 -> 10
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Number(10.0));

    // Skip to last row by consuming
    for _ in 1..9999 {
        let _ = stream.next_row().unwrap();
    }

    // Row 10000: key=10000 -> 100000
    let (_, row) = stream.next_row().unwrap().unwrap();
    assert_eq!(row[1], Value::Number(100_000.0));
}
