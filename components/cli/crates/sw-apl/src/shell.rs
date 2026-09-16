//! The two shells around the session: batch (file or stdin, with
//! echoed input) and the interactive six-space-indent loop.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use apl_session::{INDENT, Reply, Session};

use crate::decode::{Line, lines, read_line};

/// Run every line of `path` in batch mode.
pub fn run_file(path: &Path, echo: bool) -> io::Result<()> {
    run_lines(&lines(&fs::read(path)?), echo)
}

/// Run every line read from stdin in batch mode.
pub fn run_stdin(echo: bool) -> io::Result<()> {
    let mut bytes = Vec::new();
    io::Read::read_to_end(&mut io::stdin().lock(), &mut bytes)?;
    run_lines(&lines(&bytes), echo)
}

fn run_lines(lines: &[Line], echo: bool) -> io::Result<()> {
    let mut out = io::stdout().lock();
    let mut session = Session::default();
    for line in lines {
        if echo {
            writeln!(out, "{INDENT}{}", line.text)?;
        }
        match line.respond(&mut session) {
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
    loop {
        write!(out, "{INDENT}")?;
        out.flush()?;
        let Some(line) = read_line(&mut input)? else {
            writeln!(out)?;
            return Ok(());
        };
        match line.respond(&mut session) {
            Reply::Off => return Ok(()),
            Reply::Output(output) => {
                for text in output {
                    writeln!(out, "{text}")?;
                }
            }
        }
    }
}
