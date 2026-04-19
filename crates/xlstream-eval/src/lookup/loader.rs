//! Load lookup sheets from the workbook and build hash indexes.

use std::collections::{HashMap, HashSet};

use xlstream_core::{Value, XlStreamError};
use xlstream_io::Reader;
use xlstream_parse::{extract_references, parse, LookupKey, LookupKind, Reference};

use super::LookupSheet;
use crate::interp::Interpreter;
use crate::prelude::Prelude;
use crate::scope::RowScope;
use crate::topo::topo_sort;

/// Load lookup sheets and build hash indexes based on collected requirements.
///
/// `lookup_keys` — extracted from the ASTs via `collect_lookup_keys`.
/// `reader` — the workbook reader (calls `reader.cells(sheet)` per sheet).
///
/// Returns a map from lowercased sheet name to [`LookupSheet`].
///
/// # Errors
///
/// Returns [`XlStreamError::Xlsx`] if a referenced sheet cannot be read.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_eval::lookup::load_lookup_sheets;
/// use xlstream_io::Reader;
///
/// let mut reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
/// let sheets = load_lookup_sheets(&[], &mut reader).unwrap();
/// assert!(sheets.is_empty());
/// ```
pub fn load_lookup_sheets(
    lookup_keys: &[LookupKey],
    reader: &mut Reader,
) -> Result<HashMap<String, LookupSheet>, XlStreamError> {
    if lookup_keys.is_empty() {
        return Ok(HashMap::new());
    }

    let mut requirements: HashMap<String, SheetRequirements> = HashMap::new();
    for key in lookup_keys {
        let lower_sheet = key.sheet.to_ascii_lowercase();
        let req = requirements.entry(lower_sheet).or_insert_with(|| SheetRequirements {
            col_keys: HashSet::new(),
            row_keys: HashSet::new(),
        });
        match key.kind {
            LookupKind::VLookup | LookupKind::XLookup | LookupKind::IndexMatch => {
                req.col_keys.insert(key.key_index.saturating_sub(1));
            }
            LookupKind::HLookup => {
                req.row_keys.insert(0);
            }
        }
    }

    let sheet_names = reader.sheet_names();
    let mut result = HashMap::new();

    for (lower_name, req) in &requirements {
        let original_name = sheet_names
            .iter()
            .find(|n| n.to_ascii_lowercase() == *lower_name)
            .ok_or_else(|| {
                XlStreamError::Xlsx(format!("lookup sheet '{lower_name}' not found in workbook"))
            })?;

        // TODO(perf): calamine requires separate passes for cells and formulas
        let formulas = reader.formulas(original_name)?;

        let mut stream = reader.cells(original_name)?;
        let mut rows = Vec::new();
        while let Some((_row_idx, row_values)) = stream.next_row()? {
            rows.push(row_values);
        }
        drop(stream);

        evaluate_lookup_formulas(original_name, &mut rows, &formulas)?;

        let mut sheet = LookupSheet::new(rows);
        for &col in &req.col_keys {
            sheet.build_col_index(col);
            sheet.build_col_sorted(col);
        }
        for &row in &req.row_keys {
            sheet.build_row_index(row);
            sheet.build_row_sorted(row);
        }

        result.insert(lower_name.clone(), sheet);
    }

    Ok(result)
}

struct SheetRequirements {
    col_keys: HashSet<u32>,
    row_keys: HashSet<u32>,
}

