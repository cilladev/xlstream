//! Builtin function dispatch.
//!
//! Entry point: [`dispatch`]. The interpreter calls this for every
//! `NodeView::Function` node. Returns `Some(value)` if the function is
//! known, `None` otherwise (caller falls back to `#VALUE!`).

pub mod aggregate;
mod conditional;
pub(crate) mod date;
pub(crate) mod financial;
pub(crate) mod info;
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
        "VLOOKUP" => Some(lookup::builtin_vlookup(args, interp, scope)),
        "XLOOKUP" | "_XLFN.XLOOKUP" => Some(lookup::builtin_xlookup(args, interp, scope)),
        "HLOOKUP" => Some(lookup::builtin_hlookup(args, interp, scope)),
        "MATCH" => Some(lookup::builtin_match(args, interp, scope)),
        "XMATCH" | "_XLFN.XMATCH" => Some(lookup::builtin_xmatch(args, interp, scope)),
        "INDEX" => Some(lookup::builtin_index(args, interp, scope)),
        "CHOOSE" => Some(lookup::builtin_choose(args, interp, scope)),
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
        "CONCAT" | "CONCATENATE" => Some(string::builtin_concat(&eval_args(args, interp, scope))),
        "TEXTJOIN" => Some(string::builtin_textjoin(&eval_args(args, interp, scope))),
        "FIND" => Some(string::builtin_find(&eval_args(args, interp, scope))),
        "SEARCH" => Some(string::builtin_search(&eval_args(args, interp, scope))),
        _ => None,
    }
}
