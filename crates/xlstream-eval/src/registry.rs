//! Centralized function registry.
//!
//! Maps every supported Excel function name (and its aliases) to a
//! [`FunctionEntry`] containing metadata and a uniform handler. The
//! registry is built once at startup via [`LazyLock`] and never mutated.
#![allow(clippy::too_many_lines)]

use std::collections::HashMap;
use std::sync::LazyLock;

use xlstream_core::Value;
use xlstream_parse::rewrite::AggKind;
use xlstream_parse::{FnCaps, FnCategory, FunctionMeta, NodeRef};

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// How a function's arguments are prepared before calling.
///
/// Each variant carries the implementation fn pointer. The registry's
/// [`dispatch`] function matches on this to handle arg preparation
/// generically — eliminating per-function boilerplate wrappers.
pub(crate) enum Dispatch {
    /// Eagerly evaluate all args to `&[Value]`, then call.
    Eager(fn(&[Value]) -> Value),
    /// Expand + coerce args for aggregate semantics, then call.
    Aggregate(fn(&[Value]) -> Value),
    /// No arg prep — handler receives raw `&[NodeRef]` and does everything.
    Custom(fn(&[NodeRef<'_>], &Interpreter<'_>, &RowScope<'_>) -> Value),
}

/// A single function's metadata plus its dispatch strategy.
pub struct FunctionEntry {
    /// Classification metadata (name, caps, category, `agg_kind`).
    pub meta: FunctionMeta,
    /// Alternative names that resolve to this entry (e.g. `CONCATENATE` for `CONCAT`).
    pub aliases: &'static [&'static str],
    /// The dispatch strategy and implementation fn pointer.
    pub(crate) dispatch: Dispatch,
}

// ---------------------------------------------------------------------------
// Static table of every supported function.
// ---------------------------------------------------------------------------

static ALL_ENTRIES: &[FunctionEntry] = &[
    // -- Conditional (SHORT_CIRCUIT) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "IF",
            caps: FnCaps::SHORT_CIRCUIT,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_if),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "IFS",
            caps: FnCaps::SHORT_CIRCUIT,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_ifs),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SWITCH",
            caps: FnCaps::SHORT_CIRCUIT,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_switch),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "IFERROR",
            caps: FnCaps::SHORT_CIRCUIT,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_iferror),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "IFNA",
            caps: FnCaps::SHORT_CIRCUIT,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_ifna),
    },
    // -- Conditional (SHORT_CIRCUIT | RANGE_EXPAND) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "AND",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_and),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "OR",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_or),
    },
    // -- Conditional (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "NOT",
            caps: FnCaps::PURE,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_not),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "XOR",
            caps: FnCaps::PURE,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::conditional::builtin_xor),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TRUE",
            caps: FnCaps::PURE,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_true),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FALSE",
            caps: FnCaps::PURE,
            category: FnCategory::Conditional,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_false),
    },
    // -- Multi-conditional aggregate (SHORT_CIRCUIT | NEEDS_PRELUDE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "SUMIFS",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_sumifs),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COUNTIFS",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_countifs),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "AVERAGEIFS",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_averageifs),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SUMIF",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_sumif),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COUNTIF",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_countif),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "AVERAGEIF",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_averageif),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MINIFS",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_minifs),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MAXIFS",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::multi_conditional::builtin_maxifs),
    },
    // -- Simple aggregate (PURE | RANGE_EXPAND | AGG_COERCE | NEEDS_PRELUDE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "SUM",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Sum),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::sum),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COUNT",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Count),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::count),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COUNTA",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::CountA),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::counta),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COUNTBLANK",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::CountBlank),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::countblank),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "AVERAGE",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Average),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::average),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MIN",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Min),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::min),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MAX",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Max),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::max),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MEDIAN",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Median),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::median),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PRODUCT",
            caps: FnCaps::PURE
                .union(FnCaps::RANGE_EXPAND)
                .union(FnCaps::AGG_COERCE)
                .union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Product),
        },
        aliases: &[],
        dispatch: Dispatch::Aggregate(crate::builtins::aggregate::product),
    },
    // -- Lookup (LOOKUP | NEEDS_PRELUDE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "VLOOKUP",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_vlookup),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "HLOOKUP",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_hlookup),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "XLOOKUP",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_xlookup),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MATCH",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_match),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "XMATCH",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_xmatch),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "INDEX",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_index),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CHOOSE",
            caps: FnCaps::LOOKUP.union(FnCaps::NEEDS_PRELUDE),
            category: FnCategory::Lookup,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::lookup::builtin_choose),
    },
    // -- Volatile (VOLATILE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "TODAY",
            caps: FnCaps::VOLATILE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::date::builtin_today),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "NOW",
            caps: FnCaps::VOLATILE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::date::builtin_now),
    },
    // -- Date pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "DATE",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_date),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "YEAR",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_year),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MONTH",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_month),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DAY",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_day),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "WEEKDAY",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_weekday),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "EDATE",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_edate),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "EOMONTH",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_eomonth),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DATEDIF",
            caps: FnCaps::PURE,
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::date::builtin_datedif),
    },
    // -- Date range-expanding (PURE | RANGE_EXPAND) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "NETWORKDAYS",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::date::builtin_networkdays),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "WORKDAY",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Date,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::date::builtin_workday),
    },
    // -- String pure eager (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "LEFT",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_left),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "RIGHT",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_right),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MID",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_mid),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LEN",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_len),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "UPPER",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_upper),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LOWER",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_lower),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PROPER",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_proper),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TRIM",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_trim),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CLEAN",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_clean),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FIND",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_find),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SEARCH",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_search),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SUBSTITUTE",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_substitute),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "REPLACE",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_replace),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TEXT",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_text),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "VALUE",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_value),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "EXACT",
            caps: FnCaps::PURE,
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::string::builtin_exact),
    },
    // -- String range-expanding (PURE | RANGE_EXPAND) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "CONCAT",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &["CONCATENATE"],
        dispatch: Dispatch::Custom(crate::builtins::string::builtin_concat),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TEXTJOIN",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::String,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::string::builtin_textjoin),
    },
    // -- Math pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "ROUND",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_round),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ROUNDUP",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_roundup),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ROUNDDOWN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_rounddown),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "INT",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_int),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MOD",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_mod),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ABS",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_abs),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SIGN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_sign),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SQRT",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_sqrt),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "POWER",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_power),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CEILING",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_ceiling),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FLOOR",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_floor),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "EVEN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_even),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ODD",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_odd),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TRUNC",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_trunc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MROUND",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_mround),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CEILING.MATH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_ceiling_math),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FLOOR.MATH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_floor_math),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CEILING.PRECISE",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &["ISO.CEILING"],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_ceiling_precise),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FLOOR.PRECISE",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_floor_precise),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PI",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_pi),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_ln),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LOG",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_log),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LOG10",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_log10),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "EXP",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_exp),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SIN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_sin),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COS",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_cos),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TAN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_tan),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ASIN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_asin),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ACOS",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_acos),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ATAN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_atan),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ATAN2",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_atan2),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FACT",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_fact),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FACTDOUBLE",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_factdouble),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PERMUT",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_permut),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PERMUTATIONA",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_permutationa),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COMBIN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_combin),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COMBINA",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_combina),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "GCD",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_gcd),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LCM",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_lcm),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ROMAN",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_roman),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ARABIC",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_arabic),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ACOSH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_acosh),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ASINH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_asinh),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ATANH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_atanh),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COSH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_cosh),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SINH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_sinh),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TANH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_tanh),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COT",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_cot),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CSC",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_csc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SEC",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_sec),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COTH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_coth),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CSCH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_csch),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SECH",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_sech),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DEGREES",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_degrees),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "RADIANS",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::math::builtin_radians),
    },
    // -- Info pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISBLANK",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_isblank),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISNUMBER",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_isnumber),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISTEXT",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_istext),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISERROR",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_iserror),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISNA",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_isna),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISLOGICAL",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_islogical),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISNONTEXT",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_isnontext),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ISREF",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_isref),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "NA",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_na),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "TYPE",
            caps: FnCaps::PURE,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::info::builtin_type),
    },
    // -- Info range-metadata (RANGE_METADATA) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "ROW",
            caps: FnCaps::RANGE_METADATA,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_row),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COLUMN",
            caps: FnCaps::RANGE_METADATA,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_column),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ROWS",
            caps: FnCaps::RANGE_METADATA,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_rows),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COLUMNS",
            caps: FnCaps::RANGE_METADATA,
            category: FnCategory::Info,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_columns),
    },
    // -- Financial pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "PMT",
            caps: FnCaps::PURE,
            category: FnCategory::Financial,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::financial::builtin_pmt),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PV",
            caps: FnCaps::PURE,
            category: FnCategory::Financial,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::financial::builtin_pv),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FV",
            caps: FnCaps::PURE,
            category: FnCategory::Financial,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::financial::builtin_fv),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "RATE",
            caps: FnCaps::PURE,
            category: FnCategory::Financial,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::financial::builtin_rate),
    },
    // -- Financial range-expanding (PURE | RANGE_EXPAND) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "NPV",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Financial,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::financial::builtin_npv),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "IRR",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Financial,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::financial::builtin_irr),
    },
    // -- Meta-dispatch (SHORT_CIRCUIT | RANGE_EXPAND) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "SUBTOTAL",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::subtotal::builtin_subtotal),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "AGGREGATE",
            caps: FnCaps::SHORT_CIRCUIT.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Aggregate,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::subtotal::builtin_aggregate),
    },
    // -- Statistical range-expanding (PURE | RANGE_EXPAND) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "SUMPRODUCT",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_sumproduct),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "AVEDEV",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_avedev),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "LARGE",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_large),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SMALL",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_small),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "VAR.S",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_var_s),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "VAR.P",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_var_p),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "STDEV.S",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_stdev_s),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "STDEV.P",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_stdev_p),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SKEW",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_skew),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SKEW.P",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_skew_p),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "KURT",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_kurt),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "MODE.SNGL",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_mode_sngl),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PERCENTILE.INC",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_percentile_inc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "PERCENTILE.EXC",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_percentile_exc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "QUARTILE.INC",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_quartile_inc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "QUARTILE.EXC",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_quartile_exc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "RANK.EQ",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_rank_eq),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "RANK.AVG",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_rank_avg),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "EXPON.DIST",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_expon_dist),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "CORREL",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_correl),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COVARIANCE.P",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_covariance_p),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COVARIANCE.S",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_covariance_s),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "SLOPE",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_slope),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "INTERCEPT",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_intercept),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "RSQ",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_rsq),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "FORECAST.LINEAR",
            caps: FnCaps::PURE.union(FnCaps::RANGE_EXPAND),
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_forecast_linear),
    },
    // -- Statistical pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "POISSON.DIST",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &["POISSON"],
        dispatch: Dispatch::Custom(crate::builtins::handle_poisson_dist),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "T.DIST",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::statistical::builtin_t_dist),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "T.DIST.RT",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::statistical::builtin_t_dist_rt),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "T.DIST.2T",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::statistical::builtin_t_dist_2t),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "T.INV",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::statistical::builtin_t_inv),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "T.INV.2T",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::statistical::builtin_t_inv_2t),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BINOM.DIST",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_binom_dist),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BINOM.INV",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_binom_inv),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "NORM.DIST",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_norm_dist),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "NORM.INV",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_norm_inv),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "NORM.S.DIST",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_norm_s_dist),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "NORM.S.INV",
            caps: FnCaps::PURE,
            category: FnCategory::Statistical,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Custom(crate::builtins::handle_norm_s_inv),
    },
    // -- Engineering pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "HEX2DEC",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_hex2dec),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DEC2HEX",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_dec2hex),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "COMPLEX",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_complex),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "IMREAL",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_imreal),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "IMAGINARY",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_imaginary),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BITAND",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bitand),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BITOR",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bitor),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BITXOR",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bitxor),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BITLSHIFT",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bitlshift),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BITRSHIFT",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bitrshift),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BIN2DEC",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bin2dec),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DEC2BIN",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_dec2bin),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "OCT2DEC",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_oct2dec),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DEC2OCT",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_dec2oct),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "HEX2BIN",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_hex2bin),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BIN2HEX",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bin2hex),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "HEX2OCT",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_hex2oct),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "OCT2HEX",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_oct2hex),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BIN2OCT",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_bin2oct),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "OCT2BIN",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_oct2bin),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "BASE",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_base),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "DELTA",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_delta),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "GESTEP",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_gestep),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ERF",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_erf),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ERFC",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_erfc),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ERF.PRECISE",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_erf_precise),
    },
    FunctionEntry {
        meta: FunctionMeta {
            name: "ERFC.PRECISE",
            caps: FnCaps::PURE,
            category: FnCategory::Engineering,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::engineering::builtin_erfc_precise),
    },
    // -- Conversion pure (PURE) --
    FunctionEntry {
        meta: FunctionMeta {
            name: "CONVERT",
            caps: FnCaps::PURE,
            category: FnCategory::Conversion,
            agg_kind: None,
        },
        aliases: &[],
        dispatch: Dispatch::Eager(crate::builtins::convert::builtin_convert),
    },
];

