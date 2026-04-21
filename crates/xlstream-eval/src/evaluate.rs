//! The [`evaluate`] entry point and [`EvaluateSummary`] return type.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use xlstream_core::{col_row_to_a1, Value, XlStreamError};
use xlstream_io::{Reader, Writer};
use xlstream_parse::{
    classify, collect_lookup_keys, extract_references, parse, resolve_named_ranges, rewrite,
    AggregateKey, Ast, Classification, ClassificationContext, LookupKey, Reference,
};

use crate::interp::Interpreter;
use crate::prelude::Prelude;
use crate::scope::RowScope;
use crate::topo::topo_sort;

/// Message type sent from parallel workers to the writer loop.
type WorkerMsg = Result<(u32, Vec<Value>, u64), XlStreamError>;

/// Summary of a completed evaluation run.
///
/// # Examples
///
/// ```
/// use xlstream_eval::EvaluateSummary;
/// let s = EvaluateSummary::default();
/// assert_eq!(s.rows_processed, 0);
/// assert_eq!(s.formulas_evaluated, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvaluateSummary {
    /// Total rows written across all sheets (including header rows).
    pub rows_processed: u64,
    /// Total formula cells evaluated.
    pub formulas_evaluated: u64,
    /// Wall-clock duration of the full evaluation run.
    pub duration: std::time::Duration,
}

/// Pre-computed evaluation plan: prelude, ASTs, topo order, and sheet metadata.
struct EvalPlan {
    prelude: Prelude,
    col_asts: HashMap<u32, Ast>,
    topo_order: Vec<u32>,
    main_sheet: Option<String>,
    sheet_names: Vec<String>,
    /// Data rows counted during prelude (or fallback count). Used by the
    /// parallel streamer to decide chunk sizes and dispatch logic.
    total_data_rows: u32,
}

/// Evaluate every formula in `input`, write results to `output`, and return
/// an [`EvaluateSummary`].
///
/// Finds the first sheet that contains formulas, classifies them, builds an
/// intra-row topo evaluation order, then streams rows through the interpreter
/// and writes results. All other sheets are copied verbatim.
///
/// `workers` controls parallelism:
/// - `None` — auto-detect via `num_cpus::get()`.
/// - `Some(1)` — force single-threaded.
/// - `Some(n)` where `n > 1` — use `n` worker threads if the main sheet
///   has >= 10,000 data rows; otherwise fall back to single-threaded.
///
/// # Errors
///
/// - [`XlStreamError::Xlsx`] if `input` cannot be opened or parsed.
/// - [`XlStreamError::XlsxWrite`] if `output` cannot be written.
/// - [`XlStreamError::FormulaParse`] if a formula cannot be parsed.
/// - [`XlStreamError::Unsupported`] if a formula cannot be streamed.
/// - [`XlStreamError::CircularReference`] if formula columns form a cycle.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_eval::evaluate;
/// let err = evaluate(Path::new("missing.xlsx"), Path::new("out.xlsx"), None).unwrap_err();
/// assert!(matches!(err, xlstream_core::XlStreamError::Xlsx(_)));
/// ```
#[must_use = "evaluation results must be inspected for errors"]
pub fn evaluate(
    input: &Path,
    output: &Path,
    workers: Option<usize>,
) -> Result<EvaluateSummary, XlStreamError> {
    let start = Instant::now();
    let (plan, mut reader) = build_plan(input)?;
    let mut writer = Writer::create(output)?;

    let num_workers = workers.unwrap_or_else(num_cpus::get).max(1);

    let use_parallel = num_workers > 1
        && plan.main_sheet.is_some()
        && !plan.col_asts.is_empty()
        && plan.total_data_rows >= 10_000;

    let (rows_processed, formulas_evaluated) = if use_parallel {
        let main_sheet_name = plan
            .main_sheet
            .as_ref()
            .ok_or_else(|| XlStreamError::Internal("no main sheet".into()))?
            .clone();

        tracing::info!(
            workers = num_workers,
            total_rows = plan.total_data_rows,
            "parallel evaluation: spawning workers"
        );

        // Write non-main sheets first (passthrough).
        for name in &plan.sheet_names {
            if Some(name.as_str()) != plan.main_sheet.as_deref() {
                let mut sh = writer.add_sheet(name)?;
                let mut stream = reader.cells(name)?;
                while let Some((row_idx, row_values)) = stream.next_row()? {
                    sh.write_row(row_idx, &row_values)?;
                }
                drop(sh);
            }
        }

        let prelude = Arc::new(plan.prelude);
        let col_asts = Arc::new(plan.col_asts);

        stream_parallel(
            input,
            &mut writer,
            &prelude,
            &col_asts,
            &plan.topo_order,
            &main_sheet_name,
            plan.total_data_rows,
            num_workers,
        )?
    } else {
        tracing::debug!("single-threaded evaluation");
        stream_single_threaded(&mut reader, &mut writer, &plan)?
    };

    writer.finish()?;
    Ok(EvaluateSummary { rows_processed, formulas_evaluated, duration: start.elapsed() })
}

