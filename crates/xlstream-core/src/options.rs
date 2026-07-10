//! Evaluation options controlling parallelism and iterative calculation.

/// Default maximum iterations for iterative calculation (matches Excel).
pub const ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS: u32 = 100;
/// Default convergence threshold for iterative calculation (matches Excel).
pub const ITERATIVE_CALC_DEFAULT_MAX_CHANGE: f64 = 0.001;

/// Controls what is written to formula cells in the output xlsx.
///
/// # Examples
///
/// ```
/// use xlstream_core::OutputMode;
/// assert_eq!(OutputMode::default(), OutputMode::Formulas);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputMode {
    /// Write both formula text and cached values: `<f>formula</f><v>cached</v>`.
    #[default]
    Formulas,
    /// Write only cached values: `<v>cached</v>`.
    ValuesOnly,
}

impl OutputMode {
    /// Whether this mode omits formula text from the output.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::OutputMode;
    /// assert!(!OutputMode::Formulas.is_values_only());
    /// assert!(OutputMode::ValuesOnly.is_values_only());
    /// ```
    #[must_use]
    pub fn is_values_only(self) -> bool {
        matches!(self, Self::ValuesOnly)
    }
}

/// Options controlling formula evaluation behavior.
///
/// # Examples
///
/// ```
/// use xlstream_core::{EvaluateOptions, OutputMode};
/// let opts = EvaluateOptions::default();
/// assert!(opts.iterative_calc);
/// assert_eq!(opts.max_iterations, 100);
/// assert_eq!(opts.output_mode, OutputMode::Formulas);
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
    /// What to write in formula cells.
    pub output_mode: OutputMode,
}

impl Default for EvaluateOptions {
    fn default() -> Self {
        Self {
            workers: None,
            iterative_calc: true,
            max_iterations: ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS,
            max_change: ITERATIVE_CALC_DEFAULT_MAX_CHANGE,
            output_mode: OutputMode::default(),
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
    fn default_output_mode_is_formulas() {
        let opts = EvaluateOptions::default();
        assert_eq!(opts.output_mode, OutputMode::Formulas);
    }

    #[test]
    fn output_mode_is_values_only() {
        assert!(!OutputMode::Formulas.is_values_only());
        assert!(OutputMode::ValuesOnly.is_values_only());
    }

    #[test]
    fn constants_match_default_options() {
        let opts = EvaluateOptions::default();
        assert_eq!(opts.max_iterations, ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS);
        assert!((opts.max_change - ITERATIVE_CALC_DEFAULT_MAX_CHANGE).abs() < f64::EPSILON);
    }
}
