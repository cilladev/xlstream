//! Lookup function builtins: VLOOKUP, XLOOKUP, HLOOKUP, MATCH, XMATCH,
//! INDEX, CHOOSE.

use xlstream_core::{CellError, Value};
use xlstream_parse::{NodeRef, NodeView};

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// `VLOOKUP(lookup_value, table_array, col_index_num, [range_lookup])`
pub(crate) fn builtin_vlookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 3 || args.len() > 4 {
        return Value::Error(CellError::Value);
    }

    let lookup_val = interp.eval(args[0], scope);
    if let Value::Error(e) = lookup_val {
        return Value::Error(e);
    }

    let NodeView::RangeRef { sheet: Some(sheet_name), start_col: Some(range_start_col), .. } =
        args[1].view()
    else {
        return Value::Error(CellError::Value);
    };

    let col_index_val = interp.eval(args[2], scope);
    let col_index = match xlstream_core::coerce::to_number(&col_index_val) {
        Ok(n) => {
            let rounded = n.round();
            if rounded < 1.0 || rounded > f64::from(u32::MAX) {
                return Value::Error(CellError::Value);
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                rounded as u32
            }
        }
        Err(e) => return Value::Error(e),
    };

    let exact_match = if let Some(arg3) = args.get(3) {
        let v = interp.eval(*arg3, scope);
        match xlstream_core::coerce::to_bool(&v) {
            Ok(b) => !b,
            Err(e) => return Value::Error(e),
        }
    } else {
        false
    };

    let Some(sheet) = interp.prelude().lookup_sheet(sheet_name) else {
        return Value::Error(CellError::Na);
    };

    let Some(key) = crate::lookup::LookupValue::from_value(&lookup_val) else {
        return Value::Error(CellError::Na);
    };

    let key_col = range_start_col.saturating_sub(1);
    let value_col =
        range_start_col.saturating_sub(1).saturating_add(col_index).saturating_sub(1) as usize;

    if exact_match {
        match sheet.col_lookup(key_col, &key) {
            Some(row_idx) => {
                sheet.cell(row_idx, value_col).cloned().unwrap_or(Value::Error(CellError::Ref))
            }
            None => Value::Error(CellError::Na),
        }
    } else {
        match sheet.col_approx_lookup(key_col, &key) {
            Some(row_idx) => {
                sheet.cell(row_idx, value_col).cloned().unwrap_or(Value::Error(CellError::Ref))
            }
            None => Value::Error(CellError::Na),
        }
    }
}

/// `XLOOKUP(lookup_value, lookup_array, return_array, [if_not_found],
/// [match_mode], [search_mode])`
pub(crate) fn builtin_xlookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 3 || args.len() > 6 {
        return Value::Error(CellError::Value);
    }

    let lookup_val = interp.eval(args[0], scope);
    if let Value::Error(e) = lookup_val {
        return Value::Error(e);
    }

    let NodeView::RangeRef { sheet: Some(sheet_name), start_col: Some(key_col_1based), .. } =
        args[1].view()
    else {
        return Value::Error(CellError::Value);
    };

    let NodeView::RangeRef { start_col: Some(value_col_1based), .. } = args[2].view() else {
        return Value::Error(CellError::Value);
    };

    let Some(sheet) = interp.prelude().lookup_sheet(sheet_name) else {
        return Value::Error(CellError::Na);
    };

    let Some(key) = crate::lookup::LookupValue::from_value(&lookup_val) else {
        return Value::Error(CellError::Na);
    };

    let key_col = key_col_1based.saturating_sub(1);
    let value_col = (value_col_1based - 1) as usize;

    match sheet.col_lookup(key_col, &key) {
        Some(row_idx) => {
            sheet.cell(row_idx, value_col).cloned().unwrap_or(Value::Error(CellError::Ref))
        }
        None => {
            if let Some(fallback_node) = args.get(3) {
                interp.eval(*fallback_node, scope)
            } else {
                Value::Error(CellError::Na)
            }
        }
    }
}