/// Evaluate formulas within a lookup sheet, modifying `rows` in place.
///
/// Groups formulas by column, topo-sorts by inter-column dependencies,
/// then evaluates row-by-row in dependency order. Only same-sheet cell
/// references are allowed; cross-sheet refs produce an error.
#[allow(clippy::cast_possible_truncation)]
fn evaluate_lookup_formulas(
    sheet_name: &str,
    rows: &mut [Vec<Value>],
    formulas: &[(u32, u32, String)],
) -> Result<(), XlStreamError> {
    if formulas.is_empty() {
        return Ok(());
    }

    // Group formulas by column. Use the first formula per column as
    // representative; warn if different rows have different formulas.
    let mut col_formulas: HashMap<u32, (String, HashSet<(u32, u32)>)> = HashMap::new();
    for (row, col, text) in formulas {
        let entry = col_formulas.entry(*col).or_insert_with(|| (text.clone(), HashSet::new()));
        if entry.0 != *text {
            tracing::warn!(
                sheet = sheet_name,
                col = col,
                "lookup sheet column has varying formulas; using first as representative"
            );
        }
        entry.1.insert((*row, *col));
    }

    // Build a set of (row, col) pairs that are formula cells for quick lookup.
    let formula_positions: HashSet<(u32, u32)> =
        formulas.iter().map(|(r, c, _)| (*r, *c)).collect();

    // Parse each representative formula, extract references, build deps.
    let mut col_asts: HashMap<u32, xlstream_parse::Ast> = HashMap::new();
    let mut formula_deps: Vec<(u32, Vec<u32>)> = Vec::new();
    let mut formula_col_set: HashSet<u32> = HashSet::new();

    for (&col, (text, _)) in &col_formulas {
        let ast = parse(text)?;
        let refs = extract_references(&ast);

        // Refuse cross-sheet references in cells.
        for r in &refs.cells {
            if let Reference::Cell { sheet: Some(s), .. } = r {
                if !s.eq_ignore_ascii_case(sheet_name) {
                    return Err(XlStreamError::Classification {
                        address: format!("{sheet_name}!col{col}"),
                        message: format!("lookup sheet formula references external sheet '{s}'"),
                    });
                }
            }
        }
        // Refuse cross-sheet references in ranges.
        for r in &refs.ranges {
            if let Reference::Range { sheet: Some(s), .. } = r {
                if !s.eq_ignore_ascii_case(sheet_name) {
                    return Err(XlStreamError::Classification {
                        address: format!("{sheet_name}!col{col}"),
                        message: format!("lookup sheet formula references external sheet '{s}'"),
                    });
                }
            }
        }

        // Collect column dependencies from cell refs (1-based col → 0-based).
        let mut deps = Vec::new();
        for r in &refs.cells {
            if let Reference::Cell { col: ref_col, .. } = r {
                deps.push(ref_col.saturating_sub(1));
            }
        }

        formula_col_set.insert(col);
        formula_deps.push((col, deps));
        col_asts.insert(col, ast);
    }

    let topo_order = topo_sort(&formula_deps, &formula_col_set)?;

    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);

    for (row_idx, row) in rows.iter_mut().enumerate() {
        for &fcol in &topo_order {
            if !formula_positions.contains(&(row_idx as u32, fcol)) {
                continue;
            }
            let Some(ast) = col_asts.get(&fcol) else {
                continue;
            };
            let fcol_idx = fcol as usize;
            if fcol_idx >= row.len() {
                row.resize(fcol_idx + 1, Value::Empty);
            }
            // Scoped block: borrow row immutably for RowScope, then write.
            let result = {
                let scope = RowScope::new(row, row_idx as u32);
                interp.eval(ast.root(), &scope)
            };
            row[fcol_idx] = result;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use rust_xlsxwriter::{Formula, Workbook};
    use tempfile::NamedTempFile;
    use xlstream_core::Value;
    use xlstream_io::Reader;
    use xlstream_parse::{LookupKey, LookupKind};

    use super::*;

    fn make_lookup_key(sheet: &str, kind: LookupKind, key_index: u32) -> LookupKey {
        LookupKey { sheet: sheet.to_string(), kind, key_index, value_index: key_index }
    }

    #[test]
    fn lookup_sheet_with_formulas_evaluates_helper_column() {
        // Col A: region code, Col B: sub-code, Col C: =A{row}&"|"&B{row}
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let path = tmp.path().to_path_buf();
        {
            let mut wb = Workbook::new();
            let ws = wb.add_worksheet().set_name("Regions").unwrap();
            // Row 0 (header)
            ws.write_string(0, 0, "Region").unwrap();
            ws.write_string(0, 1, "Sub").unwrap();
            ws.write_string(0, 2, "Key").unwrap();
            // Row 1
            ws.write_string(1, 0, "EMEA").unwrap();
            ws.write_string(1, 1, "EU").unwrap();
            ws.write_formula(1, 2, Formula::new("A2&\"|\"&B2")).unwrap();
            // Row 2
            ws.write_string(2, 0, "APAC").unwrap();
            ws.write_string(2, 1, "AP").unwrap();
            ws.write_formula(2, 2, Formula::new("A3&\"|\"&B3")).unwrap();
            wb.save(&path).unwrap();
        }

        let mut reader = Reader::open(&path).unwrap();
        let keys = vec![make_lookup_key("Regions", LookupKind::VLookup, 1)];
        let sheets = load_lookup_sheets(&keys, &mut reader).unwrap();
        let sheet = sheets.get("regions").unwrap();

        // Row 1, Col 2 should be "EMEA|EU"
        assert_eq!(sheet.cell(1, 2), Some(&Value::Text("EMEA|EU".into())));
        // Row 2, Col 2 should be "APAC|AP"
        assert_eq!(sheet.cell(2, 2), Some(&Value::Text("APAC|AP".into())));
    }

    #[test]
    fn lookup_sheet_formula_column_is_indexable() {
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let path = tmp.path().to_path_buf();
        {
            let mut wb = Workbook::new();
            let ws = wb.add_worksheet().set_name("Regions").unwrap();
            ws.write_string(0, 0, "Region").unwrap();
            ws.write_string(0, 1, "Sub").unwrap();
            ws.write_string(0, 2, "Key").unwrap();
            ws.write_string(1, 0, "EMEA").unwrap();
            ws.write_string(1, 1, "EU").unwrap();
            ws.write_formula(1, 2, Formula::new("A2&\"|\"&B2")).unwrap();
            ws.write_string(2, 0, "APAC").unwrap();
            ws.write_string(2, 1, "AP").unwrap();
            ws.write_formula(2, 2, Formula::new("A3&\"|\"&B3")).unwrap();
            wb.save(&path).unwrap();
        }

        let mut reader = Reader::open(&path).unwrap();
        let keys = vec![make_lookup_key("Regions", LookupKind::VLookup, 3)];
        let sheets = load_lookup_sheets(&keys, &mut reader).unwrap();
        let sheet = sheets.get("regions").unwrap();

        let key = crate::lookup::LookupValue::from_value(&Value::Text("APAC|AP".into())).unwrap();
        assert_eq!(sheet.col_lookup(2, &key), Some(2));
    }

    #[test]
    fn lookup_sheet_simple_cell_ref_formula() {
        // Col B = A{row}
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let path = tmp.path().to_path_buf();
        {
            let mut wb = Workbook::new();
            let ws = wb.add_worksheet().set_name("Data").unwrap();
            ws.write_string(0, 0, "Name").unwrap();
            ws.write_string(0, 1, "Copy").unwrap();
            ws.write_string(1, 0, "hello").unwrap();
            ws.write_formula(1, 1, Formula::new("A2")).unwrap();
            wb.save(&path).unwrap();
        }

        let mut reader = Reader::open(&path).unwrap();
        let keys = vec![make_lookup_key("Data", LookupKind::VLookup, 1)];
        let sheets = load_lookup_sheets(&keys, &mut reader).unwrap();
        let sheet = sheets.get("data").unwrap();

        // cell(1,1) should equal cell(1,0)
        assert_eq!(sheet.cell(1, 1), Some(&Value::Text("hello".into())));
        assert_eq!(sheet.cell(1, 0), sheet.cell(1, 1));
    }

    #[test]
    fn lookup_sheet_no_formulas_unchanged() {
        // Pure data sheet — no formulas. Should load identically to before.
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let path = tmp.path().to_path_buf();
        {
            let mut wb = Workbook::new();
            let ws = wb.add_worksheet().set_name("Static").unwrap();
            ws.write_string(0, 0, "A").unwrap();
            ws.write_number(0, 1, 42.0).unwrap();
            ws.write_string(1, 0, "B").unwrap();
            ws.write_number(1, 1, 99.0).unwrap();
            wb.save(&path).unwrap();
        }

        let mut reader = Reader::open(&path).unwrap();
        let keys = vec![make_lookup_key("Static", LookupKind::VLookup, 1)];
        let sheets = load_lookup_sheets(&keys, &mut reader).unwrap();
        let sheet = sheets.get("static").unwrap();

        assert_eq!(sheet.cell(0, 0), Some(&Value::Text("A".into())));
        assert_eq!(sheet.cell(0, 1), Some(&Value::Number(42.0)));
        assert_eq!(sheet.cell(1, 0), Some(&Value::Text("B".into())));
        assert_eq!(sheet.cell(1, 1), Some(&Value::Number(99.0)));
    }

    #[test]
    fn lookup_sheet_cross_sheet_ref_errors() {
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let path = tmp.path().to_path_buf();
        {
            let mut wb = Workbook::new();
            let ws = wb.add_worksheet().set_name("Lookup").unwrap();
            ws.write_string(0, 0, "X").unwrap();
            ws.write_formula(0, 1, Formula::new("Main!A1")).unwrap();
            // Need a Main sheet to exist in the workbook
            let ws2 = wb.add_worksheet().set_name("Main").unwrap();
            ws2.write_string(0, 0, "val").unwrap();
            wb.save(&path).unwrap();
        }

        let mut reader = Reader::open(&path).unwrap();
        let keys = vec![make_lookup_key("Lookup", LookupKind::VLookup, 1)];
        let err = load_lookup_sheets(&keys, &mut reader).err();
        assert!(err.is_some(), "expected error for cross-sheet ref");
        let msg = format!("{}", err.unwrap());
        assert!(msg.contains("external sheet"), "error should mention external sheet, got: {msg}");
    }

    #[test]
    fn lookup_sheet_formula_chain_dependency() {
        // Col A: data "hello", Col B: =A{row}, Col C: =B{row}&"!"
        // Tests topo ordering where C depends on B depends on A.
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let path = tmp.path().to_path_buf();
        {
            let mut wb = Workbook::new();
            let ws = wb.add_worksheet().set_name("Chain").unwrap();
            ws.write_string(0, 0, "Val").unwrap();
            ws.write_string(0, 1, "Copy").unwrap();
            ws.write_string(0, 2, "Bang").unwrap();
            ws.write_string(1, 0, "hello").unwrap();
            ws.write_formula(1, 1, Formula::new("A2")).unwrap();
            ws.write_formula(1, 2, Formula::new("B2&\"!\"")).unwrap();
            wb.save(&path).unwrap();
        }

        let mut reader = Reader::open(&path).unwrap();
        let keys = vec![make_lookup_key("Chain", LookupKind::VLookup, 1)];
        let sheets = load_lookup_sheets(&keys, &mut reader).unwrap();
        let sheet = sheets.get("chain").unwrap();

        assert_eq!(sheet.cell(1, 1), Some(&Value::Text("hello".into())));
        assert_eq!(sheet.cell(1, 2), Some(&Value::Text("hello!".into())));
    }
}
