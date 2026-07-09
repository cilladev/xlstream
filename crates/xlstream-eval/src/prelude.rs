//! [`Prelude`] — data computed before the row-streaming pass.
//!
//! Phase 7 adds aggregate scalars. Phase 8 adds lookup indexes. Phase 9
//! adds volatile data (TODAY/NOW).

use std::collections::HashMap;
use std::sync::RwLock;

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
///     start_row: None,
///     end_row: None,
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
    /// First Excel row (1-based, inclusive) of the ranges. `None` for
    /// whole-column ranges. All ranges of one formula share these bounds;
    /// incongruent ranges are rejected before a key is built.
    pub start_row: Option<u32>,
    /// Last Excel row (1-based, inclusive) of the ranges. `None` for
    /// whole-column ranges.
    pub end_row: Option<u32>,
}

/// Partial per-bucket accumulator for multi-conditional aggregates.
///
/// Buckets store partials, not finished values, so operator-criteria
/// lookups (`COUNTIF(B:B,">500")`) can merge buckets exactly: averages
/// stay row-weighted instead of averaging per-bucket averages, and
/// empty buckets contribute nothing to MIN/MAX instead of a poisoned
/// `0.0` or `#DIV/0!`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct BucketAgg {
    /// Sum of numeric values fed to the bucket. COUNTIFS-style buckets
    /// feed `1.0` per matching row, so this doubles as the row count.
    pub(crate) sum: f64,
    /// Count of numeric values fed.
    pub(crate) count: u64,
    /// Minimum numeric value fed, `None` if none.
    pub(crate) min: Option<f64>,
    /// Maximum numeric value fed, `None` if none.
    pub(crate) max: Option<f64>,
    /// First error fed. Propagates through [`BucketAgg::finish`].
    pub(crate) error: Option<CellError>,
}

