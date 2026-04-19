//! [`LookupSheet`] — pre-loaded lookup sheet with hash indexes.

use std::collections::HashMap;

use xlstream_core::Value;

use super::value::LookupValue;

/// A fully-loaded lookup sheet with optional hash indexes for exact match.
///
/// Row data is stored once. Multiple key columns each get their own
/// hash index via [`build_col_index`](Self::build_col_index).
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::lookup::{LookupSheet, LookupValue};
///
/// let rows = vec![
///     vec![Value::Text("A".into()), Value::Number(1.0)],
///     vec![Value::Text("B".into()), Value::Number(2.0)],
/// ];
/// let mut sheet = LookupSheet::new(rows);
/// sheet.build_col_index(0);
/// let key = LookupValue::from_value(&Value::Text("B".into())).unwrap();
/// assert_eq!(sheet.col_lookup(0, &key), Some(1));
/// ```
pub struct LookupSheet {
    rows: Vec<Vec<Value>>,
    col_indexes: HashMap<u32, HashMap<LookupValue, usize>>,
    row_indexes: HashMap<u32, HashMap<LookupValue, usize>>,
    col_sorted: HashMap<u32, Vec<(LookupValue, usize)>>,
}

impl LookupSheet {
    /// Wrap pre-loaded row data. No indexes built yet.
    #[must_use]
    pub fn new(rows: Vec<Vec<Value>>) -> Self {
        Self {
            rows,
            col_indexes: HashMap::new(),
            row_indexes: HashMap::new(),
            col_sorted: HashMap::new(),
        }
    }

    /// Build a column-keyed exact-match index for VLOOKUP/XLOOKUP/MATCH.
    ///
    /// `col` is 0-based. First match wins for duplicate keys.
    pub fn build_col_index(&mut self, col: u32) {
        if self.col_indexes.contains_key(&col) {
            return;
        }
        let mut index = HashMap::new();
        for (row_idx, row) in self.rows.iter().enumerate() {
            if let Some(cell) = row.get(col as usize) {
                if let Some(key) = LookupValue::from_value(cell) {
                    index.entry(key).or_insert(row_idx);
                }
            }
        }
        self.col_indexes.insert(col, index);
    }

    /// Build a row-keyed exact-match index for HLOOKUP.
    ///
    /// `row` is 0-based. Indexes all cells in that row by value,
    /// mapping to their 0-based column index. First match wins.
    pub fn build_row_index(&mut self, row: u32) {
        if self.row_indexes.contains_key(&row) {
            return;
        }
        let mut index = HashMap::new();
        if let Some(row_data) = self.rows.get(row as usize) {
            for (col_idx, cell) in row_data.iter().enumerate() {
                if let Some(key) = LookupValue::from_value(cell) {
                    index.entry(key).or_insert(col_idx);
                }
            }
        }
        self.row_indexes.insert(row, index);
    }

    /// Build a sorted index for approximate match (binary search).
    ///
    /// `col` is 0-based. Collects `(key, row_idx)` pairs and sorts by key.
    pub fn build_col_sorted(&mut self, col: u32) {
        if self.col_sorted.contains_key(&col) {
            return;
        }
        let mut sorted: Vec<(LookupValue, usize)> = Vec::new();
        for (row_idx, row) in self.rows.iter().enumerate() {
            if let Some(cell) = row.get(col as usize) {
                if let Some(key) = LookupValue::from_value(cell) {
                    sorted.push((key, row_idx));
                }
            }
        }
        sorted.sort_by(|(a, _), (b, _)| a.cmp(b));
        self.col_sorted.insert(col, sorted);
    }

    /// Probe a column-keyed index. Returns 0-based row index.
    #[must_use]
    pub fn col_lookup(&self, col: u32, key: &LookupValue) -> Option<usize> {
        self.col_indexes.get(&col)?.get(key).copied()
    }

    /// Probe a row-keyed index. Returns 0-based column index.
    #[must_use]
    pub fn row_lookup(&self, row: u32, key: &LookupValue) -> Option<usize> {
        self.row_indexes.get(&row)?.get(key).copied()
    }

    /// Approximate match: find the largest key <= the given key.
    ///
    /// Returns the 0-based row index, or `None` if the key is below
    /// all values in the sorted index.
    #[must_use]
    pub fn col_approx_lookup(&self, col: u32, key: &LookupValue) -> Option<usize> {
        let sorted = self.col_sorted.get(&col)?;
        let pos = sorted.partition_point(|(k, _)| k <= key);
        if pos == 0 {
            None
        } else {
            Some(sorted[pos - 1].1)
        }
    }

