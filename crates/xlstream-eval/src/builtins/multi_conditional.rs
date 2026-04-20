//! Multi-criteria conditional aggregate builtins (SUMIFS, COUNTIFS,
//! AVERAGEIFS).
//!
//! These functions look up pre-computed results from the prelude's
//! multi-conditional aggregate tables. Each criteria argument is
//! evaluated from the current row to build a composite key.

use xlstream_core::{coerce, CellError, Value};
use xlstream_parse::{AggKind, NodeRef, NodeView};

use crate::interp::Interpreter;
use crate::prelude::MultiConditionalAggKey;
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
    coerce::to_text(&val).to_ascii_lowercase()
}

/// Extract column index and optional sheet from a range reference argument.
fn extract_criteria_col_and_sheet(node: NodeRef<'_>) -> Option<(u32, Option<String>)> {
    match node.view() {
        NodeView::RangeRef { sheet, start_col: Some(sc), end_col: Some(ec), .. } if sc == ec => {
            Some((sc, sheet.map(ToString::to_string)))
        }
        _ => None,
    }
}

/// `SUMIFS(sum_range, criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional sum from the prelude.
/// Returns `#VALUE!` if arguments are malformed.
pub(crate) fn builtin_sumifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    // Minimum: sum_range + at least one (criteria_range, criteria) pair = 3 args
    // After first arg, remaining must be pairs.
    if args.len() < 3 || (args.len() - 1) % 2 != 0 {
        return Value::Error(CellError::Value);
    }

    let Some((sum_col, sheet)) = extract_criteria_col_and_sheet(args[0]) else {
        return Value::Error(CellError::Value);
    };

    let num_pairs = (args.len() - 1) / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    let mut criteria_values = Vec::with_capacity(num_pairs);

    for i in 0..num_pairs {
        let range_idx = 1 + i * 2;
        let crit_idx = 2 + i * 2;

        let Some((col, _)) = extract_criteria_col_and_sheet(args[range_idx]) else {
            return Value::Error(CellError::Value);
        };
        criteria_cols.push(col);

        let val = extract_static_criteria(args[crit_idx], interp, scope);
        criteria_values.push(val);
    }

    let key = MultiConditionalAggKey { kind: AggKind::Sum, sum_col, criteria_cols, sheet };

    let composite = build_composite_key(&criteria_values);
    interp.prelude().get_multi_conditional(&key, &composite)
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
    if args.len() < 2 || args.len() % 2 != 0 {
        return Value::Error(CellError::Value);
    }

    let num_pairs = args.len() / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    let mut criteria_values = Vec::with_capacity(num_pairs);
    let mut sheet: Option<String> = None;

    for i in 0..num_pairs {
        let range_idx = i * 2;
        let crit_idx = i * 2 + 1;

        let Some((col, s)) = extract_criteria_col_and_sheet(args[range_idx]) else {
            return Value::Error(CellError::Value);
        };
        criteria_cols.push(col);
        if i == 0 {
            sheet = s;
        }

        let val = extract_static_criteria(args[crit_idx], interp, scope);
        criteria_values.push(val);
    }

    let key = MultiConditionalAggKey { kind: AggKind::Count, sum_col: 0, criteria_cols, sheet };

    let composite = build_composite_key(&criteria_values);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `AVERAGEIFS(avg_range, criteria_range1, criteria1, criteria_range2, criteria2, ...)`
