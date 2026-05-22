//! Pure metadata types describing function capabilities.
//!
//! These types live in xlstream-parse so the classifier can read them
//! without importing xlstream-eval. The registry in xlstream-eval
//! populates them; parse only consumes.

use bitflags::bitflags;

use crate::rewrite::AggKind;

bitflags! {
    /// Capability flags describing a function's evaluation behavior.
    ///
    /// Used by the classifier to route functions and by the evaluator to
    /// select the right dispatch path. Each flag is independent; combine
    /// with `|`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::FnCaps;
    /// let caps = FnCaps::PURE | FnCaps::RANGE_EXPAND;
    /// assert!(caps.contains(FnCaps::PURE));
    /// assert!(!caps.contains(FnCaps::LOOKUP));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FnCaps: u8 {
        /// Deterministic, no side effects.
        const PURE           = 0b0000_0001;
        /// Lazy arg evaluation (IF, AND, OR).
        const SHORT_CIRCUIT  = 0b0000_0010;
        /// Expand range refs before call.
        const RANGE_EXPAND   = 0b0000_0100;
        /// Coerce scalar bools/text in aggregate context.
        const AGG_COERCE     = 0b0000_1000;
        /// Requires pass-1 (prelude) data.
        const NEEDS_PRELUDE  = 0b0001_0000;
        /// Uses prelude-loaded lookup index.
        const LOOKUP         = 0b0010_0000;
        /// Changes between runs (TODAY, NOW).
        const VOLATILE       = 0b0100_0000;
        /// Range args inspected for metadata only (ROW, COLUMN).
        const RANGE_METADATA = 0b1000_0000;
    }
}

/// Broad category a function belongs to.
///
/// Used for classification routing and future documentation grouping.
/// A function has exactly one category.
///
/// # Examples
///
/// ```
/// use xlstream_parse::FnCategory;
/// let cat = FnCategory::Math;
/// assert_eq!(cat, FnCategory::Math);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FnCategory {
    /// Math functions (ABS, ROUND, SQRT, ...).
    Math,
    /// String functions (LEFT, UPPER, CONCAT, ...).
    String,
    /// Date functions (DATE, YEAR, TODAY, ...).
    Date,
    /// Lookup functions (VLOOKUP, XLOOKUP, INDEX, ...).
    Lookup,
    /// Aggregate functions (SUM, COUNT, AVERAGE, SUMIF, ...).
    Aggregate,
    /// Conditional functions (IF, IFS, SWITCH, ...).
    Conditional,
    /// Info functions (ISBLANK, ISNUMBER, TYPE, ...).
    Info,
    /// Financial functions (PMT, PV, FV, IRR, ...).
    Financial,
    /// Statistical functions (VAR.S, STDEV.S, NORM.DIST, ...).
    Statistical,
    /// Engineering functions (HEX2DEC, COMPLEX, BITAND, ...).
    Engineering,
    /// Unit conversion (CONVERT).
    Conversion,
    /// Database functions (DSUM, DAVERAGE, ...). Reserved for v0.5.
    Database,
    /// Compatibility aliases. Reserved for v0.5.
    Compatibility,
}

/// Metadata describing a single Excel function's capabilities.
///
/// Populated by the registry in xlstream-eval. Parse code only reads
/// these — it never constructs them.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{FnCaps, FnCategory, FunctionMeta};
/// let meta = FunctionMeta {
///     name: "SUM",
///     caps: FnCaps::PURE | FnCaps::RANGE_EXPAND | FnCaps::AGG_COERCE | FnCaps::NEEDS_PRELUDE,
///     category: FnCategory::Aggregate,
///     agg_kind: None,
/// };
/// assert!(meta.caps.contains(FnCaps::NEEDS_PRELUDE));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionMeta {
    /// Canonical uppercase name (e.g. `"SUM"`).
    pub name: &'static str,
    /// Capability flags.
    pub caps: FnCaps,
    /// Broad category.
    pub category: FnCategory,
    /// For simple aggregates, which `AggKind` this function computes.
    /// `None` for conditional aggregates and non-aggregates.
    pub agg_kind: Option<AggKind>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn fn_caps_bitwise_or_combines_flags() {
        let caps = FnCaps::PURE | FnCaps::RANGE_EXPAND;
        assert!(caps.contains(FnCaps::PURE));
        assert!(caps.contains(FnCaps::RANGE_EXPAND));
        assert!(!caps.contains(FnCaps::SHORT_CIRCUIT));
    }

    #[test]
    fn fn_caps_all_flags_are_distinct() {
        let all = [
            FnCaps::PURE,
            FnCaps::SHORT_CIRCUIT,
            FnCaps::RANGE_EXPAND,
            FnCaps::AGG_COERCE,
            FnCaps::NEEDS_PRELUDE,
            FnCaps::LOOKUP,
            FnCaps::VOLATILE,
            FnCaps::RANGE_METADATA,
        ];
        for (i, a) in all.iter().enumerate() {
            for (j, b) in all.iter().enumerate() {
                if i != j {
                    assert!(!a.intersects(*b), "flags {a:?} and {b:?} overlap");
                }
            }
        }
    }

    #[test]
    fn fn_caps_empty_contains_nothing() {
        let caps = FnCaps::empty();
        assert!(!caps.contains(FnCaps::PURE));
        assert!(!caps.contains(FnCaps::VOLATILE));
    }

    #[test]
    fn function_meta_stores_agg_kind() {
        let meta = FunctionMeta {
            name: "SUM",
            caps: FnCaps::PURE | FnCaps::RANGE_EXPAND | FnCaps::AGG_COERCE | FnCaps::NEEDS_PRELUDE,
            category: FnCategory::Aggregate,
            agg_kind: Some(AggKind::Sum),
        };
        assert_eq!(meta.agg_kind, Some(AggKind::Sum));
    }

    #[test]
    fn function_meta_non_aggregate_has_no_agg_kind() {
        let meta = FunctionMeta {
            name: "SQRT",
            caps: FnCaps::PURE,
            category: FnCategory::Math,
            agg_kind: None,
        };
        assert_eq!(meta.agg_kind, None);
    }
}