/// `HLOOKUP(lookup_value, table_array, row_index_num, [range_lookup])`
pub(crate) fn builtin_hlookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 3 || args.len() > 4 {
        return Value::Error(CellError::Value);
    }

    let lookup_val = interp.eval(args[0], scope);
    if let Value::Error(e) = lookup_val {
        return Value::Error(e);
    }

    let NodeView::RangeRef { sheet: Some(sheet_name), .. } = args[1].view() else {
        return Value::Error(CellError::Value);
    };

    let row_index_val = interp.eval(args[2], scope);
    let row_index = match xlstream_core::coerce::to_number(&row_index_val) {
        Ok(n) => {
            let rounded = n.round();
            if rounded < 1.0 || rounded > f64::from(u32::MAX) {
                return Value::Error(CellError::Value);
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                rounded as usize
            }
        }
        Err(e) => return Value::Error(e),
    };

    let exact_match = if let Some(arg3) = args.get(3) {
        let v = interp.eval(*arg3, scope);
        match xlstream_core::coerce::to_bool(&v) {
            Ok(b) => !b,
            Err(e) => return Value::Error(e),
        }
    } else {
        false
    };

    let Some(sheet) = interp.prelude().lookup_sheet(sheet_name) else {
        return Value::Error(CellError::Na);
    };

    let Some(key) = crate::lookup::LookupValue::from_value(&lookup_val) else {
        return Value::Error(CellError::Na);
    };

    let col_idx =
        if exact_match { sheet.row_lookup(0, &key) } else { sheet.row_approx_lookup(0, &key) };

    let Some(col_idx) = col_idx else {
        return Value::Error(CellError::Na);
    };

    let target_row = row_index - 1;
    sheet.cell(target_row, col_idx).cloned().unwrap_or(Value::Error(CellError::Ref))
}

/// `MATCH(lookup_value, lookup_array, [match_type])`
pub(crate) fn builtin_match(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() || args.len() > 3 {
        return Value::Error(CellError::Value);
    }

    let lookup_val = interp.eval(args[0], scope);
    if let Value::Error(e) = lookup_val {
        return Value::Error(e);
    }

    let NodeView::RangeRef { sheet: Some(sheet_name), start_col: Some(key_col_1based), .. } =
        args[1].view()
    else {
        return Value::Error(CellError::Value);
    };

    let match_type = if let Some(arg2) = args.get(2) {
        let v = interp.eval(*arg2, scope);
        match xlstream_core::coerce::to_number(&v) {
            Ok(n) => {
                #[allow(clippy::cast_possible_truncation)]
                {
                    n.round() as i32
                }
            }
            Err(e) => return Value::Error(e),
        }
    } else {
        1
    };

    if match_type != 0 {
        return Value::Error(CellError::Na);
    }

    let Some(sheet) = interp.prelude().lookup_sheet(sheet_name) else {
        return Value::Error(CellError::Na);
    };

    let Some(key) = crate::lookup::LookupValue::from_value(&lookup_val) else {
        return Value::Error(CellError::Na);
    };

    let key_col = key_col_1based.saturating_sub(1);
    match sheet.col_lookup(key_col, &key) {
        Some(row_idx) => {
            #[allow(clippy::cast_precision_loss)]
            let pos = (row_idx + 1) as f64;
            Value::Number(pos)
        }
        None => Value::Error(CellError::Na),
    }
}

