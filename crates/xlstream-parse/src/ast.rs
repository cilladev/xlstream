//! The [`Ast`] placeholder.

/// Parsed-formula AST. Phase 1 ships this as an opaque placeholder; Phase 2
/// replaces the internals with a wrapper around `formualizer_parse::Ast`.
///
/// # Examples
///
/// ```
/// use xlstream_parse::Ast;
/// let a = Ast::default();
/// let b = Ast::default();
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ast {
    // Intentionally private and empty in Phase 1. Phase 2 adds the
    // upstream-AST field.
    _private: (),
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn default_constructs_empty_placeholder() {
        let a = Ast::default();
        let b = Ast::default();
        assert_eq!(a, b);
    }
}
