//! Prelude plan: collect aggregate keys from rewritten ASTs and execute
//! the prelude pass to compute them.
//!
//! [`collect_aggregate_keys`] walks a rewritten AST for `PreludeRef`
//! nodes and returns the set of [`AggregateKey`]s needed. The
//! [`execute_prelude`] function streams the main sheet, folds each
//! column's values through `FoldState`, and builds a filled
//! [`Prelude`].

use std::collections::{HashMap, HashSet};

use xlstream_core::{CellError, Value, XlStreamError};
use xlstream_io::Reader;
use xlstream_parse::{AggKind, AggregateKey, Ast, NodeRef, NodeView, PreludeKey};

use crate::prelude::Prelude;

/// Accumulator for streaming aggregate computation over a single column.
///
/// Fed values one at a time via [`feed`](FoldState::feed). After all
/// values are consumed, call [`finish`](FoldState::finish) with the
/// desired [`AggKind`] to extract the result.
///
#[derive(Debug, Clone)]
pub(crate) struct FoldState {
    sum: f64,
    count: u64,
    counta: u64,
    countblank: u64,
    min: Option<f64>,
    max: Option<f64>,
    product: f64,
    has_numeric: bool,
    /// Collected numeric values for median computation.
    nums: Vec<f64>,
    /// First error encountered.
    error: Option<CellError>,
}

impl FoldState {
    /// Create a new empty accumulator.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            sum: 0.0,
            count: 0,
            counta: 0,
            countblank: 0,
            min: None,
            max: None,
            product: 1.0,
            has_numeric: false,
            nums: Vec::new(),
            error: None,
        }
    }

    /// Feed a single cell value into the accumulator.
    ///
    /// Follows range semantics: numeric values accumulate, text and
    /// booleans are skipped for numeric aggregates but counted for
    /// COUNTA, errors are captured (first one wins).
    pub(crate) fn feed(&mut self, v: &Value) {
        match v {
            Value::Error(e) => {
                if self.error.is_none() {
                    self.error = Some(*e);
                }
                // Errors count as non-empty for COUNTA
                self.counta += 1;
            }
            Value::Number(n) => {
                self.sum += n;
                self.count += 1;
                self.counta += 1;
                self.min = Some(self.min.map_or(*n, |cur| cur.min(*n)));
                self.max = Some(self.max.map_or(*n, |cur| cur.max(*n)));
                self.product *= n;
                self.has_numeric = true;
                self.nums.push(*n);
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => {
                let f = *i as f64;
                self.sum += f;
                self.count += 1;
                self.counta += 1;
                self.min = Some(self.min.map_or(f, |cur| cur.min(f)));
                self.max = Some(self.max.map_or(f, |cur| cur.max(f)));
                self.product *= f;
                self.has_numeric = true;
                self.nums.push(f);
            }
            Value::Date(d) => {
                self.sum += d.serial;
                self.count += 1;
                self.counta += 1;
                self.min = Some(self.min.map_or(d.serial, |cur| cur.min(d.serial)));
                self.max = Some(self.max.map_or(d.serial, |cur| cur.max(d.serial)));
                self.product *= d.serial;
                self.has_numeric = true;
                self.nums.push(d.serial);
            }
            Value::Text(_) | Value::Bool(_) => {
                // Non-empty for COUNTA, skipped for numeric aggregates.
                self.counta += 1;
            }
            Value::Empty => {
                self.countblank += 1;
            }
        }
    }

    /// Convert to a multi-conditional bucket partial.
    ///
    /// Drops median support (`nums`) — multi-conditional keys are only
    /// ever Sum, Count, Average, Min, or Max.
    #[must_use]
    pub(crate) fn into_bucket(self) -> crate::prelude::BucketAgg {
        crate::prelude::BucketAgg {
            sum: self.sum,
            count: self.count,
            min: self.min,
            max: self.max,
            error: self.error,
        }
    }

    /// Produce the final aggregate value for the given kind.
    ///
    /// For error-propagating aggregates (SUM, AVERAGE, MIN, MAX,
    /// PRODUCT, MEDIAN), returns the first captured error if any.
    /// COUNT, COUNTA, and COUNTBLANK never propagate errors.
    #[must_use]
    pub(crate) fn finish(mut self, kind: AggKind) -> Value {
        match kind {
            AggKind::Sum => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                Value::Number(self.sum)
            }
            AggKind::Count =>
            {
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.count as f64)
            }
            AggKind::CountA =>
            {
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.counta as f64)
            }
            AggKind::CountBlank =>
            {
                #[allow(clippy::cast_precision_loss)]
                Value::Number((xlstream_core::EXCEL_MAX_ROWS - self.counta) as f64)
            }
            AggKind::Average => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                if self.count == 0 {
                    return Value::Error(CellError::Div0);
                }
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.sum / self.count as f64)
            }
            AggKind::Min => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                Value::Number(self.min.unwrap_or(0.0))
            }
            AggKind::Max => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                Value::Number(self.max.unwrap_or(0.0))
            }
            AggKind::Product => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                if self.has_numeric {
                    Value::Number(self.product)
                } else {
                    Value::Number(0.0)
                }
            }
            AggKind::Median => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                if self.nums.is_empty() {
                    return Value::Error(CellError::Num);
                }
                self.nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mid = self.nums.len() / 2;
                if self.nums.len().is_multiple_of(2) {
                    Value::Number(f64::midpoint(self.nums[mid - 1], self.nums[mid]))
                } else {
                    Value::Number(self.nums[mid])
                }
            }
        }
    }
}

impl Default for FoldState {
    fn default() -> Self {
        Self::new()
    }
}

/// Walk a rewritten AST and collect all [`AggregateKey`]s referenced by
/// `PreludeRef(Aggregate(_))` nodes.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{parse, classify, rewrite, ClassificationContext, FunctionMeta};
/// use xlstream_eval::prelude_plan::collect_aggregate_keys;
///
/// fn real_meta(name: &str) -> Option<&FunctionMeta> {
///     xlstream_eval::registry::lookup_meta(name)
/// }
/// let ast = parse("SUM(A:A)").unwrap();
/// let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
/// let verdict = classify(&ast, &ctx, &real_meta);
/// let rewritten = rewrite(ast, &ctx, &verdict, &real_meta);
/// let keys = collect_aggregate_keys(&rewritten);
/// assert_eq!(keys.len(), 1);
/// ```
#[must_use]
pub fn collect_aggregate_keys(ast: &Ast) -> Vec<AggregateKey> {
    let mut keys = Vec::new();
    collect_keys_recursive(ast.root(), &mut keys);
    keys
}

fn collect_keys_recursive(node: xlstream_parse::NodeRef<'_>, keys: &mut Vec<AggregateKey>) {
    match node.view() {
        NodeView::PreludeRef(PreludeKey::Aggregate(agg_key)) => {
            keys.push(agg_key.clone());
        }
        NodeView::BinaryOp { .. } => {
            if let Some(left) = node.left() {
                collect_keys_recursive(left, keys);
            }
            if let Some(right) = node.right() {
                collect_keys_recursive(right, keys);
            }
        }
        NodeView::UnaryOp { .. } => {
            if let Some(operand) = node.operand() {
                collect_keys_recursive(operand, keys);
            }
        }
        NodeView::Function { .. } => {
            for arg in node.args() {
                collect_keys_recursive(arg, keys);
            }
        }
        NodeView::Array { .. } => {
            for row in node.array_cells() {
                for cell in row {
                    collect_keys_recursive(cell, keys);
                }
            }
        }
        _ => {}
    }
}

