//! Evaluation options controlling parallelism and iterative calculation.

/// Default maximum iterations for iterative calculation (matches Excel).
pub const ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS: u32 = 100;
/// Default convergence threshold for iterative calculation (matches Excel).
pub const ITERATIVE_CALC_DEFAULT_MAX_CHANGE: f64 = 0.001;

/// Options controlling formula evaluation behavior.
///
/// # Examples
///
/// ```
/// use xlstream_core::EvaluateOptions;
/// let opts = EvaluateOptions::default();
/// assert!(opts.iterative_calc);
/// assert_eq!(opts.max_iterations, 100);
/// ```
#[derive(Debug, Clone)]
pub struct EvaluateOptions {
    /// Number of parallel workers. `None` = auto-detect via `num_cpus`.
    pub workers: Option<usize>,
    /// Enable iterative calculation for self-referential formulas.
    pub iterative_calc: bool,
    /// Maximum iterations before stopping (only when `iterative_calc` is true).
    pub max_iterations: u32,
    /// Convergence threshold — stop when delta < this (only for numeric results).
    pub max_change: f64,
}

impl Default for EvaluateOptions {
    fn default() -> Self {
        Self {
            workers: None,
            iterative_calc: true,
            max_iterations: ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS,
            max_change: ITERATIVE_CALC_DEFAULT_MAX_CHANGE,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    use super::*;

    #[test]
    fn default_options_match_excel_defaults() {
        let opts = EvaluateOptions::default();
        assert!(opts.iterative_calc);
        assert_eq!(opts.max_iterations, 100);
        assert!((opts.max_change - 0.001).abs() < f64::EPSILON);
        assert!(opts.workers.is_none());
    }

    #[test]
    fn constants_match_default_options() {
        let opts = EvaluateOptions::default();
        assert_eq!(opts.max_iterations, ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS);
        assert!((opts.max_change - ITERATIVE_CALC_DEFAULT_MAX_CHANGE).abs() < f64::EPSILON);
    }
}