impl BucketAgg {
    /// Fold another bucket's partials into this one.
    fn merge(&mut self, other: &BucketAgg) {
        self.sum += other.sum;
        self.count += other.count;
        self.min = match (self.min, other.min) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        self.max = match (self.max, other.max) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (a, b) => a.or(b),
        };
        if self.error.is_none() {
            self.error = other.error;
        }
    }

    /// Finish the partial for the given key kind.
    ///
    /// COUNTIFS-style buckets feed `1.0` per row, so `Count` finishes as
    /// the sum. Only Sum, Count, Average, Min, and Max keys are ever
    /// built for multi-conditional aggregates; other kinds finish as 0.
    fn finish(&self, kind: AggKind) -> Value {
        if let Some(e) = self.error {
            return Value::Error(e);
        }
        match kind {
            AggKind::Count | AggKind::Sum => Value::Number(self.sum),
            AggKind::Average => {
                if self.count == 0 {
                    return Value::Error(CellError::Div0);
                }
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.sum / self.count as f64)
            }
            AggKind::Min => Value::Number(self.min.unwrap_or(0.0)),
            AggKind::Max => Value::Number(self.max.unwrap_or(0.0)),
            AggKind::CountA | AggKind::CountBlank | AggKind::Product | AggKind::Median => {
                Value::Number(0.0)
            }
        }
    }

    /// Wrap an already-finished value as a single-row bucket.
    ///
    /// Compatibility shim for [`Prelude::with_all`], which accepts inner
    /// maps of finished values (test fixtures build preludes that way).
    fn from_value(v: &Value) -> Self {
        match v {
            Value::Number(n) => {
                Self { sum: *n, count: 1, min: Some(*n), max: Some(*n), error: None }
            }
            Value::Error(e) => Self { error: Some(*e), ..Self::default() },
            _ => Self::default(),
        }
    }
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
    /// Multi-criteria conditional aggregate partials keyed by
    /// `MultiConditionalAggKey`, with inner maps from composite key
    /// (lowercased criteria values joined by `\0`) to bucket partials.
    /// Finished at lookup time by [`BucketAgg::finish`].
    multi_conditional_aggregates: HashMap<MultiConditionalAggKey, HashMap<String, BucketAgg>>,
    /// Pre-loaded lookup sheet data, keyed by lowercased sheet name.
    lookup_sheets: HashMap<String, crate::lookup::LookupSheet>,
    /// Volatile data (TODAY/NOW). `None` until set via `with_volatile`.
    volatile: Option<VolatileData>,
    /// Cached bounded ranges for range-expanding functions.
    cached_ranges: HashMap<BoundedRangeKey, Vec<Value>>,
    /// Lazily populated cache for operator criteria lookups (e.g.,
    /// `COUNTIF(B:B,">500")`). Avoids re-scanning the inner map on every
    /// row — the scan runs once per unique (key, composite) pair, then
    /// all subsequent calls hit this cache.
    operator_cache: RwLock<HashMap<(MultiConditionalAggKey, String), Value>>,
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
            operator_cache: RwLock::new(HashMap::new()),
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
    ///     AggregateKey { kind: AggKind::Sum, sheet: None, column: 1, start_row: None, end_row: None },
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
            operator_cache: RwLock::new(HashMap::new()),
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
            operator_cache: RwLock::new(HashMap::new()),
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
        let multi_buckets = multi_conditional_aggregates
            .into_iter()
            .map(|(key, inner)| {
                let buckets =
                    inner.iter().map(|(c, v)| (c.clone(), BucketAgg::from_value(v))).collect();
                (key, buckets)
            })
            .collect();
        Self {
            aggregates,
            conditional_aggregates,
            multi_conditional_aggregates: multi_buckets,
            lookup_sheets: HashMap::new(),
            volatile: None,
            cached_ranges: HashMap::new(),
            operator_cache: RwLock::new(HashMap::new()),
        }
    }

    /// Build a prelude with simple aggregates and multi-conditional
    /// bucket partials. This is the prelude-plan path: partials survive
    /// to lookup time so operator criteria can merge buckets exactly.
    #[must_use]
    pub(crate) fn with_multi_buckets(
        aggregates: HashMap<AggregateKey, Value>,
        multi_conditional_aggregates: HashMap<MultiConditionalAggKey, HashMap<String, BucketAgg>>,
    ) -> Self {
        Self {
            aggregates,
            conditional_aggregates: HashMap::new(),
            multi_conditional_aggregates,
            lookup_sheets: HashMap::new(),
            volatile: None,
            cached_ranges: HashMap::new(),
            operator_cache: RwLock::new(HashMap::new()),
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

    /// Merge another prelude into this one.
    ///
    /// Extends all inner maps. Duplicate keys are overwritten by `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Prelude;
    /// let mut a = Prelude::empty();
    /// let b = Prelude::empty();
    /// a.merge(b);
    /// ```
    pub fn merge(&mut self, other: Self) {
        self.aggregates.extend(other.aggregates);
        self.conditional_aggregates.extend(other.conditional_aggregates);
        self.multi_conditional_aggregates.extend(other.multi_conditional_aggregates);
        self.lookup_sheets.extend(other.lookup_sheets);
        if other.volatile.is_some() {
            self.volatile = other.volatile;
        }
        self.cached_ranges.extend(other.cached_ranges);
        if let Ok(other_cache) = other.operator_cache.into_inner() {
            if let Ok(ref mut self_cache) = self.operator_cache.write() {
                self_cache.extend(other_cache);
            }
        }
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
    /// let key = AggregateKey { kind: AggKind::Sum, sheet: None, column: 1, start_row: None, end_row: None };
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
    ///     sheet: None, start_row: None, end_row: None,
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
            if let Some(bucket) = inner.get(composite_key) {
                return bucket.finish(key.kind);
            }

            let cache_key = (key.clone(), composite_key.to_string());

            if let Ok(cache) = self.operator_cache.read() {
                if let Some(v) = cache.get(&cache_key) {
                    return v.clone();
                }
            }

            let result = try_operator_criteria(key, inner, composite_key);
            if let Ok(mut cache) = self.operator_cache.write() {
                return cache.entry(cache_key).or_insert(result).clone();
            }
            return result;
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
    /// if volatile data was never configured via `with_volatile`.
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
    /// if volatile data was never configured via `with_volatile`.
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

fn try_operator_criteria(
    key: &MultiConditionalAggKey,
    inner: &HashMap<String, BucketAgg>,
    composite_key: &str,
) -> Value {
    let parts: Vec<&str> = composite_key.split('\0').collect();
    let criteria: Vec<crate::Criteria> = parts.iter().map(|p| crate::Criteria::parse(p)).collect();
    let mut merged = BucketAgg::default();
    for (stored_key, bucket) in inner {
        let stored_parts: Vec<&str> = stored_key.split('\0').collect();
        if stored_parts.len() != criteria.len() {
            continue;
        }
        let all_match = stored_parts.iter().zip(&criteria).all(|(sp, crit)| {
            let val = crate::criteria::parse_criteria_value(sp);
            crit.matches(&val)
        });
        if all_match {
            merged.merge(bucket);
        }
    }
    merged.finish(key.kind)
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
        let key = AggregateKey {
            kind: AggKind::Sum,
            sheet: None,
            column: 1,
            start_row: None,
            end_row: None,
        };
        aggs.insert(key.clone(), Value::Number(42.0));
        let prelude = Prelude::with_aggregates(aggs);
        assert_eq!(prelude.get_aggregate(&key), Some(&Value::Number(42.0)));
    }

    #[test]
    fn get_aggregate_missing_returns_none() {
        let prelude = Prelude::empty();
        let key = AggregateKey {
            kind: AggKind::Sum,
            sheet: None,
            column: 1,
            start_row: None,
            end_row: None,
        };
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
        let agg_key = AggregateKey {
            kind: AggKind::Max,
            sheet: None,
            column: 3,
            start_row: None,
            end_row: None,
        };
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
            start_row: None,
            end_row: None,
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
            start_row: None,
            end_row: None,
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
            start_row: None,
            end_row: None,
        };
        let prelude = Prelude::empty();
        assert_eq!(prelude.get_multi_conditional(&key, "east\0q1"), Value::Error(CellError::Div0));
    }

    fn bucket_key(kind: AggKind) -> MultiConditionalAggKey {
        MultiConditionalAggKey {
            kind,
            sum_col: 2,
            criteria_cols: vec![1],
            sheet: None,
            start_row: None,
            end_row: None,
        }
    }

    fn prelude_with_buckets(
        key: MultiConditionalAggKey,
        buckets: Vec<(&str, BucketAgg)>,
    ) -> Prelude {
        let inner: HashMap<String, BucketAgg> =
            buckets.into_iter().map(|(c, b)| (c.to_string(), b)).collect();
        let mut multi = HashMap::new();
        multi.insert(key, inner);
        Prelude::with_multi_buckets(HashMap::new(), multi)
    }

    #[test]
    fn operator_criteria_average_weights_buckets_by_row_count() {
        // AVERAGEIF(A:A, ">30", B:B) over buckets {50: two rows summing
        // 300, 80: one row of 800} must weight by row count:
        // (300 + 800) / 3, not the average of per-bucket averages.
        let key = bucket_key(AggKind::Average);
        let prelude = prelude_with_buckets(
            key.clone(),
            vec![
                ("10", BucketAgg { sum: 100.0, count: 1, ..BucketAgg::default() }),
                ("50", BucketAgg { sum: 300.0, count: 2, ..BucketAgg::default() }),
                ("80", BucketAgg { sum: 800.0, count: 1, ..BucketAgg::default() }),
            ],
        );
        assert_eq!(prelude.get_multi_conditional(&key, ">30"), Value::Number(1100.0 / 3.0));
    }

    #[test]
    fn operator_criteria_average_skips_buckets_with_no_numeric_values() {
        // A matched bucket whose value cells are all empty contributes
        // nothing — it must not poison the merge with #DIV/0!.
        let key = bucket_key(AggKind::Average);
        let prelude = prelude_with_buckets(
            key.clone(),
            vec![
                ("50", BucketAgg::default()),
                ("80", BucketAgg { sum: 800.0, count: 1, ..BucketAgg::default() }),
            ],
        );
        assert_eq!(prelude.get_multi_conditional(&key, ">30"), Value::Number(800.0));
    }

    #[test]
    fn operator_criteria_min_ignores_buckets_with_no_numeric_values() {
        // An empty matched bucket must not contribute a poisoned 0.0 min.
        let key = bucket_key(AggKind::Min);
        let prelude = prelude_with_buckets(
            key.clone(),
            vec![
                ("50", BucketAgg::default()),
                ("80", BucketAgg { min: Some(5.0), max: Some(5.0), ..BucketAgg::default() }),
            ],
        );
        assert_eq!(prelude.get_multi_conditional(&key, ">30"), Value::Number(5.0));
    }

    #[test]
    fn operator_criteria_propagates_bucket_error() {
        let key = bucket_key(AggKind::Sum);
        let prelude = prelude_with_buckets(
            key.clone(),
            vec![
                ("50", BucketAgg { error: Some(CellError::Value), ..BucketAgg::default() }),
                ("80", BucketAgg { sum: 800.0, count: 1, ..BucketAgg::default() }),
            ],
        );
        assert_eq!(prelude.get_multi_conditional(&key, ">30"), Value::Error(CellError::Value));
    }

    #[test]
    fn direct_lookup_finishes_average_from_partials() {
        let key = bucket_key(AggKind::Average);
        let prelude = prelude_with_buckets(
            key.clone(),
            vec![("east", BucketAgg { sum: 90.0, count: 3, ..BucketAgg::default() })],
        );
        assert_eq!(prelude.get_multi_conditional(&key, "east"), Value::Number(30.0));
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

    #[test]
    fn prelude_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Prelude>();
    }

    #[test]
    fn value_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Value>();
    }
}