// ---------------------------------------------------------------------------
// Lazy HashMap keyed by uppercase name (canonical + aliases).
// ---------------------------------------------------------------------------

static REGISTRY: LazyLock<HashMap<&'static str, &'static FunctionEntry>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity(ALL_ENTRIES.len() * 2);
    for entry in ALL_ENTRIES {
        map.insert(entry.meta.name, entry);
        for &alias in entry.aliases {
            map.insert(alias, entry);
        }
    }
    map
});

/// Look up a function entry by name (case-insensitive, strips `_XLFN.` prefix).
///
/// Returns `None` for unknown functions.
///
/// # Examples
///
/// ```
/// use xlstream_eval::registry::lookup;
/// let entry = lookup("sum").unwrap();
/// assert_eq!(entry.meta.name, "SUM");
///
/// // aliases resolve too
/// let entry = lookup("CONCATENATE").unwrap();
/// assert_eq!(entry.meta.name, "CONCAT");
///
/// // _XLFN. prefix is stripped
/// let entry = lookup("_XLFN.XLOOKUP").unwrap();
/// assert_eq!(entry.meta.name, "XLOOKUP");
/// ```
pub fn lookup(name: &str) -> Option<&'static FunctionEntry> {
    let upper = name.to_ascii_uppercase();
    let normalized = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
    REGISTRY.get(normalized).copied()
}

