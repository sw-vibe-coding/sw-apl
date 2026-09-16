//! Command-line surface for `sw-apl`: arguments, help, and the
//! version block (Software Wrighter CLI convention).

use std::path::PathBuf;

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
