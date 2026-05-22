//! Function-name sets used by the classifier.
//!
//! Only unsupported-function detection remains here. All other function
//! routing (aggregate, lookup, volatile, range-expanding, range-metadata)
//! is now handled by the centralized registry via [`FunctionMeta`] flags.
//!
//! [`FunctionMeta`]: crate::function_meta::FunctionMeta

use phf::{phf_set, Set};

/// Functions xlstream cannot stream. The classifier emits
/// `UnsupportedReason::UnsupportedFunction(name)` for any match.
pub(crate) static UNSUPPORTED_FUNCTIONS: Set<&'static str> = phf_set! {
    "OFFSET", "INDIRECT", "FILTER", "UNIQUE", "SORT", "SORTBY",
    "SEQUENCE", "RANDARRAY", "LAMBDA", "LET", "HYPERLINK",
    "WEBSERVICE", "CUBEVALUE", "CUBEMEMBER", "CUBESET",
    "RAND", "RANDBETWEEN",
    "CELL", "INFO", "AREAS", "SHEET", "SHEETS",
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn unsupported_set_lists_offset_indirect_filter_rand() {
        assert!(is_unsupported("OFFSET"));
        assert!(is_unsupported("INDIRECT"));
        assert!(is_unsupported("FILTER"));
        assert!(is_unsupported("RAND"));
        assert!(is_unsupported("RANDBETWEEN"));
        assert!(!is_unsupported("ROWS"));
        assert!(!is_unsupported("COLUMNS"));
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
    #[allow(clippy::explicit_iter_loop)]
    fn all_set_entries_are_uppercase() {
        for set in [&UNSUPPORTED_FUNCTIONS, &DYNAMIC_ARRAY_FUNCTIONS, &VOLATILE_UNSUPPORTED] {
            for &entry in set.iter() {
                assert_eq!(entry, entry.to_uppercase(), "set entry {entry:?} must be uppercase");
            }
        }
    }
}
