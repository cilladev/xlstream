//! Builtin function dispatch.
//!
//! Entry point: `dispatch`. The interpreter calls this for every
//! `NodeView::Function` node. Returns `Some(value)` if the function is
//! known, `None` otherwise (caller falls back to `#VALUE!`).

pub mod aggregate;
mod conditional;
pub(crate) mod date;
pub mod financial;
pub mod info;
mod lookup;
pub(crate) mod math;
mod multi_conditional;
pub(crate) mod string;

use xlstream_core::Value;
use xlstream_parse::NodeRef;

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// Evaluate all arguments eagerly, returning a `Vec<Value>`.
///
/// Used by pure builtins (string, math, etc.) that don't need
/// short-circuit evaluation.
fn eval_args(args: &[NodeRef<'_>], interp: &Interpreter<'_>, scope: &RowScope<'_>) -> Vec<Value> {
    args.iter().map(|a| interp.eval(*a, scope)).collect()
}

/// Expand a single AST node into a `Vec<Value>`.
///
/// For `RangeRef` nodes this resolves from lookup sheets or the prelude
/// bounded range cache. For any other node it evaluates normally and
/// returns a single-element vec.
pub(crate) fn expand_range(
    node: NodeRef<'_>,
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Vec<Value> {
    use xlstream_core::CellError;
    use xlstream_parse::NodeView;

    match node.view() {
        NodeView::RangeRef { sheet, start_row, end_row, start_col, .. } => {
            let sc = start_col.unwrap_or(1);

            // Try lookup sheet (already fully loaded).
            if let Some(sheet_name) = sheet {
                if let Some(ls) = interp.prelude().lookup_sheet(sheet_name) {
                    let sr = start_row.map_or(0, |r| (r - 1) as usize);
                    let er = end_row.map_or(ls.num_rows().saturating_sub(1), |r| (r - 1) as usize);
                    let col = (sc - 1) as usize;
                    return (sr..=er)
                        .map(|r| ls.cell(r, col).cloned().unwrap_or(Value::Empty))
                        .collect();
                }
            }

            // Fall back to cached bounded range (main sheet).
            if let (Some(sr), Some(er)) = (start_row, end_row) {
                let key = crate::prelude::BoundedRangeKey {
                    sheet: sheet.map(ToString::to_string),
                    col: sc,
                    start_row: sr,
                    end_row: er,
                };
                if let Some(values) = interp.prelude().get_cached_range(&key) {
                    return values.clone();
                }
            }

            vec![Value::Error(CellError::Ref)]
        }
        _ => vec![interp.eval(node, scope)],
    }
}

/// Look up `name` (case-insensitive) and call the matching builtin.
///
/// Returns `None` for unknown functions — the caller decides the fallback.
pub(crate) fn dispatch(
    name: &str,
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Option<Value> {
    let upper = name.to_ascii_uppercase();
    let normalized = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
    match normalized {
        "IF" => Some(conditional::builtin_if(args, interp, scope)),
        "IFS" => Some(conditional::builtin_ifs(args, interp, scope)),
        "SWITCH" => Some(conditional::builtin_switch(args, interp, scope)),
        "IFERROR" => Some(conditional::builtin_iferror(args, interp, scope)),
        "IFNA" => Some(conditional::builtin_ifna(args, interp, scope)),
        "AND" => Some(conditional::builtin_and(args, interp, scope)),
        "OR" => Some(conditional::builtin_or(args, interp, scope)),
        "NOT" => Some(conditional::builtin_not(args, interp, scope)),
        "XOR" => Some(conditional::builtin_xor(args, interp, scope)),
        "TRUE" => Some(conditional::builtin_true(args)),
        "FALSE" => Some(conditional::builtin_false(args)),
        "SUMIFS" => Some(multi_conditional::builtin_sumifs(args, interp, scope)),
        "COUNTIFS" => Some(multi_conditional::builtin_countifs(args, interp, scope)),
        "AVERAGEIFS" => Some(multi_conditional::builtin_averageifs(args, interp, scope)),
        "SUMIF" => Some(multi_conditional::builtin_sumif(args, interp, scope)),
        "COUNTIF" => Some(multi_conditional::builtin_countif(args, interp, scope)),
        "AVERAGEIF" => Some(multi_conditional::builtin_averageif(args, interp, scope)),
        "VLOOKUP" => Some(lookup::builtin_vlookup(args, interp, scope)),
        "XLOOKUP" | "_XLFN.XLOOKUP" => Some(lookup::builtin_xlookup(args, interp, scope)),
        "HLOOKUP" => Some(lookup::builtin_hlookup(args, interp, scope)),
        "MATCH" => Some(lookup::builtin_match(args, interp, scope)),
        "XMATCH" | "_XLFN.XMATCH" => Some(lookup::builtin_xmatch(args, interp, scope)),
        "INDEX" => Some(lookup::builtin_index(args, interp, scope)),
        "CHOOSE" => Some(lookup::builtin_choose(args, interp, scope)),
        // -- date builtins (stateful — need prelude) --
        "TODAY" => Some(date::builtin_today(args, interp, scope)),
        "NOW" => Some(date::builtin_now(args, interp, scope)),
        // -- date builtins (pure, eager eval) --
        "DATE" => Some(date::builtin_date(&eval_args(args, interp, scope))),
        "YEAR" => Some(date::builtin_year(&eval_args(args, interp, scope))),
        "MONTH" => Some(date::builtin_month(&eval_args(args, interp, scope))),
        "DAY" => Some(date::builtin_day(&eval_args(args, interp, scope))),
        "WEEKDAY" => Some(date::builtin_weekday(&eval_args(args, interp, scope))),
        "EDATE" => Some(date::builtin_edate(&eval_args(args, interp, scope))),
        "EOMONTH" => Some(date::builtin_eomonth(&eval_args(args, interp, scope))),
        "DATEDIF" => Some(date::builtin_datedif(&eval_args(args, interp, scope))),
        "NETWORKDAYS" => Some(date::builtin_networkdays(args, interp, scope)),
        "WORKDAY" => Some(date::builtin_workday(args, interp, scope)),
        // -- string builtins (pure, eager eval) --
        "LEFT" => Some(string::builtin_left(&eval_args(args, interp, scope))),
        "RIGHT" => Some(string::builtin_right(&eval_args(args, interp, scope))),
        "MID" => Some(string::builtin_mid(&eval_args(args, interp, scope))),
        "LEN" => Some(string::builtin_len(&eval_args(args, interp, scope))),
        "UPPER" => Some(string::builtin_upper(&eval_args(args, interp, scope))),
        "LOWER" => Some(string::builtin_lower(&eval_args(args, interp, scope))),
        "PROPER" => Some(string::builtin_proper(&eval_args(args, interp, scope))),
        "TRIM" => Some(string::builtin_trim(&eval_args(args, interp, scope))),
        "CLEAN" => Some(string::builtin_clean(&eval_args(args, interp, scope))),
        "CONCAT" | "CONCATENATE" => Some(string::builtin_concat(args, interp, scope)),
        "TEXTJOIN" => Some(string::builtin_textjoin(args, interp, scope)),
        "FIND" => Some(string::builtin_find(&eval_args(args, interp, scope))),
        "SEARCH" => Some(string::builtin_search(&eval_args(args, interp, scope))),
        "SUBSTITUTE" => Some(string::builtin_substitute(&eval_args(args, interp, scope))),
        "REPLACE" => Some(string::builtin_replace(&eval_args(args, interp, scope))),
        "TEXT" => Some(string::builtin_text(&eval_args(args, interp, scope))),
        "VALUE" => Some(string::builtin_value(&eval_args(args, interp, scope))),
        "EXACT" => Some(string::builtin_exact(&eval_args(args, interp, scope))),
        // -- math builtins (pure, eager eval) --
        "ROUND" => Some(math::builtin_round(&eval_args(args, interp, scope))),
        "ROUNDUP" => Some(math::builtin_roundup(&eval_args(args, interp, scope))),
        "ROUNDDOWN" => Some(math::builtin_rounddown(&eval_args(args, interp, scope))),
        "INT" => Some(math::builtin_int(&eval_args(args, interp, scope))),
        "MOD" => Some(math::builtin_mod(&eval_args(args, interp, scope))),
        "ABS" => Some(math::builtin_abs(&eval_args(args, interp, scope))),
        "SIGN" => Some(math::builtin_sign(&eval_args(args, interp, scope))),
        "SQRT" => Some(math::builtin_sqrt(&eval_args(args, interp, scope))),
        "POWER" => Some(math::builtin_power(&eval_args(args, interp, scope))),
        "CEILING" => Some(math::builtin_ceiling(&eval_args(args, interp, scope))),
        "FLOOR" => Some(math::builtin_floor(&eval_args(args, interp, scope))),
        "PI" => Some(math::builtin_pi(&eval_args(args, interp, scope))),
        "LN" => Some(math::builtin_ln(&eval_args(args, interp, scope))),
        "LOG" => Some(math::builtin_log(&eval_args(args, interp, scope))),
        "LOG10" => Some(math::builtin_log10(&eval_args(args, interp, scope))),
        "EXP" => Some(math::builtin_exp(&eval_args(args, interp, scope))),
        "SIN" => Some(math::builtin_sin(&eval_args(args, interp, scope))),
        "COS" => Some(math::builtin_cos(&eval_args(args, interp, scope))),
        "TAN" => Some(math::builtin_tan(&eval_args(args, interp, scope))),
        "ASIN" => Some(math::builtin_asin(&eval_args(args, interp, scope))),
        "ACOS" => Some(math::builtin_acos(&eval_args(args, interp, scope))),
        "ATAN" => Some(math::builtin_atan(&eval_args(args, interp, scope))),
        "ATAN2" => Some(math::builtin_atan2(&eval_args(args, interp, scope))),
        // -- info builtins (pure, eager eval) --
        "ISBLANK" => Some(info::builtin_isblank(&eval_args(args, interp, scope))),
        "ISNUMBER" => Some(info::builtin_isnumber(&eval_args(args, interp, scope))),
        "ISTEXT" => Some(info::builtin_istext(&eval_args(args, interp, scope))),
        "ISERROR" => Some(info::builtin_iserror(&eval_args(args, interp, scope))),
        "ISNA" => Some(info::builtin_isna(&eval_args(args, interp, scope))),
        "ISLOGICAL" => Some(info::builtin_islogical(&eval_args(args, interp, scope))),
        "ISNONTEXT" => Some(info::builtin_isnontext(&eval_args(args, interp, scope))),
        "ISREF" => Some(info::builtin_isref(&eval_args(args, interp, scope))),
        "NA" => Some(info::builtin_na(&eval_args(args, interp, scope))),
        "TYPE" => Some(info::builtin_type(&eval_args(args, interp, scope))),
        // -- financial builtins (pure, eager eval) --
        "PMT" => Some(financial::builtin_pmt(&eval_args(args, interp, scope))),
        "PV" => Some(financial::builtin_pv(&eval_args(args, interp, scope))),
        "FV" => Some(financial::builtin_fv(&eval_args(args, interp, scope))),
        "NPV" => Some(financial::builtin_npv(args, interp, scope)),
        "IRR" => Some(financial::builtin_irr(args, interp, scope)),
        "RATE" => Some(financial::builtin_rate(&eval_args(args, interp, scope))),
        _ => None,
    }
}
