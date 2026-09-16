//! The two shells around the session: batch (file or stdin, with
//! echoed input) and the interactive six-space-indent loop.

use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use apl_session::{INDENT, Reply, Session};

/// Run every line of `path` in batch mode.
pub fn run_file(path: &Path, echo: bool) -> io::Result<()> {
    let text = fs::read_to_string(path)?;
    run_lines(text.lines().map(str::to_string), echo)
}

/// Run every line read from stdin in batch mode.
pub fn run_stdin(echo: bool) -> io::Result<()> {
    let lines: Vec<String> = io::stdin().lock().lines().collect::<io::Result<_>>()?;
    run_lines(lines.into_iter(), echo)
}

fn run_lines(lines: impl Iterator<Item = String>, echo: bool) -> io::Result<()> {
    let mut out = io::stdout().lock();
    let mut session = Session::default();
    for line in lines {
        if echo {
            writeln!(out, "{INDENT}{line}")?;
        }
        match session.respond(&line) {
            Reply::Off => break,
            Reply::Output(output) => {
                for text in output {
                    writeln!(out, "{text}")?;
                }
            }
        }
    }
    out.flush()
}

/// Interactive loop: print the indent, read a line, print the
/// reply in column one. Ends at `)OFF` or end of input.
pub fn run_interactive() -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut out = io::stdout().lock();
    let mut session = Session::default();
    let mut line = String::new();
    loop {
        write!(out, "{INDENT}")?;
        out.flush()?;
        line.clear();
        if input.read_line(&mut line)? == 0 {
            writeln!(out)?;
            return Ok(());
        }
        match session.respond(line.trim_end_matches(['\n', '\r'])) {
            Reply::Off => return Ok(()),
            Reply::Output(output) => {
                for text in output {
                    writeln!(out, "{text}")?;
                }
            }
        }
    }
}
