//! Function-name sets used by the classifier.
//!
//! Sets are stored in upper-case; lookups normalise the incoming name to
//! upper-case to give Excel-style case-insensitivity.

use phf::{phf_set, Set};

/// Functions xlstream cannot stream.
pub static UNSUPPORTED_FUNCTIONS: Set<&'static str> = phf_set! {
    "OFFSET", "INDIRECT", "FILTER", "UNIQUE", "SORT", "SORTBY",
    "SEQUENCE", "RANDARRAY", "LAMBDA", "LET", "HYPERLINK",
    "WEBSERVICE", "CUBEVALUE", "CUBEMEMBER", "CUBESET",
    "RAND", "RANDBETWEEN",
};

/// Functions evaluable in a single column pre-pass.
pub static AGGREGATE_FUNCTIONS: Set<&'static str> = phf_set! {
    "SUM", "COUNT", "COUNTA", "AVERAGE", "MIN", "MAX", "PRODUCT",
    "SUMIF", "COUNTIF", "AVERAGEIF",
    "SUMIFS", "COUNTIFS", "AVERAGEIFS", "MINIFS", "MAXIFS",
    "MEDIAN",
};

/// Lookup functions allowed against pre-loaded lookup sheets.
pub static LOOKUP_FUNCTIONS: Set<&'static str> = phf_set! {
    "VLOOKUP", "HLOOKUP", "XLOOKUP", "MATCH", "XMATCH", "INDEX", "CHOOSE",
};

/// Volatile functions whose semantics fit single-evaluation-per-run
/// (the evaluator memoises in Phase 4).
pub static VOLATILE_STREAMING_OK: Set<&'static str> = phf_set! {
    "TODAY", "NOW",
};

/// `true` if `name` is in [`UNSUPPORTED_FUNCTIONS`] (case-insensitive).
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

/// `true` if `name` is in [`AGGREGATE_FUNCTIONS`] (case-insensitive).
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

/// `true` if `name` is in [`LOOKUP_FUNCTIONS`] (case-insensitive).
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

/// `true` if `name` is in [`VOLATILE_STREAMING_OK`] (case-insensitive).
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
}
