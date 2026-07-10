//! Multi-criteria conditional aggregate builtins (SUMIFS, COUNTIFS,
//! AVERAGEIFS, MINIFS, MAXIFS).
//!
//! These functions look up pre-computed results from the prelude's
//! multi-conditional aggregate tables. Each criteria argument is
//! evaluated from the current row to build a composite key.

use xlstream_core::{coerce, CellError, Value};
use xlstream_parse::{AggKind, NodeRef, NodeView};

use crate::interp::Interpreter;
use crate::prelude::MultiConditionalAggKey;
use crate::prelude_plan::{if_row_bounds, ifs_row_bounds};
use crate::scope::RowScope;

/// Build a composite key from criteria values: lowercase each, join with
/// `\0`.
fn build_composite_key(criteria_values: &[String]) -> String {
    criteria_values.join("\x00")
}

/// Extract a static text literal from a node. Returns `None` if the node
/// is not a text or number literal.
fn extract_static_criteria(
    node: NodeRef<'_>,
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> String {
    let val = interp.eval(node, scope);
    let text = coerce::to_text(&val).to_ascii_lowercase();
    text.strip_prefix('=').unwrap_or(&text).to_string()
}

/// Extract column index and optional sheet from a range reference argument.
fn extract_criteria_col_and_sheet(
    interp: &Interpreter<'_>,
    node: NodeRef<'_>,
) -> Option<(u32, Option<String>)> {
    match node.view() {
        NodeView::RangeRef { sheet, start_col: Some(sc), end_col: Some(ec), .. } if sc == ec => {
            let resolved = sheet
                .map(ToString::to_string)
                .or_else(|| interp.current_sheet().map(ToString::to_string));
            Some((sc, resolved))
        }
        _ => None,
    }
}

/// Shared lookup for value-range *IFS builtins (SUMIFS, AVERAGEIFS,
/// MINIFS, MAXIFS): `FN(value_range, crit_range1, crit1, ...)`.
///
/// All ranges must share identical row bounds — Excel's congruent-shape
/// rule, which Excel itself enforces with `#VALUE!`. Key construction
/// must mirror `extract_value_ifs_key` in `prelude_plan` exactly, or the
/// prelude lookup silently misses.
fn value_ifs_lookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
    kind: AggKind,
) -> Value {
    // Minimum: value_range + at least one (criteria_range, criteria) pair = 3
    // args. After first arg, remaining must be pairs.
    if args.len() < 3 || !(args.len() - 1).is_multiple_of(2) {
        return Value::Error(CellError::Value);
    }

    let Some((sum_col, sheet)) = extract_criteria_col_and_sheet(interp, args[0]) else {
        return Value::Error(CellError::Value);
    };

    let num_pairs = (args.len() - 1) / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    let mut criteria_values = Vec::with_capacity(num_pairs);

    for i in 0..num_pairs {
        let range_idx = 1 + i * 2;
        let crit_idx = 2 + i * 2;

        let Some((col, _)) = extract_criteria_col_and_sheet(interp, args[range_idx]) else {
            return Value::Error(CellError::Value);
        };
        criteria_cols.push(col);

        let val = extract_static_criteria(args[crit_idx], interp, scope);
        criteria_values.push(val);
    }

    let Some((start_row, end_row)) =
        ifs_row_bounds(Some(args[0]), (0..num_pairs).map(|i| args[1 + i * 2]))
    else {
        return Value::Error(CellError::Value);
    };

    let key = MultiConditionalAggKey { kind, sum_col, criteria_cols, sheet, start_row, end_row };
    let composite = build_composite_key(&criteria_values);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `SUMIFS(sum_range, criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional sum from the prelude.
/// Returns `#VALUE!` if arguments are malformed or the ranges do not
/// share identical row bounds.
pub(crate) fn builtin_sumifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    value_ifs_lookup(args, interp, scope, AggKind::Sum)
}

/// `COUNTIFS(criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional count from the prelude.
/// Returns `#VALUE!` if arguments are malformed.
pub(crate) fn builtin_countifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    // Must have pairs: at least 2 args, even count.
    if args.len() < 2 || !args.len().is_multiple_of(2) {
        return Value::Error(CellError::Value);
    }

    let num_pairs = args.len() / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    let mut criteria_values = Vec::with_capacity(num_pairs);
    let mut sheet: Option<String> = None;

    for i in 0..num_pairs {
        let range_idx = i * 2;
        let crit_idx = i * 2 + 1;

        let Some((col, s)) = extract_criteria_col_and_sheet(interp, args[range_idx]) else {
            return Value::Error(CellError::Value);
        };
        criteria_cols.push(col);
        if i == 0 {
            sheet = s;
        }

        let val = extract_static_criteria(args[crit_idx], interp, scope);
        criteria_values.push(val);
    }

    let Some((start_row, end_row)) = ifs_row_bounds(None, (0..num_pairs).map(|i| args[i * 2]))
    else {
        return Value::Error(CellError::Value);
    };

    let key = MultiConditionalAggKey {
        kind: AggKind::Count,
        sum_col: 0,
        criteria_cols,
        sheet,
        start_row,
        end_row,
    };

    let composite = build_composite_key(&criteria_values);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `AVERAGEIFS(avg_range, criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional average from the prelude.
/// Returns `#VALUE!` if arguments are malformed or the ranges do not
/// share identical row bounds.
pub(crate) fn builtin_averageifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    value_ifs_lookup(args, interp, scope, AggKind::Average)
}

