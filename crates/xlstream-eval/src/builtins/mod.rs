//! Builtin function dispatch.
//!
//! Entry point: [`dispatch`]. The interpreter calls this for every
//! `NodeView::Function` node. Returns `Some(value)` if the function is
//! known, `None` otherwise (caller falls back to `#VALUE!`).

pub mod aggregate;
mod conditional;

use xlstream_core::Value;
use xlstream_parse::NodeRef;

use crate::interp::Interpreter;
use crate::scope::RowScope;

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
        _ => None,
    }
}
