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

use apl_attn::catch_interrupt;

use crate::host::{Pending, Script};

use apl_session::{Host, Reply, Session};
use apl_strike::read;

/// One input line: its text (lossy when invalid) and, when the bytes
/// were not valid UTF-8, the offset of the first bad sequence.
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub bad_at: Option<usize>,
}

/// Split a byte stream into lines (LF or CRLF), decoding each. A
/// final newline does not start an extra empty line.
///
/// A leading `#!` line is dropped: it belongs to the shell that ran
/// the file, and the kernel has already acted on it. Only the first
/// line, and only those two characters -- `#` is not in the APL\360
/// character set, so it stays a CHARACTER ERROR everywhere else.
#[must_use]
pub fn lines(bytes: &[u8]) -> Vec<Line> {
    // Only at the very start, and only the first line: a `#` after
    // that is a CHARACTER ERROR like any other.
    let mut bytes = bytes;
    if bytes.starts_with(b"#!") {
        let end = bytes.iter().position(|&b| b == b'\n');
        bytes = end.map_or(&[][..], |at| &bytes[at + 1..]);
    }
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
/// `host` is the workspace size in bytes, where the libraries are
/// kept, and the mode.
///
/// # Errors
/// I/O errors reading the input or writing the transcript.
pub fn run_batch(path: Option<&Path>, echo: bool, host: Host) -> io::Result<()> {
    let mut bytes = Vec::new();
    if let Some(p) = path {
        bytes = fs::read(p)?;
    } else {
        io::stdin().lock().read_to_end(&mut bytes)?;
    }
    let pending: Pending = Rc::new(RefCell::new(lines(&bytes).into()));
    let mut session = Session::attached(Box::new(Script(Rc::clone(&pending), echo)), host);
    catch_interrupt();
    run_lines(&mut session, &pending, echo);
    io::stdout().flush()
}

/// Run what is left of the script, stopping at `)OFF`. Lines are
/// taken with `let ... else` rather than `while let`, which would
/// hold the borrow across the body: a statement that reads borrows
/// the same queue.
///
/// Overstrikes are formed before the line is echoed: the paper shows
/// the struck glyph, not the keystrokes that made it, and a file may
/// carry the 2741's own backspace. Typing continues a line `⍞←` left
/// open rather than starting one, so the prompt is not printed again
/// and pressing return is what ends it.
fn run_lines(session: &mut Session, pending: &Pending, echo: bool) {
    let mut open = false;
    loop {
        let Some(line) = pending.borrow_mut().pop_front() else {
            break;
        };
        let struck = read(&line.text, session.ws.mode.overstrikes());
        if echo {
            let prompt = if open { "" } else { &session.prompt() };
            println!("{prompt}{}", struck.as_deref().unwrap_or(&line.text));
        }
        let reply = match (line.bad_at, struck) {
            (Some(at), _) => {
                Reply::from(vec![format!("CHARACTER ERROR: invalid UTF-8 at byte {at}")])
            }
            (None, Err(report)) => Reply::failed(vec![report]),
            (None, Ok(text)) => session.respond(&text),
        };
        open = !show(&reply).is_empty();
        if reply.off {
            break;
        }
    }
}

/// Print a reply and hand back the line it left open, if any.
///
/// The last line of an open reply is written without ending it, so
/// that what comes next carries on where `⍞←` stopped. Both shells
/// print replies and both must honour that.
#[must_use]
pub fn show(reply: &Reply) -> String {
    let (finished, carried) = reply.split();
    for text in finished {
        println!("{text}");
    }
    let Some(text) = carried else {
        return String::new();
    };
    print!("{text}");
    let _ = io::stdout().flush();
    text.to_string()
}
