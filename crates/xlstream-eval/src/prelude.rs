//! [`Prelude`] — data computed before the row-streaming pass.
//!
//! Phase 7 adds aggregate scalars. Phase 8 adds lookup indexes. Phase 9
//! adds volatile data (TODAY/NOW).

use std::collections::HashMap;

use xlstream_core::{CellError, ExcelDate, Value};
use xlstream_parse::{AggKind, AggregateKey};

/// Runtime-injected volatile values for `TODAY()` and `NOW()`.
///
/// The caller constructs this once per workbook evaluation, freezing the
/// clock so every row sees the same instant.
///
/// # Examples
///
/// ```
/// use xlstream_core::ExcelDate;
/// use xlstream_eval::prelude::VolatileData;
///
/// let v = VolatileData {
///     today: ExcelDate::from_serial(46130.0),
///     now: ExcelDate::from_serial(46130.75),
/// };
/// assert_eq!(v.today.serial, 46130.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VolatileData {
    /// Date-only serial for `TODAY()`.
    pub today: ExcelDate,
    /// Date+time serial for `NOW()`.
    pub now: ExcelDate,
}

/// Key for a conditional aggregate (SUMIF, COUNTIF, AVERAGEIF) prelude
/// entry.
///
/// The prelude builds a `HashMap<String, Value>` for each
/// `ConditionalAggKey`, mapping lowercased criteria values to
/// pre-computed results.
///
/// # Examples
///
/// ```
/// use xlstream_parse::AggKind;
/// use xlstream_eval::prelude::ConditionalAggKey;
///
/// let key = ConditionalAggKey {
///     kind: AggKind::Sum,
///     criteria_col: 1,
///     sum_col: 2,
///     sheet: None,
/// };
/// assert_eq!(key.kind, AggKind::Sum);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalAggKey {
    /// The aggregate function (Sum for SUMIF, Count for COUNTIF, Average
    /// for AVERAGEIF).
    pub kind: AggKind,
    /// 1-based column index for the criteria range.
    pub criteria_col: u32,
    /// 1-based column index for the sum/count/average range.
    pub sum_col: u32,
    /// Sheet name (`None` = current streaming sheet).
    pub sheet: Option<String>,
}

/// Key for a multi-criteria conditional aggregate (SUMIFS, COUNTIFS,
/// AVERAGEIFS) prelude entry.
///
/// The prelude builds a `HashMap<String, Value>` for each key, where
/// the `String` is the concatenation of all lowercased criteria values
/// joined by `\0`.
///
/// # Examples
///
/// ```
/// use xlstream_parse::AggKind;
/// use xlstream_eval::prelude::MultiConditionalAggKey;
///
/// let key = MultiConditionalAggKey {
///     kind: AggKind::Sum,
///     sum_col: 3,
///     criteria_cols: vec![1, 2],
///     sheet: None,
/// };
/// assert_eq!(key.criteria_cols.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiConditionalAggKey {
    /// The aggregate function (Sum for SUMIFS, Count for COUNTIFS, Average
    /// for AVERAGEIFS).
    pub kind: AggKind,
    /// 1-based column index for the sum/count/average range.
    pub sum_col: u32,
    /// 1-based column indices for the criteria ranges, in order.
    pub criteria_cols: Vec<u32>,
    /// Sheet name (`None` = current streaming sheet).
    pub sheet: Option<String>,
}

/// Key for a bounded single-column range that must be cached during prelude.
///
/// Range-expanding functions (IRR, NPV, CONCAT, TEXTJOIN, NETWORKDAYS,
/// WORKDAY, AND, OR) may reference bounded ranges like `A2:A100` on the
/// main sheet. The prelude scans those rows once and stores the collected
/// values so the row-pass evaluator can expand them without re-reading.
///
/// # Examples
///
/// ```
/// use xlstream_eval::prelude::BoundedRangeKey;
///
/// let key = BoundedRangeKey {
///     sheet: None,
///     col: 1,
///     start_row: 2,
///     end_row: 100,
/// };
/// assert_eq!(key.col, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundedRangeKey {
    /// Sheet name (`None` = main streaming sheet).
    pub sheet: Option<String>,
    /// 1-based column index.
    pub col: u32,
    /// 1-based start row (inclusive).
    pub start_row: u32,
    /// 1-based end row (inclusive).
    pub end_row: u32,
}

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
    /// Aggregate scalars keyed by `AggregateKey`.
    aggregates: HashMap<AggregateKey, Value>,
    /// Conditional aggregate results keyed by `ConditionalAggKey`, with
    /// inner maps from lowercased criteria value to result.
    conditional_aggregates: HashMap<ConditionalAggKey, HashMap<String, Value>>,
    /// Multi-criteria conditional aggregate results keyed by
    /// `MultiConditionalAggKey`, with inner maps from composite key
    /// (lowercased criteria values joined by `\0`) to result.
    multi_conditional_aggregates: HashMap<MultiConditionalAggKey, HashMap<String, Value>>,
    /// Pre-loaded lookup sheet data, keyed by lowercased sheet name.
    lookup_sheets: HashMap<String, crate::lookup::LookupSheet>,
    /// Volatile data (TODAY/NOW). `None` until set via `with_volatile`.
    volatile: Option<VolatileData>,
    /// Cached bounded ranges for range-expanding functions.
    cached_ranges: HashMap<BoundedRangeKey, Vec<Value>>,
}

