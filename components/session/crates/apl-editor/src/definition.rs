//! The function open in the editor: its lines, where the prompt
//! stands, and how it reads back as a transcript.

use apl_ast::Defn;
use apl_scan::header_text;

use crate::command::format_line;

/// The six columns immediate execution indents by; the line-number
/// prompt fills the same width.
pub const WIDTH: usize = 6;

/// A function open for definition or editing.
#[derive(Debug)]
pub struct Definition {
    /// The header. Its body is the one editing began from; the live
    /// lines are in `lines`.
    pub(crate) header: Defn,
    /// The name the function had when editing began.
    was: String,
    /// The body by line number in thousandths, in order.
    pub(crate) lines: Vec<(i64, String)>,
    /// The number the prompt is offering.
    pub(crate) next: i64,
}

/// What one input line in definition mode did.
#[derive(Debug, Default)]
pub struct Step {
    /// Transcript lines to display.
    pub lines: Vec<String>,
    /// Set when the definition ended; true when del-tilde locked it.
    pub closed: Option<bool>,
}

impl Definition {
    /// Begin editing `defn`: its body takes the numbers 1, 2, 3 and
    /// the prompt offers the line after the last.
    #[must_use]
    pub fn start(defn: Defn) -> Definition {
        let lines: Vec<(i64, String)> = defn
            .body
            .iter()
            .enumerate()
            .map(|(i, text)| (i64::try_from(i + 1).unwrap_or(0) * 1000, text.clone()))
            .collect();
        let next = lines.last().map_or(1000, |(n, _)| n + 1000);
        let was = defn.name.clone();
        Definition {
            header: defn,
            was,
            lines,
            next,
        }
    }

    /// The prompt for the next line: its number in brackets, padded
    /// to the width immediate execution indents by.
    #[must_use]
    pub fn prompt(&self) -> String {
        format!("{:<WIDTH$}", format!("[{}]", format_line(self.next)))
    }

    /// The function from line `from` as a transcript. Line 0 is the
    /// header, so `[⎕]` frames the whole function in its dels.
    pub(crate) fn show(&self, from: i64) -> Vec<String> {
        let mut out = Vec::new();
        if from == 0 {
            out.push(format!("{:WIDTH$}∇{}", "", header_text(&self.header)));
        }
        for (n, text) in self.lines.iter().filter(|(n, _)| *n >= from) {
            let number = format!("[{}]", format_line(*n));
            out.push(format!("{number:<WIDTH$}{text}"));
        }
        if from == 0 {
            out.push(format!("{:WIDTH$}∇", ""));
        }
        out
    }

    /// Finish: renumber the lines from 1 and hand back the function,
    /// with the name it used to have if `[0]` renamed it.
    #[must_use]
    pub fn close(self, locked: bool) -> (Defn, Option<String>) {
        let mut defn = self.header;
        defn.body = self.lines.into_iter().map(|(_, text)| text).collect();
        defn.locked = locked;
        let renamed = (defn.name != self.was).then_some(self.was);
        (defn, renamed)
    }
}