/// Build the full evaluation plan: open reader, find main sheet, parse +
/// classify + topo sort formulas, run prelude, load lookups.
#[allow(clippy::too_many_lines)]
fn build_plan(input: &Path) -> Result<(EvalPlan, Reader), XlStreamError> {
    let mut reader = Reader::open(input)?;
    let sheet_names = reader.sheet_names();
    let named_ranges: HashMap<String, String> =
        reader.defined_names().into_iter().map(|(k, v)| (k.to_ascii_lowercase(), v)).collect();

    // Find first sheet with formulas.
    let mut main_sheet: Option<String> = None;
    let mut main_formulas: Vec<(u32, u32, String)> = Vec::new();
    for name in &sheet_names {
        let formulas = reader.formulas(name)?;
        if !formulas.is_empty() {
            main_sheet = Some(name.clone());
            main_formulas = formulas;
            break;
        }
    }

    // Parse + classify formula columns; build topo order.
    let (topo_order, col_asts, lookup_keys) = if let Some(ref main) = main_sheet {
        build_eval_plan(main, &main_formulas, &sheet_names, &named_ranges)?
    } else {
        (Vec::new(), HashMap::new(), Vec::new())
    };

    // Collect aggregate keys from all rewritten ASTs and run prelude.
    let all_agg_keys: Vec<AggregateKey> = {
        let mut seen = std::collections::HashSet::new();
        col_asts
            .values()
            .flat_map(crate::prelude_plan::collect_aggregate_keys)
            .filter(|k| seen.insert(k.clone()))
            .collect()
    };

    // Collect multi-conditional keys (SUMIFS/COUNTIFS/AVERAGEIFS).
    let all_multi_keys: Vec<crate::prelude::MultiConditionalAggKey> = {
        let mut seen = std::collections::HashSet::new();
        col_asts
            .values()
            .flat_map(crate::prelude_plan::collect_multi_conditional_keys)
            .filter(|k| seen.insert(k.clone()))
            .collect()
    };

    // Collect bounded range keys for range-expanding functions.
    let all_range_keys: Vec<crate::prelude::BoundedRangeKey> = {
        let mut seen = std::collections::HashSet::new();
        col_asts
            .values()
            .flat_map(crate::prelude_plan::collect_bounded_range_keys)
            .filter(|k| seen.insert(k.clone()))
            .collect()
    };

    let has_aggregates =
        !all_agg_keys.is_empty() || !all_multi_keys.is_empty() || !all_range_keys.is_empty();

    let (agg_prelude, total_data_rows) = if !has_aggregates {
        let count =
            if let Some(ref main) = main_sheet { count_data_rows(&mut reader, main)? } else { 0 };
        (Prelude::empty(), count)
    } else if let Some(ref main) = main_sheet {
        crate::prelude_plan::execute_prelude(
            &mut reader,
            main,
            &all_agg_keys,
            &all_multi_keys,
            &all_range_keys,
        )?
    } else {
        (Prelude::empty(), 0)
    };

    // Cross-sheet bounded ranges in range-expanding functions (e.g.
    // NETWORKDAYS holidays) need the referenced sheet loaded as a lookup
    // sheet so expand_range can resolve them.
    let mut extended_lookup_keys = lookup_keys;
    for rk in &all_range_keys {
        if let Some(ref sheet_name) = rk.sheet {
            let already_present =
                extended_lookup_keys.iter().any(|lk| lk.sheet.eq_ignore_ascii_case(sheet_name));
            if !already_present {
                extended_lookup_keys.push(xlstream_parse::LookupKey {
                    kind: xlstream_parse::LookupKind::VLookup,
                    sheet: sheet_name.clone(),
                    key_index: 1,
                    value_index: 1,
                });
            }
        }
    }
    let lookup_sheets = crate::lookup::load_lookup_sheets(&extended_lookup_keys, &mut reader)?;
    let prelude = if lookup_sheets.is_empty() {
        agg_prelude
    } else {
        agg_prelude.with_lookup_sheets(lookup_sheets)
    };

    let plan = EvalPlan { prelude, col_asts, topo_order, main_sheet, sheet_names, total_data_rows };
    Ok((plan, reader))
}

