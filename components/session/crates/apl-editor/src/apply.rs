//! What each input line in definition mode does.

use apl_patch::{Patch, Taken};
use apl_scan::{header_text, parse_header};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::command::{Edit, advance, format_line, parse_edit};
use crate::definition::{Definition, Step, WIDTH};

impl Definition {
    /// Take one input line. A line that is nothing but a del closes
    /// the definition and del-tilde closes it locked; a line starting
    /// with a bracket is an editor command; anything else is text for
    /// the line the prompt is offering.
    ///
    /// # Errors
    /// DEFN ERROR for a malformed command, a line that is not there
    /// to delete, or a header that does not parse.
    pub fn line(&mut self, text: &str) -> AplResult<Step> {
        if let Some(patch) = self.patch.take() {
            return match patch.take(text, &[])? {
                Taken::Again(patch, edited) => {
                    self.patch = Some(patch);
                    Ok(Step {
                        lines: vec![edited],
                        ..Step::default()
                    })
                }
                Taken::Line(line) => self.line(&line),
            };
        }
        let trimmed = text.trim();
        if trimmed == "∇" || trimmed == "⍫" {
            return Ok(Step {
                closed: Some(trimmed == "⍫"),
                ..Step::default()
            });
        }
        match parse_edit(trimmed)? {
            Some((edit, rest)) => self.command(edit, rest.trim()),
            None => self.text(text.trim_end()).map(|()| Step::default()),
        }
    }

    /// Run one editor command, then take whatever was written after
    /// it as a line of its own: text for the line the command
    /// selected, or the del that closes the definition.
    fn command(&mut self, edit: Edit, rest: &str) -> AplResult<Step> {
        let lines = match edit {
            Edit::At(n) => {
                self.next = n;
                Vec::new()
            }
            Edit::Show(n, true) if !self.glyphs.is_empty() => self.patch(n, 0)?,
            Edit::Show(from, only) => self.show(from, only)?,
            Edit::Delete(n) => {
                let i = self.lines.binary_search_by_key(&n, |(k, _)| *k);
                self.lines
                    .remove(i.map_err(|_| AplError::new(ErrorKind::Defn))?);
                Vec::new()
            }
            Edit::Patch(n, at) => self.patch(n, at)?,
        };
        let mut step = if rest.is_empty() {
            Step::default()
        } else {
            self.line(rest)?
        };
        step.lines = [lines, step.lines].concat();
        Ok(step)
    }

    /// Put `text` at the line the prompt is offering, then move on.
    /// Line 0 is the header, which is read again from the text.
    fn text(&mut self, text: &str) -> AplResult<()> {
        if self.next == 0 {
            let body = std::mem::take(&mut self.header.body);
            self.header = parse_header(text, self.glyphs)?;
            self.header.body = body;
        } else {
            let line = (self.next, text.to_string());
            match self.lines.binary_search_by_key(&self.next, |(n, _)| *n) {
                Ok(i) => self.lines[i] = line,
                Err(i) => self.lines.insert(i, line),
            }
        }
        self.next = advance(self.next);
        Ok(())
    }

    /// Begin a line edit of statement `n`, and hand back what it
    /// displays. In (A), APL\360's slash line: the statement is shown
    /// and the marks are typed from position `at`. In (B), the 5110's:
    /// nothing is shown, the statement is offered to the input to be
    /// typed over in place, and `at` is dropped. (A) is the mode with
    /// no glyphs beyond APL\360's.
    ///
    /// # Errors
    /// DEFN ERROR when the function has no statement `n` -- the
    /// manual's "improper request for a line edit".
    fn patch(&mut self, n: i64, at: usize) -> AplResult<Vec<String>> {
        let text = match n {
            0 => Some(header_text(&self.header)),
            _ => self
                .lines
                .iter()
                .find(|(k, _)| *k == n)
                .map(|(_, t)| t.clone()),
        };
        let text = text.ok_or_else(|| AplError::new(ErrorKind::Defn))?;
        let line = format!("{:<WIDTH$}{text}", format!("[{}]", format_line(n)));
        if !self.glyphs.is_empty() {
            self.patch = Some(Patch::Offer { line });
            return Ok(Vec::new());
        }
        self.patch = Some(Patch::open(&line, at));
        Ok(vec![line])
    }
}
