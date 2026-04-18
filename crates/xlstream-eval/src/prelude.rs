//! [`Prelude`] — data computed before the row-streaming pass.
//!
//! Empty in Phase 4. Phase 7 adds aggregate scalars, Phase 8 adds lookup
//! indexes.

/// Prelude data computed before the row-streaming pass.
///
/// Constructed once after the prelude pass (pass 1) completes, then shared
/// immutably with every row evaluation during pass 2.
///
/// # Examples
///
/// ```
/// use xlstream_eval::Prelude;
/// let p = Prelude::empty();
/// drop(p);
/// ```
pub struct Prelude {
    _private: (),
}

impl Prelude {
    /// Build an empty prelude. Used in Phase 4 tests and as the default
    /// when no aggregates or lookups are needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// let _ = Prelude::empty();
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn empty_prelude_constructs() {
        let _ = Prelude::empty();
    }
}