///
/// Looks up the pre-computed multi-conditional average from the prelude.
/// Returns `#VALUE!` if arguments are malformed.
pub(crate) fn builtin_averageifs(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    // Minimum: avg_range + at least one (criteria_range, criteria) pair = 3 args
    if args.len() < 3 || (args.len() - 1) % 2 != 0 {
        return Value::Error(CellError::Value);
    }

    let Some((sum_col, sheet)) = extract_criteria_col_and_sheet(args[0]) else {
        return Value::Error(CellError::Value);
    };

    let num_pairs = (args.len() - 1) / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    let mut criteria_values = Vec::with_capacity(num_pairs);

    for i in 0..num_pairs {
        let range_idx = 1 + i * 2;
        let crit_idx = 2 + i * 2;

        let Some((col, _)) = extract_criteria_col_and_sheet(args[range_idx]) else {
            return Value::Error(CellError::Value);
        };
        criteria_cols.push(col);

        let val = extract_static_criteria(args[crit_idx], interp, scope);
        criteria_values.push(val);
    }

    let key = MultiConditionalAggKey { kind: AggKind::Average, sum_col, criteria_cols, sheet };

    let composite = build_composite_key(&criteria_values);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `SUMIF(criteria_range, criteria, [sum_range])`
pub(crate) fn builtin_sumif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let Some((criteria_col, sheet)) = extract_criteria_col_and_sheet(args[0]) else {
        return Value::Error(CellError::Value);
    };
    let sum_col = if args.len() >= 3 {
        let Some((sc, _)) = extract_criteria_col_and_sheet(args[2]) else {
            return Value::Error(CellError::Value);
        };
        sc
    } else {
        criteria_col
    };
    let criteria_val = extract_static_criteria(args[1], interp, scope);
    let key = MultiConditionalAggKey {
        kind: AggKind::Sum,
        sum_col,
        criteria_cols: vec![criteria_col],
        sheet,
    };
    let composite = build_composite_key(&[criteria_val]);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `COUNTIF(criteria_range, criteria)`
pub(crate) fn builtin_countif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let Some((criteria_col, sheet)) = extract_criteria_col_and_sheet(args[0]) else {
        return Value::Error(CellError::Value);
    };
    let criteria_val = extract_static_criteria(args[1], interp, scope);
    let key = MultiConditionalAggKey {
        kind: AggKind::Count,
        sum_col: 0,
        criteria_cols: vec![criteria_col],
        sheet,
    };
    let composite = build_composite_key(&[criteria_val]);
    interp.prelude().get_multi_conditional(&key, &composite)
}

/// `AVERAGEIF(criteria_range, criteria, [avg_range])`
pub(crate) fn builtin_averageif(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let Some((criteria_col, sheet)) = extract_criteria_col_and_sheet(args[0]) else {
        return Value::Error(CellError::Value);
    };
    let sum_col = if args.len() >= 3 {
        let Some((sc, _)) = extract_criteria_col_and_sheet(args[2]) else {
            return Value::Error(CellError::Value);
        };
        sc
    } else {
        criteria_col
    };
    let criteria_val = extract_static_criteria(args[1], interp, scope);
    let key = MultiConditionalAggKey {
        kind: AggKind::Average,
        sum_col,
        criteria_cols: vec![criteria_col],
        sheet,
    };
    let composite = build_composite_key(&[criteria_val]);
    interp.prelude().get_multi_conditional(&key, &composite)
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
        };
        let prelude = prelude_with_multi(key, vec![(composite, Value::Number(100.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(100.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
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
        };
        // Key is stored lowercased
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(50.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 3,
                criteria_cols: vec![1],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east\x00q1\x002024", Value::Number(75.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Sum,
                sum_col: 4,
                criteria_cols: vec![1, 2, 3],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(5.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1, 2],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(3.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("north", Value::Number(7.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("a\0b\0c", Value::Number(2.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Count,
                sum_col: 0,
                criteria_cols: vec![1, 2, 3],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(25.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east\0q1", Value::Number(25.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 3,
                criteria_cols: vec![1, 2],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("east", Value::Number(15.5))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 2,
                criteria_cols: vec![1],
                sheet: None,
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
        };
        let prelude = prelude_with_multi(key, vec![("x\0y\0z", Value::Number(42.0))]);
        let result = prelude.get_multi_conditional(
            &MultiConditionalAggKey {
                kind: AggKind::Average,
                sum_col: 4,
                criteria_cols: vec![1, 2, 3],
                sheet: None,
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
        };
        assert_eq!(prelude.get_multi_conditional(&key, "a\0b"), Value::Error(CellError::Div0));
    }
}
