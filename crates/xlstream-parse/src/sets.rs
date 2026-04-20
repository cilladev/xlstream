//! Function-name sets used by the classifier.
//!
//! Sets are stored in upper-case; lookups normalise the incoming name to
//! upper-case to give Excel-style case-insensitivity.
//!
//! The classifier checks `is_unsupported` first and emits
//! `UnsupportedFunction(name)` for all entries. The sub-sets
//! (`DYNAMIC_ARRAY_FUNCTIONS`, `VOLATILE_UNSUPPORTED`) exist for future
//! use when the evaluator needs to distinguish refusal categories at
//! runtime — the classifier itself does not use them today.

use phf::{phf_set, Set};

/// Functions xlstream cannot stream. The classifier emits
/// `UnsupportedReason::UnsupportedFunction(name)` for any match.
pub(crate) static UNSUPPORTED_FUNCTIONS: Set<&'static str> = phf_set! {
    "OFFSET", "INDIRECT", "FILTER", "UNIQUE", "SORT", "SORTBY",
    "SEQUENCE", "RANDARRAY", "LAMBDA", "LET", "HYPERLINK",
    "WEBSERVICE", "CUBEVALUE", "CUBEMEMBER", "CUBESET",
    "RAND", "RANDBETWEEN",
    "CELL", "INFO", "ROWS", "COLUMNS", "AREAS", "SHEET", "SHEETS",
};

/// Dynamic-array functions (subset of unsupported). Reserved for
/// future evaluator use — classifier does not distinguish today.
pub(crate) static DYNAMIC_ARRAY_FUNCTIONS: Set<&'static str> = phf_set! {
    "FILTER", "UNIQUE", "SORT", "SORTBY", "SEQUENCE", "RANDARRAY",
};

/// Volatile functions whose per-cell semantics don't fit
/// single-evaluation-per-run (subset of unsupported). Reserved for
/// future evaluator use — classifier does not distinguish today.
pub(crate) static VOLATILE_UNSUPPORTED: Set<&'static str> = phf_set! {
    "RAND", "RANDBETWEEN",
};

/// Functions evaluable in a single column pre-pass.
pub(crate) static AGGREGATE_FUNCTIONS: Set<&'static str> = phf_set! {
    "SUM", "COUNT", "COUNTA", "COUNTBLANK", "AVERAGE", "MIN", "MAX", "PRODUCT",
    "SUMIF", "COUNTIF", "AVERAGEIF",
    "SUMIFS", "COUNTIFS", "AVERAGEIFS", "MINIFS", "MAXIFS",
    "MEDIAN",
};

/// Lookup functions allowed against pre-loaded lookup sheets.
pub(crate) static LOOKUP_FUNCTIONS: Set<&'static str> = phf_set! {
    "VLOOKUP", "HLOOKUP", "XLOOKUP", "MATCH", "XMATCH", "INDEX", "CHOOSE",
};

/// Volatile functions whose semantics fit single-evaluation-per-run
/// (the evaluator memoises in Phase 4).
pub(crate) static VOLATILE_STREAMING_OK: Set<&'static str> = phf_set! {
    "TODAY", "NOW",
};