    /// Access a cell by 0-based (row, col).
    #[must_use]
    pub fn cell(&self, row: usize, col: usize) -> Option<&Value> {
        self.rows.get(row).and_then(|r| r.get(col))
    }

    /// Number of rows in the sheet.
    #[must_use]
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    /// Number of columns (from the first row; 0 if empty).
    #[must_use]
    pub fn num_cols(&self) -> usize {
        self.rows.first().map_or(0, Vec::len)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use xlstream_core::Value;

    use super::*;

    fn sample_rows() -> Vec<Vec<Value>> {
        vec![
            vec![Value::Text("EMEA".into()), Value::Number(100.0), Value::Text("Europe".into())],
            vec![Value::Text("APAC".into()), Value::Number(200.0), Value::Text("Asia".into())],
            vec![Value::Text("AMER".into()), Value::Number(300.0), Value::Text("Americas".into())],
        ]
    }

    #[test]
    fn col_index_exact_hit() {
        let mut sheet = LookupSheet::new(sample_rows());
        sheet.build_col_index(0);
        let key = LookupValue::from_value(&Value::Text("apac".into())).unwrap();
        assert_eq!(sheet.col_lookup(0, &key), Some(1));
    }

    #[test]
    fn col_index_exact_miss() {
        let mut sheet = LookupSheet::new(sample_rows());
        sheet.build_col_index(0);
        let key = LookupValue::from_value(&Value::Text("LATAM".into())).unwrap();
        assert_eq!(sheet.col_lookup(0, &key), None);
    }

    #[test]
    fn col_index_first_match_wins() {
        let rows = vec![
            vec![Value::Text("A".into()), Value::Number(1.0)],
            vec![Value::Text("A".into()), Value::Number(2.0)],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_index(0);
        let key = LookupValue::from_value(&Value::Text("A".into())).unwrap();
        let row_idx = sheet.col_lookup(0, &key).unwrap();
        assert_eq!(sheet.cell(row_idx, 1), Some(&Value::Number(1.0)));
    }

    #[test]
    fn cell_access_by_position() {
        let sheet = LookupSheet::new(sample_rows());
        assert_eq!(sheet.cell(1, 2), Some(&Value::Text("Asia".into())));
    }

    #[test]
    fn cell_out_of_bounds_returns_none() {
        let sheet = LookupSheet::new(sample_rows());
        assert_eq!(sheet.cell(10, 0), None);
        assert_eq!(sheet.cell(0, 10), None);
    }

    #[test]
    fn row_index_for_hlookup() {
        let rows = vec![
            vec![Value::Text("Name".into()), Value::Text("Age".into()), Value::Text("City".into())],
            vec![Value::Text("Alice".into()), Value::Number(30.0), Value::Text("NYC".into())],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_row_index(0);
        let key = LookupValue::from_value(&Value::Text("age".into())).unwrap();
        assert_eq!(sheet.row_lookup(0, &key), Some(1));
    }

    #[test]
    fn num_rows_and_cols() {
        let sheet = LookupSheet::new(sample_rows());
        assert_eq!(sheet.num_rows(), 3);
        assert_eq!(sheet.num_cols(), 3);
    }

    #[test]
    fn col_approx_lookup_finds_largest_lte() {
        let rows = vec![
            vec![Value::Number(10.0), Value::Text("ten".into())],
            vec![Value::Number(20.0), Value::Text("twenty".into())],
            vec![Value::Number(30.0), Value::Text("thirty".into())],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_sorted(0);
        let key = LookupValue::from_value(&Value::Number(25.0)).unwrap();
        assert_eq!(sheet.col_approx_lookup(0, &key), Some(1));
    }

    #[test]
    fn col_approx_lookup_below_first_returns_none() {
        let rows = vec![vec![Value::Number(10.0)], vec![Value::Number(20.0)]];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_sorted(0);
        let key = LookupValue::from_value(&Value::Number(5.0)).unwrap();
        assert_eq!(sheet.col_approx_lookup(0, &key), None);
    }

    #[test]
    fn col_approx_lookup_exact_hit() {
        let rows = vec![vec![Value::Number(10.0)], vec![Value::Number(20.0)]];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_sorted(0);
        let key = LookupValue::from_value(&Value::Number(20.0)).unwrap();
        assert_eq!(sheet.col_approx_lookup(0, &key), Some(1));
    }
}
