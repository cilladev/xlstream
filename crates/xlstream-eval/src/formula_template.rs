//! Formula template for reconstructing per-row formula text.

/// A column-level formula template that reconstructs per-row formula text
/// by substituting row numbers at known byte positions.
///
/// Created once per formula column during `build_eval_plan`. The template
/// stores the normalized formula text from the column's first row and the
/// byte positions of relative row-number references within it.
pub(crate) struct FormulaTemplate {
    text: String,
    row_refs: Vec<(usize, usize, u32)>,
    base_row: u32,
}

impl FormulaTemplate {
    /// Build a template from normalized formula text and its 1-based source row.
    pub(crate) fn new(text: String, base_row: u32) -> Self {
        let row_refs = extract_row_refs(&text);
        Self { text, row_refs, base_row }
    }

    /// Reconstruct formula text for `target_row` (1-based Excel row).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub(crate) fn reconstruct(&self, target_row: u32) -> String {
        if self.row_refs.is_empty() {
            return self.text.clone();
        }

        let row_delta: i64 = i64::from(target_row) - i64::from(self.base_row);
        #[allow(clippy::cast_possible_wrap)]
        let max_row = xlstream_core::EXCEL_MAX_ROWS as i64;
        let mut result = String::with_capacity(self.text.len() + 8);
        let mut last_end = 0;

        for &(start, end, base_ref_row) in &self.row_refs {
            result.push_str(&self.text[last_end..start]);
            let new_row = (i64::from(base_ref_row) + row_delta).clamp(1, max_row) as u32;
            result.push_str(&new_row.to_string());
            last_end = end;
        }
        result.push_str(&self.text[last_end..]);
        result
    }
}

