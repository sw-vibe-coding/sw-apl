//! What each input line in definition mode does.

use apl_scan::parse_header;
use apl_value::{AplError, AplResult, ErrorKind};

use crate::command::{Edit, advance, parse_edit};
use crate::definition::{Definition, Step};

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
        let trimmed = text.trim();
        if trimmed == "∇" || trimmed == "⍫" {
            let closed = Some(trimmed == "⍫");
            return Ok(Step {
                closed,
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
            Edit::Show(from) => self.show(from),
            Edit::Delete(n) => {
                self.remove(n)?;
                Vec::new()
            }
        };
        if rest.is_empty() {
            return Ok(Step {
                lines,
                ..Step::default()
            });
        }
        let mut step = self.line(rest)?;
        step.lines = [lines, step.lines].concat();
        Ok(step)
    }

    /// Put `text` at the line the prompt is offering, then move on.
    /// Line 0 is the header, which is read again from the text.
    fn text(&mut self, text: &str) -> AplResult<()> {
        if self.next == 0 {
            let body = std::mem::take(&mut self.header.body);
            self.header = parse_header(text)?;
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

    /// Delete line `n`.
    ///
    /// # Errors
    /// DEFN ERROR when the function has no such line.
    fn remove(&mut self, n: i64) -> AplResult<()> {
        let Ok(i) = self.lines.binary_search_by_key(&n, |(k, _)| *k) else {
            return Err(AplError::new(ErrorKind::Defn));
        };
        self.lines.remove(i);
        Ok(())
    }
}
