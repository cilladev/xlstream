//! xlstream development CLI.

#![warn(missing_docs, rust_2018_idioms, clippy::pedantic, clippy::cargo)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::print_stdout,
    clippy::dbg_macro
)]
#![allow(clippy::module_name_repetitions, clippy::cargo_common_metadata)]

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use xlstream_core::col_row_to_a1;

/// xlstream CLI — streaming Excel formula evaluator.
#[derive(Debug, Parser)]
#[command(name = "xlstream", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Evaluate an xlsx workbook and write the result to another xlsx file.
    Evaluate {
        /// Input workbook.
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output workbook path.
        #[arg(long, short = 'o', value_name = "OUTPUT")]
        output: PathBuf,
        /// Number of parallel workers (default: auto).
        #[arg(long, short = 'w', value_name = "N")]
        workers: Option<usize>,
        /// Maximum iterations for self-referential formulas.
        #[arg(long, value_name = "N")]
        max_iterations: Option<u32>,
        /// Convergence threshold for iterative calculation.
        #[arg(long, value_name = "DELTA")]
        max_change: Option<f64>,
        /// Disable iterative calculation (self-referential formulas will error).
        #[arg(long)]
        no_iterative_calc: bool,
        /// Print phase timings and evaluation summary.
        #[arg(long, short = 'v')]
        verbose: bool,
    },
    /// Parse and classify a single Excel formula expression.
    Classify {
        /// The formula text (without leading `=`).
        formula: String,
        /// Sheet name to use as the streaming sheet.
        #[arg(long, default_value = "Sheet1")]
        sheet: String,
        /// 1-based row to use as the formula's anchor.
        #[arg(long, default_value_t = 1)]
        row: u32,
        /// 1-based column to use as the formula's anchor.
        #[arg(long, default_value_t = 1)]
        col: u32,
        /// Register a sheet as a prelude-loaded lookup sheet (repeatable).
        #[arg(long = "lookup-sheet", value_name = "SHEET")]
        lookup_sheets: Vec<String>,
        /// Increase log verbosity.
        #[arg(long, short = 'v')]
        verbose: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let verbose = match &cli.command {
        Command::Evaluate { verbose, .. } | Command::Classify { verbose, .. } => *verbose,
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_new(if verbose { "debug" } else { "info" })
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{e}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<(), xlstream_core::XlStreamError> {
    match cli.command {
        Command::Evaluate {
            input,
            output,
            workers,
            max_iterations,
            max_change,
            no_iterative_calc,
            verbose,
        } => {
            let options = xlstream_eval::EvaluateOptions {
                workers,
                iterative_calc: !no_iterative_calc,
                max_iterations: max_iterations
                    .unwrap_or(xlstream_core::ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS),
                max_change: max_change.unwrap_or(xlstream_core::ITERATIVE_CALC_DEFAULT_MAX_CHANGE),
                values_only: false,
            };
            let summary = xlstream_eval::evaluate(&input, &output, &options)?;
            if verbose {
                let rss_mb = memory_stats::memory_stats().map_or(0, |s| s.physical_mem / 1_000_000);
                info!(
                    rows = summary.rows_processed,
                    formulas = summary.formulas_evaluated,
                    duration_ms = summary.duration.as_millis(),
                    rss_mb,
                    "evaluate complete"
                );
            }
            Ok(())
        }
        Command::Classify { formula, sheet, row, col, lookup_sheets, .. } => {
            run_classify(&formula, &sheet, row, col, &lookup_sheets)
        }
    }
}

#[allow(clippy::print_stdout)]
fn run_classify(
    formula: &str,
    sheet: &str,
    row: u32,
    col: u32,
    lookup_sheets: &[String],
) -> Result<(), xlstream_core::XlStreamError> {
    let ast = xlstream_parse::parse(formula).map_err(|e| match e {
        xlstream_core::XlStreamError::FormulaParse { formula: f, message, position, .. } => {
            xlstream_core::XlStreamError::FormulaParse {
                address: format!("{sheet}!{}", col_row_to_a1(col, row)),
                formula: f,
                message,
                position,
            }
        }
        other => other,
    })?;

    let mut ctx = xlstream_parse::ClassificationContext::for_cell(sheet, row, col);
    for s in lookup_sheets {
        ctx = ctx.with_lookup_sheet(s);
    }

    let verdict = xlstream_parse::classify(&ast, &ctx);
    println!("{formula}\t{verdict:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use clap::Parser;

    use super::*;

    #[test]
    fn evaluate_subcommand_parses_required_and_optional_args() {
        let cli = Cli::try_parse_from([
            "xlstream",
            "evaluate",
            "in.xlsx",
            "--output",
            "out.xlsx",
            "--workers",
            "4",
            "--verbose",
        ])
        .unwrap();

        match cli.command {
            Command::Evaluate { input, output, workers, verbose, .. } => {
                assert_eq!(input.to_str(), Some("in.xlsx"));
                assert_eq!(output.to_str(), Some("out.xlsx"));
                assert_eq!(workers, Some(4));
                assert!(verbose);
            }
            Command::Classify { .. } => panic!("expected Evaluate"),
        }
    }

    #[test]
    fn classify_subcommand_parses_formula_arg_with_defaults() {
        let cli = Cli::try_parse_from(["xlstream", "classify", "SUM(A:A)"]).unwrap();
        match cli.command {
            Command::Classify { formula, sheet, row, col, lookup_sheets, .. } => {
                assert_eq!(formula, "SUM(A:A)");
                assert_eq!(sheet, "Sheet1");
                assert_eq!(row, 1);
                assert_eq!(col, 1);
                assert!(lookup_sheets.is_empty());
            }
            Command::Evaluate { .. } => panic!("expected Classify"),
        }
    }

    #[test]
    fn classify_subcommand_accepts_lookup_sheet_flag() {
        let cli = Cli::try_parse_from([
            "xlstream",
            "classify",
            "VLOOKUP(A1, 'Region Info'!A:C, 2, FALSE)",
            "--lookup-sheet",
            "Region Info",
        ])
        .unwrap();
        match cli.command {
            Command::Classify { lookup_sheets, .. } => {
                assert_eq!(lookup_sheets, vec!["Region Info"]);
            }
            Command::Evaluate { .. } => panic!("expected Classify"),
        }
    }
}
