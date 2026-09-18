//! Batch mode: run a file or stdin, echoing each input line behind the
//! prompt it would have been typed at so the transcript reads like a
//! session. Invalid UTF-8 on a line is reported as a CHARACTER ERROR
//! and the run continues.
//!
//! The script is shared with the session's console, so a statement
//! that reads takes the next line from it and the run carries on
//! after whatever it consumed.

use std::cell::RefCell;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::rc::Rc;

use crate::host::{Pending, Script, catch_interrupt};

use apl_session::{Reply, Session};

/// One input line: its text (lossy when invalid) and, when the bytes
/// were not valid UTF-8, the offset of the first bad sequence.
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub bad_at: Option<usize>,
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
        .map(|l| l.strip_suffix(b"\r").unwrap_or(l))
        .map(|l| Line {
            text: String::from_utf8_lossy(l).into_owned(),
            bad_at: std::str::from_utf8(l).err().map(|e| e.valid_up_to()),
        })
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
    let pending: Pending = Rc::new(RefCell::new(lines(&bytes).into()));
    let mut session = Session::attached(Box::new(Script(Rc::clone(&pending), echo)));
    catch_interrupt();
    run_lines(&mut session, &pending, echo);
    io::stdout().flush()
}

/// Run what is left of the script, stopping at `)OFF`. Lines are
/// taken with `let ... else` rather than `while let`, which would
/// hold the borrow across the body: a statement that reads borrows
/// the same queue.
fn run_lines(session: &mut Session, pending: &Pending, echo: bool) {
    loop {
        let Some(line) = pending.borrow_mut().pop_front() else {
            break;
        };
        if echo {
            println!("{}{}", session.prompt(), line.text);
        }
        let reply = match line.bad_at {
            Some(at) => Reply::from(vec![format!("CHARACTER ERROR: invalid UTF-8 at byte {at}")]),
            None => session.respond(&line.text),
        };
        for text in &reply.lines {
            println!("{text}");
        }
        if reply.off {
            break;
        }
    }
}