/// Count data rows (non-header) in the main sheet without computing aggregates.
///
/// Used when no prelude pass is needed but the caller still wants a row count.
fn count_data_rows(reader: &mut Reader, main_sheet: &str) -> Result<u32, XlStreamError> {
    let mut stream = reader.cells(main_sheet)?;
    let mut count: u32 = 0;
    let mut header_skipped = false;
    while let Some((_row_idx, _)) = stream.next_row()? {
        if !header_skipped {
            header_skipped = true;
            continue;
        }
        count = count.saturating_add(1);
    }
    Ok(count)
}

/// Stream all sheets single-threaded: evaluate formula columns on the main
/// sheet and copy all other sheets verbatim.
///
/// Returns `(rows_processed, formulas_evaluated)`.
fn stream_single_threaded(
    reader: &mut Reader,
    writer: &mut Writer,
    plan: &EvalPlan,
) -> Result<(u64, u64), XlStreamError> {
    let interp = Interpreter::new(&plan.prelude);
    let mut rows_processed: u64 = 0;
    let mut formulas_evaluated: u64 = 0;

    for name in &plan.sheet_names {
        let mut sh = writer.add_sheet(name)?;
        let mut stream = reader.cells(name)?;
        let is_main = plan.main_sheet.as_deref() == Some(name.as_str());
        let mut header_pending = is_main;

        while let Some((row_idx, mut row_values)) = stream.next_row()? {
            if header_pending {
                // First row of the main sheet is headers -- write verbatim.
                sh.write_row(row_idx, &row_values)?;
                header_pending = false;
                rows_processed += 1;
                continue;
            }

            if is_main {
                for &fcol in &plan.topo_order {
                    let Some(ast) = plan.col_asts.get(&fcol) else {
                        continue;
                    };
                    let fcol_idx = fcol as usize;
                    if fcol_idx >= row_values.len() {
                        row_values.resize(fcol_idx + 1, Value::Empty);
                    }
                    // Inner block ensures the immutable borrow of row_values
                    // through RowScope is released before the mutation below.
                    let result = {
                        let scope = RowScope::new(&row_values, row_idx);
                        interp.eval(ast.root(), &scope)
                    };
                    row_values[fcol_idx] = result;
                    formulas_evaluated += 1;
                }
            }

            sh.write_row(row_idx, &row_values)?;
            rows_processed += 1;
        }

        drop(sh);
    }

    Ok((rows_processed, formulas_evaluated))
}

