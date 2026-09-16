//! Input lines as UTF-8, with invalid bytes reported as a CHARACTER
//! ERROR instead of aborting the run.

use std::io::{self, BufRead};

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
    let body = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    if bytes.is_empty() {
        return Vec::new();
    }
    body.split(|&b| b == b'\n')
        .map(|l| Line::decode(l.strip_suffix(b"\r").unwrap_or(l)))
        .collect()
}

/// Read one line from a terminal; `None` at end of input.
///
/// # Errors
/// Any I/O error (invalid UTF-8 is not one; it becomes `bad_at`).
pub fn read_line(input: &mut impl BufRead) -> io::Result<Option<Line>> {
    let mut bytes = Vec::new();
    if input.read_until(b'\n', &mut bytes)? == 0 {
        return Ok(None);
    }
    let end = bytes.len() - usize::from(bytes.ends_with(b"\n"));
    let end = end - usize::from(bytes[..end].ends_with(b"\r"));
    Ok(Some(Line::decode(&bytes[..end])))
}