/// Scan formula text for cell-reference patterns and return byte positions
/// of relative (non-`$`-locked) row-number substrings.
///
/// Each entry: `(byte_start, byte_end, row_number)`.
///
/// Calamine normalizes cell references to uppercase, so only uppercase
/// column letters are matched.
fn extract_row_refs(text: &str) -> Vec<(usize, usize, u32)> {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut refs = Vec::new();
    let mut i = 0;

    while i < len {
        // Skip double-quoted string literals (Excel escapes "" inside strings).
        if bytes[i] == b'"' {
            i += 1;
            while i < len {
                if bytes[i] == b'"' {
                    if i + 1 < len && bytes[i + 1] == b'"' {
                        i += 2;
                    } else {
                        i += 1;
                        break;
                    }
                } else {
                    i += 1;
                }
            }
            continue;
        }

        // Skip single-quoted sheet names: 'Sheet Name'!
        if bytes[i] == b'\'' {
            i += 1;
            while i < len && bytes[i] != b'\'' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            if i < len && bytes[i] == b'!' {
                i += 1;
            }
            continue;
        }

        // Boundary: preceding char must not be alphanumeric or underscore.
        if i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_') {
            i += 1;
            continue;
        }

        let save = i;

        // Optional '$' for absolute column.
        if i < len && bytes[i] == b'$' {
            i += 1;
        }

        // Column letters: 1-3 uppercase ASCII.
        let col_start = i;
        while i < len && bytes[i].is_ascii_uppercase() {
            i += 1;
        }
        let letter_count = i - col_start;
        if letter_count == 0 || letter_count > 3 {
            i = save + 1;
            continue;
        }

        // Optional '$' for absolute row.
        let has_dollar = i < len && bytes[i] == b'$';
        if has_dollar {
            i += 1;
        }

        // Row digits.
        let digit_start = i;
        while i < len && bytes[i].is_ascii_digit() {
            i += 1;
        }
        let digit_end = i;

        if digit_start == digit_end {
            i = save + 1;
            continue;
        }

        // Trailing boundary: must not be followed by alphanumeric/underscore.
        if i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i = save + 1;
            continue;
        }

        // If followed by '!', this is a sheet name prefix — not a cell ref.
        if i < len && bytes[i] == b'!' {
            i += 1;
            continue;
        }

        if !has_dollar {
            if let Ok(row_num) = text[digit_start..digit_end].parse::<u32>() {
                if row_num > 0 {
                    refs.push((digit_start, digit_end, row_num));
                }
            }
        }
    }

    refs
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    use super::*;

    #[test]
    fn simple_refs_both_detected() {
        let refs = extract_row_refs("A2+B2");
        assert_eq!(refs, vec![(1, 2, 2), (4, 5, 2)]);
    }

    #[test]
    fn absolute_row_skipped() {
        let refs = extract_row_refs("$A$2+B2");
        assert_eq!(refs, vec![(6, 7, 2)]);
    }

    #[test]
    fn mixed_absolute_and_relative_in_range() {
        let refs = extract_row_refs("SUM($A$1:A2)");
        assert_eq!(refs, vec![(10, 11, 2)]);
    }

    #[test]
    fn cross_sheet_ref() {
        let refs = extract_row_refs("Sheet1!A2");
        assert_eq!(refs, vec![(8, 9, 2)]);
    }

    #[test]
    fn whole_column_ref_has_no_row_refs() {
        let refs = extract_row_refs("SUM(A:A)");
        assert_eq!(refs, vec![]);
    }

    #[test]
    fn multi_digit_row() {
        let refs = extract_row_refs("A100");
        assert_eq!(refs, vec![(1, 4, 100)]);
    }

    #[test]
    fn no_cell_refs_in_pure_arithmetic() {
        let refs = extract_row_refs("1+2");
        assert_eq!(refs, vec![]);
    }

    #[test]
    fn quoted_sheet_name_skipped() {
        let refs = extract_row_refs("'My Sheet'!A2");
        assert_eq!(refs, vec![(12, 13, 2)]);
    }

    #[test]
    fn function_name_not_matched() {
        let refs = extract_row_refs("SUM(A2)");
        assert_eq!(refs, vec![(5, 6, 2)]);
    }

    #[test]
    fn range_both_endpoints_detected() {
        let refs = extract_row_refs("A2:B10");
        assert_eq!(refs, vec![(1, 2, 2), (4, 6, 10)]);
    }

    #[test]
    fn absolute_col_relative_row() {
        let refs = extract_row_refs("$A2");
        assert_eq!(refs, vec![(2, 3, 2)]);
    }

    #[test]
    fn sheet_name_looks_like_cell_ref_skipped() {
        let refs = extract_row_refs("AB!C2");
        assert_eq!(refs, vec![(4, 5, 2)]);
    }

    #[test]
    fn max_row_number() {
        let refs = extract_row_refs("A1048576");
        assert_eq!(refs, vec![(1, 8, 1_048_576)]);
    }

    #[test]
    fn string_literal_not_scanned() {
        let refs = extract_row_refs(r#"IF(A2="B3",C2,0)"#);
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].2, 2);
        assert_eq!(refs[1].2, 2);
    }

    #[test]
    fn escaped_quotes_in_string_literal() {
        let refs = extract_row_refs(r#"IF(A2="""",C2,0)"#);
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].2, 2);
        assert_eq!(refs[1].2, 2);
    }

    #[test]
    fn reconstruct_shifts_row_forward() {
        let t = FormulaTemplate::new("A2+B2".into(), 2);
        assert_eq!(t.reconstruct(5), "A5+B5");
    }

    #[test]
    fn reconstruct_shifts_row_backward() {
        let t = FormulaTemplate::new("A10+B10".into(), 10);
        assert_eq!(t.reconstruct(3), "A3+B3");
    }

    #[test]
    fn reconstruct_absolute_refs_unchanged() {
        let t = FormulaTemplate::new("$A$1+B2".into(), 2);
        assert_eq!(t.reconstruct(5), "$A$1+B5");
    }

    #[test]
    fn reconstruct_no_refs_returns_text_unchanged() {
        let t = FormulaTemplate::new("SUM(A:A)".into(), 2);
        assert_eq!(t.reconstruct(100), "SUM(A:A)");
    }

    #[test]
    fn reconstruct_large_row_number() {
        let t = FormulaTemplate::new("A2".into(), 2);
        assert_eq!(t.reconstruct(1_048_576), "A1048576");
    }

    #[test]
    fn reconstruct_same_row_returns_original() {
        let t = FormulaTemplate::new("A5+B5".into(), 5);
        assert_eq!(t.reconstruct(5), "A5+B5");
    }

    #[test]
    fn reconstruct_multi_digit_to_single() {
        let t = FormulaTemplate::new("A100".into(), 100);
        assert_eq!(t.reconstruct(2), "A2");
    }

    #[test]
    fn reconstruct_clamps_to_max_row() {
        let t = FormulaTemplate::new("A1".into(), 1);
        let result = t.reconstruct(1_048_577);
        assert!(result.contains("1048576"));
    }

    #[test]
    fn reconstruct_clamps_to_min_row() {
        let t = FormulaTemplate::new("A2".into(), 100);
        let result = t.reconstruct(1);
        assert_eq!(result, "A1");
    }
}