/// Look up only the [`FunctionMeta`] for a function name.
///
/// Convenience wrapper over [`lookup`] for callers that only need metadata.
///
/// # Examples
///
/// ```
/// use xlstream_eval::registry::lookup_meta;
/// use xlstream_parse::FnCategory;
/// let meta = lookup_meta("ROUND").unwrap();
/// assert_eq!(meta.category, FnCategory::Math);
/// ```
#[must_use]
pub fn lookup_meta(name: &str) -> Option<&'static FunctionMeta> {
    lookup(name).map(|e| &e.meta)
}

/// Dispatch a function call by name: look up, prepare args, invoke.
///
/// Returns `None` for unknown function names.
pub(crate) fn dispatch(
    name: &str,
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Option<Value> {
    let entry = lookup(name)?;
    Some(match &entry.dispatch {
        Dispatch::Eager(f) => f(&crate::builtins::eval_args(args, interp, scope)),
        Dispatch::Aggregate(f) => {
            f(&crate::builtins::expand_args_for_aggregate(args, interp, scope))
        }
        Dispatch::Custom(f) => f(args, interp, scope),
    })
}

/// Iterate over all registered function entries.
///
/// Returns entries in static definition order (not alphabetical).
///
/// # Examples
///
/// ```
/// use xlstream_eval::registry::all;
/// let entries: Vec<_> = all().collect();
/// assert!(entries.len() > 200);
/// assert!(entries.iter().any(|e| e.meta.name == "SUM"));
/// ```
pub fn all() -> impl Iterator<Item = &'static FunctionEntry> {
    ALL_ENTRIES.iter()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn lookup_sum_returns_correct_metadata() {
        let entry = lookup("SUM").unwrap();
        assert_eq!(entry.meta.name, "SUM");
        assert_eq!(entry.meta.category, FnCategory::Aggregate);
        assert!(entry.meta.caps.contains(FnCaps::PURE));
        assert!(entry.meta.caps.contains(FnCaps::AGG_COERCE));
        assert!(entry.meta.caps.contains(FnCaps::NEEDS_PRELUDE));
        assert_eq!(entry.meta.agg_kind, Some(AggKind::Sum));
    }

    #[test]
    fn lookup_case_insensitive() {
        let a = lookup("sum").unwrap();
        let b = lookup("SUM").unwrap();
        let c = lookup("Sum").unwrap();
        assert_eq!(a.meta.name, b.meta.name);
        assert_eq!(b.meta.name, c.meta.name);
    }

    #[test]
    fn lookup_xlfn_prefix_stripped() {
        let entry = lookup("_XLFN.XLOOKUP").unwrap();
        assert_eq!(entry.meta.name, "XLOOKUP");
    }

    #[test]
    fn lookup_unknown_returns_none() {
        assert!(lookup("NOTAFUNCTION").is_none());
    }

    #[test]
    fn lookup_alias_resolves_to_canonical() {
        let entry = lookup("CONCATENATE").unwrap();
        assert_eq!(entry.meta.name, "CONCAT");
    }

    #[test]
    fn all_entries_have_uppercase_names() {
        for entry in all() {
            assert_eq!(
                entry.meta.name,
                entry.meta.name.to_uppercase(),
                "entry name {:?} must be uppercase",
                entry.meta.name
            );
        }
    }

    #[test]
    fn lookup_implies_needs_prelude() {
        for entry in all() {
            if entry.meta.caps.contains(FnCaps::LOOKUP) {
                assert!(
                    entry.meta.caps.contains(FnCaps::NEEDS_PRELUDE),
                    "{}: LOOKUP must imply NEEDS_PRELUDE",
                    entry.meta.name
                );
            }
        }
    }

    #[test]
    fn agg_coerce_implies_range_expand() {
        for entry in all() {
            if entry.meta.caps.contains(FnCaps::AGG_COERCE) {
                assert!(
                    entry.meta.caps.contains(FnCaps::RANGE_EXPAND),
                    "{}: AGG_COERCE must imply RANGE_EXPAND",
                    entry.meta.name
                );
            }
        }
    }

    #[test]
    fn agg_kind_implies_agg_coerce() {
        for entry in all() {
            if entry.meta.agg_kind.is_some() {
                assert!(
                    entry.meta.caps.contains(FnCaps::AGG_COERCE),
                    "{}: agg_kind requires AGG_COERCE",
                    entry.meta.name
                );
            }
        }
    }

    #[test]
    fn no_duplicate_names() {
        let mut seen = std::collections::HashSet::new();
        for entry in all() {
            assert!(seen.insert(entry.meta.name), "duplicate name: {}", entry.meta.name);
        }
    }

    #[test]
    fn all_aliases_resolve_to_canonical_entry() {
        for entry in all() {
            for &alias in entry.aliases {
                let resolved = lookup(alias).unwrap_or_else(|| {
                    panic!("alias {alias:?} of {} not found in registry", entry.meta.name)
                });
                assert_eq!(
                    resolved.meta.name, entry.meta.name,
                    "alias {alias:?} should resolve to {}",
                    entry.meta.name
                );
            }
        }
    }

    #[test]
    fn all_aliases_are_uppercase() {
        for entry in all() {
            for &alias in entry.aliases {
                assert_eq!(alias, alias.to_uppercase(), "alias {alias:?} must be uppercase");
            }
        }
    }

    #[test]
    fn entry_count_is_exact() {
        assert_eq!(all().count(), 208, "registry entry count changed — update this test");
    }

    #[test]
    fn aggregate_category_implies_needs_prelude() {
        let meta_dispatch = ["SUBTOTAL", "AGGREGATE"];
        for entry in all() {
            if entry.meta.category == FnCategory::Aggregate
                && !meta_dispatch.contains(&entry.meta.name)
            {
                assert!(
                    entry.meta.caps.contains(FnCaps::NEEDS_PRELUDE),
                    "{}: Aggregate category must imply NEEDS_PRELUDE",
                    entry.meta.name
                );
            }
        }
    }
}