/// Walk a rewritten AST and collect all [`MultiConditionalAggKey`]s
/// needed by SUMIFS, COUNTIFS, AVERAGEIFS function nodes.
#[must_use]
pub(crate) fn collect_multi_conditional_keys(
    ast: &Ast,
) -> Vec<crate::prelude::MultiConditionalAggKey> {
    let mut keys = Vec::new();
    collect_multi_keys_recursive(ast.root(), &mut keys);
    keys
}

fn collect_multi_keys_recursive(
    node: xlstream_parse::NodeRef<'_>,
    keys: &mut Vec<crate::prelude::MultiConditionalAggKey>,
) {
    match node.view() {
        NodeView::Function { name } => {
            let upper = name.to_ascii_uppercase();
            let normalized = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
            match normalized {
                "SUMIFS" => {
                    if let Some(key) = extract_value_ifs_key(node, AggKind::Sum) {
                        keys.push(key);
                    }
                }
                "COUNTIFS" => {
                    if let Some(key) = extract_countifs_key(node) {
                        keys.push(key);
                    }
                }
                "AVERAGEIFS" => {
                    if let Some(key) = extract_value_ifs_key(node, AggKind::Average) {
                        keys.push(key);
                    }
                }
                "SUMIF" => {
                    if let Some(key) = extract_if_key(node, AggKind::Sum) {
                        keys.push(key);
                    }
                }
                "COUNTIF" => {
                    if let Some(key) = extract_countif_key(node) {
                        keys.push(key);
                    }
                }
                "AVERAGEIF" => {
                    if let Some(key) = extract_if_key(node, AggKind::Average) {
                        keys.push(key);
                    }
                }
                "MINIFS" => {
                    if let Some(key) = extract_value_ifs_key(node, AggKind::Min) {
                        keys.push(key);
                    }
                }
                "MAXIFS" => {
                    if let Some(key) = extract_value_ifs_key(node, AggKind::Max) {
                        keys.push(key);
                    }
                }
                _ => {}
            }
            // Also recurse into args in case of nested function calls.
            for arg in node.args() {
                collect_multi_keys_recursive(arg, keys);
            }
        }
        NodeView::BinaryOp { .. } => {
            if let Some(left) = node.left() {
                collect_multi_keys_recursive(left, keys);
            }
            if let Some(right) = node.right() {
                collect_multi_keys_recursive(right, keys);
            }
        }
        NodeView::UnaryOp { .. } => {
            if let Some(operand) = node.operand() {
                collect_multi_keys_recursive(operand, keys);
            }
        }
        NodeView::Array { .. } => {
            for row in node.array_cells() {
                for cell in row {
                    collect_multi_keys_recursive(cell, keys);
                }
            }
        }
        _ => {}
    }
}

/// Extract column and optional sheet from a range-ref node (whole-column like `A:A` or `Sheet!A:A`).
fn extract_range_col_and_sheet(node: xlstream_parse::NodeRef<'_>) -> Option<(u32, Option<String>)> {
    match node.view() {
        NodeView::RangeRef { sheet, start_col: Some(sc), end_col: Some(ec), .. } if sc == ec => {
            Some((sc, sheet.map(ToString::to_string)))
        }
        _ => None,
    }
}

/// Row bounds of a single-column range-ref node: `(start_row, end_row)`.
///
/// Whole-column refs (`A:A`) and refs missing either row bound normalize
/// to `(None, None)`. Returns `None` if the node is not a single-column
/// range.
///
/// Build-time key extraction and eval-time key reconstruction both call
/// this (via [`if_row_bounds`] / [`ifs_row_bounds`]) on the same argument
/// nodes: the two sides must produce identical bounds or prelude lookups
/// silently miss.
fn range_row_bounds(node: xlstream_parse::NodeRef<'_>) -> Option<(Option<u32>, Option<u32>)> {
    match node.view() {
        NodeView::RangeRef {
            start_row, end_row, start_col: Some(sc), end_col: Some(ec), ..
        } if sc == ec => match (start_row, end_row) {
            (Some(sr), Some(er)) => Some((Some(sr), Some(er))),
            _ => Some((None, None)),
        },
        _ => None,
    }
}

/// Shared row bounds for SUMIF/AVERAGEIF/COUNTIF-style ranges.
///
/// The criteria range drives the bounds. A value (sum/avg) range, when
/// present, must start on the same row: Excel resizes the value range to
/// the criteria range's shape from its top-left cell, so equal starts make
/// same-row pairing exact. Offset value ranges return `None` — callers
/// refuse the formula (`#VALUE!`) rather than silently mis-pair rows.
pub(crate) fn if_row_bounds(
    criteria: xlstream_parse::NodeRef<'_>,
    value: Option<xlstream_parse::NodeRef<'_>>,
) -> Option<(Option<u32>, Option<u32>)> {
    let bounds = range_row_bounds(criteria)?;
    if let Some(v) = value {
        let (value_start, _) = range_row_bounds(v)?;
        if value_start != bounds.0 {
            return None;
        }
    }
    Some(bounds)
}

/// Shared row bounds for *IFS-style ranges (SUMIFS, COUNTIFS, AVERAGEIFS,
/// MINIFS, MAXIFS).
///
/// Excel requires every range of a *IFS call to have the same shape and
/// returns `#VALUE!` otherwise. This mirrors that rule: all criteria
/// ranges and the optional value range must have identical row bounds,
/// else `None`.
pub(crate) fn ifs_row_bounds<'a, I>(
    value: Option<xlstream_parse::NodeRef<'a>>,
    mut criteria: I,
) -> Option<(Option<u32>, Option<u32>)>
where
    I: Iterator<Item = xlstream_parse::NodeRef<'a>>,
{
    let first = criteria.next()?;
    let bounds = range_row_bounds(first)?;
    for range in criteria {
        if range_row_bounds(range)? != bounds {
            return None;
        }
    }
    if let Some(v) = value {
        if range_row_bounds(v)? != bounds {
            return None;
        }
    }
    Some(bounds)
}

/// Extract a `MultiConditionalAggKey` from a value-range *IFS node
/// (SUMIFS, AVERAGEIFS, MINIFS, MAXIFS):
/// `FN(value_range, crit_range1, crit1, crit_range2, crit2, ...)`
fn extract_value_ifs_key(
    node: xlstream_parse::NodeRef<'_>,
    kind: AggKind,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() < 3 || !(args.len() - 1).is_multiple_of(2) {
        return None;
    }
    let (sum_col, sheet) = extract_range_col_and_sheet(args[0])?;
    let num_pairs = (args.len() - 1) / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    for i in 0..num_pairs {
        let (col, _) = extract_range_col_and_sheet(args[1 + i * 2])?;
        criteria_cols.push(col);
    }
    let (start_row, end_row) =
        ifs_row_bounds(Some(args[0]), (0..num_pairs).map(|i| args[1 + i * 2]))?;
    Some(crate::prelude::MultiConditionalAggKey {
        kind,
        sum_col,
        criteria_cols,
        sheet,
        start_row,
        end_row,
    })
}

/// Extract a `MultiConditionalAggKey` from a COUNTIFS function node.
/// `COUNTIFS(crit_range1, crit1, crit_range2, crit2, ...)`
fn extract_countifs_key(
    node: xlstream_parse::NodeRef<'_>,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() < 2 || !args.len().is_multiple_of(2) {
        return None;
    }
    let num_pairs = args.len() / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    let mut sheet: Option<String> = None;
    for i in 0..num_pairs {
        let (col, s) = extract_range_col_and_sheet(args[i * 2])?;
        criteria_cols.push(col);
        if i == 0 {
            sheet = s;
        }
    }
    let (start_row, end_row) = ifs_row_bounds(None, (0..num_pairs).map(|i| args[i * 2]))?;
    Some(crate::prelude::MultiConditionalAggKey {
        kind: AggKind::Count,
        sum_col: 0,
        criteria_cols,
        sheet,
        start_row,
        end_row,
    })
}

