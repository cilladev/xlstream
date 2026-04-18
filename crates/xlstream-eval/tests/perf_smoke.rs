#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::cast_precision_loss)]
#![cfg(not(debug_assertions))]

#[allow(dead_code)]
mod helpers;

use xlstream_eval::evaluate;

/// 10k rows × 10 data cols + 10 formula cols (all `=A{row}` style).
fn generate_10k_fixture() -> tempfile::NamedTempFile {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = rust_xlsxwriter::Workbook::new();
    let ws = wb.add_worksheet();

    // Header row: A..J (data) + K..T (formula cols referencing A..J).
    for c in 0u16..10 {
        let col_name = char::from(b'A' + u8::try_from(c).unwrap());
        ws.write_string(0, c, col_name.to_string()).unwrap();
    }
    for c in 10u16..20 {
        let col_name = char::from(b'K' + u8::try_from(c - 10).unwrap());
        ws.write_string(0, c, col_name.to_string()).unwrap();
    }

    for r in 1u32..=10_000 {
        // Data columns A–J.
        for c in 0u16..10 {
            ws.write_number(r, c, r as f64 * f64::from(c + 1)).unwrap();
        }
        // Formula columns K–T: =A{excel_row} through =J{excel_row}.
        let excel_row = r + 1; // calamine row r → Excel row r+1
        for c in 10u16..20 {
            let src_col = char::from(b'A' + u8::try_from(c - 10).unwrap());
            let formula = format!("={src_col}{excel_row}");
            let result_val = r as f64 * f64::from(c - 9);
            ws.write_formula(
                r,
                c,
                rust_xlsxwriter::Formula::new(&formula).set_result(result_val.to_string()),
            )
            .unwrap();
        }
    }

    wb.save(tmp.path()).unwrap();
    tmp
}

#[test]
fn eval_10k_rows_10_formula_cols_under_2_seconds() {
    let input = generate_10k_fixture();
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let start = std::time::Instant::now();
    let summary = evaluate(input.path(), output.path(), None).unwrap();
    let elapsed = start.elapsed();

    // 1 header + 10k data rows.
    assert_eq!(summary.rows_processed, 10_001, "rows_processed mismatch");
    // 10k rows × 10 formula cols.
    assert_eq!(summary.formulas_evaluated, 100_000, "formulas_evaluated mismatch");
    assert!(elapsed.as_secs() < 2, "eval took {elapsed:?}, expected < 2s");
    eprintln!("eval_10k_10formula: {elapsed:?}");
}
