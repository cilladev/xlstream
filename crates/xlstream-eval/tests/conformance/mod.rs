#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::uninlined_format_args
)]

use std::path::{Path, PathBuf};

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use tempfile::NamedTempFile;
use xlstream_eval::evaluate;

const EPSILON: f64 = 1e-6;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

fn col_letter(col: usize) -> String {
    let mut s = String::new();
    let mut c = col;
    loop {
        s.insert(0, (b'A' + (c % 26) as u8) as char);
        if c < 26 {
            break;
        }
        c = c / 26 - 1;
    }
    s
}

fn error_to_string(e: &calamine::CellErrorType) -> &'static str {
    use calamine::CellErrorType;
    match e {
        CellErrorType::Div0 => "#DIV/0!",
        CellErrorType::NA => "#N/A",
        CellErrorType::Name => "#NAME?",
        CellErrorType::Null => "#NULL!",
        CellErrorType::Num => "#NUM!",
        CellErrorType::Ref => "#REF!",
        CellErrorType::Value | CellErrorType::GettingData => "#VALUE!",
    }
}

fn data_as_f64(d: &Data) -> Option<f64> {
    match d {
        Data::Float(f) => Some(*f),
        Data::Int(i) => Some(*i as f64),
        Data::DateTime(dt) => Some(dt.as_f64()),
        _ => None,
    }
}

fn values_match(expected: &Data, actual: &Data) -> bool {
    match (expected, actual) {
        (a, b) if data_as_f64(a).is_some() && data_as_f64(b).is_some() => {
            let av = data_as_f64(a).unwrap();
            let bv = data_as_f64(b).unwrap();
            (av - bv).abs() < EPSILON || (av == 0.0 && bv == 0.0)
        }
        (Data::String(a), Data::String(b)) => a == b,
        (Data::Bool(a), Data::Bool(b)) => a == b,
        (Data::Bool(true), Data::Int(1))
        | (Data::Int(1), Data::Bool(true))
        | (Data::Bool(false), Data::Int(0))
        | (Data::Int(0), Data::Bool(false))
        | (Data::Empty, Data::Empty) => true,
        (Data::String(s), Data::Empty) | (Data::Empty, Data::String(s)) if s.is_empty() => true,
        (Data::Error(e), Data::String(s)) | (Data::String(s), Data::Error(e)) => {
            s == error_to_string(e)
        }
        (Data::Error(a), Data::Error(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
        _ => false,
    }
}

pub fn run_conformance(fixture_rel_path: &str) {
    run_conformance_with_options(fixture_rel_path, &xlstream_eval::EvaluateOptions::default());
}

pub fn run_conformance_with_options(
    fixture_rel_path: &str,
    options: &xlstream_eval::EvaluateOptions,
) {
    let fixture = fixtures_dir().join(fixture_rel_path);
    if !fixture.exists() {
        panic!("fixture not found: {}", fixture.display());
    }

    let mut expected_wb: Xlsx<_> = open_workbook(&fixture).unwrap();
    let sheet_names: Vec<String> = expected_wb.sheet_names().to_vec();

    let mut formula_cells: std::collections::HashSet<(String, u32, u32)> =
        std::collections::HashSet::new();
    for sheet_name in &sheet_names {
        if let Ok(formula_range) = expected_wb.worksheet_formula(sheet_name) {
            for (row_idx, row) in formula_range.rows().enumerate() {
                for (col_idx, cell) in row.iter().enumerate() {
                    if !cell.is_empty() {
                        formula_cells.insert((sheet_name.clone(), row_idx as u32, col_idx as u32));
                    }
                }
            }
        }
    }

    let output = NamedTempFile::with_suffix(".xlsx").unwrap();
    evaluate(&fixture, output.path(), options).unwrap();

    let mut actual_wb: Xlsx<_> = open_workbook(output.path()).unwrap();

    let mut mismatches = Vec::new();

    for sheet_name in &sheet_names {
        let expected_range = match expected_wb.worksheet_range(sheet_name) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let actual_range = match actual_wb.worksheet_range(sheet_name) {
            Ok(r) => r,
            Err(_) => {
                mismatches.push(format!("sheet '{sheet_name}' missing from output"));
                continue;
            }
        };

        let expected_rows: Vec<_> = expected_range.rows().collect();
        let actual_rows: Vec<_> = actual_range.rows().collect();

        let row_count = expected_rows.len().min(actual_rows.len());

        for row_idx in 0..row_count {
            let exp_row = expected_rows[row_idx];
            let act_row = actual_rows[row_idx];

            let col_count = exp_row.len().min(act_row.len());
            for col_idx in 0..col_count {
                if !formula_cells.contains(&(sheet_name.clone(), row_idx as u32, col_idx as u32)) {
                    continue;
                }
                if !values_match(&exp_row[col_idx], &act_row[col_idx]) {
                    mismatches.push(format!(
                        "  [{sheet_name}] {}{}: expected {:?}  actual {:?}",
                        col_letter(col_idx),
                        row_idx + 1,
                        exp_row[col_idx],
                        act_row[col_idx],
                    ));
                }
            }
        }
    }

    let count = mismatches.len();
    let detail = mismatches.join("\n");
    assert!(mismatches.is_empty(), "{count} cell mismatches in {fixture_rel_path}:\n{detail}");
}