/// Parallel row evaluation. Spawns `num_workers` threads, each with its
/// own calamine reader seeked to its row range. Results flow through a
/// bounded channel; the caller drains them in row order.
///
/// Non-main sheets must be written by the caller before calling this.
fn stream_parallel(
    input: &Path,
    output: &mut Writer,
    prelude: &Arc<Prelude>,
    col_asts: &Arc<HashMap<u32, Ast>>,
    topo_order: &[u32],
    main_sheet: &str,
    total_data_rows: u32,
    num_workers: usize,
) -> Result<(u64, u64), XlStreamError> {
    let chunk_size = (total_data_rows as usize).div_ceil(num_workers);

    // Header row: read from a fresh reader, write before spawning workers.
    let header_row = {
        let mut header_reader = Reader::open(input)?;
        let mut stream = header_reader.cells(main_sheet)?;
        stream
            .next_row()?
            .ok_or_else(|| XlStreamError::Internal("main sheet has no rows".into()))?
    };

    let mut sh = output.add_sheet(main_sheet)?;
    sh.write_row(header_row.0, &header_row.1)?;

    let (tx, rx) = crossbeam_channel::bounded::<WorkerMsg>(num_workers * 1024);

    let input_path = input.to_path_buf();
    let main_name = main_sheet.to_string();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_workers)
        .build()
        .map_err(|e| XlStreamError::Internal(format!("failed to build thread pool: {e}")))?;

    for worker_id in 0..num_workers {
        let tx = tx.clone();
        let prelude = Arc::clone(prelude);
        let col_asts = Arc::clone(col_asts);
        let topo_order = topo_order.to_owned();
        let input_path = input_path.clone();
        let main_name = main_name.clone();

        let start_row = 1 + u32::try_from(worker_id * chunk_size).unwrap_or(u32::MAX - 1);
        let end_row = u32::try_from(1 + (worker_id + 1) * chunk_size)
            .unwrap_or(u32::MAX)
            .min(1 + total_data_rows);

        pool.spawn(move || {
            if let Err(e) = run_worker(
                &input_path,
                &main_name,
                &prelude,
                &col_asts,
                &topo_order,
                start_row,
                end_row,
                &tx,
            ) {
                let _ = tx.send(Err(e));
            }
        });
    }

    drop(tx);

    let mut rows_processed: u64 = 1; // header already written
    let mut formulas_evaluated: u64 = 0;
    let mut expected_row: u32 = 1;
    let mut buffer: BTreeMap<u32, (Vec<Value>, u64)> = BTreeMap::new();
    let mut first_error: Option<XlStreamError> = None;

    for msg in &rx {
        match msg {
            Ok((row_idx, values, formulas_count)) => {
                buffer.insert(row_idx, (values, formulas_count));
                while let Some((vals, fc)) = buffer.remove(&expected_row) {
                    sh.write_row(expected_row, &vals)?;
                    rows_processed += 1;
                    formulas_evaluated += fc;
                    expected_row += 1;
                }
            }
            Err(e) => {
                if first_error.is_none() {
                    first_error = Some(e);
                }
            }
        }
    }

    while let Some((row_idx, (vals, fc))) = buffer.pop_first() {
        if first_error.is_some() {
            break;
        }
        sh.write_row(row_idx, &vals)?;
        rows_processed += 1;
        formulas_evaluated += fc;
    }

    drop(sh);

    if let Some(e) = first_error {
        return Err(e);
    }

    Ok((rows_processed, formulas_evaluated))
}

/// Worker: open reader, seek to `start_row`, evaluate
/// [`start_row`, `end_row`), send each row through the channel.
fn run_worker(
    input: &Path,
    main_sheet: &str,
    prelude: &Prelude,
    col_asts: &HashMap<u32, Ast>,
    topo_order: &[u32],
    start_row: u32,
    end_row: u32,
    tx: &crossbeam_channel::Sender<WorkerMsg>,
) -> Result<(), XlStreamError> {
    tracing::debug!(start_row, end_row, rows = end_row - start_row, "worker starting");

    let mut reader = Reader::open(input)?;
    let mut stream = reader.cells(main_sheet)?;
    stream.seek_to_row(start_row)?;

    let interp = Interpreter::new(prelude);

    while let Some((row_idx, mut row_values)) = stream.next_row()? {
        if row_idx >= end_row {
            break;
        }

        let mut formulas_in_row: u64 = 0;
        for &fcol in topo_order {
            let Some(ast) = col_asts.get(&fcol) else {
                continue;
            };
            let fcol_idx = fcol as usize;
            if fcol_idx >= row_values.len() {
                row_values.resize(fcol_idx + 1, Value::Empty);
            }
            let result = {
                let scope = RowScope::new(&row_values, row_idx);
                interp.eval(ast.root(), &scope)
            };
            row_values[fcol_idx] = result;
            formulas_in_row += 1;
        }

        if tx.send(Ok((row_idx, row_values, formulas_in_row))).is_err() {
            return Ok(());
        }
    }

    Ok(())
}

