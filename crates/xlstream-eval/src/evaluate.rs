//! The [`evaluate`] entry point and [`EvaluateSummary`] return type.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use xlstream_core::{col_row_to_a1, EvaluateOptions, Value, XlStreamError, PARALLEL_ROW_THRESHOLD};
use xlstream_io::{Reader, Writer};
use xlstream_parse::{
    classify, collect_lookup_keys, extract_references, parse, resolve_named_ranges, rewrite,
    AggregateKey, Ast, Classification, ClassificationContext, LookupKey, NodeRef, NodeView,
    Reference,
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
    row_overrides: HashMap<u32, HashMap<u32, Ast>>,
    topo_order: Vec<u32>,
    self_referential_cols: HashSet<u32>,
    secondary_plans: HashMap<String, SheetEvalPlan>,
    main_sheet: Option<String>,
    sheet_names: Vec<String>,
    /// Data rows counted during prelude (or fallback count). Used by the
    /// parallel streamer to decide chunk sizes and dispatch logic.
    total_data_rows: u32,
    iterative_calc: bool,
    max_iterations: u32,
    max_change: f64,
}

/// Per-sheet formula evaluation data for secondary (non-main) sheets.
struct SheetEvalPlan {
    col_asts: HashMap<u32, Ast>,
    row_overrides: HashMap<u32, HashMap<u32, Ast>>,
    topo_order: Vec<u32>,
    self_referential_cols: HashSet<u32>,
}

