//! Where a statement gets a line when it reads one.

use std::fmt::Debug;

use crate::output::Shown;

/// The six spaces immediate execution indents an input line by. It is
/// defined here, below everything that prints, so the prompt, the
/// error display and a console echoing what it read all use the one
/// width.
pub const INDENT: &str = "      ";

/// The terminal a statement reads from. A read happens part way
/// through a statement, so it is handed everything shown so far: the
/// prompt has to land after that, not before it.
pub trait Console: Debug {
    /// Show `shown`, then read a line for `prompt`. `None` when there
    /// is no line to be had, which the reader reports as INTERRUPT.
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String>;

    /// Show `lines` now, while the statement that printed them is
    /// still running, and say whether they were shown. A console that
    /// keeps a transcript does not, and they wait for the next read or
    /// the statement's end.
    fn show(&mut self, _lines: &[String]) -> bool {
        false
    }

    /// The lines shown so far, taken away. A console that prints as it
    /// goes has none to give back.
    fn take(&mut self) -> Vec<String> {
        Vec::new()
    }
}
