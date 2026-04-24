use super::helpers;
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
fn unreferenced_sheet_formulas_are_evaluated() {
    let input = helpers::generate_multi_sheet_formula_fixture();
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary = evaluate(input.path(), output.path(), None).unwrap();

    let mut reader = Reader::open(output.path()).unwrap();

    // Sheet1 (main) — should always work.
    let rows1 = read_all_rows(&mut reader, "Sheet1");
    assert_eq!(rows1.len(), 3); // header + 2 data
    assert_eq!(rows1[1].1[1], Value::Number(10000.0), "Sheet1 B2: 5000*2");
    assert_eq!(rows1[2].1[1], Value::Number(200.0), "Sheet1 B3: 100*2");

    // Sheet2 (unreferenced) — the bug: currently produces 0/Empty.
    let rows2 = read_all_rows(&mut reader, "Sheet2");
    assert_eq!(rows2.len(), 4); // header + 3 data
    assert_eq!(rows2[1].1[1], Value::Number(0.16), "Sheet2 B2: 0.08*2");
    assert_eq!(rows2[2].1[1], Value::Number(100.0), "Sheet2 B3: 50*2");

    // Formulas on both sheets counted.
    // Sheet1: 2 rows * 1 formula col = 2
    // Sheet2: 3 rows * 1 formula col = 3
    assert_eq!(summary.formulas_evaluated, 5, "formulas_evaluated mismatch");
}

#[test]
fn secondary_sheet_cross_ref_to_main_sheet() {
    let input = helpers::generate_multi_sheet_formula_fixture();
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), None).unwrap();

    let mut reader = Reader::open(output.path()).unwrap();
    let rows2 = read_all_rows(&mut reader, "Sheet2");

    // Row 3 has =Sheet1!A2*2 -> Sheet1 A2 = 5000 -> result = 10000.
    assert_eq!(rows2[3].1[1], Value::Number(10000.0), "Sheet2 B4: Sheet1!A2*2");
}

#[test]
fn same_sheet_mixed_formulas() {
    use rust_xlsxwriter::{Formula, Workbook};

    let input = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "X").unwrap();
    ws.write_string(0, 1, "Result").unwrap();
    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_formula(1, 1, Formula::new("=A2*2")).unwrap();
    ws.write_number(2, 0, 20.0).unwrap();
    ws.write_formula(2, 1, Formula::new("=A3+2")).unwrap();
    ws.write_number(3, 0, 30.0).unwrap();
    ws.write_formula(3, 1, Formula::new("=A4*10")).unwrap();

    wb.save(input.path()).unwrap();

    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    evaluate(input.path(), output.path(), None).unwrap();

    let mut reader = Reader::open(output.path()).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    assert_eq!(rows[1].1[1], Value::Number(20.0), "B2: 10*2");
    assert_eq!(rows[2].1[1], Value::Number(22.0), "B3: 20+2");
    assert_eq!(rows[3].1[1], Value::Number(300.0), "B4: 30*10");
}
