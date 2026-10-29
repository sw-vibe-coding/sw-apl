//! The function open in the editor: its lines, where the prompt
//! stands, and how it reads back as a transcript.

use apl_ast::Defn;
use apl_patch::Patch;
use apl_scan::header_text;
use apl_value::{AplError, AplResult, ErrorKind};

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
    /// Opened with del-tilde, so locked when it closes, whichever del
    /// closes it. The manual: del-tilde "used instead of ∇ to open or
    /// close a function definition" locks it -- either one is enough.
    pub locking: bool,
    /// The mode's extra glyphs, as the lexer takes them, for a header
    /// edited at `[0]`: in (B) it may make a setting local. Empty is
    /// (A).
    pub(crate) glyphs: &'static str,
    /// A line edit, `[n⎕m]`, part way through: the lines typed while
    /// it lasts are its marks and its fill, not lines of the function
    /// -- nor system commands, though one may begin with `)`.
    pub patch: Option<Patch>,
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
    /// the prompt offers the line after the last. `locking` when it was
    /// opened with del-tilde, so that it is locked when it closes.
    /// `glyphs` is the mode's, as the lexer takes them.
    #[must_use]
    pub fn start(defn: Defn, locking: bool, glyphs: &'static str) -> Definition {
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
            locking,
            glyphs,
            patch: None,
        }
    }

    /// The prompt for the next line: its number in brackets, padded
    /// to the width immediate execution indents by -- or, during a line
    /// edit, what its marks or its fill are typed behind.
    #[must_use]
    pub fn prompt(&self) -> String {
        if let Some(patch) = &self.patch {
            return patch.prompt();
        }
        format!("{:<WIDTH$}", format!("[{}]", format_line(self.next)))
    }

    /// A display, as a transcript. `[⎕]` -- line 0, not `only` --
    /// frames the whole function in its dels and leaves the prompt.
    /// Otherwise the lines from `from` on, or line `from` alone when
    /// `only`, line 0 being the header; and the prompt offers the last
    /// one shown for replacement, as the manual says.
    ///
    /// # Errors
    /// DEFN ERROR when there is nothing to show.
    pub(crate) fn show(&mut self, from: i64, only: bool) -> AplResult<Vec<String>> {
        let numbered =
            |n: i64, text: &str| format!("{:<WIDTH$}{text}", format!("[{}]", format_line(n)));
        if from == 0 && !only {
            let header = format!("{:WIDTH$}∇{}", "", header_text(&self.header));
            let body = self.lines.iter().map(|(n, text)| numbered(*n, text));
            let close = format!("{:WIDTH$}∇", "");
            return Ok(std::iter::once(header).chain(body).chain([close]).collect());
        }
        let header = (0, header_text(&self.header));
        let shown: Vec<(i64, String)> = std::iter::once(header)
            .chain(self.lines.iter().cloned())
            .filter(|(n, _)| if only { *n == from } else { *n >= from })
            .collect();
        let (last, _) = shown.last().ok_or_else(|| AplError::new(ErrorKind::Defn))?;
        self.next = *last;
        Ok(shown.iter().map(|(n, text)| numbered(*n, text)).collect())
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