/// Extract a `MultiConditionalAggKey` from a SUMIF/AVERAGEIF node:
/// `FN(criteria_range, criteria, [value_range])`
fn extract_if_key(
    node: xlstream_parse::NodeRef<'_>,
    kind: AggKind,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() < 2 || args.len() > 3 {
        return None;
    }
    let (criteria_col, sheet) = extract_range_col_and_sheet(args[0])?;
    let sum_col = if args.len() >= 3 {
        let (sc, _) = extract_range_col_and_sheet(args[2])?;
        sc
    } else {
        criteria_col
    };
    let (start_row, end_row) = if_row_bounds(args[0], args.get(2).copied())?;
    Some(crate::prelude::MultiConditionalAggKey {
        kind,
        sum_col,
        criteria_cols: vec![criteria_col],
        sheet,
        start_row,
        end_row,
    })
}

/// Extract a `MultiConditionalAggKey` from a COUNTIF function node.
/// `COUNTIF(criteria_range, criteria)`
fn extract_countif_key(
    node: xlstream_parse::NodeRef<'_>,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() != 2 {
        return None;
    }
    let (criteria_col, sheet) = extract_range_col_and_sheet(args[0])?;
    let (start_row, end_row) = if_row_bounds(args[0], None)?;
    Some(crate::prelude::MultiConditionalAggKey {
        kind: AggKind::Count,
        sum_col: 0,
        criteria_cols: vec![criteria_col],
        sheet,
        start_row,
        end_row,
    })
}

/// Walk an AST and collect all `BoundedRangeKey`s referenced by
/// `RangeRef` nodes with bounded rows and a single column.
///
/// These are bounded ranges inside range-expanding functions that
/// survive the rewrite pass (they are NOT rewritten to `PreludeRef`).
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// use xlstream_eval::prelude_plan::collect_bounded_range_keys;
///
/// let ast = parse("IRR(A2:A10)").unwrap();
/// let keys = collect_bounded_range_keys(&ast);
/// assert_eq!(keys.len(), 1);
/// assert_eq!(keys[0].col, 1);
/// assert_eq!(keys[0].start_row, 2);
/// assert_eq!(keys[0].end_row, 10);
/// ```
#[must_use]
pub fn collect_bounded_range_keys(ast: &Ast) -> Vec<crate::prelude::BoundedRangeKey> {
    let mut keys = Vec::new();
    collect_range_keys_recursive(ast.root(), &mut keys);
    keys
}

fn collect_range_keys_recursive(
    node: xlstream_parse::NodeRef<'_>,
    keys: &mut Vec<crate::prelude::BoundedRangeKey>,
) {
    match node.view() {
        NodeView::RangeRef {
            sheet,
            start_row: Some(sr),
            end_row: Some(er),
            start_col: Some(sc),
            end_col: Some(ec),
        } if sc == ec => {
            keys.push(crate::prelude::BoundedRangeKey {
                sheet: sheet.map(ToString::to_string),
                col: sc,
                start_row: sr,
                end_row: er,
            });
        }
        NodeView::BinaryOp { .. } => {
            if let Some(left) = node.left() {
                collect_range_keys_recursive(left, keys);
            }
            if let Some(right) = node.right() {
                collect_range_keys_recursive(right, keys);
            }
        }
        NodeView::UnaryOp { .. } => {
            if let Some(operand) = node.operand() {
                collect_range_keys_recursive(operand, keys);
            }
        }
        NodeView::Function { .. } => {
            for arg in node.args() {
                collect_range_keys_recursive(arg, keys);
            }
        }
        NodeView::Array { .. } => {
            for row in node.array_cells() {
                for cell in row {
                    collect_range_keys_recursive(cell, keys);
                }
            }
        }
        _ => {}
    }
}

/// Formula evaluation context for a single sheet during prelude.
///
/// # Examples
///
/// ```
/// use std::collections::{HashMap, HashSet};
/// use xlstream_eval::prelude_plan::SheetFormulaCtx;
///
/// let ctx = SheetFormulaCtx {
///     col_asts: &HashMap::new(),
///     row_overrides: &HashMap::new(),
///     topo_order: &[],
///     formula_rows: &HashMap::new(),
/// };
/// assert!(ctx.topo_order.is_empty());
/// ```
pub struct SheetFormulaCtx<'a> {
    /// Per-column formula ASTs.
    pub col_asts: &'a HashMap<u32, Ast>,
    /// Per-row formula overrides.
    pub row_overrides: &'a HashMap<u32, HashMap<u32, Ast>>,
    /// Column evaluation order.
    pub topo_order: &'a [u32],
    /// 0-based row indices that actually contain formulas, per column.
    pub formula_rows: &'a HashMap<u32, HashSet<u32>>,
}

/// Returns `true` if the formula can be evaluated during prelude.
///
/// Only `PreludeRef`, `ExternalRef`, and
/// `ThreeDimensionalRef` nodes are unevaluable. Everything else
/// resolves with loaded lookups and volatile data.
///
/// # Examples
///
/// ```
/// use xlstream_eval::prelude_plan::is_prelude_evaluable;
/// let ast = xlstream_parse::parse("A1*2").unwrap();
/// assert!(is_prelude_evaluable(&ast));
/// ```
#[must_use]
pub fn is_prelude_evaluable(ast: &Ast) -> bool {
    !contains_unevaluable_node(ast.root())
}

fn contains_unevaluable_node(node: NodeRef<'_>) -> bool {
    match node.view() {
        NodeView::PreludeRef(_)
        | NodeView::ExternalRef { .. }
        | NodeView::ThreeDimensionalRef { .. } => true,
        NodeView::Number(_)
        | NodeView::Bool(_)
        | NodeView::Text(_)
        | NodeView::Error(_)
        | NodeView::CellRef { .. }
        | NodeView::RangeRef { .. }
        | NodeView::NamedRef(_)
        | NodeView::TableRef { .. } => false,
        NodeView::BinaryOp { .. } => {
            node.left().is_some_and(contains_unevaluable_node)
                || node.right().is_some_and(contains_unevaluable_node)
        }
        NodeView::UnaryOp { .. } => node.operand().is_some_and(contains_unevaluable_node),
        NodeView::Function { .. } => node.args().iter().any(|a| contains_unevaluable_node(*a)),
        NodeView::Array { .. } => {
            node.array_cells().iter().any(|row| row.iter().any(|c| contains_unevaluable_node(*c)))
        }
    }
}

