//! Integration tests for output sheet ordering (#197).
//!
//! Both write paths must preserve the input workbook's sheet order. The
//! parallel path only runs with workers > 1, a formula column on the main
//! sheet, and >= 10,000 data rows (`PARALLEL_ROW_THRESHOLD`) — a small
//! workbook exercises the serial path only and cannot catch parallel
//! reordering.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use calamine::{open_workbook, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

const SHEETS: [&str; 4] = ["Sheet1", "RegionFactors", "CategoryWeights", "RegionCodes"];

/// Multi-sheet workbook: a formula column on Sheet1, data-only secondaries.
fn make_workbook(data_rows: u32) -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let main = wb.add_worksheet();
    main.set_name(SHEETS[0]).unwrap();
    main.write_string(0, 0, "x").unwrap();
    main.write_string(0, 1, "doubled").unwrap();
    for row in 1..=data_rows {
        main.write_number(row, 0, f64::from(row)).unwrap();
        main.write_formula(row, 1, Formula::new(format!("=A{}*2", row + 1)).set_result("0"))
            .unwrap();
    }
    for name in SHEETS[1..].iter().copied() {
        let ws = wb.add_worksheet();
        ws.set_name(name).unwrap();
        ws.write_string(0, 0, "k").unwrap();
        ws.write_number(1, 0, 1.0).unwrap();
    }
    wb.save(file.path()).unwrap();
    file
}

fn output_sheet_order(workers: usize, data_rows: u32) -> Vec<String> {
    let input = make_workbook(data_rows);
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();
    let opts = EvaluateOptions { workers: Some(workers), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();
    let wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    wb.sheet_names().to_vec()
}

#[test]
fn serial_path_preserves_sheet_order() {
    assert_eq!(output_sheet_order(1, 10), SHEETS);
}

#[test]
fn parallel_path_preserves_sheet_order() {
    // 12k rows + 2 workers forces the parallel branch (threshold 10k).
    assert_eq!(output_sheet_order(2, 12_000), SHEETS);
}
