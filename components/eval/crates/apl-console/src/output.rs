//! What a statement produced, and the settings that decide how much
//! of a number is printed and how wide a line runs.

use apl_display::Precision;
use apl_value::Array;

/// What a statement produced: what to display, and where to go next.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    /// Nothing to display (blank line, comment, assignment), and the
    /// next line follows this one.
    Nothing,
    /// One array to display.
    Value(Array),
    /// Mixed output: the parts are displayed side by side.
    Mixed(Vec<Array>),
    /// Lines already rendered: what a function showed while it ran,
    /// with the print settings in force then. A function can change
    /// them, in (B), before the statement that called it ends.
    Lines(Vec<String>),
    /// Characters written with `⍞←`, with no line ending: whatever
    /// comes next carries on where these left off, which is how a
    /// prompt and its answer share a line.
    Bare(String),
    /// A branch. Nothing is displayed. `Some(n)` runs line n next, or
    /// returns when the function has no line n; `None` is a bare
    /// arrow, which falls through in a body and clears the top of the
    /// state indicator in immediate execution.
    Branch(Option<i64>),
}

impl Output {
    /// The single displayed array, if that is what this is.
    #[must_use]
    pub fn value(self) -> Option<Array> {
        match self {
            Output::Value(a) => Some(a),
            _ => None,
        }
    }
}

/// Print settings, which APL\360 keeps with the workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Print {
    /// Significant digits (`)DIGITS`).
    pub digits: usize,
    /// Line width (`)WIDTH`).
    pub width: usize,
    /// How many digits a whole number is always shown in full with:
    /// none in (A), ten in (B). See `apl_display::Precision`.
    pub whole: usize,
}

impl Print {
    /// The digits a number is shown with.
    #[must_use]
    pub fn precision(self) -> Precision {
        Precision {
            digits: self.digits,
            whole: self.whole,
        }
    }
}

impl Default for Print {
    fn default() -> Self {
        Print {
            digits: 10,
            width: 120,
            whole: 0,
        }
    }
}

/// Transcript lines, and whether the last of them was left open by
/// `⍞←`. A console that is about to read needs to know: an open line
/// takes the answer straight after it, on the same line.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Shown {
    /// The lines, in order.
    pub lines: Vec<String>,
    /// Set when the last line is unfinished.
    pub open: bool,
}

impl Shown {
    /// The finished lines, and the last one on its own when `⍞←` left
    /// it open. A console prints the finished ones as lines and the
    /// open one without ending it, so what comes next carries on.
    #[must_use]
    pub fn split(&self) -> (&[String], Option<&str>) {
        match self.lines.split_last() {
            Some((last, rest)) if self.open => (rest, Some(last.as_str())),
            _ => (&self.lines, None),
        }
    }
}