fn evaluate_row_formulas(
    row_values: Vec<Value>,
    row_idx: u32,
    ctx: &SheetFormulaCtx<'_>,
    interp: &crate::Interpreter<'_>,
    aggregate_cols: &HashSet<u32>,
    errored_cols: &mut HashSet<u32>,
) -> Result<Vec<Value>, XlStreamError> {
    let mut evaluated = row_values;
    for &col in ctx.topo_order {
        let has_formula = ctx.formula_rows.get(&col).is_some_and(|rows| rows.contains(&row_idx));
        if !has_formula {
            continue;
        }
        let idx = col as usize;
        let needs_eval = match evaluated.get(idx) {
            None | Some(Value::Empty) => true,
            Some(Value::Text(s)) => s.is_empty(),
            _ => false,
        };
        if !needs_eval {
            continue;
        }
        let ast = ctx
            .row_overrides
            .get(&col)
            .and_then(|rm| rm.get(&row_idx))
            .or_else(|| ctx.col_asts.get(&col));
        if let Some(ast) = ast {
            if is_prelude_evaluable(ast) {
                if idx >= evaluated.len() {
                    evaluated.resize(idx + 1, Value::Empty);
                }
                let result = {
                    let scope = crate::scope::RowScope::new(&evaluated, row_idx).with_col_idx(col);
                    interp.eval(ast.root(), &scope)
                };
                evaluated[idx] = result;
            } else if aggregate_cols.contains(&col) && errored_cols.insert(col) {
                let col_a1 = xlstream_core::col_row_to_a1(col + 1, 1);
                let col_letter = col_a1.trim_end_matches(char::is_numeric);
                return Err(XlStreamError::Internal(format!(
                    "cannot compute aggregate over column {col_letter}: \
                     formula contains aggregate references that create a circular \
                     dependency during prelude. Save the workbook in Excel to \
                     populate cached values"
                )));
            } else if !is_prelude_evaluable(ast) && errored_cols.insert(col) {
                let col_a1 = xlstream_core::col_row_to_a1(col + 1, 1);
                let col_letter = col_a1.trim_end_matches(char::is_numeric);
                tracing::warn!(
                    column = col_letter,
                    "unevaluable formula column has empty cached values; \
                     downstream columns depending on it may produce wrong results"
                );
            }
        }
    }
    Ok(evaluated)
}

/// Formula-evaluation context passed to [`execute_prelude`].
///
/// Bundles the base prelude (lookups + volatile), main-sheet formula
/// context, and cross-sheet formula contexts into a single argument.
///
/// # Examples
///
/// ```
/// use xlstream_eval::prelude_plan::PreludeFormulaCtx;
/// use xlstream_eval::Prelude;
///
/// let base = Prelude::empty();
/// let ctx = PreludeFormulaCtx {
///     current_sheet_formulas: None,
///     cross_sheet_formulas: &[],
///     base_prelude: &base,
/// };
/// assert!(ctx.current_sheet_formulas.is_none());
/// ```
pub struct PreludeFormulaCtx<'a> {
    /// Formula context for the main (streaming) sheet, if it has formulas.
    pub current_sheet_formulas: Option<&'a SheetFormulaCtx<'a>>,
    /// Formula contexts for secondary sheets, keyed by sheet name.
    pub cross_sheet_formulas: &'a [(&'a str, &'a SheetFormulaCtx<'a>)],
    /// Base prelude with loaded lookup sheets and volatile data.
    pub base_prelude: &'a Prelude,
}