/// `MINIFS(min_range, criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional min from the prelude.
/// Returns `#VALUE!` if arguments are malformed or the ranges do not
/// share identical row bounds.
pub(crate) fn builtin_minifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    value_ifs_lookup(args, interp, scope, AggKind::Min)
}

/// `MAXIFS(max_range, criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional max from the prelude.
/// Returns `#VALUE!` if arguments are malformed or the ranges do not
/// share identical row bounds.
pub(crate) fn builtin_maxifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    value_ifs_lookup(args, interp, scope, AggKind::Max)
}

/// Shared lookup for single-criteria conditional builtins (SUMIF,
/// AVERAGEIF): `FN(criteria_range, criteria, [value_range])`.
///
/// The criteria range drives the row bounds; a value range must start on
/// the same row or the formula returns `#VALUE!` (Excel would resize an
/// offset value range, which same-row streaming cannot express). Key
/// construction must mirror `extract_if_key` in `prelude_plan` exactly,
/// or the prelude lookup silently misses.
fn if_lookup(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
    kind: AggKind,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let Some((criteria_col, sheet)) = extract_criteria_col_and_sheet(interp, args[0]) else {
        return Value::Error(CellError::Value);
    };
    let sum_col = if args.len() >= 3 {
        let Some((sc, _)) = extract_criteria_col_and_sheet(interp, args[2]) else {
            return Value::Error(CellError::Value);
        };
        sc
    } else {
        criteria_col
    };
    let Some((start_row, end_row)) = if_row_bounds(args[0], args.get(2).copied()) else {
        return Value::Error(CellError::Value);
    };
    let criteria_val = extract_static_criteria(args[1], interp, scope);
    let key = MultiConditionalAggKey {
        kind,
        sum_col,
        criteria_cols: vec![criteria_col],
        sheet,
        start_row,
        end_row,
    };
    let composite = build_composite_key(&[criteria_val]);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `SUMIF(criteria_range, criteria, [sum_range])`
///
/// Looks up the pre-computed conditional sum from the prelude. Returns
/// `#VALUE!` if arguments are malformed or the sum range does not start
/// on the criteria range's first row.
pub(crate) fn builtin_sumif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if_lookup(args, interp, scope, AggKind::Sum)
}

/// `COUNTIF(criteria_range, criteria)`
///
/// Looks up the pre-computed conditional count from the prelude. Returns
/// `#VALUE!` if arguments are malformed.
pub(crate) fn builtin_countif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let Some((criteria_col, sheet)) = extract_criteria_col_and_sheet(interp, args[0]) else {
        return Value::Error(CellError::Value);
    };
    let Some((start_row, end_row)) = if_row_bounds(args[0], None) else {
        return Value::Error(CellError::Value);
    };
    let criteria_val = extract_static_criteria(args[1], interp, scope);
    let key = MultiConditionalAggKey {
        kind: AggKind::Count,
        sum_col: 0,
        criteria_cols: vec![criteria_col],
        sheet,
        start_row,
        end_row,
    };
    let composite = build_composite_key(&[criteria_val]);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `AVERAGEIF(criteria_range, criteria, [avg_range])`
