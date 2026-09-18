//! Batch mode: run a file or stdin, echoing each input line behind the
//! prompt it would have been typed at so the transcript reads like a
//! session. Invalid UTF-8 on a line is reported as a CHARACTER ERROR
//! and the run continues.
//!
//! The script is shared with the session's console, so a statement
//! that reads takes the next line from it and the run carries on
//! after whatever it consumed.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::rc::Rc;

use apl_session::{Console, INDENT, Reply, Session, Shown};

/// One input line: its text (lossy when invalid) and, when the bytes
/// were not valid UTF-8, the offset of the first bad sequence.
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub bad_at: Option<usize>,
}

/// The lines not yet run, shared between the run and the console so a
/// read and the loop draw from the same place.
type Pending = Rc<RefCell<VecDeque<Line>>>;

/// The console a batch run reads through: it prints as it goes and
/// takes a line from the script when a statement asks for one, with
/// the flag saying whether input lines are echoed.
#[derive(Debug)]
struct Script(Pending, bool);

impl Console for Script {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (lines, open) = shown.split();
        for line in lines {
            println!("{line}");
        }
        if let Some(open) = open {
            print!("{open}");
        }
        if !prompt.is_empty() {
            println!("{prompt}");
        }
        let typed = self.0.borrow_mut().pop_front()?.text;
        match (self.1, open.is_some() && prompt.is_empty()) {
            (true, true) => println!("{typed}"),
            (true, false) => println!("{INDENT}{typed}"),
            (false, true) => println!(),
            (false, false) => {}
        }
        Some(typed)
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
            Some(at) => Reply::Output(vec![format!("CHARACTER ERROR: invalid UTF-8 at byte {at}")]),
            None => session.respond(&line.text),
        };
        let Reply::Output(output) = reply else { break };
        for text in &output {
            println!("{text}");
        }
    }
}