impl Prelude {
    /// Build an empty prelude. Used in tests and as the default when no
    /// aggregates or lookups are needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// let _ = Prelude::empty();
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self {
            aggregates: HashMap::new(),
            conditional_aggregates: HashMap::new(),
            multi_conditional_aggregates: HashMap::new(),
            lookup_sheets: HashMap::new(),
            volatile: None,
            cached_ranges: HashMap::new(),
        }
    }

    /// Build a prelude with aggregate scalars.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_parse::{AggKind, AggregateKey};
    /// use xlstream_eval::Prelude;
    ///
    /// let mut aggs = HashMap::new();
    /// aggs.insert(
    ///     AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 },
    ///     Value::Number(100.0),
    /// );
    /// let prelude = Prelude::with_aggregates(aggs);
    /// ```
    #[must_use]
    pub fn with_aggregates(aggregates: HashMap<AggregateKey, Value>) -> Self {
        Self {
            aggregates,
            conditional_aggregates: HashMap::new(),
            multi_conditional_aggregates: HashMap::new(),
            lookup_sheets: HashMap::new(),
            volatile: None,
            cached_ranges: HashMap::new(),
        }
    }

    /// Build a prelude with both simple and conditional aggregates.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_parse::{AggKind, AggregateKey};
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::prelude::ConditionalAggKey;
    ///
    /// let aggs = HashMap::new();
    /// let cond = HashMap::new();
    /// let prelude = Prelude::with_conditional(aggs, cond);
    /// ```
    #[must_use]
    pub fn with_conditional(
        aggregates: HashMap<AggregateKey, Value>,
        conditional_aggregates: HashMap<ConditionalAggKey, HashMap<String, Value>>,
    ) -> Self {
        Self {
            aggregates,
            conditional_aggregates,
            multi_conditional_aggregates: HashMap::new(),
            lookup_sheets: HashMap::new(),
            volatile: None,
            cached_ranges: HashMap::new(),
        }
    }

    /// Build a prelude with simple, single-criteria conditional, and
    /// multi-criteria conditional aggregates.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_eval::Prelude;
    ///
    /// let prelude = Prelude::with_all(HashMap::new(), HashMap::new(), HashMap::new());
    /// ```
    #[must_use]
    pub fn with_all(
        aggregates: HashMap<AggregateKey, Value>,
        conditional_aggregates: HashMap<ConditionalAggKey, HashMap<String, Value>>,
        multi_conditional_aggregates: HashMap<MultiConditionalAggKey, HashMap<String, Value>>,
    ) -> Self {
        Self {
            aggregates,
            conditional_aggregates,
            multi_conditional_aggregates,
            lookup_sheets: HashMap::new(),
            volatile: None,
            cached_ranges: HashMap::new(),
        }
    }

    /// Attach pre-loaded lookup sheets. Keys must be lowercased.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::lookup::LookupSheet;
    ///
    /// let mut sheets = HashMap::new();
    /// sheets.insert("data".to_string(), LookupSheet::new(vec![vec![Value::Number(1.0)]]));
    /// let p = Prelude::empty().with_lookup_sheets(sheets);
    /// assert!(p.lookup_sheet("data").is_some());
    /// ```
    #[must_use]
    pub fn with_lookup_sheets(
        mut self,
        lookup_sheets: HashMap<String, crate::lookup::LookupSheet>,
    ) -> Self {
        self.lookup_sheets = lookup_sheets;
        self
    }

    /// Look up a pre-loaded sheet by name (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// let p = Prelude::empty();
    /// assert!(p.lookup_sheet("Sheet1").is_none());
    /// ```
    #[must_use]
    pub fn lookup_sheet(&self, name: &str) -> Option<&crate::lookup::LookupSheet> {
        self.lookup_sheets.get(&name.to_ascii_lowercase())
    }

    /// Look up a simple aggregate by key. Returns `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_parse::{AggKind, AggregateKey};
    /// use xlstream_eval::Prelude;
    ///
    /// let mut aggs = HashMap::new();
    /// let key = AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 };
    /// aggs.insert(key.clone(), Value::Number(42.0));
    /// let prelude = Prelude::with_aggregates(aggs);
    /// assert_eq!(prelude.get_aggregate(&key), Some(&Value::Number(42.0)));
    /// ```
    #[must_use]
    pub fn get_aggregate(&self, key: &AggregateKey) -> Option<&Value> {
        self.aggregates.get(key)
    }

    /// Look up a conditional aggregate by key and criteria value.
    ///
    /// The `criteria_value` is lowercased before lookup. Missing keys
    /// return `Value::Number(0.0)` for Sum/Count types and
    /// `Value::Error(CellError::Div0)` for Average.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_parse::AggKind;
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::prelude::ConditionalAggKey;
    ///
    /// let key = ConditionalAggKey {
    ///     kind: AggKind::Sum,
    ///     criteria_col: 1,
    ///     sum_col: 2,
    ///     sheet: None,
    /// };
    /// let mut inner = HashMap::new();
    /// inner.insert("east".to_string(), Value::Number(100.0));
    /// let mut cond = HashMap::new();
    /// cond.insert(key.clone(), inner);
    /// let prelude = Prelude::with_conditional(HashMap::new(), cond);
    /// assert_eq!(prelude.get_conditional(&key, "East"), Value::Number(100.0));
    /// assert_eq!(prelude.get_conditional(&key, "West"), Value::Number(0.0));
    /// ```
    #[must_use]
    pub fn get_conditional(&self, key: &ConditionalAggKey, criteria_value: &str) -> Value {
        let lowered = criteria_value.to_ascii_lowercase();
        if let Some(inner) = self.conditional_aggregates.get(key) {
            if let Some(v) = inner.get(&lowered) {
                return v.clone();
            }
        }
        // Missing: return default based on kind
        match key.kind {
            AggKind::Average => Value::Error(CellError::Div0),
            AggKind::Sum
            | AggKind::Count
            | AggKind::CountA
            | AggKind::CountBlank
            | AggKind::Min
            | AggKind::Max
            | AggKind::Product
            | AggKind::Median => Value::Number(0.0),
        }
    }

    /// Look up a multi-criteria conditional aggregate by key and composite
    /// criteria value.
    ///
    /// `composite_key` should be the lowercased criteria values joined by
    /// `\0`. Missing keys return `Value::Number(0.0)` for Sum/Count types
    /// and `Value::Error(CellError::Div0)` for Average.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_parse::AggKind;
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::prelude::MultiConditionalAggKey;
    ///
    /// let key = MultiConditionalAggKey {
    ///     kind: AggKind::Sum,
    ///     sum_col: 3,
    ///     criteria_cols: vec![1, 2],
    ///     sheet: None,
    /// };
    /// let mut inner = HashMap::new();
    /// inner.insert("east\0q1".to_string(), Value::Number(100.0));
    /// let mut multi = HashMap::new();
    /// multi.insert(key.clone(), inner);
    /// let prelude = Prelude::with_all(HashMap::new(), HashMap::new(), multi);
    /// assert_eq!(prelude.get_multi_conditional(&key, "east\0q1"), Value::Number(100.0));
    /// assert_eq!(prelude.get_multi_conditional(&key, "west\0q1"), Value::Number(0.0));
    /// ```
    #[must_use]
    pub fn get_multi_conditional(
        &self,
        key: &MultiConditionalAggKey,
        composite_key: &str,
    ) -> Value {
        if let Some(inner) = self.multi_conditional_aggregates.get(key) {
            if let Some(v) = inner.get(composite_key) {
                return v.clone();
            }
        }
        match key.kind {
            AggKind::Average => Value::Error(CellError::Div0),
            AggKind::Sum
            | AggKind::Count
            | AggKind::CountA
            | AggKind::CountBlank
            | AggKind::Min
            | AggKind::Max
            | AggKind::Product
            | AggKind::Median => Value::Number(0.0),
        }
    }

    /// Attach volatile data (TODAY/NOW). Builder-style.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::ExcelDate;
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::prelude::VolatileData;
    ///
    /// let v = VolatileData {
    ///     today: ExcelDate::from_serial(46130.0),
    ///     now: ExcelDate::from_serial(46130.75),
    /// };
    /// let p = Prelude::empty().with_volatile(v);
    /// assert_eq!(p.volatile_today().serial, 46130.0);
    /// ```
    #[must_use]
    pub fn with_volatile(mut self, volatile: VolatileData) -> Self {
        self.volatile = Some(volatile);
        self
    }

    /// Attach pre-collected bounded range values. Builder-style.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use xlstream_core::Value;
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::prelude::BoundedRangeKey;
    ///
    /// let key = BoundedRangeKey { sheet: None, col: 1, start_row: 2, end_row: 4 };
    /// let values = vec![Value::Number(10.0), Value::Number(20.0), Value::Number(30.0)];
    /// let mut ranges = HashMap::new();
    /// ranges.insert(key.clone(), values);
    /// let p = Prelude::empty().with_cached_ranges(ranges);
    /// assert_eq!(p.get_cached_range(&key).unwrap().len(), 3);
    /// ```
    #[must_use]
    pub fn with_cached_ranges(
        mut self,
        cached_ranges: HashMap<BoundedRangeKey, Vec<Value>>,
    ) -> Self {
        self.cached_ranges = cached_ranges;
        self
    }

    /// Look up a cached bounded range by key.
    ///
    /// Returns `None` if the range was not collected during prelude.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// use xlstream_eval::prelude::BoundedRangeKey;
    ///
    /// let key = BoundedRangeKey { sheet: None, col: 1, start_row: 2, end_row: 4 };
    /// let p = Prelude::empty();
    /// assert!(p.get_cached_range(&key).is_none());
    /// ```
    #[must_use]
    pub fn get_cached_range(&self, key: &BoundedRangeKey) -> Option<&Vec<Value>> {
        self.cached_ranges.get(key)
    }

    /// Return the `TODAY()` serial. Logs a warning and returns serial 0
    /// if volatile data was never configured via [`with_volatile`].
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// let p = Prelude::empty();
    /// assert_eq!(p.volatile_today().serial, 0.0);
    /// ```
    #[must_use]
    pub fn volatile_today(&self) -> ExcelDate {
        if let Some(v) = self.volatile {
            v.today
        } else {
            tracing::warn!("TODAY() called but volatile data not configured");
            ExcelDate::from_serial(0.0)
        }
    }

    /// Return the `NOW()` serial. Logs a warning and returns serial 0
    /// if volatile data was never configured via [`with_volatile`].
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// let p = Prelude::empty();
    /// assert_eq!(p.volatile_now().serial, 0.0);
    /// ```
    #[must_use]
    pub fn volatile_now(&self) -> ExcelDate {
        if let Some(v) = self.volatile {
            v.now
        } else {
            tracing::warn!("NOW() called but volatile data not configured");
            ExcelDate::from_serial(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    #[test]
    fn empty_prelude_constructs() {
        let _ = Prelude::empty();
    }

    #[test]
    fn get_aggregate_returns_stored_value() {
        let mut aggs = HashMap::new();
        let key = AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 };
        aggs.insert(key.clone(), Value::Number(42.0));
        let prelude = Prelude::with_aggregates(aggs);
        assert_eq!(prelude.get_aggregate(&key), Some(&Value::Number(42.0)));
    }

    #[test]
    fn get_aggregate_missing_returns_none() {
        let prelude = Prelude::empty();
        let key = AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 };
        assert_eq!(prelude.get_aggregate(&key), None);
    }

    #[test]
    fn get_conditional_returns_stored_value() {
        let key =
            ConditionalAggKey { kind: AggKind::Sum, criteria_col: 1, sum_col: 2, sheet: None };
        let mut inner = HashMap::new();
        inner.insert("east".to_string(), Value::Number(100.0));
        let mut cond = HashMap::new();
        cond.insert(key.clone(), inner);
        let prelude = Prelude::with_conditional(HashMap::new(), cond);
        assert_eq!(prelude.get_conditional(&key, "East"), Value::Number(100.0));
    }

    #[test]
    fn get_conditional_case_insensitive_lookup() {
        let key =
            ConditionalAggKey { kind: AggKind::Sum, criteria_col: 1, sum_col: 2, sheet: None };
        let mut inner = HashMap::new();
        inner.insert("west".to_string(), Value::Number(50.0));
        let mut cond = HashMap::new();
        cond.insert(key.clone(), inner);
        let prelude = Prelude::with_conditional(HashMap::new(), cond);
        assert_eq!(prelude.get_conditional(&key, "WEST"), Value::Number(50.0));
        assert_eq!(prelude.get_conditional(&key, "west"), Value::Number(50.0));
    }

    #[test]
    fn get_conditional_missing_sum_returns_zero() {
        let key =
            ConditionalAggKey { kind: AggKind::Sum, criteria_col: 1, sum_col: 2, sheet: None };
        let prelude = Prelude::empty();
        assert_eq!(prelude.get_conditional(&key, "missing"), Value::Number(0.0));
    }

    #[test]
    fn get_conditional_missing_average_returns_div0() {
        let key =
            ConditionalAggKey { kind: AggKind::Average, criteria_col: 1, sum_col: 2, sheet: None };
        let prelude = Prelude::empty();
        assert_eq!(prelude.get_conditional(&key, "missing"), Value::Error(CellError::Div0));
    }

    #[test]
    fn with_conditional_stores_both() {
        let agg_key = AggregateKey { kind: AggKind::Max, sheet: None, column: 3 };
        let mut aggs = HashMap::new();
        aggs.insert(agg_key.clone(), Value::Number(99.0));

        let cond_key =
            ConditionalAggKey { kind: AggKind::Sum, criteria_col: 1, sum_col: 2, sheet: None };
        let mut inner = HashMap::new();
        inner.insert("a".to_string(), Value::Number(10.0));
        let mut cond = HashMap::new();
        cond.insert(cond_key.clone(), inner);

        let prelude = Prelude::with_conditional(aggs, cond);
        assert_eq!(prelude.get_aggregate(&agg_key), Some(&Value::Number(99.0)));
        assert_eq!(prelude.get_conditional(&cond_key, "a"), Value::Number(10.0));
    }

    #[test]
    fn get_multi_conditional_returns_stored_value() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
        };
        let mut inner = HashMap::new();
        inner.insert("east\0q1".to_string(), Value::Number(200.0));
        let mut multi = HashMap::new();
        multi.insert(key.clone(), inner);
        let prelude = Prelude::with_all(HashMap::new(), HashMap::new(), multi);
        assert_eq!(prelude.get_multi_conditional(&key, "east\0q1"), Value::Number(200.0));
    }

    #[test]
    fn get_multi_conditional_missing_returns_zero() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Sum,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
        };
        let prelude = Prelude::empty();
        assert_eq!(prelude.get_multi_conditional(&key, "east\0q1"), Value::Number(0.0));
    }

    #[test]
    fn get_multi_conditional_missing_average_returns_div0() {
        let key = MultiConditionalAggKey {
            kind: AggKind::Average,
            sum_col: 3,
            criteria_cols: vec![1, 2],
            sheet: None,
        };
        let prelude = Prelude::empty();
        assert_eq!(prelude.get_multi_conditional(&key, "east\0q1"), Value::Error(CellError::Div0));
    }

    #[test]
    fn volatile_today_returns_zero_when_unset() {
        let p = Prelude::empty();
        assert_eq!(p.volatile_today().serial, 0.0);
    }

    #[test]
    fn volatile_now_returns_zero_when_unset() {
        let p = Prelude::empty();
        assert_eq!(p.volatile_now().serial, 0.0);
    }

    #[test]
    fn volatile_today_returns_set_value() {
        use xlstream_core::ExcelDate;

        let v = VolatileData {
            today: ExcelDate::from_serial(46130.0),
            now: ExcelDate::from_serial(46130.75),
        };
        let p = Prelude::empty().with_volatile(v);
        assert_eq!(p.volatile_today().serial, 46130.0);
    }

    #[test]
    fn volatile_now_returns_set_value() {
        use xlstream_core::ExcelDate;

        let v = VolatileData {
            today: ExcelDate::from_serial(46130.0),
            now: ExcelDate::from_serial(46130.75),
        };
        let p = Prelude::empty().with_volatile(v);
        assert_eq!(p.volatile_now().serial, 46130.75);
    }

    #[test]
    fn with_volatile_is_builder_pattern() {
        use xlstream_core::ExcelDate;

        let v = VolatileData {
            today: ExcelDate::from_serial(100.0),
            now: ExcelDate::from_serial(100.5),
        };
        // Chaining works
        let p = Prelude::empty().with_volatile(v);
        assert_eq!(p.volatile_today().serial, 100.0);
        assert_eq!(p.volatile_now().serial, 100.5);
    }
}