/// `XMATCH(lookup_value, lookup_array, [match_mode], [search_mode])`
pub(crate) fn builtin_xmatch(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 4 {
        return Value::Error(CellError::Value);
    }

    let lookup_val = interp.eval(args[0], scope);
    if let Value::Error(e) = lookup_val {
        return Value::Error(e);
    }

    let NodeView::RangeRef { sheet: Some(sheet_name), start_col: Some(key_col_1based), .. } =
        args[1].view()
    else {
        return Value::Error(CellError::Value);
    };

    if let Some(arg2) = args.get(2) {
        let v = interp.eval(*arg2, scope);
        match xlstream_core::coerce::to_number(&v) {
            Ok(n) =>
            {
                #[allow(clippy::cast_possible_truncation)]
                if n.round() as i32 != 0 {
                    return Value::Error(CellError::Na);
                }
            }
            Err(e) => return Value::Error(e),
        }
    }

    let Some(sheet) = interp.prelude().lookup_sheet(sheet_name) else {
        return Value::Error(CellError::Na);
    };

    let Some(key) = crate::lookup::LookupValue::from_value(&lookup_val) else {
        return Value::Error(CellError::Na);
    };

    let key_col = key_col_1based.saturating_sub(1);
    match sheet.col_lookup(key_col, &key) {
        Some(row_idx) => {
            #[allow(clippy::cast_precision_loss)]
            let pos = (row_idx + 1) as f64;
            Value::Number(pos)
        }
        None => Value::Error(CellError::Na),
    }
}

/// `INDEX(array, row_num, [col_num])`
pub(crate) fn builtin_index(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }

    let NodeView::RangeRef { sheet: Some(sheet_name), start_col: Some(range_start_col), .. } =
        args[0].view()
    else {
        return Value::Error(CellError::Value);
    };

    let row_val = interp.eval(args[1], scope);
    let row_num = match xlstream_core::coerce::to_number(&row_val) {
        Ok(n) => {
            let rounded = n.round();
            if rounded < 1.0 {
                return Value::Error(CellError::Value);
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                rounded as usize
            }
        }
        Err(e) => return Value::Error(e),
    };

    let col_num = if let Some(arg2) = args.get(2) {
        let v = interp.eval(*arg2, scope);
        match xlstream_core::coerce::to_number(&v) {
            Ok(n) => {
                let rounded = n.round();
                if rounded < 1.0 {
                    return Value::Error(CellError::Value);
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                {
                    rounded as usize
                }
            }
            Err(e) => return Value::Error(e),
        }
    } else {
        1
    };

    let Some(sheet) = interp.prelude().lookup_sheet(sheet_name) else {
        return Value::Error(CellError::Ref);
    };

    let abs_col = (range_start_col - 1) as usize + col_num - 1;
    sheet.cell(row_num - 1, abs_col).cloned().unwrap_or(Value::Error(CellError::Ref))
}