/// `true` if `name` is in `UNSUPPORTED_FUNCTIONS` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_unsupported;
/// assert!(is_unsupported("offset"));
/// ```
#[must_use]
pub fn is_unsupported(name: &str) -> bool {
    UNSUPPORTED_FUNCTIONS.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in `AGGREGATE_FUNCTIONS` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_aggregate;
/// assert!(is_aggregate("Sum"));
/// ```
#[must_use]
pub fn is_aggregate(name: &str) -> bool {
    AGGREGATE_FUNCTIONS.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in `LOOKUP_FUNCTIONS` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_lookup;
/// assert!(is_lookup("vlookup"));
/// ```
#[must_use]
pub fn is_lookup(name: &str) -> bool {
    LOOKUP_FUNCTIONS.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in `VOLATILE_STREAMING_OK` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_volatile_streaming_ok;
/// assert!(is_volatile_streaming_ok("TODAY"));
/// ```
#[must_use]
pub fn is_volatile_streaming_ok(name: &str) -> bool {
    VOLATILE_STREAMING_OK.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in `DYNAMIC_ARRAY_FUNCTIONS` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_dynamic_array;
/// assert!(is_dynamic_array("FILTER"));
/// ```
#[must_use]
pub fn is_dynamic_array(name: &str) -> bool {
    DYNAMIC_ARRAY_FUNCTIONS.contains(name.to_uppercase().as_str())
}

/// `true` if `name` is in `VOLATILE_UNSUPPORTED` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_volatile_unsupported;
/// assert!(is_volatile_unsupported("RAND"));
/// ```
#[must_use]
pub fn is_volatile_unsupported(name: &str) -> bool {
    VOLATILE_UNSUPPORTED.contains(name.to_uppercase().as_str())
}

/// Functions whose range arguments should be expanded row-by-row
/// rather than treated as aggregates or refused outright. Only
/// bounded single-column ranges are accepted; unbounded or
/// multi-column ranges are still refused.
pub(crate) static RANGE_EXPANDING_FUNCTIONS: Set<&'static str> = phf_set! {
    "IRR", "NPV", "CONCAT", "CONCATENATE", "TEXTJOIN",
    "NETWORKDAYS", "WORKDAY", "AND", "OR",
};

/// `true` if `name` is in `RANGE_EXPANDING_FUNCTIONS` (case-insensitive).
///
/// # Examples
///
/// ```
/// use xlstream_parse::sets::is_range_expanding;
/// assert!(is_range_expanding("IRR"));
/// assert!(is_range_expanding("concat"));
/// assert!(!is_range_expanding("SUM"));
/// ```
#[must_use]
pub fn is_range_expanding(name: &str) -> bool {
    RANGE_EXPANDING_FUNCTIONS.contains(name.to_uppercase().as_str())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn aggregate_set_recognises_sum_case_insensitively() {
        assert!(is_aggregate("SUM"));
        assert!(is_aggregate("sum"));
        assert!(is_aggregate("Sum"));
    }

    #[test]
    fn unsupported_set_lists_offset_indirect_filter_rand() {
        assert!(is_unsupported("OFFSET"));
        assert!(is_unsupported("INDIRECT"));
        assert!(is_unsupported("FILTER"));
        assert!(is_unsupported("RAND"));
        assert!(is_unsupported("RANDBETWEEN"));
    }

    #[test]
    fn lookup_set_lists_vlookup_xlookup_index() {
        assert!(is_lookup("VLOOKUP"));
        assert!(is_lookup("XLOOKUP"));
        assert!(is_lookup("INDEX"));
    }

    #[test]
    fn volatile_streaming_ok_set_lists_today_now() {
        assert!(is_volatile_streaming_ok("TODAY"));
        assert!(is_volatile_streaming_ok("NOW"));
        assert!(!is_volatile_streaming_ok("RAND"));
    }

    #[test]
    fn dynamic_array_set_lists_filter_unique_sort() {
        assert!(is_dynamic_array("FILTER"));
        assert!(is_dynamic_array("UNIQUE"));
        assert!(is_dynamic_array("SORT"));
        assert!(is_dynamic_array("RANDARRAY"));
        assert!(!is_dynamic_array("SUM"));
    }

    #[test]
    fn volatile_unsupported_set_lists_rand() {
        assert!(is_volatile_unsupported("RAND"));
        assert!(is_volatile_unsupported("RANDBETWEEN"));
        assert!(!is_volatile_unsupported("TODAY"));
    }

    #[test]
    fn range_expanding_set_lists_irr_concat_networkdays() {
        assert!(is_range_expanding("IRR"));
        assert!(is_range_expanding("irr"));
        assert!(is_range_expanding("CONCAT"));
        assert!(is_range_expanding("CONCATENATE"));
        assert!(is_range_expanding("TEXTJOIN"));
        assert!(is_range_expanding("NETWORKDAYS"));
        assert!(is_range_expanding("WORKDAY"));
        assert!(is_range_expanding("AND"));
        assert!(is_range_expanding("OR"));
        assert!(is_range_expanding("NPV"));
        assert!(!is_range_expanding("SUM"));
    }

    #[test]
    #[allow(clippy::explicit_iter_loop)]
    fn all_set_entries_are_uppercase() {
        for set in [
            &UNSUPPORTED_FUNCTIONS,
            &AGGREGATE_FUNCTIONS,
            &LOOKUP_FUNCTIONS,
            &VOLATILE_STREAMING_OK,
            &DYNAMIC_ARRAY_FUNCTIONS,
            &VOLATILE_UNSUPPORTED,
            &RANGE_EXPANDING_FUNCTIONS,
        ] {
            for &entry in set.iter() {
                assert_eq!(entry, entry.to_uppercase(), "set entry {entry:?} must be uppercase");
            }
        }
    }
}
