//! xlstream development CLI. Phase 1 exposes a single `evaluate` subcommand
//! that wires through to [`xlstream_eval::evaluate`] — which is itself a
//! stub, so this binary currently exits with an `Internal("unimplemented
//! ...")` error. Useful only to verify that `--help` works and the argument
//! surface is correct.

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
use tracing::error;
use tracing_subscriber::EnvFilter;

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
        /// Increase log verbosity.
        #[arg(long, short = 'v')]
        verbose: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let filter = match &cli.command {
        Command::Evaluate { verbose: true, .. } => "debug",
        Command::Evaluate { verbose: false, .. } => "info",
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_new(filter).unwrap_or_else(|_| EnvFilter::new("info")))
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
        Command::Evaluate { input, output, workers, .. } => {
            let _summary = xlstream_eval::evaluate(&input, &output, workers)?;
            Ok(())
        }
    }
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
            Command::Evaluate { input, output, workers, verbose } => {
                assert_eq!(input.to_str(), Some("in.xlsx"));
                assert_eq!(output.to_str(), Some("out.xlsx"));
                assert_eq!(workers, Some(4));
                assert!(verbose);
            }
        }
    }
}
