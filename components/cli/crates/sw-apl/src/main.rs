//! `sw-apl`: a clean-room APL\360 interpreter for the terminal.
//!
//! This binary is a thin shell: it decides between interactive and
//! batch mode and hands each input line to the session.

mod host;
mod repl;
mod shell;

use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

/// Full `-V` / `--version` block: name, copyright, license,
/// repository, then build information from `build.rs`.
pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\nCopyright (c) 2026 Michael A Wright\n",
    "License: MIT\n",
    "Repository: https://github.com/sw-vibe-coding/sw-apl\n\n",
    "Build Information:\n  Host: ",
    env!("BUILD_HOST"),
    "\n  Commit: ",
    env!("GIT_HASH"),
    "\n  Timestamp: ",
    env!("BUILD_TIMESTAMP"),
);

/// A clean-room APL\360 interpreter with traditional glyphs.
#[derive(Parser, Debug)]
#[command(
    name = "sw-apl",
    version = VERSION,
    about,
    long_about = "A clean-room APL\\360 interpreter with traditional glyphs, \
                  written in Rust from scratch. Flat arrays, floating point, \
                  the del editor, and the APL\\360 system commands.",
    after_long_help = include_str!("cli_help.txt")
)]
pub struct Args {
    /// Run FILE in batch mode instead of the interactive session.
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Do not echo input lines (with their indent) in batch mode.
    #[arg(long)]
    pub no_echo: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
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
