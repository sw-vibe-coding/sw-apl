//! `sw-apl`: a clean-room APL\360 interpreter for the terminal.
//!
//! This binary is a thin shell: it decides between interactive and
//! batch mode and hands each input line to the session.

mod cli;
mod repl;
mod shell;

use std::io::IsTerminal;
use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let args = cli::Args::parse();
    let echo = !args.no_echo;
    let outcome = match args.file {
        Some(path) => shell::run_batch(Some(&path), echo),
        None if std::io::stdin().is_terminal() => repl::run_interactive(),
        None => shell::run_batch(None, echo),
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("sw-apl: {err}");
            ExitCode::FAILURE
        }
    }
}