/// `CHOOSE(index_num, value1, [value2], ...)`
pub(crate) fn builtin_choose(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 {
        return Value::Error(CellError::Value);
    }

    let index_val = interp.eval(args[0], scope);
    let index = match xlstream_core::coerce::to_number(&index_val) {
        Ok(n) => {
            let rounded = n.round();
            #[allow(clippy::cast_precision_loss)]
            let max_index = (args.len() - 1) as f64;
            if rounded < 1.0 || rounded > max_index {
                return Value::Error(CellError::Value);
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                rounded as usize
            }
        }
        Err(e) => return Value::Error(e),
    };

    interp.eval(args[index], scope)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use std::collections::HashMap;

    use xlstream_core::{CellError, Value};
    use xlstream_parse::parse;

    use crate::interp::Interpreter;
    use crate::lookup::LookupSheet;
    use crate::prelude::Prelude;
    use crate::scope::RowScope;

    fn prelude_with_region_lookup() -> Prelude {
        let rows = vec![
            vec![Value::Text("EMEA".into()), Value::Text("Europe".into()), Value::Number(1.0)],
            vec![Value::Text("APAC".into()), Value::Text("Asia".into()), Value::Number(2.0)],
            vec![Value::Text("AMER".into()), Value::Text("Americas".into()), Value::Number(3.0)],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_index(0);
        sheet.build_col_sorted(0);
        let mut sheets = HashMap::new();
        sheets.insert("region info".to_string(), sheet);
        Prelude::empty().with_lookup_sheets(sheets)
    }

    // --- VLOOKUP ---

    #[test]
    fn vlookup_exact_hit() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(\"APAC\", 'Region Info'!A:C, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("Asia".into()));
    }

    #[test]
    fn vlookup_exact_miss_returns_na() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(\"LATAM\", 'Region Info'!A:C, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }

    #[test]
    fn vlookup_case_insensitive_text() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(\"emea\", 'Region Info'!A:C, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("Europe".into()));
    }

    #[test]
    fn vlookup_key_from_current_row() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(A1, 'Region Info'!A:C, 2, FALSE)").unwrap();
        let row = vec![Value::Text("AMER".into())];
        let scope = RowScope::new(&row, 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("Americas".into()));
    }

    #[test]
    fn vlookup_error_key_propagates() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(#VALUE!, 'Region Info'!A:C, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Value));
    }

    #[test]
    fn vlookup_approx_match_hit() {
        let rows = vec![
            vec![Value::Number(10.0), Value::Text("ten".into())],
            vec![Value::Number(20.0), Value::Text("twenty".into())],
            vec![Value::Number(30.0), Value::Text("thirty".into())],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_index(0);
        sheet.build_col_sorted(0);
        let mut sheets = HashMap::new();
        sheets.insert("data".to_string(), sheet);
        let prelude = Prelude::empty().with_lookup_sheets(sheets);
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(25, 'Data'!A:B, 2, TRUE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("twenty".into()));
    }

    #[test]
    fn vlookup_approx_below_first_returns_na() {
        let rows = vec![
            vec![Value::Number(10.0), Value::Text("ten".into())],
            vec![Value::Number(20.0), Value::Text("twenty".into())],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_index(0);
        sheet.build_col_sorted(0);
        let mut sheets = HashMap::new();
        sheets.insert("data".to_string(), sheet);
        let prelude = Prelude::empty().with_lookup_sheets(sheets);
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(5, 'Data'!A:B, 2, TRUE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }

    #[test]
    fn vlookup_number_key_not_equal_to_text() {
        let rows = vec![
            vec![Value::Text("1".into()), Value::Text("text-one".into())],
            vec![Value::Number(1.0), Value::Text("num-one".into())],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_col_index(0);
        let mut sheets = HashMap::new();
        sheets.insert("data".to_string(), sheet);
        let prelude = Prelude::empty().with_lookup_sheets(sheets);
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(1, 'Data'!A:B, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("num-one".into()));
    }

    // --- XLOOKUP ---

    #[test]
    fn xlookup_exact_hit() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("XLOOKUP(\"APAC\", 'Region Info'!A:A, 'Region Info'!B:B)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("Asia".into()));
    }

    #[test]
    fn xlookup_miss_default_na() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("XLOOKUP(\"LATAM\", 'Region Info'!A:A, 'Region Info'!B:B)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }

    #[test]
    fn xlookup_with_not_found_fallback() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast =
            parse("XLOOKUP(\"LATAM\", 'Region Info'!A:A, 'Region Info'!B:B, \"Unknown\")").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("Unknown".into()));
    }

    // --- HLOOKUP ---

    fn prelude_with_hlookup_data() -> Prelude {
        let rows = vec![
            vec![Value::Text("Name".into()), Value::Text("Age".into()), Value::Text("City".into())],
            vec![Value::Text("Alice".into()), Value::Number(30.0), Value::Text("NYC".into())],
            vec![Value::Text("Bob".into()), Value::Number(25.0), Value::Text("LA".into())],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_row_index(0);
        let mut sheets = HashMap::new();
        sheets.insert("people".to_string(), sheet);
        Prelude::empty().with_lookup_sheets(sheets)
    }

    #[test]
    fn hlookup_exact_hit() {
        let prelude = prelude_with_hlookup_data();
        let interp = Interpreter::new(&prelude);
        let ast = parse("HLOOKUP(\"Age\", 'People'!A:C, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(30.0));
    }

    #[test]
    fn hlookup_exact_miss() {
        let prelude = prelude_with_hlookup_data();
        let interp = Interpreter::new(&prelude);
        let ast = parse("HLOOKUP(\"Email\", 'People'!A:C, 2, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }

    #[test]
    fn hlookup_case_insensitive() {
        let prelude = prelude_with_hlookup_data();
        let interp = Interpreter::new(&prelude);
        let ast = parse("HLOOKUP(\"city\", 'People'!A:C, 3, FALSE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("LA".into()));
    }

    fn prelude_with_hlookup_sorted_data() -> Prelude {
        let rows = vec![
            vec![Value::Number(10.0), Value::Number(20.0), Value::Number(30.0)],
            vec![
                Value::Text("ten".into()),
                Value::Text("twenty".into()),
                Value::Text("thirty".into()),
            ],
            vec![Value::Number(100.0), Value::Number(200.0), Value::Number(300.0)],
        ];
        let mut sheet = LookupSheet::new(rows);
        sheet.build_row_index(0);
        sheet.build_row_sorted(0);
        let mut sheets = HashMap::new();
        sheets.insert("rates".to_string(), sheet);
        Prelude::empty().with_lookup_sheets(sheets)
    }

    #[test]
    fn hlookup_approx_match_hit() {
        let prelude = prelude_with_hlookup_sorted_data();
        let interp = Interpreter::new(&prelude);
        let ast = parse("HLOOKUP(25, 'Rates'!A:C, 2, TRUE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("twenty".into()));
    }

    #[test]
    fn hlookup_approx_match_exact_boundary() {
        let prelude = prelude_with_hlookup_sorted_data();
        let interp = Interpreter::new(&prelude);
        let ast = parse("HLOOKUP(30, 'Rates'!A:C, 3, TRUE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(300.0));
    }

    #[test]
    fn hlookup_approx_below_first_returns_na() {
        let prelude = prelude_with_hlookup_sorted_data();
        let interp = Interpreter::new(&prelude);
        let ast = parse("HLOOKUP(5, 'Rates'!A:C, 2, TRUE)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }

    // --- MATCH ---

    #[test]
    fn match_exact_hit() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("MATCH(\"APAC\", 'Region Info'!A:A, 0)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(2.0));
    }

    #[test]
    fn match_exact_miss() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("MATCH(\"LATAM\", 'Region Info'!A:A, 0)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }

    #[test]
    fn match_case_insensitive() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("MATCH(\"amer\", 'Region Info'!A:A, 0)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(3.0));
    }

    // --- INDEX ---

    #[test]
    fn index_returns_cell_value() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("INDEX('Region Info'!A:C, 2, 2)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("Asia".into()));
    }

    #[test]
    fn index_out_of_bounds() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("INDEX('Region Info'!A:C, 100, 1)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Ref));
    }

    #[test]
    fn index_single_column_omit_col() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("INDEX('Region Info'!A:A, 3)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("AMER".into()));
    }

    // --- CHOOSE ---

    #[test]
    fn choose_picks_correct_value() {
        let prelude = Prelude::empty();
        let interp = Interpreter::new(&prelude);
        let ast = parse("CHOOSE(2, \"a\", \"b\", \"c\")").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("b".into()));
    }

    #[test]
    fn choose_index_out_of_range() {
        let prelude = Prelude::empty();
        let interp = Interpreter::new(&prelude);
        let ast = parse("CHOOSE(5, \"a\", \"b\")").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Value));
    }

    #[test]
    fn choose_index_from_cell() {
        let prelude = Prelude::empty();
        let interp = Interpreter::new(&prelude);
        let ast = parse("CHOOSE(A1, 10, 20, 30)").unwrap();
        let row = vec![Value::Number(3.0)];
        let scope = RowScope::new(&row, 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(30.0));
    }

    // --- XMATCH ---

    #[test]
    fn xmatch_exact_hit() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("XMATCH(\"AMER\", 'Region Info'!A:A)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(3.0));
    }

    #[test]
    fn xmatch_exact_miss() {
        let prelude = prelude_with_region_lookup();
        let interp = Interpreter::new(&prelude);
        let ast = parse("XMATCH(\"NONE\", 'Region Info'!A:A)").unwrap();
        let scope = RowScope::new(&[], 1);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Na));
    }
}
