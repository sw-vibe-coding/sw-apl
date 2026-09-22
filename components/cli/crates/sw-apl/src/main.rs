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

use apl_session::{Files, Host, Mode};
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

    /// Workspace size in bytes: how much the workspace may hold
    /// before WS FULL.
    #[arg(long = "ws-size", value_name = "BYTES", default_value_t = apl_session::QUOTA)]
    pub ws_size: usize,

    /// Where the workspace libraries are: the directory holding
    /// work/ (library 0) and ws/ (the shipped libraries).
    #[arg(long = "library", value_name = "DIR", default_value = ".")]
    pub library: PathBuf,

    /// The mode: 68 for (A), 75 for (B). Each lists and loads only
    /// the workspaces that run in it.
    #[arg(long, value_name = "MODE", default_value = "68", value_parser = mode)]
    pub mode: Mode,
}

/// A mode as `--mode` names it: its year or its letter.
fn mode(word: &str) -> Result<Mode, String> {
    Mode::parse(word).ok_or_else(|| format!("{word} is not a mode: 68 or 75"))
}

fn main() -> ExitCode {
    let args = Args::parse();
    let echo = !args.no_echo;
    let ws = Host {
        quota: args.ws_size,
        store: Box::new(Files(args.library)),
        mode: args.mode,
    };
    let outcome = match args.file {
        Some(path) => shell::run_batch(Some(&path), echo, ws),
        None if std::io::stdin().is_terminal() => repl::run_interactive(ws),
        None => shell::run_batch(None, echo, ws),
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("sw-apl: {err}");
            ExitCode::FAILURE
        }
    }
}
