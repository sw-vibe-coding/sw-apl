//! The console a statement reads through when the terminal is at the
//! far end of a link.
//!
//! This is where the blocking read lives, and it is the reason the
//! interpreter runs in a service at all. `Console::read` is called
//! part way through a statement and must answer with a line; here it
//! shows what the statement has printed so far, prompts, and then
//! waits. A browser holding the interpreter could not stop to wait; a
//! thread on a socket does it for free, so `⎕`, `⍞` and every line of
//! the del editor work as they do at the CLI.

use std::rc::Rc;

use apl_session::{Console, INDENT, Shown};

use crate::held::Terminal;

/// The console side of an attached terminal.
#[derive(Debug)]
pub struct Reader(pub Terminal);

impl Console for Reader {
    /// Show, prompt, and wait. A `⎕:` prompt is a line of the
    /// transcript and the typing lands at the six-space indent under
    /// it; a line `⍞←` left open is itself the prompt, because the
    /// carriage has stopped on it. `None` -- the terminal has gone --
    /// is a read with no line to be had, which the reader reports as
    /// INTERRUPT, leaving the function suspended exactly as an
    /// interrupt at the CLI would.
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (finished, open) = shown.split();
        let mut lines = finished.to_vec();
        if !prompt.is_empty() {
            lines.push(prompt.to_string());
        }
        let at = open.unwrap_or(INDENT).to_string();
        Rc::clone(&self.0)
            .borrow_mut()
            .ask(lines, at)
            .ok()
            .flatten()
    }
}