/// Evaluate every formula in `input`, write results to `output`, and return
/// an [`EvaluateSummary`].
///
/// Finds the first sheet that contains formulas, classifies them, builds an
/// intra-row topo evaluation order, then streams rows through the interpreter
/// and writes results. All other sheets are copied verbatim.
///
/// `options` controls parallelism and iterative calculation behavior.
/// See [`EvaluateOptions`] for details.
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
/// use xlstream_eval::{evaluate, EvaluateOptions};
/// let err = evaluate(
///     Path::new("missing.xlsx"),
///     Path::new("out.xlsx"),
///     &EvaluateOptions::default(),
/// ).unwrap_err();
/// assert!(matches!(err, xlstream_core::XlStreamError::Xlsx(_)));
/// ```
#[must_use = "evaluation results must be inspected for errors"]
pub fn evaluate(
    input: &Path,
    output: &Path,
    options: &EvaluateOptions,
) -> Result<EvaluateSummary, XlStreamError> {
    let start = Instant::now();
    let (mut plan, mut reader) = build_plan(input)?;
    plan.iterative_calc = options.iterative_calc;
    plan.max_iterations = options.max_iterations;
    plan.max_change = options.max_change;
    let mut writer = Writer::create(output)?;

    let num_workers = options.workers.unwrap_or_else(num_cpus::get).max(1);

    let use_parallel = num_workers > 1
        && plan.main_sheet.is_some()
        && !plan.col_asts.is_empty()
        && plan.total_data_rows >= PARALLEL_ROW_THRESHOLD;

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

        // Write non-main sheets — evaluate formulas on secondary formula sheets.
        for name in &plan.sheet_names {
            if Some(name.as_str()) != plan.main_sheet.as_deref() {
                let mut sh = writer.add_sheet(name)?;
                let mut stream = reader.cells(name)?;

                if let Some(sp) = plan.secondary_plans.get(name.as_str()) {
                    let sec_interp = Interpreter::new(&plan.prelude);
                    let sec_options = EvaluateOptions {
                        workers: None,
                        iterative_calc: plan.iterative_calc,
                        max_iterations: plan.max_iterations,
                        max_change: plan.max_change,
                    };
                    let mut header_pending = true;
                    while let Some((row_idx, mut row_values)) = stream.next_row()? {
                        if header_pending {
                            sh.write_row(row_idx, &row_values)?;
                            header_pending = false;
                            continue;
                        }
                        for &fcol in &sp.topo_order {
                            let ast = sp
                                .row_overrides
                                .get(&fcol)
                                .and_then(|overrides| overrides.get(&row_idx))
                                .or_else(|| sp.col_asts.get(&fcol));
                            let Some(ast) = ast else { continue };
                            eval_column(
                                &sec_interp,
                                ast,
                                &mut row_values,
                                row_idx,
                                fcol,
                                sp.self_referential_cols.contains(&fcol),
                                &sec_options,
                            );
                        }
                        sh.write_row(row_idx, &row_values)?;
                    }
                } else {
                    while let Some((row_idx, row_values)) = stream.next_row()? {
                        sh.write_row(row_idx, &row_values)?;
                    }
                }

                drop(sh);
            }
        }

        let eval_options = Arc::new(EvaluateOptions {
            workers: options.workers,
            iterative_calc: plan.iterative_calc,
            max_iterations: plan.max_iterations,
            max_change: plan.max_change,
        });
        let self_referential_cols_arc = Arc::new(plan.self_referential_cols);
        let prelude = Arc::new(plan.prelude);
        let col_asts = Arc::new(plan.col_asts);
        let row_overrides_arc = Arc::new(plan.row_overrides);

        stream_parallel(
            input,
            &mut writer,
            &prelude,
            &col_asts,
            &row_overrides_arc,
            &plan.topo_order,
            &self_referential_cols_arc,
            &eval_options,
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

    let table_infos: HashMap<String, xlstream_parse::TableInfo> = reader
        .table_metadata()
        .unwrap_or_else(|_| Vec::new())
        .into_iter()
        .map(|m| {
            (
                m.name.to_ascii_lowercase(),
                xlstream_parse::TableInfo {
                    sheet_name: m.sheet_name,
                    columns: m.columns,
                    header_row: m.header_row,
                    data_start_row: m.data_start_row,
                    data_end_row: m.data_end_row,
                    start_col: m.start_col,
                },
            )
        })
        .collect();

    // Collect formulas from all sheets; first with formulas becomes main.
    let mut sheets_with_formulas = Vec::<(String, Vec<(u32, u32, String)>)>::new();
    for name in &sheet_names {
        let formulas = reader.formulas(name)?;
        if !formulas.is_empty() {
            sheets_with_formulas.push((name.clone(), formulas));
        }
    }
    let main_sheet = sheets_with_formulas.first().map(|(n, _)| n.clone());
    let main_formulas: Vec<(u32, u32, String)> =
        sheets_with_formulas.first().map(|(_, f)| f.clone()).unwrap_or_default();

    // Parse + classify formula columns; build topo order.
    let (topo_order, col_asts, row_overrides, lookup_keys, self_referential_cols) =
        if let Some(ref main) = main_sheet {
            build_eval_plan(main, &main_formulas, &sheet_names, &named_ranges, &table_infos)?
        } else {
            (Vec::new(), HashMap::new(), HashMap::new(), Vec::new(), HashSet::new())
        };

    // Build eval plans for secondary formula-bearing sheets.
    let mut secondary_plans: HashMap<String, SheetEvalPlan> = HashMap::new();
    let mut secondary_lookup_keys: Vec<LookupKey> = Vec::new();
    for (sheet_name, formulas) in sheets_with_formulas.iter().skip(1) {
        let (sec_topo, sec_asts, sec_overrides, lk, sec_self_ref_cols) =
            build_eval_plan(sheet_name, formulas, &sheet_names, &named_ranges, &table_infos)?;
        // Detect cross-sheet cell references and add as lookup keys.
        for ast in sec_asts.values() {
            let refs = extract_references(ast);
            for r in &refs.cells {
                if let Reference::Cell { sheet: Some(ref s), .. } = r {
                    let already =
                        secondary_lookup_keys.iter().any(|k| k.sheet.eq_ignore_ascii_case(s))
                            || lk.iter().any(|k| k.sheet.eq_ignore_ascii_case(s));
                    if !already {
                        secondary_lookup_keys.push(LookupKey {
                            kind: xlstream_parse::LookupKind::VLookup,
                            sheet: s.clone(),
                            key_index: 1,
                            value_index: 1,
                        });
                    }
                }
            }
        }
        for per_row in sec_overrides.values() {
            for ast in per_row.values() {
                let refs = extract_references(ast);
                for r in &refs.cells {
                    if let Reference::Cell { sheet: Some(ref s), .. } = r {
                        let already =
                            secondary_lookup_keys.iter().any(|k| k.sheet.eq_ignore_ascii_case(s))
                                || lk.iter().any(|k| k.sheet.eq_ignore_ascii_case(s));
                        if !already {
                            secondary_lookup_keys.push(LookupKey {
                                kind: xlstream_parse::LookupKind::VLookup,
                                sheet: s.clone(),
                                key_index: 1,
                                value_index: 1,
                            });
                        }
                    }
                }
            }
        }
        secondary_lookup_keys.extend(lk);
        secondary_plans.insert(
            sheet_name.clone(),
            SheetEvalPlan {
                col_asts: sec_asts,
                row_overrides: sec_overrides,
                topo_order: sec_topo,
                self_referential_cols: sec_self_ref_cols,
            },
        );
    }

    // Collect main sheet aggregate keys and run prelude.
    let main_agg_keys: Vec<AggregateKey> = {
        let mut seen = HashSet::new();
        let primary = col_asts.values();
        let overrides = row_overrides.values().flat_map(HashMap::values);
        primary
            .chain(overrides)
            .flat_map(crate::prelude_plan::collect_aggregate_keys)
            .filter(|k| seen.insert(k.clone()))
            .collect()
    };
    let main_multi_keys: Vec<crate::prelude::MultiConditionalAggKey> = {
        let mut seen = HashSet::new();
        let primary = col_asts.values();
        let overrides = row_overrides.values().flat_map(HashMap::values);
        primary
            .chain(overrides)
            .flat_map(crate::prelude_plan::collect_multi_conditional_keys)
            .filter(|k| seen.insert(k.clone()))
            .collect()
    };
    let main_range_keys: Vec<crate::prelude::BoundedRangeKey> = {
        let mut seen = HashSet::new();
        let primary = col_asts.values();
        let overrides = row_overrides.values().flat_map(HashMap::values);
        primary
            .chain(overrides)
            .flat_map(crate::prelude_plan::collect_bounded_range_keys)
            .filter(|k| seen.insert(k.clone()))
            .collect()
    };

    let has_main_aggregates =
        !main_agg_keys.is_empty() || !main_multi_keys.is_empty() || !main_range_keys.is_empty();

    let (mut merged_prelude, total_data_rows) = if !has_main_aggregates {
        let count =
            if let Some(ref main) = main_sheet { count_data_rows(&mut reader, main)? } else { 0 };
        (Prelude::empty(), count)
    } else if let Some(ref main) = main_sheet {
        crate::prelude_plan::execute_prelude(
            &mut reader,
            main,
            &main_agg_keys,
            &main_multi_keys,
            &main_range_keys,
        )?
    } else {
        (Prelude::empty(), 0)
    };

    // Run prelude for each secondary sheet that has aggregates, merge results.
    for (sheet_name, sp) in &secondary_plans {
        let sec_agg: Vec<AggregateKey> = {
            let mut seen = HashSet::new();
            let primary = sp.col_asts.values();
            let overrides = sp.row_overrides.values().flat_map(HashMap::values);
            primary
                .chain(overrides)
                .flat_map(crate::prelude_plan::collect_aggregate_keys)
                .filter(|k| seen.insert(k.clone()))
                .collect()
        };
        let sec_multi: Vec<crate::prelude::MultiConditionalAggKey> = {
            let mut seen = HashSet::new();
            let primary = sp.col_asts.values();
            let overrides = sp.row_overrides.values().flat_map(HashMap::values);
            primary
                .chain(overrides)
                .flat_map(crate::prelude_plan::collect_multi_conditional_keys)
                .filter(|k| seen.insert(k.clone()))
                .collect()
        };
        let sec_range: Vec<crate::prelude::BoundedRangeKey> = {
            let mut seen = HashSet::new();
            let primary = sp.col_asts.values();
            let overrides = sp.row_overrides.values().flat_map(HashMap::values);
            primary
                .chain(overrides)
                .flat_map(crate::prelude_plan::collect_bounded_range_keys)
                .filter(|k| seen.insert(k.clone()))
                .collect()
        };
        if !sec_agg.is_empty() || !sec_multi.is_empty() || !sec_range.is_empty() {
            let (sec_prelude, _) = crate::prelude_plan::execute_prelude(
                &mut reader,
                sheet_name,
                &sec_agg,
                &sec_multi,
                &sec_range,
            )?;
            merged_prelude.merge(sec_prelude);
        }
    }

    // Cross-sheet bounded ranges in range-expanding functions (e.g.
    // NETWORKDAYS holidays) need the referenced sheet loaded as a lookup
    // sheet so expand_range can resolve them.
    let mut extended_lookup_keys = lookup_keys;
    for rk in &main_range_keys {
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
    // Include lookup keys from secondary sheets.
    for lk in &secondary_lookup_keys {
        let already_present =
            extended_lookup_keys.iter().any(|k| k.sheet.eq_ignore_ascii_case(&lk.sheet));
        if !already_present {
            extended_lookup_keys.push(lk.clone());
        }
    }
    // Include cross-sheet bounded range keys from secondary sheets.
    for sp in secondary_plans.values() {
        let primary = sp.col_asts.values();
        let overrides = sp.row_overrides.values().flat_map(HashMap::values);
        let sec_range: Vec<crate::prelude::BoundedRangeKey> = primary
            .chain(overrides)
            .flat_map(crate::prelude_plan::collect_bounded_range_keys)
            .collect();
        for rk in &sec_range {
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
    }

    let lookup_sheets = crate::lookup::load_lookup_sheets(&extended_lookup_keys, &mut reader)?;
    let prelude = if lookup_sheets.is_empty() {
        merged_prelude
    } else {
        merged_prelude.with_lookup_sheets(lookup_sheets)
    };

    let plan = EvalPlan {
        prelude,
        col_asts,
        row_overrides,
        topo_order,
        self_referential_cols,
        secondary_plans,
        main_sheet,
        sheet_names,
        total_data_rows,
        iterative_calc: true,
        max_iterations: 100,
        max_change: 0.001,
    };
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
    let eval_options = EvaluateOptions {
        workers: None,
        iterative_calc: plan.iterative_calc,
        max_iterations: plan.max_iterations,
        max_change: plan.max_change,
    };
    let mut rows_processed: u64 = 0;
    let mut formulas_evaluated: u64 = 0;

    for name in &plan.sheet_names {
        let mut sh = writer.add_sheet(name)?;
        let mut stream = reader.cells(name)?;

        let is_main = plan.main_sheet.as_deref() == Some(name.as_str());
        #[allow(clippy::type_complexity)]
        let sheet_plan: Option<(
            &HashMap<u32, Ast>,
            &HashMap<u32, HashMap<u32, Ast>>,
            &[u32],
            &HashSet<u32>,
        )> = if is_main {
            if plan.col_asts.is_empty() {
                None
            } else {
                Some((
                    &plan.col_asts,
                    &plan.row_overrides,
                    &plan.topo_order,
                    &plan.self_referential_cols,
                ))
            }
        } else {
            plan.secondary_plans.get(name.as_str()).map(|sp| {
                (
                    &sp.col_asts,
                    &sp.row_overrides,
                    sp.topo_order.as_slice(),
                    &sp.self_referential_cols,
                )
            })
        };

        let mut header_pending = sheet_plan.is_some();

        while let Some((row_idx, mut row_values)) = stream.next_row()? {
            if header_pending {
                sh.write_row(row_idx, &row_values)?;
                header_pending = false;
                rows_processed += 1;
                continue;
            }

            if let Some((col_asts, row_overrides, topo_order, self_ref_cols)) = &sheet_plan {
                for &fcol in *topo_order {
                    let ast = row_overrides
                        .get(&fcol)
                        .and_then(|overrides| overrides.get(&row_idx))
                        .or_else(|| col_asts.get(&fcol));
                    let Some(ast) = ast else {
                        continue;
                    };
                    eval_column(
                        &interp,
                        ast,
                        &mut row_values,
                        row_idx,
                        fcol,
                        self_ref_cols.contains(&fcol),
                        &eval_options,
                    );
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
#[allow(clippy::too_many_arguments)] // worker contract: all args logically required
fn stream_parallel(
    input: &Path,
    output: &mut Writer,
    prelude: &Arc<Prelude>,
    col_asts: &Arc<HashMap<u32, Ast>>,
    row_overrides: &Arc<HashMap<u32, HashMap<u32, Ast>>>,
    topo_order: &[u32],
    self_referential_cols: &Arc<HashSet<u32>>,
    eval_options: &Arc<EvaluateOptions>,
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
        let row_overrides = Arc::clone(row_overrides);
        let self_ref_cols = Arc::clone(self_referential_cols);
        let opts = Arc::clone(eval_options);
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
                &row_overrides,
                &topo_order,
                &self_ref_cols,
                &opts,
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
#[allow(clippy::too_many_arguments)] // worker contract: all args logically required
fn run_worker(
    input: &Path,
    main_sheet: &str,
    prelude: &Prelude,
    col_asts: &HashMap<u32, Ast>,
    row_overrides: &HashMap<u32, HashMap<u32, Ast>>,
    topo_order: &[u32],
    self_referential_cols: &HashSet<u32>,
    options: &EvaluateOptions,
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
            let ast = row_overrides
                .get(&fcol)
                .and_then(|overrides| overrides.get(&row_idx))
                .or_else(|| col_asts.get(&fcol));
            let Some(ast) = ast else {
                continue;
            };
            eval_column(
                &interp,
                ast,
                &mut row_values,
                row_idx,
                fcol,
                self_referential_cols.contains(&fcol),
                options,
            );
            formulas_in_row += 1;
        }

        if tx.send(Ok((row_idx, row_values, formulas_in_row))).is_err() {
            return Ok(());
        }
    }

    Ok(())
}

/// Evaluate a single formula column for one row. If the column is
/// self-referential and iterative calc is enabled, loop until convergence
/// or `max_iterations`.
fn eval_column(
    interp: &Interpreter<'_>,
    ast: &Ast,
    row_values: &mut Vec<Value>,
    row_idx: u32,
    fcol: u32,
    is_self_ref: bool,
    options: &EvaluateOptions,
) {
    let fcol_idx = fcol as usize;
    if fcol_idx >= row_values.len() {
        row_values.resize(fcol_idx + 1, Value::Empty);
    }

    if !is_self_ref || !options.iterative_calc {
        let result = {
            let scope = RowScope::new(row_values, row_idx);
            interp.eval(ast.root(), &scope)
        };
        row_values[fcol_idx] = result;
        return;
    }

    let mut previous = row_values[fcol_idx].clone();
    for iteration in 0..options.max_iterations {
        let result = {
            let scope = RowScope::new(row_values, row_idx);
            interp.eval(ast.root(), &scope)
        };

        if matches!(result, Value::Error(_)) {
            row_values[fcol_idx] = result;
            return;
        }

        let Value::Number(current_num) = &result else {
            row_values[fcol_idx] = result;
            return;
        };

        if iteration > 0 {
            if let Value::Number(prev_num) = &previous {
                if (current_num - prev_num).abs() < options.max_change {
                    row_values[fcol_idx] = result;
                    return;
                }
            }
        }

        previous = result.clone();
        row_values[fcol_idx] = result;
    }
}

/// Parse + classify all formula columns for `main_sheet`. Returns the
/// topo-sorted column evaluation order and the per-column [`Ast`] cache.
#[allow(clippy::type_complexity, clippy::too_many_lines)]
fn build_eval_plan(
    main_sheet: &str,
    all_formulas: &[(u32, u32, String)],
    all_sheet_names: &[String],
    named_ranges: &HashMap<String, String>,
    table_infos: &HashMap<String, xlstream_parse::TableInfo>,
) -> Result<
    (Vec<u32>, HashMap<u32, Ast>, HashMap<u32, HashMap<u32, Ast>>, Vec<LookupKey>, HashSet<u32>),
    XlStreamError,
> {
    let mut col_formulas: HashMap<u32, Vec<(u32, String)>> = HashMap::new();
    for (row, col, text) in all_formulas {
        col_formulas.entry(*col).or_default().push((*row, text.clone()));
    }

    let mut col_asts: HashMap<u32, Ast> = HashMap::new();
    let mut row_overrides: HashMap<u32, HashMap<u32, Ast>> = HashMap::new();
    let mut all_lookup_keys: Vec<LookupKey> = Vec::new();

    for (&col, formulas) in &col_formulas {
        let (first_row, first_text) = &formulas[0];

        let formula_str = first_text.strip_prefix('=').unwrap_or(first_text.as_str());
        let formula_str = strip_xlfn_prefix(formula_str);
        let first_ast = parse(&formula_str)?;
        let first_ast = resolve_named_ranges(first_ast, named_ranges);
        let first_ast = xlstream_parse::resolve_table_references(
            first_ast,
            table_infos,
            Some(main_sheet),
            first_row + 1,
        );

        let mut ctx = ClassificationContext::for_cell(main_sheet, first_row + 1, col + 1);
        for name in all_sheet_names {
            if !name.eq_ignore_ascii_case(main_sheet) {
                ctx = ctx.with_lookup_sheet(name);
            }
        }
        let verdict = classify(&first_ast, &ctx);
        if let Classification::Unsupported(ref reason) = verdict {
            return Err(XlStreamError::Unsupported {
                address: format!("{}!{}", main_sheet, col_row_to_a1(col + 1, first_row + 1)),
                formula: first_text.clone(),
                reason: reason.to_string(),
                doc_link: reason.doc_link(),
            });
        }

        let rewritten = rewrite(first_ast.clone(), &ctx, &verdict);
        all_lookup_keys.extend(collect_lookup_keys(&rewritten));
        col_asts.insert(col, rewritten);

        for (row, text) in &formulas[1..] {
            if text == first_text {
                continue;
            }

            let formula_str = text.strip_prefix('=').unwrap_or(text.as_str());
            let formula_str = strip_xlfn_prefix(formula_str);
            let ast = parse(&formula_str)?;
            let ast = resolve_named_ranges(ast, named_ranges);
            let ast = xlstream_parse::resolve_table_references(
                ast,
                table_infos,
                Some(main_sheet),
                row + 1,
            );

            if ast_streaming_eq(first_ast.root(), ast.root()) {
                continue;
            }

            let mut row_ctx = ClassificationContext::for_cell(main_sheet, row + 1, col + 1);
            for name in all_sheet_names {
                if !name.eq_ignore_ascii_case(main_sheet) {
                    row_ctx = row_ctx.with_lookup_sheet(name);
                }
            }
            let row_verdict = classify(&ast, &row_ctx);
            if let Classification::Unsupported(ref reason) = row_verdict {
                return Err(XlStreamError::Unsupported {
                    address: format!("{}!{}", main_sheet, col_row_to_a1(col + 1, row + 1)),
                    formula: text.clone(),
                    reason: reason.to_string(),
                    doc_link: reason.doc_link(),
                });
            }

            let rewritten = rewrite(ast, &row_ctx, &row_verdict);
            all_lookup_keys.extend(collect_lookup_keys(&rewritten));
            row_overrides.entry(col).or_default().insert(*row, rewritten);
        }
    }

    let formula_cols: HashSet<u32> = col_asts.keys().copied().collect();
    let mut self_referential_cols: HashSet<u32> = HashSet::new();
    let deps: Vec<(u32, Vec<u32>)> = col_asts
        .iter()
        .map(|(&col, ast)| {
            let mut dep_cols: HashSet<u32> = HashSet::new();
            for r in &extract_references(ast).cells {
                if let Reference::Cell { col: ref_col, .. } = r {
                    dep_cols.insert(ref_col.saturating_sub(1));
                }
            }
            if let Some(overrides) = row_overrides.get(&col) {
                for override_ast in overrides.values() {
                    for r in &extract_references(override_ast).cells {
                        if let Reference::Cell { col: ref_col, .. } = r {
                            dep_cols.insert(ref_col.saturating_sub(1));
                        }
                    }
                }
            }
            if dep_cols.remove(&col) {
                self_referential_cols.insert(col);
            }
            (col, dep_cols.into_iter().collect())
        })
        .collect();

    let topo_order = topo_sort(&deps, &formula_cols)?;
    Ok((topo_order, col_asts, row_overrides, all_lookup_keys, self_referential_cols))
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

/// Two ASTs are streaming-equivalent if every row would produce the same
/// result.  Same-sheet cell-ref rows are ignored because the evaluator
/// resolves them from the current row (`scope.get(col)`).  Everything
/// else must match exactly.
fn ast_streaming_eq(a: NodeRef<'_>, b: NodeRef<'_>) -> bool {
    match (a.view(), b.view()) {
        (NodeView::Number(x), NodeView::Number(y)) => (x - y).abs() < f64::EPSILON,
        (NodeView::Bool(x), NodeView::Bool(y)) => x == y,
        (NodeView::Text(x), NodeView::Text(y)) => x == y,
        (NodeView::Error(x), NodeView::Error(y)) => x == y,

        // Same-sheet cell ref: row is ignored by the evaluator.
        (
            NodeView::CellRef { sheet: None, col: col_a, .. },
            NodeView::CellRef { sheet: None, col: col_b, .. },
        ) => col_a == col_b,

        // Cross-sheet cell ref: row matters (absolute lookup).
        (
            NodeView::CellRef { sheet: Some(sheet_a), row: row_a, col: col_a },
            NodeView::CellRef { sheet: Some(sheet_b), row: row_b, col: col_b },
        ) => sheet_a.eq_ignore_ascii_case(sheet_b) && row_a == row_b && col_a == col_b,

        // RangeRef: rows compared exactly (unlike CellRef). Range refs
        // denote fixed regions (e.g., SUM(A1:A10)), not "current row."
        (
            NodeView::RangeRef {
                sheet: sheet_a,
                start_row: start_row_a,
                end_row: end_row_a,
                start_col: start_col_a,
                end_col: end_col_a,
            },
            NodeView::RangeRef {
                sheet: sheet_b,
                start_row: start_row_b,
                end_row: end_row_b,
                start_col: start_col_b,
                end_col: end_col_b,
            },
        ) => {
            match (sheet_a, sheet_b) {
                (None, None) | (Some(_), Some(_)) => {}
                _ => return false,
            }
            if let (Some(sa), Some(sb)) = (sheet_a, sheet_b) {
                if !sa.eq_ignore_ascii_case(sb) {
                    return false;
                }
            }
            start_row_a == start_row_b
                && end_row_a == end_row_b
                && start_col_a == start_col_b
                && end_col_a == end_col_b
        }

        (NodeView::NamedRef(na), NodeView::NamedRef(nb)) => na.eq_ignore_ascii_case(nb),

        (NodeView::ExternalRef { raw: raw_a, .. }, NodeView::ExternalRef { raw: raw_b, .. }) => {
            raw_a == raw_b
        }

        (
            NodeView::TableRef { name: name_a, specifier: spec_a },
            NodeView::TableRef { name: name_b, specifier: spec_b },
        ) => name_a.eq_ignore_ascii_case(name_b) && spec_a == spec_b,

        (NodeView::BinaryOp { op: op_a }, NodeView::BinaryOp { op: op_b }) => {
            op_a == op_b
                && a.left().zip(b.left()).is_some_and(|(l1, l2)| ast_streaming_eq(l1, l2))
                && a.right().zip(b.right()).is_some_and(|(r1, r2)| ast_streaming_eq(r1, r2))
        }

        (NodeView::UnaryOp { op: op_a }, NodeView::UnaryOp { op: op_b }) => {
            op_a == op_b
                && a.operand().zip(b.operand()).is_some_and(|(o1, o2)| ast_streaming_eq(o1, o2))
        }

        (NodeView::Function { name: name_a }, NodeView::Function { name: name_b }) => {
            name_a.eq_ignore_ascii_case(name_b) && {
                let args_a = a.args();
                let args_b = b.args();
                args_a.len() == args_b.len()
                    && args_a.iter().zip(args_b.iter()).all(|(x, y)| ast_streaming_eq(*x, *y))
            }
        }

        (
            NodeView::Array { rows: rows_a, cols: cols_a },
            NodeView::Array { rows: rows_b, cols: cols_b },
        ) => {
            rows_a == rows_b && cols_a == cols_b && {
                let cells_a = a.array_cells();
                let cells_b = b.array_cells();
                cells_a.iter().zip(cells_b.iter()).all(|(row_a, row_b)| {
                    row_a.iter().zip(row_b.iter()).all(|(x, y)| ast_streaming_eq(*x, *y))
                })
            }
        }

        (NodeView::PreludeRef(key_a), NodeView::PreludeRef(key_b)) => key_a == key_b,

        _ => false,
    }
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
        let err = evaluate(
            Path::new("this_file_does_not_exist.xlsx"),
            Path::new("out.xlsx"),
            &EvaluateOptions::default(),
        )
        .unwrap_err();
        assert!(matches!(err, XlStreamError::Xlsx(_)), "expected Xlsx error, got {err:?}");
    }

    #[test]
    fn ast_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<xlstream_parse::Ast>();
    }

    #[test]
    fn ast_streaming_eq_same_row_local_formulas() {
        let a = parse("A2*2").unwrap();
        let b = parse("A3*2").unwrap();
        assert!(super::ast_streaming_eq(a.root(), b.root()));
    }

    #[test]
    fn ast_streaming_eq_different_structure() {
        let a = parse("A2*2").unwrap();
        let b = parse("A2+2").unwrap();
        assert!(!super::ast_streaming_eq(a.root(), b.root()));
    }

    #[test]
    fn ast_streaming_eq_cross_sheet_vs_local() {
        let a = parse("A2*2").unwrap();
        let b = parse("Sheet1!A2*2").unwrap();
        assert!(!super::ast_streaming_eq(a.root(), b.root()));
    }

    #[test]
    fn ast_streaming_eq_identical_cross_sheet() {
        let a = parse("Sheet1!A2*2").unwrap();
        let b = parse("Sheet1!A2*2").unwrap();
        assert!(super::ast_streaming_eq(a.root(), b.root()));
    }

    #[test]
    fn ast_streaming_eq_different_cross_sheet_rows() {
        let a = parse("Sheet1!A2*2").unwrap();
        let b = parse("Sheet1!A3*2").unwrap();
        assert!(!super::ast_streaming_eq(a.root(), b.root()));
    }
}
