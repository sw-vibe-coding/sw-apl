//! `aplterm`: an IBM 2741 for an APL\360 service.
//!
//! The terminal owns the keyboard and the paper. Overstruck glyphs
//! are formed here, by base and overstrike key, because only the
//! terminal sees the keystrokes: the service is sent `⍟` and never
//! learns it was typed as `○`, a key, and `*`. So is the recall of
//! earlier lines, and the redrawing of a line too long for the
//! window.
//!
//! What the service owns is the session. This is the 1968
//! arrangement: a typewriter on one end of a line, a time-sharing
//! system on the other.

mod run;

use std::fs;
use std::io::{self, IsTerminal};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::ExitCode;

use apl_keyboard::Keyboard;
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

/// An IBM 2741 terminal for a local sw-apl service.
#[derive(Parser, Debug)]
#[command(
    name = "aplterm",
    version = VERSION,
    about,
    long_about = "An IBM 2741 for a local sw-apl service. The keyboard and the \\
                  paper are here: a glyph is typed base, overstrike key, \\
                  overstrike, and what goes on the wire is the composed line.",
    after_long_help = include_str!("cli_help.txt")
)]
pub struct Args {
    /// The service to dial.
    #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:2741")]
    pub connect: String,

    /// A keymap of your own, in the shape of the built-in
    /// keymap.json.
    #[arg(long, value_name = "FILE")]
    pub keymap: Option<PathBuf>,

    /// Take keys as the Unicode they already are, translating
    /// nothing. For a keyboard that sends APL glyphs itself.
    #[arg(long)]
    pub literal: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match dial(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("aplterm: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Build the keyboard the reader asked for, connect, and run.
fn dial(args: &Args) -> io::Result<()> {
    let mut keyboard = Keyboard::default();
    keyboard.literal = args.literal;
    if let Some(path) = &args.keymap {
        keyboard.map =
            serde_json::from_str(&fs::read_to_string(path)?).map_err(io::Error::other)?;
    }
    if !io::stdin().is_terminal() {
        return Err(io::Error::other("aplterm needs a terminal to type at"));
    }
    let socket = TcpStream::connect(&args.connect)?;
    socket.set_nodelay(true)?;
    println!(
        "2741 on {}. Ctrl-] overstrikes; Ctrl-D hangs up.",
        args.connect
    );
    run::run(socket, keyboard)
}