/// Execute the prelude pass: stream the main sheet, fold column values
/// through `FoldState` accumulators, and build a filled [`Prelude`].
///
/// Skips the first row (header). Each subsequent row feeds the relevant
/// column values into accumulators. After the stream is exhausted,
/// finishes each accumulator and stores the result.
///
/// Also computes multi-criteria conditional aggregate groupby tables
/// for SUMIFS/COUNTIFS/AVERAGEIFS, and collects bounded range values
/// for range-expanding functions.
///
/// Returns `(prelude, data_row_count)` where `data_row_count` is the
/// number of non-header rows streamed during the prelude pass.
///
/// # Errors
///
/// Returns `Err(XlStreamError::Xlsx)` if the sheet cannot be read.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_io::Reader;
/// use xlstream_parse::{AggKind, AggregateKey};
/// use xlstream_eval::prelude_plan::execute_prelude;
///
/// let mut reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
/// let keys = vec![AggregateKey { kind: AggKind::Sum, sheet: None, column: 1, start_row: None, end_row: None }];
/// use xlstream_eval::prelude_plan::PreludeFormulaCtx;
///
/// let base = xlstream_eval::Prelude::empty();
/// let formula_ctx = PreludeFormulaCtx {
///     current_sheet_formulas: None,
///     cross_sheet_formulas: &[],
///     base_prelude: &base,
/// };
/// let (prelude, _count) = execute_prelude(
///     &mut reader, "Sheet1", &keys, &[], &[], &formula_ctx,
/// ).unwrap();
/// ```
#[allow(clippy::too_many_lines)]
pub fn execute_prelude(
    reader: &mut Reader,
    main_sheet: &str,
    simple_keys: &[AggregateKey],
    multi_keys: &[crate::prelude::MultiConditionalAggKey],
    range_keys: &[crate::prelude::BoundedRangeKey],
    formula_ctx: &PreludeFormulaCtx<'_>,
) -> Result<(Prelude, u32), XlStreamError> {
    // (column, start_row, end_row) — AggKind excluded because one FoldState serves all kinds
    // for the same column+bounds (cloned and finished per kind).
    type FoldKey = (u32, Option<u32>, Option<u32>);

    if simple_keys.is_empty() && multi_keys.is_empty() && range_keys.is_empty() {
        return Ok((Prelude::empty(), 0));
    }

    // Group main-sheet keys by (column, start_row, end_row); cross-sheet keys go to a separate map.
    let mut col_kinds: HashMap<FoldKey, Vec<(AggKind, AggregateKey)>> = HashMap::new();
    let mut cross_simple_keys: HashMap<String, Vec<AggregateKey>> = HashMap::new();
    for key in simple_keys {
        let is_same_sheet = key.sheet.as_deref().is_none_or(|s| s.eq_ignore_ascii_case(main_sheet));
        if is_same_sheet {
            let fk = (key.column, key.start_row, key.end_row);
            col_kinds.entry(fk).or_default().push((key.kind, key.clone()));
        } else {
            cross_simple_keys
                .entry(key.sheet.clone().unwrap_or_default())
                .or_default()
                .push(key.clone());
        }
    }

    // One FoldState per (column, bounds) pair (main-sheet only).
    let mut col_folds: HashMap<FoldKey, FoldState> = HashMap::new();
    for &fk in col_kinds.keys() {
        col_folds.insert(fk, FoldState::new());
    }

    // Collect all columns needed for multi-conditional keys.
    let mut multi_needed_cols: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for mk in multi_keys {
        if mk.sum_col > 0 {
            multi_needed_cols.insert(mk.sum_col);
        }
        for &c in &mk.criteria_cols {
            multi_needed_cols.insert(c);
        }
    }

    // Multi-conditional accumulators: for each key, a map from composite
    // criteria key to FoldState.
    let mut multi_folds: HashMap<usize, HashMap<String, FoldState>> = HashMap::new();
    for i in 0..multi_keys.len() {
        multi_folds.insert(i, HashMap::new());
    }
    // Rows actually fed per key. Count-kind keys use the deficit against
    // the range size to account for never-streamed blank rows.
    let mut multi_rows_seen: Vec<u64> = vec![0; multi_keys.len()];

    // Initialize bounded range collectors, split by sheet.
    let mut range_collectors: HashMap<crate::prelude::BoundedRangeKey, Vec<Value>> = HashMap::new();
    let mut cross_range_keys: HashMap<String, Vec<crate::prelude::BoundedRangeKey>> =
        HashMap::new();
    for rk in range_keys {
        let capacity = (rk.end_row.saturating_sub(rk.start_row) + 1) as usize;
        let is_cross = rk.sheet.as_deref().is_some_and(|s| !s.eq_ignore_ascii_case(main_sheet));
        if is_cross {
            if let Some(ref sheet) = rk.sheet {
                cross_range_keys.entry(sheet.clone()).or_default().push(rk.clone());
            }
        }
        range_collectors.insert(rk.clone(), Vec::with_capacity(capacity));
    }

    // Columns consumed by aggregates — used to detect unevaluable formulas
    // feeding into aggregate folds (which would produce silently wrong results).
    let aggregate_cols: HashSet<u32> = {
        let mut cols = HashSet::new();
        for &(col, _, _) in col_kinds.keys() {
            cols.insert(col.saturating_sub(1));
        }
        for mk in multi_keys {
            let is_same = mk.sheet.as_deref().is_none_or(|s| s.eq_ignore_ascii_case(main_sheet));
            if is_same {
                if mk.sum_col > 0 {
                    cols.insert(mk.sum_col.saturating_sub(1));
                }
                for &c in &mk.criteria_cols {
                    cols.insert(c.saturating_sub(1));
                }
            }
        }
        cols
    };

    let interp = crate::Interpreter::new(formula_ctx.base_prelude).with_main_sheet(main_sheet);
    let mut errored_cols: HashSet<u32> = HashSet::new();

    // Stream the sheet, skipping header row.
    let mut stream = reader.cells(main_sheet)?;
    let mut header_skipped = false;
    let mut data_row_count: u32 = 0;

    while let Some((row_idx, row_values)) = stream.next_row()? {
        let excel_row = row_idx + 1; // 1-based

        if header_skipped {
            data_row_count = data_row_count.saturating_add(1);
        } else {
            header_skipped = true;
        }

        // Evaluate formulas in this row if a formula context was provided.
        let evaluated_row = if let Some(ctx) = formula_ctx.current_sheet_formulas {
            evaluate_row_formulas(
                row_values,
                row_idx,
                ctx,
                &interp,
                &aggregate_cols,
                &mut errored_cols,
            )?
        } else {
            row_values
        };

        // Feed simple aggregate folds, respecting row bounds.
        for (&(col, start_row, end_row), fold) in &mut col_folds {
            let in_bounds = match (start_row, end_row) {
                (Some(sr), Some(er)) => excel_row >= sr && excel_row <= er,
                _ => true,
            };
            if in_bounds {
                let idx = (col as usize).saturating_sub(1);
                let val = evaluated_row.get(idx).unwrap_or(&Value::Empty);
                fold.feed(val);
            }
        }

        // Collect bounded range values (current-sheet only).
        for (rk, collector) in &mut range_collectors {
            let is_cross = rk.sheet.as_deref().is_some_and(|s| !s.eq_ignore_ascii_case(main_sheet));
            if is_cross {
                continue;
            }
            if excel_row >= rk.start_row && excel_row <= rk.end_row {
                let idx = (rk.col as usize).saturating_sub(1);
                let val = evaluated_row.get(idx).cloned().unwrap_or(Value::Empty);
                collector.push(val);
            }
        }

        // Feed multi-conditional folds (current-sheet keys only),
        // respecting row bounds.
        for (i, mk) in multi_keys.iter().enumerate() {
            let is_cross = mk.sheet.as_deref().is_some_and(|s| !s.eq_ignore_ascii_case(main_sheet));
            if is_cross {
                continue;
            }
            let in_bounds = match (mk.start_row, mk.end_row) {
                (Some(sr), Some(er)) => excel_row >= sr && excel_row <= er,
                _ => true,
            };
            if !in_bounds {
                continue;
            }
            multi_rows_seen[i] += 1;
            // Build composite key from criteria columns.
            let mut composite_parts: Vec<String> = Vec::with_capacity(mk.criteria_cols.len());
            for &cc in &mk.criteria_cols {
                let idx = (cc as usize).saturating_sub(1);
                let val = evaluated_row.get(idx).unwrap_or(&Value::Empty);
                composite_parts.push(xlstream_core::coerce::to_text(val).to_ascii_lowercase());
            }
            let composite = composite_parts.join("\0");

            // Get the sum/avg column value (for COUNTIFS, sum_col=0 so we
            // feed a Number(1.0) to count).
            let feed_val = if mk.sum_col > 0 {
                let idx = (mk.sum_col as usize).saturating_sub(1);
                evaluated_row.get(idx).unwrap_or(&Value::Empty)
            } else {
                // COUNTIFS: count rows, feed 1.0
                &Value::Number(1.0)
            };

            // Pre-populated for all indices in the loop above.
            let Some(folds_map) = multi_folds.get_mut(&i) else {
                continue;
            };
            folds_map.entry(composite).or_insert_with(FoldState::new).feed(feed_val);
        }
    }

    // Release mutable borrow on reader before cross-sheet scanning.
    drop(stream);

    // Cross-sheet multi-conditional pass: scan non-main sheets.
    let mut cross_sheet_keys: HashMap<
        String,
        Vec<(usize, &crate::prelude::MultiConditionalAggKey)>,
    > = HashMap::new();
    for (i, mk) in multi_keys.iter().enumerate() {
        let is_cross = mk.sheet.as_deref().is_some_and(|s| !s.eq_ignore_ascii_case(main_sheet));
        if is_cross {
            if let Some(ref sheet_name) = mk.sheet {
                cross_sheet_keys.entry(sheet_name.clone()).or_default().push((i, mk));
            }
        }
    }

    for (sheet_name, sheet_keys) in &cross_sheet_keys {
        let cs_ctx = formula_ctx
            .cross_sheet_formulas
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(sheet_name))
            .map(|(_, ctx)| ctx);
        let cs_agg_cols: HashSet<u32> = sheet_keys
            .iter()
            .flat_map(|(_, mk)| {
                let mut cols = mk.criteria_cols.clone();
                if mk.sum_col > 0 {
                    cols.push(mk.sum_col);
                }
                cols.into_iter().map(|c| c.saturating_sub(1))
            })
            .collect();
        let mut cs_errored: HashSet<u32> = HashSet::new();
        let cs_interp =
            crate::Interpreter::new(formula_ctx.base_prelude).with_main_sheet(sheet_name);

        let mut cs_stream = reader.cells(sheet_name)?;
        while let Some((row_idx, row_values)) = cs_stream.next_row()? {
            let evaluated_row = if let Some(ctx) = cs_ctx {
                evaluate_row_formulas(
                    row_values,
                    row_idx,
                    ctx,
                    &cs_interp,
                    &cs_agg_cols,
                    &mut cs_errored,
                )?
            } else {
                row_values
            };

            let cs_excel_row = row_idx + 1;
            for &(i, mk) in sheet_keys {
                let in_bounds = match (mk.start_row, mk.end_row) {
                    (Some(sr), Some(er)) => cs_excel_row >= sr && cs_excel_row <= er,
                    _ => true,
                };
                if !in_bounds {
                    continue;
                }
                multi_rows_seen[i] += 1;
                let mut composite_parts: Vec<String> = Vec::with_capacity(mk.criteria_cols.len());
                for &cc in &mk.criteria_cols {
                    let idx = (cc as usize).saturating_sub(1);
                    let val = evaluated_row.get(idx).unwrap_or(&Value::Empty);
                    composite_parts.push(xlstream_core::coerce::to_text(val).to_ascii_lowercase());
                }
                let composite = composite_parts.join("\0");

                let feed_val = if mk.sum_col > 0 {
                    let idx = (mk.sum_col as usize).saturating_sub(1);
                    evaluated_row.get(idx).unwrap_or(&Value::Empty)
                } else {
                    &Value::Number(1.0)
                };

                let Some(folds_map) = multi_folds.get_mut(&i) else {
                    continue;
                };
                folds_map.entry(composite).or_insert_with(FoldState::new).feed(feed_val);
            }
        }
    }

    // Finish main-sheet simple aggregate folds.
    let mut aggregates: HashMap<AggregateKey, Value> = HashMap::new();
    for (fk, fold) in col_folds {
        let kinds = col_kinds.get(&fk).map_or(&[][..], |v| v.as_slice());
        for (kind, key) in kinds {
            let result = fold.clone().finish(*kind);
            aggregates.insert(key.clone(), result);
        }
    }

    // Cross-sheet simple aggregate pass: stream each non-main sheet.
    for (sheet_name, keys) in &cross_simple_keys {
        let cs_ctx = formula_ctx
            .cross_sheet_formulas
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(sheet_name))
            .map(|(_, ctx)| ctx);
        let mut cs_col_kinds: HashMap<FoldKey, Vec<(AggKind, AggregateKey)>> = HashMap::new();
        for key in keys {
            let fk = (key.column, key.start_row, key.end_row);
            cs_col_kinds.entry(fk).or_default().push((key.kind, key.clone()));
        }
        let cs_agg_cols: HashSet<u32> =
            cs_col_kinds.keys().map(|&(col, _, _)| col.saturating_sub(1)).collect();
        let mut cs_errored: HashSet<u32> = HashSet::new();
        let mut cs_folds: HashMap<FoldKey, FoldState> = HashMap::new();
        for &fk in cs_col_kinds.keys() {
            cs_folds.insert(fk, FoldState::new());
        }
        let cs_interp =
            crate::Interpreter::new(formula_ctx.base_prelude).with_main_sheet(sheet_name);

        let mut cs_stream = reader.cells(sheet_name)?;
        while let Some((row_idx, row_values)) = cs_stream.next_row()? {
            let evaluated_row = if let Some(ctx) = cs_ctx {
                evaluate_row_formulas(
                    row_values,
                    row_idx,
                    ctx,
                    &cs_interp,
                    &cs_agg_cols,
                    &mut cs_errored,
                )?
            } else {
                row_values
            };
            let cs_excel_row = row_idx + 1;
            for (&(col, start_row, end_row), fold) in &mut cs_folds {
                let in_bounds = match (start_row, end_row) {
                    (Some(sr), Some(er)) => cs_excel_row >= sr && cs_excel_row <= er,
                    _ => true,
                };
                if in_bounds {
                    let idx = (col as usize).saturating_sub(1);
                    let val = evaluated_row.get(idx).unwrap_or(&Value::Empty);
                    fold.feed(val);
                }
            }
        }

        for (fk, fold) in cs_folds {
            let kinds = cs_col_kinds.get(&fk).map_or(&[][..], |v| v.as_slice());
            for (kind, key) in kinds {
                aggregates.insert(key.clone(), fold.clone().finish(*kind));
            }
        }
    }

    // Cross-sheet bounded range pass: stream each non-main sheet.
    for (sheet_name, keys) in &cross_range_keys {
        let cs_ctx = formula_ctx
            .cross_sheet_formulas
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(sheet_name))
            .map(|(_, ctx)| ctx);
        let cs_range_cols: HashSet<u32> = keys.iter().map(|rk| rk.col.saturating_sub(1)).collect();
        let mut cs_errored: HashSet<u32> = HashSet::new();
        let cs_interp =
            crate::Interpreter::new(formula_ctx.base_prelude).with_main_sheet(sheet_name);

        let mut cs_stream = reader.cells(sheet_name)?;
        while let Some((row_idx, row_values)) = cs_stream.next_row()? {
            let evaluated_row = if let Some(ctx) = cs_ctx {
                evaluate_row_formulas(
                    row_values,
                    row_idx,
                    ctx,
                    &cs_interp,
                    &cs_range_cols,
                    &mut cs_errored,
                )?
            } else {
                row_values
            };
            let excel_row = row_idx + 1;
            for rk in keys {
                if excel_row >= rk.start_row && excel_row <= rk.end_row {
                    let idx = (rk.col as usize).saturating_sub(1);
                    let val = evaluated_row.get(idx).cloned().unwrap_or(Value::Empty);
                    if let Some(collector) = range_collectors.get_mut(rk) {
                        collector.push(val);
                    }
                }
            }
        }
    }

    // Convert multi-conditional folds to bucket partials. Buckets are
    // finished at lookup time ([`crate::prelude::BucketAgg::finish`]) so
    // operator criteria can merge them exactly.
    let mut multi_aggs: HashMap<
        crate::prelude::MultiConditionalAggKey,
        HashMap<String, crate::prelude::BucketAgg>,
    > = HashMap::new();
    for (i, mk) in multi_keys.iter().enumerate() {
        let mut folds_map = multi_folds.remove(&i).unwrap_or_default();
        if mk.kind == AggKind::Count {
            // Rows never streamed — bounds past the sheet's last row, or the
            // implicit tail of a whole-column range up to EXCEL_MAX_ROWS —
            // are blank in every column, so they belong to the all-empty
            // composite bucket. Count buckets store row counts as sums, so
            // one feed of the deficit keeps COUNTIF(range, "") and "<>x"
            // Excel-exact. Sum/Average/Min/Max buckets ignore blanks.
            let range_rows = match (mk.start_row, mk.end_row) {
                (Some(sr), Some(er)) => u64::from(er.saturating_sub(sr)) + 1,
                _ => xlstream_core::EXCEL_MAX_ROWS,
            };
            let missing = range_rows.saturating_sub(multi_rows_seen[i]);
            if missing > 0 {
                let blank_composite = "\0".repeat(mk.criteria_cols.len().saturating_sub(1));
                #[allow(clippy::cast_precision_loss)]
                folds_map.entry(blank_composite).or_default().feed(&Value::Number(missing as f64));
            }
        }
        let inner: HashMap<String, crate::prelude::BucketAgg> = folds_map
            .into_iter()
            .map(|(composite, fold)| (composite, fold.into_bucket()))
            .collect();
        multi_aggs.insert(mk.clone(), inner);
    }

    let prelude = if multi_aggs.is_empty() {
        Prelude::with_aggregates(aggregates)
    } else {
        Prelude::with_multi_buckets(aggregates, multi_aggs)
    };

    if range_collectors.is_empty() {
        Ok((prelude, data_row_count))
    } else {
        Ok((prelude.with_cached_ranges(range_collectors), data_row_count))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};
    use xlstream_parse::AggKind;

    use super::*;

    fn real_meta(name: &str) -> Option<&xlstream_parse::FunctionMeta> {
        crate::registry::lookup_meta(name)
    }

    // ===== FoldState: SUM =====

    #[test]
    fn fold_sum_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Number(3.0));
        assert_eq!(s.finish(AggKind::Sum), Value::Number(6.0));
    }

    #[test]
    fn fold_sum_empty() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Sum), Value::Number(0.0));
    }

    #[test]
    fn fold_sum_skips_text() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(5.0));
        s.feed(&Value::Text("x".into()));
        assert_eq!(s.finish(AggKind::Sum), Value::Number(5.0));
    }

    #[test]
    fn fold_sum_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Error(CellError::Div0));
        assert_eq!(s.finish(AggKind::Sum), Value::Error(CellError::Div0));
    }

    // ===== FoldState: COUNT =====

    #[test]
    fn fold_count_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Text("x".into()));
        s.feed(&Value::Number(2.0));
        assert_eq!(s.finish(AggKind::Count), Value::Number(2.0));
    }

    #[test]
    fn fold_count_empty() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Count), Value::Number(0.0));
    }

    // ===== FoldState: COUNTA =====

    #[test]
    fn fold_counta_counts_non_empty() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Empty);
        s.feed(&Value::Text("x".into()));
        s.feed(&Value::Bool(true));
        assert_eq!(s.finish(AggKind::CountA), Value::Number(3.0));
    }

    #[test]
    fn fold_counta_counts_errors() {
        let mut s = FoldState::new();
        s.feed(&Value::Error(CellError::Na));
        s.feed(&Value::Empty);
        assert_eq!(s.finish(AggKind::CountA), Value::Number(1.0));
    }

    // ===== FoldState: COUNTBLANK =====

    #[test]
    fn fold_countblank_uses_excel_max_rows() {
        let mut s = FoldState::new();
        s.feed(&Value::Empty);
        s.feed(&Value::Number(0.0));
        s.feed(&Value::Empty);
        // COUNTBLANK = EXCEL_MAX_ROWS - counta; counta=1 (the number)
        assert_eq!(s.finish(AggKind::CountBlank), Value::Number(1_048_575.0));
    }

    // ===== FoldState: AVERAGE =====

    #[test]
    fn fold_average_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(10.0));
        s.feed(&Value::Number(20.0));
        assert_eq!(s.finish(AggKind::Average), Value::Number(15.0));
    }

    #[test]
    fn fold_average_empty_returns_div0() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Average), Value::Error(CellError::Div0));
    }

    #[test]
    fn fold_average_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Error(CellError::Na));
        assert_eq!(s.finish(AggKind::Average), Value::Error(CellError::Na));
    }

    // ===== FoldState: MIN =====

    #[test]
    fn fold_min_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(5.0));
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(3.0));
        assert_eq!(s.finish(AggKind::Min), Value::Number(1.0));
    }

    #[test]
    fn fold_min_empty_returns_zero() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Min), Value::Number(0.0));
    }

    // ===== FoldState: MAX =====

    #[test]
    fn fold_max_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(5.0));
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(9.0));
        assert_eq!(s.finish(AggKind::Max), Value::Number(9.0));
    }

    #[test]
    fn fold_max_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Error(CellError::Ref));
        assert_eq!(s.finish(AggKind::Max), Value::Error(CellError::Ref));
    }

    // ===== FoldState: PRODUCT =====

    #[test]
    fn fold_product_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Number(3.0));
        s.feed(&Value::Number(4.0));
        assert_eq!(s.finish(AggKind::Product), Value::Number(24.0));
    }

    #[test]
    fn fold_product_empty_returns_zero() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Product), Value::Number(0.0));
    }

    #[test]
    fn fold_product_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Error(CellError::Num));
        assert_eq!(s.finish(AggKind::Product), Value::Error(CellError::Num));
    }

    // ===== FoldState: MEDIAN =====

    #[test]
    fn fold_median_odd() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(3.0));
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(2.0));
        assert_eq!(s.finish(AggKind::Median), Value::Number(2.0));
    }

    #[test]
    fn fold_median_even() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Number(3.0));
        s.feed(&Value::Number(4.0));
        assert_eq!(s.finish(AggKind::Median), Value::Number(2.5));
    }

    #[test]
    fn fold_median_empty_returns_num_error() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Median), Value::Error(CellError::Num));
    }

    #[test]
    fn fold_median_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Error(CellError::Na));
        assert_eq!(s.finish(AggKind::Median), Value::Error(CellError::Na));
    }

    #[test]
    fn fold_skips_bool_for_numeric_aggs() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(10.0));
        s.feed(&Value::Bool(true));
        assert_eq!(s.finish(AggKind::Sum), Value::Number(10.0));
    }

    // ===== collect_aggregate_keys =====

    #[test]
    fn collect_keys_from_rewritten_sum() {
        let ast = xlstream_parse::parse("SUM(A:A)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_aggregate_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Sum);
        assert_eq!(keys[0].column, 1);
    }

    #[test]
    fn collect_keys_empty_for_row_local() {
        let ast = xlstream_parse::parse("A1+B1").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 1, 3);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_aggregate_keys(&rewritten);
        assert!(keys.is_empty());
    }

    #[test]
    fn collect_multi_keys_extracts_sumif() {
        let ast = xlstream_parse::parse("SUMIF(A:A,A2,B:B)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Sum);
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_countif() {
        let ast = xlstream_parse::parse("COUNTIF(A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Count);
        assert_eq!(keys[0].sum_col, 0);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_averageif() {
        let ast = xlstream_parse::parse("AVERAGEIF(A:A,A2,B:B)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Average);
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_sumif_without_sum_range_uses_criteria_col() {
        let ast = xlstream_parse::parse("SUMIF(A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].sum_col, 1);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_cross_sheet_sumif() {
        let ast = xlstream_parse::parse("SUMIF(RefData!A:A,A2,RefData!B:B)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Main", 2, 5)
            .with_lookup_sheet("RefData");
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].sheet.as_deref(), Some("RefData"));
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_cross_sheet_countif() {
        let ast = xlstream_parse::parse("COUNTIF(RefData!A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Main", 2, 5)
            .with_lookup_sheet("RefData");
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].sheet.as_deref(), Some("RefData"));
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_cross_sheet_averageif() {
        let ast = xlstream_parse::parse("AVERAGEIF(RefData!A:A,A2,RefData!B:B)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Main", 2, 5)
            .with_lookup_sheet("RefData");
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].sheet.as_deref(), Some("RefData"));
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_cross_sheet_sumifs() {
        let ast = xlstream_parse::parse("SUMIFS(RefData!B:B,RefData!A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Main", 2, 5)
            .with_lookup_sheet("RefData");
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].sheet.as_deref(), Some("RefData"));
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_minifs() {
        let ast = xlstream_parse::parse("MINIFS(B:B,A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Min);
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_maxifs() {
        let ast = xlstream_parse::parse("MAXIFS(B:B,A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Max);
        assert_eq!(keys[0].sum_col, 2);
        assert_eq!(keys[0].criteria_cols, vec![1]);
    }

    #[test]
    fn collect_multi_keys_extracts_minifs_two_criteria() {
        let ast = xlstream_parse::parse("MINIFS(C:C,A:A,A2,B:B,B2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].criteria_cols, vec![1, 2]);
    }

    #[test]
    fn collect_multi_keys_extracts_cross_sheet_minifs() {
        let ast = xlstream_parse::parse("MINIFS(RefData!B:B,RefData!A:A,A2)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Main", 2, 5)
            .with_lookup_sheet("RefData");
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        let keys = collect_multi_conditional_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].sheet.as_deref(), Some("RefData"));
    }

    // ===== multi-key row bounds =====

    fn multi_keys_for(formula: &str) -> Vec<crate::prelude::MultiConditionalAggKey> {
        let ast = xlstream_parse::parse(formula).unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 9);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        collect_multi_conditional_keys(&rewritten)
    }

    #[test]
    fn collect_multi_keys_whole_column_ranges_have_no_row_bounds() {
        let keys = multi_keys_for("COUNTIF(A:A,\"x\")");
        assert_eq!(keys.len(), 1);
        assert_eq!((keys[0].start_row, keys[0].end_row), (None, None));
    }

    #[test]
    fn collect_multi_keys_bounded_countif_captures_row_bounds() {
        let keys = multi_keys_for("COUNTIF(A2:A11,\"x\")");
        assert_eq!(keys.len(), 1);
        assert_eq!((keys[0].start_row, keys[0].end_row), (Some(2), Some(11)));
    }

    #[test]
    fn collect_multi_keys_bounded_sumifs_captures_row_bounds() {
        let keys = multi_keys_for("SUMIFS(C2:C11,A2:A11,\"x\",B2:B11,5)");
        assert_eq!(keys.len(), 1);
        assert_eq!((keys[0].start_row, keys[0].end_row), (Some(2), Some(11)));
    }

    #[test]
    fn collect_multi_keys_bounded_minifs_captures_row_bounds() {
        let keys = multi_keys_for("MINIFS(C2:C11,A2:A11,\"x\")");
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Min);
        assert_eq!((keys[0].start_row, keys[0].end_row), (Some(2), Some(11)));
    }

    #[test]
    fn collect_multi_keys_rejects_mismatched_ifs_row_bounds() {
        // Excel returns #VALUE! for incongruent *IFS range shapes; no key
        // means the eval-side builtin errors before any lookup.
        let keys = multi_keys_for("SUMIFS(C2:C11,A2:A12,\"x\")");
        assert!(keys.is_empty());
    }

    #[test]
    fn collect_multi_keys_rejects_mixed_whole_column_and_bounded_ifs() {
        let keys = multi_keys_for("COUNTIFS(A2:A11,\"x\",B:B,\"y\")");
        assert!(keys.is_empty());
    }

    #[test]
    fn collect_multi_keys_rejects_offset_sumif_value_range() {
        // Excel would resize B5:B14 to the criteria shape and pair rows
        // with an offset; same-row streaming cannot express that, so the
        // formula is refused instead of silently mis-paired.
        let keys = multi_keys_for("SUMIF(A2:A11,\"x\",B5:B14)");
        assert!(keys.is_empty());
    }

    #[test]
    fn collect_multi_keys_accepts_sumif_value_range_with_same_start() {
        // Excel resizes the value range to the criteria shape from its
        // top-left cell, so a shorter value range with the same start row
        // pairs identically.
        let keys = multi_keys_for("SUMIF(A2:A11,\"x\",B2:B5)");
        assert_eq!(keys.len(), 1);
        assert_eq!((keys[0].start_row, keys[0].end_row), (Some(2), Some(11)));
    }

    // ===== is_prelude_evaluable =====

    #[test]
    fn is_prelude_evaluable_literal() {
        let ast = xlstream_parse::parse("42").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_cell_ref() {
        let ast = xlstream_parse::parse("A1+B1").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_function() {
        let ast = xlstream_parse::parse("IF(A1>0,B1,C1)").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_accepts_cross_sheet_ref() {
        let ast = xlstream_parse::parse("Sheet2!A1").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_accepts_range_ref() {
        let ast = xlstream_parse::parse("SUM(A1:A10)").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_accepts_vlookup() {
        let ast = xlstream_parse::parse("VLOOKUP(A1,B:C,2,FALSE)").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_accepts_today() {
        let ast = xlstream_parse::parse("TODAY()").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_accepts_now() {
        let ast = xlstream_parse::parse("NOW()").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_rejects_prelude_ref() {
        let ast = xlstream_parse::parse("SUM(A:A)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        assert!(!is_prelude_evaluable(&rewritten));
    }

    #[test]
    fn is_prelude_evaluable_rejects_nested_prelude_ref() {
        // A2 is row-local at row 2; SUM(B:B) becomes a PreludeRef after rewrite.
        let ast = xlstream_parse::parse("A2+SUM(B:B)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict, &real_meta);
        assert!(!is_prelude_evaluable(&rewritten));
    }

    #[test]
    fn is_prelude_evaluable_unary() {
        let ast = xlstream_parse::parse("-A1").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    // ===== evaluate_row_formulas =====

    #[test]
    fn evaluate_row_formulas_skips_non_formula_rows() {
        let ast = xlstream_parse::parse("A1*2").unwrap();
        let mut col_asts = HashMap::new();
        col_asts.insert(1u32, ast);
        let mut formula_rows = HashMap::new();
        formula_rows.insert(1u32, HashSet::from([0u32, 1]));
        let ctx = SheetFormulaCtx {
            col_asts: &col_asts,
            row_overrides: &HashMap::new(),
            topo_order: &[1],
            formula_rows: &formula_rows,
        };
        let prelude = crate::Prelude::empty();
        let interp = crate::Interpreter::new(&prelude);
        let empty_agg: HashSet<u32> = HashSet::new();
        let mut errored = HashSet::new();

        let row = vec![Value::Number(5.0), Value::Empty];
        let result =
            evaluate_row_formulas(row, 2, &ctx, &interp, &empty_agg, &mut errored).unwrap();
        assert_eq!(result[1], Value::Empty);
    }

    #[test]
    fn evaluate_row_formulas_evaluates_formula_rows() {
        let ast = xlstream_parse::parse("A1*2").unwrap();
        let mut col_asts = HashMap::new();
        col_asts.insert(1u32, ast);
        let mut formula_rows = HashMap::new();
        formula_rows.insert(1u32, HashSet::from([0u32, 1]));
        let ctx = SheetFormulaCtx {
            col_asts: &col_asts,
            row_overrides: &HashMap::new(),
            topo_order: &[1],
            formula_rows: &formula_rows,
        };
        let prelude = crate::Prelude::empty();
        let interp = crate::Interpreter::new(&prelude);
        let empty_agg: HashSet<u32> = HashSet::new();
        let mut errored = HashSet::new();

        let row = vec![Value::Number(5.0), Value::Empty];
        let result =
            evaluate_row_formulas(row, 1, &ctx, &interp, &empty_agg, &mut errored).unwrap();
        assert_eq!(result[1], Value::Number(10.0));
    }

    #[test]
    fn evaluate_row_formulas_preserves_cached_values() {
        let ast = xlstream_parse::parse("A1*2").unwrap();
        let mut col_asts = HashMap::new();
        col_asts.insert(1u32, ast);
        let mut formula_rows = HashMap::new();
        formula_rows.insert(1u32, HashSet::from([1u32]));
        let ctx = SheetFormulaCtx {
            col_asts: &col_asts,
            row_overrides: &HashMap::new(),
            topo_order: &[1],
            formula_rows: &formula_rows,
        };
        let prelude = crate::Prelude::empty();
        let interp = crate::Interpreter::new(&prelude);
        let empty_agg: HashSet<u32> = HashSet::new();
        let mut errored = HashSet::new();

        let row = vec![Value::Number(5.0), Value::Number(99.0)];
        let result =
            evaluate_row_formulas(row, 1, &ctx, &interp, &empty_agg, &mut errored).unwrap();
        assert_eq!(result[1], Value::Number(99.0));
    }

    #[test]
    fn evaluate_row_formulas_errors_on_unevaluable_aggregate_col() {
        let ast = xlstream_parse::parse("SUM(A:A)").unwrap();
        let ctx_c = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 2);
        let verdict = xlstream_parse::classify(&ast, &ctx_c, &real_meta);
        let rewritten = xlstream_parse::rewrite(ast, &ctx_c, &verdict, &real_meta);

        let mut col_asts = HashMap::new();
        col_asts.insert(1u32, rewritten);
        let mut formula_rows = HashMap::new();
        formula_rows.insert(1u32, HashSet::from([1u32]));
        let ctx = SheetFormulaCtx {
            col_asts: &col_asts,
            row_overrides: &HashMap::new(),
            topo_order: &[1],
            formula_rows: &formula_rows,
        };
        let prelude = crate::Prelude::empty();
        let interp = crate::Interpreter::new(&prelude);
        let agg_cols: HashSet<u32> = HashSet::from([1]);
        let mut errored = HashSet::new();

        let row = vec![Value::Number(5.0), Value::Empty];
        let result = evaluate_row_formulas(row, 1, &ctx, &interp, &agg_cols, &mut errored);
        assert!(result.is_err());
    }

    // ===== is_prelude_evaluable: coverage gaps =====

    #[test]
    fn is_prelude_evaluable_bool_literal() {
        let ast = xlstream_parse::parse("TRUE").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_text_literal() {
        let ast = xlstream_parse::parse("\"hello\"").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }

    #[test]
    fn is_prelude_evaluable_array_literal() {
        let ast = xlstream_parse::parse("{1,2;3,4}").unwrap();
        assert!(is_prelude_evaluable(&ast));
    }
}
