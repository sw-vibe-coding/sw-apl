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

/// A clean-room APL interpreter with traditional glyphs, in two modes.
#[derive(Parser, Debug)]
#[command(
    name = "sw-apl",
    version = VERSION,
    about,
    long_about = "A clean-room APL interpreter with traditional glyphs, \
                  written in Rust from scratch, in two modes: (A) '70, \
                  modelled on APL\\360, and (B) '75, modelled on the APL \
                  of the IBM 5100 family. Flat arrays, floating point, the \
                  del editor, and workspaces in numbered libraries.",
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

    /// The mode: 70 for (A), 75 for (B). Each lists and loads only
    /// the workspaces that run in it.
    #[arg(long, value_name = "MODE", default_value = "70", value_parser = mode)]
    pub mode: Mode,

    /// A library beyond 0 and 1: its number, the directory of its
    /// workspaces, and the name )LIBS gives it. May be given again for
    /// another. Read-only.
    #[arg(long = "lib", value_name = "N=DIR[,NAME]", value_parser = apl_config::parse_lib)]
    pub libs: Vec<apl_config::LibrarySpec>,

    /// The configuration file to read libraries from, in place of
    /// ./sw-apl.toml or the user's sw-apl/config.toml.
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

/// A mode as `--mode` names it: its year or its letter.
fn mode(word: &str) -> Result<Mode, String> {
    Mode::parse(word).ok_or_else(|| format!("{word} is not a mode: 70 or 75"))
}

/// The libraries: `--library`'s 0 and 1, and any `--lib` and the
/// configuration file add.
fn store(args: &Args) -> Result<Box<dyn apl_session::Store>, String> {
    let libraries = apl_config::configured(args.config.as_deref(), &args.libs)?;
    let files = Box::new(Files(args.library.clone()));
    Ok(Box::new(apl_config::attach(files, &libraries)))
}

fn main() -> ExitCode {
    let args = Args::parse();
    let echo = !args.no_echo;
    let Ok(store) = store(&args).map_err(|err| eprintln!("sw-apl: {err}")) else {
        return ExitCode::FAILURE;
    };
    let ws = Host {
        quota: args.ws_size,
        store,
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
