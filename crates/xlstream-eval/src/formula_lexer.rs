//! Lightweight formula lexer for streaming-equivalence comparison.
//!
//! [`formulas_streaming_eq`] checks if two formula strings differ only
//! in same-sheet cell reference row numbers, without parsing. Used as a
//! fast pre-filter to skip redundant calls to `formualizer_parse`.

/// Check if two formula texts are streaming-equivalent without parsing.
///
/// Uses a lightweight lexer to tokenize both formulas in lockstep. Each
/// token is classified as one of: cell-ref row digits (skippable),
/// identifier, number, string literal, operator, or other. Only
/// same-sheet cell-ref row digits are allowed to differ; everything else
/// must match exactly.
///
/// The lexer distinguishes `A2` (column + row) from `LOG10` (identifier),
/// `Sheet2!` (sheet qualifier), `RATE2` (named range), and `1E2`
/// (scientific notation) — all of which contain letters followed by
/// digits but are NOT cell references.
///
/// False negatives cause a parse + AST compare (correct but slower).
/// False positives would silently produce wrong results.
pub(crate) fn formulas_streaming_eq(a: &str, b: &str) -> bool {
    let mut la = FormulaLexer::new(a.as_bytes());
    let mut lb = FormulaLexer::new(b.as_bytes());

    loop {
        let ta = la.next_token();
        let tb = lb.next_token();

        match (&ta, &tb) {
            (FToken::End, FToken::End) => return true,
            (FToken::CellRow, FToken::CellRow) => {}
            _ => {
                if ta != tb {
                    return false;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FToken {
    /// Same-sheet cell reference row digits — skippable
    CellRow,
    /// Exact-match content (operators, identifiers, numbers, etc.)
    Exact(u8),
    /// String literal content byte — exact match
    StringByte(u8),
    /// End of input
    End,
}

struct FormulaLexer<'a> {
    s: &'a [u8],
    pos: usize,
    in_string: bool,
    /// Next digits are a same-sheet cell-ref row number
    after_column: bool,
    /// Remaining bytes of an identifier to emit as Exact (skips column check)
    ident_remaining: usize,
}

impl<'a> FormulaLexer<'a> {
    fn new(s: &'a [u8]) -> Self {
        Self { s, pos: 0, in_string: false, after_column: false, ident_remaining: 0 }
    }

    fn next_token(&mut self) -> FToken {
        if self.pos >= self.s.len() {
            return FToken::End;
        }

        // Emit remaining identifier bytes without re-entering alpha logic
        if self.ident_remaining > 0 {
            self.ident_remaining -= 1;
            let b = self.s[self.pos];
            self.pos += 1;
            self.after_column = false;
            return FToken::Exact(b);
        }

        let b = self.s[self.pos];

        // String literal: pass through byte-by-byte with exact match
        if b == b'"' {
            self.in_string = !self.in_string;
            self.pos += 1;
            self.after_column = false;
            return FToken::Exact(b'"');
        }
        if self.in_string {
            self.pos += 1;
            return FToken::StringByte(b);
        }

        // Letter run: could be a column ref, function name, sheet name,
        // or named range. Consume the full alpha run, then look ahead.
        if b.is_ascii_alphabetic() || b == b'$' {
            return self.lex_alpha_or_dollar();
        }

        // Digit run: if after_column is set, this is a cell row number.
        // Otherwise it's a plain number.
        if b.is_ascii_digit() {
            if self.after_column {
                self.after_column = false;
                // Check: not followed by : (range bound)
                let mut end = self.pos;
                while end < self.s.len() && self.s[end].is_ascii_digit() {
                    end += 1;
                }
                if end < self.s.len() && self.s[end] == b':' {
                    // Range start bound — digits must match exactly
                    self.pos += 1;
                    return FToken::Exact(b);
                }
                // Skip entire digit run as one CellRow token
                while self.pos < self.s.len() && self.s[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
                return FToken::CellRow;
            }
            self.pos += 1;
            self.after_column = false;
            return FToken::Exact(b);
        }

        // Everything else: operators, parens, commas, etc.
        self.pos += 1;
        self.after_column = false;
        FToken::Exact(b)
    }

    /// Lex an alpha run (possibly preceded by $). Determines whether
    /// it's a same-sheet column ref (sets `after_column=true`) or something
    #[allow(clippy::too_many_lines)]
    /// else (identifier, sheet name, etc.).
    fn lex_alpha_or_dollar(&mut self) -> FToken {
        let start = self.pos;

        // Optional leading $ (column-absolute or row-absolute)
        let has_dollar = self.s[self.pos] == b'$';
        if has_dollar {
            self.pos += 1;
            if self.pos >= self.s.len() || !self.s[self.pos].is_ascii_alphabetic() {
                // $ before digit when after_column — row-absolute marker
                if self.after_column && self.pos < self.s.len() && self.s[self.pos].is_ascii_digit()
                {
                    return FToken::Exact(b'$');
                }
                // Lone $ or $ before non-alpha/non-digit — emit as exact
                self.after_column = false;
                return FToken::Exact(b'$');
            }
        }

        // Consume alpha run
        let alpha_start = self.pos;
        while self.pos < self.s.len() && self.s[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }
        let alpha_len = self.pos - alpha_start;

        // Look ahead to classify
        let next = self.s.get(self.pos).copied();

        // If followed by ! or '! — this is a sheet qualifier, not a column
        if next == Some(b'!') || next == Some(b'\'') {
            let total = self.pos - start;
            self.after_column = false;
            self.pos = start + 1;
            if total > 1 {
                self.ident_remaining = total - 1;
            }
            return FToken::Exact(self.s[start]);
        }

        // If followed by ( — this is a function name, not a column
        if next == Some(b'(') {
            let total = self.pos - start;
            self.after_column = false;
            self.pos = start + 1;
            if total > 1 {
                self.ident_remaining = total - 1;
            }
            return FToken::Exact(self.s[start]);
        }

        // If alpha run is 1-3 chars and followed by $ or digit — potential column ref
        if (1..=3).contains(&alpha_len) {
            let mut peek = self.pos;
            if peek < self.s.len() && self.s[peek] == b'$' {
                peek += 1;
            }
            if peek < self.s.len() && self.s[peek].is_ascii_digit() {
                // Peek past digit run to check for ( — function like LOG10(
                let mut digit_end = peek;
                while digit_end < self.s.len() && self.s[digit_end].is_ascii_digit() {
                    digit_end += 1;
                }
                if digit_end < self.s.len() && self.s[digit_end] == b'(' {
                    self.after_column = false;
                    self.pos = start + 1;
                    return FToken::Exact(self.s[start]);
                }
                // Peek past digit run for ! — sheet name like Sheet2!
                if digit_end < self.s.len()
                    && (self.s[digit_end] == b'!' || self.s[digit_end] == b'\'')
                {
                    self.after_column = false;
                    self.pos = start + 1;
                    return FToken::Exact(self.s[start]);
                }
                // Preceded by digit — scientific notation like 1E2
                if start > 0 && self.s[start - 1].is_ascii_digit() {
                    self.after_column = false;
                    self.pos = start + 1;
                    return FToken::Exact(self.s[start]);
                }
                // Preceded by ! (or $!) — cross-sheet ref, row matters
                let mut bang_back = start;
                if bang_back > 0 && self.s[bang_back - 1] == b'$' {
                    bang_back -= 1;
                }
                if bang_back > 0 && self.s[bang_back - 1] == b'!' {
                    let total = self.pos - start;
                    self.after_column = false;
                    self.pos = start + 1;
                    if total > 1 {
                        self.ident_remaining = total - 1;
                    }
                    return FToken::Exact(self.s[start]);
                }
                // Preceded by : (or $:) — range end, row matters
                let mut back = start;
                if back > 0 && self.s[back - 1] == b'$' {
                    back -= 1;
                }
                if back > 0 && self.s[back - 1] == b':' {
                    let total = self.pos - start;
                    self.after_column = false;
                    self.pos = start + 1;
                    if total > 1 {
                        self.ident_remaining = total - 1;
                    }
                    return FToken::Exact(self.s[start]);
                }
                // Valid same-sheet column ref
                self.after_column = true;
                self.pos = start + 1;
                return FToken::Exact(self.s[start]);
            }
        }

        // 4+ chars or not followed by digit — identifier/named range.
        // Mark remaining bytes so they emit as Exact without re-entering
        // column detection (prevents RATE2 → R + ATE2 misparse).
        let total_consumed = self.pos - start;
        self.after_column = false;
        self.pos = start + 1;
        if total_consumed > 1 {
            self.ident_remaining = total_consumed - 1;
        }
        FToken::Exact(self.s[start])
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    // ===== formulas_streaming_eq =====

    #[test]
    fn streaming_eq_same_formula() {
        assert!(super::formulas_streaming_eq("A2+B2", "A2+B2"));
    }

    #[test]
    fn streaming_eq_different_rows() {
        assert!(super::formulas_streaming_eq("A2+B2", "A3+B3"));
    }

    #[test]
    fn streaming_eq_preserves_plain_numbers() {
        assert!(super::formulas_streaming_eq("MOD(A2,1000)", "MOD(A3,1000)"));
        assert!(!super::formulas_streaming_eq("MOD(A2,1000)", "MOD(A3,2000)"));
    }

    #[test]
    fn streaming_eq_absolute_refs() {
        assert!(super::formulas_streaming_eq("$A$2+$B$3", "$A$5+$B$6"));
    }

    #[test]
    fn streaming_eq_range_bounds_must_match() {
        assert!(!super::formulas_streaming_eq("SUM(A2:A10)", "SUM(A2:A5)"));
        assert!(super::formulas_streaming_eq("SUM(A2:A10)", "SUM(A2:A10)"));
    }

    #[test]
    fn streaming_eq_cross_sheet_rows_must_match() {
        assert!(!super::formulas_streaming_eq("Data!B2+B2", "Data!B3+B3"));
    }

    #[test]
    fn streaming_eq_same_sheet_cell_refs() {
        assert!(super::formulas_streaming_eq(
            "VLOOKUP(A2,Sheet1!A:B,2,FALSE)",
            "VLOOKUP(A3,Sheet1!A:B,2,FALSE)",
        ));
    }

    #[test]
    fn streaming_eq_string_literals_preserved() {
        assert!(super::formulas_streaming_eq("IF(A2=\"Row1\",B2,C2)", "IF(A3=\"Row1\",B3,C3)",));
        assert!(!super::formulas_streaming_eq("IF(A2=\"Row1\",B2,C2)", "IF(A3=\"Row2\",B3,C3)",));
    }

    #[test]
    fn streaming_eq_different_functions() {
        assert!(!super::formulas_streaming_eq("SUM(A2)", "AVG(A2)"));
    }

    #[test]
    fn streaming_eq_different_columns() {
        assert!(!super::formulas_streaming_eq("A2+B2", "A2+C2"));
    }

    #[test]
    fn streaming_eq_multi_letter_columns() {
        assert!(super::formulas_streaming_eq("AA100+AB200", "AA101+AB201"));
    }

    // --- false-positive regression tests ---

    #[test]
    fn streaming_eq_function_names_with_digits() {
        assert!(!super::formulas_streaming_eq("LOG10(A2)", "LOG2(A3)"));
    }

    #[test]
    fn streaming_eq_sheet_names_with_digits() {
        assert!(!super::formulas_streaming_eq("Sheet2!A2", "Sheet3!A3"));
    }

    #[test]
    fn streaming_eq_cross_sheet_dollar_col() {
        assert!(!super::formulas_streaming_eq("Sheet1!$A2", "Sheet1!$A3"));
    }

    #[test]
    fn streaming_eq_cross_sheet_absolute() {
        assert!(!super::formulas_streaming_eq("Sheet1!$A$2", "Sheet1!$A$3"));
    }

    #[test]
    fn streaming_eq_cross_sheet_multi_letter_col() {
        assert!(!super::formulas_streaming_eq("Sheet1!$AB2", "Sheet1!$AB3"));
    }

    #[test]
    fn streaming_eq_named_ranges_with_digits() {
        assert!(!super::formulas_streaming_eq("RATE2+A2", "RATE3+A3"));
    }

    #[test]
    fn streaming_eq_scientific_notation() {
        assert!(!super::formulas_streaming_eq("1E2+A2", "1E3+A3"));
    }

    #[test]
    fn streaming_eq_absolute_range_end() {
        assert!(!super::formulas_streaming_eq("SUM($A$2:$B$5)", "SUM($A$2:$B$6)"));
    }

    // --- cross-sheet and sheet name pattern attack vectors ---

    /// Quoted sheet name with space: cross-sheet ref rows must not be skippable.
    #[test]
    fn streaming_eq_quoted_sheet_with_space() {
        assert!(!super::formulas_streaming_eq("'My Sheet'!A2", "'My Sheet'!A3"));
    }

    /// Quoted sheet with digit and space: cross-sheet ref rows must match.
    #[test]
    fn streaming_eq_quoted_sheet_digit_space() {
        assert!(!super::formulas_streaming_eq("'Sheet 2'!A2", "'Sheet 2'!A3"));
    }

    /// Cross-sheet range: differing range start row must not be equivalent.
    #[test]
    fn streaming_eq_cross_sheet_range_start() {
        assert!(!super::formulas_streaming_eq("Sheet1!A2:A10", "Sheet1!A3:A10"));
    }

    /// Cross-sheet range: differing range end row must not be equivalent.
    #[test]
    fn streaming_eq_cross_sheet_range_end() {
        assert!(!super::formulas_streaming_eq("Sheet1!A2:A10", "Sheet1!A2:A11"));
    }

    /// Cross-sheet absolute range: differing end row must not be equivalent.
    #[test]
    fn streaming_eq_cross_sheet_absolute_range() {
        assert!(!super::formulas_streaming_eq("Sheet1!$A$2:$B$5", "Sheet1!$A$2:$B$6"));
    }

    /// Quoted sheet with dollar-col: cross-sheet ref rows must match.
    #[test]
    fn streaming_eq_quoted_sheet_dollar_col() {
        assert!(!super::formulas_streaming_eq("'Sheet1'!$A2", "'Sheet1'!$A3"));
    }

    /// Two different cross-sheet refs in one formula: rows must match on both.
    #[test]
    fn streaming_eq_two_cross_sheet_refs() {
        assert!(!super::formulas_streaming_eq("Sheet1!A2+Sheet2!A2", "Sheet1!A3+Sheet2!A3"));
    }

    /// Mixed same-sheet and cross-sheet: same-sheet row diff is OK when
    /// cross-sheet ref is identical. The evaluator resolves same-sheet refs
    /// from the current row regardless of the row number in the formula.
    #[test]
    fn streaming_eq_mixed_same_and_cross_sheet() {
        assert!(super::formulas_streaming_eq("A1+Sheet1!A1", "A2+Sheet1!A1"));
    }

    /// The inverse: when cross-sheet ref row differs, must be false.
    #[test]
    fn streaming_eq_mixed_cross_sheet_row_differs() {
        assert!(!super::formulas_streaming_eq("A1+Sheet1!A1", "A2+Sheet1!A2"));
    }

    /// Sheet name ending in digit + `!`: must not confuse digit as cell row.
    #[test]
    fn streaming_eq_sheet_name_ending_in_digit() {
        assert!(!super::formulas_streaming_eq("Sheet10!A2", "Sheet10!A3"));
    }

    // ===== penetration tests: ranges, arrays, structural patterns =====

    /// Attack 1: 2D range where both rows shift — different data rectangle.
    #[test]
    fn streaming_eq_attack_2d_range_both_rows_shift() {
        // SUM(A2:B2) vs SUM(A3:B3) — different row slices, must be false
        assert!(
            !super::formulas_streaming_eq("SUM(A2:B2)", "SUM(A3:B3)"),
            "2D range row shift: different data rectangles must not be equivalent"
        );
    }

    /// Attack 2: 2D range end row differs.
    #[test]
    fn streaming_eq_attack_2d_range_end_differs() {
        assert!(!super::formulas_streaming_eq("SUM(A2:B2)", "SUM(A2:B3)"), "range end row differs");
    }

    /// Attack 3: INDEX function with different row argument.
    #[test]
    fn streaming_eq_attack_index_different_row_arg() {
        assert!(
            !super::formulas_streaming_eq("INDEX(A2:D10,2,3)", "INDEX(A2:D10,3,3)"),
            "INDEX row argument differs — different cell selected"
        );
    }

    /// Attack 4: OFFSET with different anchor row.
    /// A2/A3 are same-sheet cell refs — in streaming context, the row is
    /// resolved from the current row, so this IS correctly equivalent.
    #[test]
    fn streaming_eq_attack_offset_anchor_row() {
        assert!(
            super::formulas_streaming_eq("OFFSET(A2,1,0)", "OFFSET(A3,1,0)"),
            "OFFSET anchor is a same-sheet ref — row resolved from current row"
        );
    }

    /// Attack 5: Array constant differs.
    #[test]
    fn streaming_eq_attack_array_constant_differs() {
        assert!(
            !super::formulas_streaming_eq("{1,2;3,4}+A2", "{1,2;3,5}+A3"),
            "array constant value differs"
        );
    }

    /// Attack 6: INDIRECT with string that looks like a cell ref.
    #[test]
    fn streaming_eq_attack_indirect_string_ref() {
        assert!(
            !super::formulas_streaming_eq("INDIRECT(\"A2\")", "INDIRECT(\"A3\")"),
            "string content differs (even if it looks like a cell ref)"
        );
    }

    /// Attack 7: Single-cell range — range bounds = row number.
    #[test]
    fn streaming_eq_attack_single_cell_range() {
        assert!(
            !super::formulas_streaming_eq("A2:A2", "A3:A3"),
            "A2:A2 vs A3:A3 — different single-cell ranges"
        );
    }

    /// Attack 8: Absolute column single-cell range.
    #[test]
    fn streaming_eq_attack_abs_col_single_cell_range() {
        assert!(
            !super::formulas_streaming_eq("$A2:$A2", "$A3:$A3"),
            "$A2:$A2 vs $A3:$A3 — different single-cell ranges"
        );
    }

    /// Attack 9: Dollar-row in range start.
    #[test]
    fn streaming_eq_attack_dollar_row_range_start() {
        assert!(
            !super::formulas_streaming_eq("SUM(A$2:A$10)", "SUM(A$3:A$10)"),
            "absolute row in range start differs"
        );
    }

    /// Attack 10: Dollar-row in range end.
    #[test]
    fn streaming_eq_attack_dollar_row_range_end() {
        assert!(
            !super::formulas_streaming_eq("SUM(A$2:A$10)", "SUM(A$2:A$11)"),
            "absolute row in range end differs"
        );
    }

    /// Attack 11: Different range size in second SUMPRODUCT arg.
    #[test]
    fn streaming_eq_attack_sumproduct_range_mismatch() {
        assert!(
            !super::formulas_streaming_eq("SUMPRODUCT(A2:A10,B2:B10)", "SUMPRODUCT(A2:A10,B2:B11)"),
            "second SUMPRODUCT range end differs"
        );
    }

    /// Attack 12: Different formula length (extra term).
    #[test]
    fn streaming_eq_attack_different_length() {
        assert!(
            !super::formulas_streaming_eq("A2+B2", "A2+B2+C2"),
            "formulas have different structure (extra term)"
        );
    }

    /// Attack 13: Same column, different row digit count (A2 vs A20).
    #[test]
    fn streaming_eq_attack_row_digit_count() {
        // Both are valid same-sheet refs with different rows — should be true
        // (row numbers are allowed to differ for same-sheet refs)
        assert!(
            super::formulas_streaming_eq("A2", "A20"),
            "A2 vs A20 are same-sheet refs differing only in row — should be equivalent"
        );
    }

    /// Attack 14: Different column width (A2 vs AA2).
    #[test]
    fn streaming_eq_attack_different_column_width() {
        assert!(!super::formulas_streaming_eq("A2", "AA2"), "A2 vs AA2 — different columns");
    }

    /// Attack 15: Empty string vs empty string.
    #[test]
    fn streaming_eq_attack_empty_strings() {
        assert!(super::formulas_streaming_eq("", ""), "empty vs empty should be equal");
    }

    // --- pen-test batch: function names, identifiers, and numbers ---

    /// Attack: IF2(A2) vs IF3(A3) — short function name with trailing digit.
    /// IF is 2 chars (fits 1-3 column pattern), but digit+( means function.
    #[test]
    fn streaming_eq_pentest_short_funcname_digit() {
        assert!(!super::formulas_streaming_eq("IF2(A2)", "IF3(A3)"));
    }

    /// Attack: PI2+A2 vs PI3+A3 — 2-letter identifier ending in digit.
    /// PI is a valid column (col 425), so PI2 IS cell ref (PI, row 2).
    /// Row-only difference → streaming-equivalent.
    #[test]
    fn streaming_eq_pentest_two_letter_col_like_ident() {
        assert!(super::formulas_streaming_eq("PI2+A2", "PI3+A3"));
    }

    /// Attack: A1B2 vs A1B3 — adjacent cell refs without operator.
    /// B is preceded by digit 1, which triggers scientific-notation guard.
    #[test]
    fn streaming_eq_pentest_adjacent_refs_no_operator() {
        assert!(!super::formulas_streaming_eq("A1B2", "A1B3"));
    }

    /// Attack: SUM(A2,B2,3) vs SUM(A3,B3,4) — different literal number arg.
    #[test]
    fn streaming_eq_pentest_different_literal_arg() {
        assert!(!super::formulas_streaming_eq("SUM(A2,B2,3)", "SUM(A3,B3,4)"));
    }

    /// Attack: 1.5E2+A2 vs 1.5E3+A3 — float scientific notation.
    /// E preceded by digit → not a column.
    #[test]
    fn streaming_eq_pentest_float_scientific() {
        assert!(!super::formulas_streaming_eq("1.5E2+A2", "1.5E3+A3"));
    }

    /// Attack: 0.1E10 vs 0.1E20 — pure scientific notation, no cell refs.
    #[test]
    fn streaming_eq_pentest_pure_scientific() {
        assert!(!super::formulas_streaming_eq("0.1E10", "0.1E20"));
    }

    /// Attack: T2+A2 vs T3+A3 — `T()` is a function name, but T2 without
    /// parens IS a cell reference (column T, row 2). Should be equivalent.
    #[test]
    fn streaming_eq_pentest_single_letter_func_t() {
        assert!(super::formulas_streaming_eq("T2+A2", "T3+A3"));
    }

    /// Attack: N2+A2 vs N3+A3 — `N()` is a function, but N2 without parens
    /// IS cell ref (column N, row 2). Should be equivalent.
    #[test]
    fn streaming_eq_pentest_single_letter_func_n() {
        assert!(super::formulas_streaming_eq("N2+A2", "N3+A3"));
    }

    /// Attack: TRUE2 vs TRUE3 — TRUE is 4 chars → identifier, not column.
    #[test]
    fn streaming_eq_pentest_true_followed_by_digit() {
        assert!(!super::formulas_streaming_eq("TRUE2", "TRUE3"));
    }

    /// Attack: MATCH2 vs MATCH3 — 5 chars → identifier, digit must match.
    #[test]
    fn streaming_eq_pentest_long_funcname_digit() {
        assert!(!super::formulas_streaming_eq("MATCH2", "MATCH3"));
    }

    /// Attack: IF(A2>0,1,2) vs IF(A3>0,1,3) — different literal in IF branch.
    #[test]
    fn streaming_eq_pentest_different_if_branch_literal() {
        assert!(!super::formulas_streaming_eq("IF(A2>0,1,2)", "IF(A3>0,1,3)"));
    }

    /// Attack: A2&"test1" vs A3&"test2" — different string literal content.
    /// Also test that same string content IS equivalent.
    #[test]
    fn streaming_eq_pentest_string_literal_content() {
        assert!(super::formulas_streaming_eq("A2&\"test1\"", "A3&\"test1\""));
        assert!(!super::formulas_streaming_eq("A2&\"test1\"", "A3&\"test2\""));
    }
}