/// Parse + classify all formula columns for `main_sheet`. Returns the
/// topo-sorted column evaluation order and the per-column [`Ast`] cache.
#[allow(clippy::type_complexity)]
fn build_eval_plan(
    main_sheet: &str,
    all_formulas: &[(u32, u32, String)],
    all_sheet_names: &[String],
    named_ranges: &HashMap<String, String>,
) -> Result<(Vec<u32>, HashMap<u32, Ast>, Vec<LookupKey>), XlStreamError> {
    // First occurrence per column (0-based col -> (0-based row, formula text)).
    let mut col_formula: HashMap<u32, (u32, String)> = HashMap::new();
    for (row, col, text) in all_formulas {
        col_formula.entry(*col).or_insert_with(|| (*row, text.clone()));
    }

    let mut col_asts: HashMap<u32, Ast> = HashMap::new();
    let mut all_lookup_keys: Vec<LookupKey> = Vec::new();
    for (&col, (row, text)) in &col_formula {
        // calamine strips the leading '='; strip defensively to handle either.
        // rust_xlsxwriter prepends `_xlfn.` to "future" Excel functions
        // (CONCAT, TEXTJOIN, etc.); strip that prefix so the parser sees
        // the canonical function name.
        let formula_str = text.strip_prefix('=').unwrap_or(text.as_str());
        let formula_str = strip_xlfn_prefix(formula_str);
        let ast = parse(&formula_str)?;
        let ast = resolve_named_ranges(ast, named_ranges);

        // Register all non-main sheets as lookup sheets so the classifier
        // accepts cross-sheet range refs in lookup functions.
        let mut ctx = ClassificationContext::for_cell(main_sheet, row + 1, col + 1);
        for name in all_sheet_names {
            if !name.eq_ignore_ascii_case(main_sheet) {
                ctx = ctx.with_lookup_sheet(name);
            }
        }
        let verdict = classify(&ast, &ctx);
        if let Classification::Unsupported(ref reason) = verdict {
            return Err(XlStreamError::Unsupported {
                address: format!("{}!{}", main_sheet, col_row_to_a1(col + 1, row + 1)),
                formula: text.clone(),
                reason: reason.to_string(),
                doc_link: reason.doc_link(),
            });
        }

        // Rewrite aggregate sub-expressions to PreludeRef nodes.
        // Lookups stay as Function nodes (rewrite_lookup returns None).
        let rewritten = rewrite(ast, &ctx, &verdict);
        all_lookup_keys.extend(collect_lookup_keys(&rewritten));
        col_asts.insert(col, rewritten);
    }

    // Build intra-row dependency graph; extract formula-column-to-formula-column edges.
    let formula_cols: HashSet<u32> = col_asts.keys().copied().collect();
    let deps: Vec<(u32, Vec<u32>)> = col_asts
        .iter()
        .map(|(&col, ast)| {
            let refs = extract_references(ast);
            let dep_cols: Vec<u32> = refs
                .cells
                .iter()
                .filter_map(|r| match r {
                    Reference::Cell { col: ref_col, .. } => {
                        // Convert 1-based reference to 0-based column index.
                        Some(ref_col.saturating_sub(1))
                    }
                    _ => None,
                })
                .collect();
            (col, dep_cols)
        })
        .collect();

    let topo_order = topo_sort(&deps, &formula_cols)?;
    Ok((topo_order, col_asts, all_lookup_keys))
}

/// Strip `_xlfn.` (and `_xlfn._xlws.`) prefixes that `rust_xlsxwriter`
/// injects for "future" Excel functions (CONCAT, TEXTJOIN, IFS, etc.).
///
/// These prefixes are internal to the xlsx format and are not part of the
/// canonical function name. Calamine preserves them verbatim, so we strip
/// them here before parsing.
fn strip_xlfn_prefix(formula: &str) -> String {
    formula.replace("_xlfn._xlws.", "").replace("_xlfn.", "")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn summary_default_fields_are_zero() {
        let s = EvaluateSummary::default();
        assert_eq!(s.rows_processed, 0);
        assert_eq!(s.formulas_evaluated, 0);
        assert_eq!(s.duration, std::time::Duration::ZERO);
    }

    #[test]
    fn evaluate_nonexistent_file_returns_xlsx_error() {
        let err = evaluate(Path::new("this_file_does_not_exist.xlsx"), Path::new("out.xlsx"), None)
            .unwrap_err();
        assert!(matches!(err, XlStreamError::Xlsx(_)), "expected Xlsx error, got {err:?}");
    }

    #[test]
    fn ast_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<xlstream_parse::Ast>();
    }
}
