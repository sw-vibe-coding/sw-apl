//! Batch mode: run a file or stdin, echoing each input line behind the
//! prompt it would have been typed at so the transcript reads like a
//! session. Invalid UTF-8 on a
//! line is reported as a CHARACTER ERROR and the run continues.

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use apl_session::{Reply, Session};

/// One input line: its text (lossy when invalid) and, when the bytes
/// were not valid UTF-8, the offset of the first bad sequence.
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub bad_at: Option<usize>,
}

impl Line {
    /// Decode one line's bytes (without the newline).
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Line {
        let bad_at = std::str::from_utf8(bytes).err().map(|e| e.valid_up_to());
        Line {
            text: String::from_utf8_lossy(bytes).into_owned(),
            bad_at,
        }
    }

    /// Hand the line to the session, or report the bad bytes.
    pub fn respond(&self, session: &mut Session) -> Reply {
        match self.bad_at {
            None => session.respond(&self.text),
            Some(offset) => Reply::Output(vec![format!(
                "CHARACTER ERROR: invalid UTF-8 at byte {offset}"
            )]),
        }
    }
}

/// Split a byte stream into lines (LF or CRLF), decoding each. A
/// final newline does not start an extra empty line.
#[must_use]
pub fn lines(bytes: &[u8]) -> Vec<Line> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let body = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    body.split(|&b| b == b'\n')
        .map(|l| Line::decode(l.strip_suffix(b"\r").unwrap_or(l)))
        .collect()
}

/// Run every line of `path` (or of stdin when `None`) in batch mode.
///
/// # Errors
/// I/O errors reading the input or writing the transcript.
pub fn run_batch(path: Option<&Path>, echo: bool) -> io::Result<()> {
    let mut bytes = Vec::new();
    if let Some(p) = path {
        bytes = fs::read(p)?;
    } else {
        io::stdin().lock().read_to_end(&mut bytes)?;
    }
    let mut out = io::stdout().lock();
    let mut session = Session::default();
    for line in &lines(&bytes) {
        if echo {
            writeln!(out, "{}{}", session.prompt(), line.text)?;
        }
        match line.respond(&mut session) {
            Reply::Off => break,
            Reply::Output(output) => output.iter().try_for_each(|t| writeln!(out, "{t}"))?,
        }
    }
    out.flush()
}
