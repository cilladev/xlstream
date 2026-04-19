//! Load lookup sheets from the workbook and build hash indexes.

use std::collections::{HashMap, HashSet};

use xlstream_core::XlStreamError;
use xlstream_io::Reader;
use xlstream_parse::{LookupKey, LookupKind};

use super::LookupSheet;

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

        let mut stream = reader.cells(original_name)?;
        let mut rows = Vec::new();
        while let Some((_row_idx, row_values)) = stream.next_row()? {
            rows.push(row_values);
        }

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