///
/// Looks up the pre-computed conditional average from the prelude.
/// Returns `#VALUE!` if arguments are malformed or the average range does
/// not start on the criteria range's first row.
pub(crate) fn builtin_averageif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if_lookup(args, interp, scope, AggKind::Average)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use std::collections::HashMap;

    use xlstream_core::{CellError, Value};
    use xlstream_parse::AggKind;

    use crate::prelude::MultiConditionalAggKey;
    use crate::Prelude;

    /// Build a prelude with a single multi-conditional entry.
    fn prelude_with_multi(key: MultiConditionalAggKey, entries: Vec<(&str, Value)>) -> Prelude {
        let mut inner = HashMap::new();
        for (k, v) in entries {
            inner.insert(k.to_string(), v);
        }
        let mut multi = HashMap::new();
        multi.insert(key, inner);
        Prelude::with_all(HashMap::new(), HashMap::new(), multi)
    }

    // ===== SUMIFS =====

    #[test]
    fn sumifs_basic_two_criteria() {
        let composite = "east\x00q1";
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![(composite, Value::Number(100.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            composite,
        );
        assert_eq!(result, Value::Number(100.0));
    }

    #[test]
    fn sumifs_missing_criteria_returns_zero() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(100.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "west\0q1",
        );
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn sumifs_case_insensitive_key() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 3,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        // Key is stored lowercased
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(50.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 3,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east",
        );
        assert_eq!(result, Value::Number(50.0));
    }

    #[test]
    fn sumifs_three_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 4,
            criteria_cols: vec![1, 2, 3],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\x00q1\x002024", Value::Number(75.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 4,
                criteria_cols: vec![1, 2, 3],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east\x00q1\x002024",
        );
        assert_eq!(result, Value::Number(75.0));
    }

    #[test]
    fn sumifs_empty_table_returns_zero() {
        let prelude = Prelude::empty();
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        assert_eq!(prelude.get_multi_conditional(&key, "a\0b"), Value::Number(0.0));
    }

    // ===== COUNTIFS =====

    #[test]
    fn countifs_basic_two_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Count,
            sum_col: 0,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(5.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east\0q1",
        );
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn countifs_missing_returns_zero() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Count,
            sum_col: 0,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(3.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "west",
        );
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn countifs_single_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Count,
            sum_col: 0,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("north", Value::Number(7.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "north",
        );
        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn countifs_three_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Count,
            sum_col: 0,
            criteria_cols: vec![1, 2, 3],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("a\0b\0c", Value::Number(2.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1, 2, 3],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "a\0b\0c",
        );
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn countifs_empty_prelude() {
        let prelude = Prelude::empty();
        let key = MultiConditionalAggKey {
            kind: AggKind::Count,
            sum_col: 0,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        assert_eq!(prelude.get_multi_conditional(&key, "x"), Value::Number(0.0));
    }

    // ===== AVERAGEIFS =====

    #[test]
    fn averageifs_basic_two_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Average,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(25.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east\0q1",
        );
        assert_eq!(result, Value::Number(25.0));
    }

    #[test]
    fn averageifs_missing_returns_div0() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Average,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(25.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "west\0q1",
        );
        assert_eq!(result, Value::Error(CellError::Div0));
    }

    #[test]
    fn averageifs_single_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Average,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(15.5))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 2,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east",
        );
        assert_eq!(result, Value::Number(15.5));
    }

    #[test]
    fn averageifs_three_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Average,
            sum_col: 4,
            criteria_cols: vec![1, 2, 3],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("x\0y\0z", Value::Number(42.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 4,
                criteria_cols: vec![1, 2, 3],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "x\0y\0z",
        );
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn averageifs_empty_prelude_returns_div0() {
        let prelude = Prelude::empty();
        let key = MultiConditionalAggKey {
            kind: AggKind::Average,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        assert_eq!(prelude.get_multi_conditional(&key, "a\0b"), Value::Error(CellError::Div0));
    }

    // ===== MINIFS =====

    #[test]
    fn minifs_basic_one_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Min,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(10.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Min,
                sum_col: 2,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east",
        );
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn minifs_missing_returns_zero() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Min,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(10.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Min,
                sum_col: 2,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "west",
        );
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn minifs_two_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Min,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\x00q1", Value::Number(5.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Min,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east\x00q1",
        );
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn minifs_empty_prelude_returns_zero() {
        let prelude = Prelude::empty();
        let key = MultiConditionalAggKey {
            kind: AggKind::Min,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        assert_eq!(prelude.get_multi_conditional(&key, "x"), Value::Number(0.0));
    }

    // ===== MAXIFS =====

    #[test]
    fn maxifs_basic_one_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Max,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(99.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Max,
                sum_col: 2,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east",
        );
        assert_eq!(result, Value::Number(99.0));
    }

    #[test]
    fn maxifs_missing_returns_zero() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Max,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(99.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Max,
                sum_col: 2,
                criteria_cols: vec![1],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "west",
        );
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn maxifs_two_criteria() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Max,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        let prelude = prelude_with_multi(key, vec![("east\x00q1", Value::Number(200.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Max,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
                start_row: None,
                end_row: None,
            },
            "east\x00q1",
        );
        assert_eq!(result, Value::Number(200.0));
    }

    #[test]
    fn maxifs_empty_prelude_returns_zero() {
        let prelude = Prelude::empty();
        let key = MultiConditionalAggKey {
            kind: AggKind::Max,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        };
        assert_eq!(prelude.get_multi_conditional(&key, "x"), Value::Number(0.0));
    }
}
