//! One typed line in definition mode: a statement, a line command, or
//! BREAK.

use apl_scan::{header_text, parse_header};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::keyed::{Keyed, Slot, Typed, WIDTH};

/// How many lines `)P` shows: a screen of 16, less the command and the
/// prompt.
const PAGE: usize = 14;

impl Keyed {
    /// Take one typed line.
    ///
    /// # Errors
    /// DEFN ERROR for a line number the function has not got, for a
    /// header that names another function, and for `)E 0`: the
    /// header cannot be typed over. Lexical errors in a new header.
    pub fn line(&mut self, text: &str) -> AplResult<Typed> {
        if text.trim().is_empty() {
            return Ok(Typed::Leave);
        }
        if let Some(command) = text.trim().strip_prefix(')') {
            return self.command(command.trim());
        }
        match self.slot.take() {
            Some(Slot::Header) => {
                let mut header = parse_header(text.trim(), self.glyphs)?;
                if header.name != self.defn.name {
                    return Err(AplError::new(ErrorKind::Defn));
                }
                header.body = std::mem::take(&mut self.defn.body);
                self.defn = header;
            }
            Some(Slot::Insert(n)) => self.defn.body.insert(n - 1, text.to_string()),
            Some(Slot::Over(n)) => self.defn.body[n - 1] = text.to_string(),
            None => self.defn.body.push(text.to_string()),
        }
        Ok(Typed::Stay(Vec::new()))
    }

    /// A line command: `)?`, `)P n`, `)n`, `)E n`; anything else is a
    /// system command, and not definition mode's.
    fn command(&mut self, command: &str) -> AplResult<Typed> {
        let upper = command.to_ascii_uppercase();
        let number = |rest: &str| rest.trim().parse::<usize>().ok();
        let lines = self.defn.body.len();
        if upper == "?" {
            self.slot = None;
            return Ok(Typed::Stay(self.shown(0, lines + 1)));
        }
        if let Some(from) = upper.strip_prefix('P').and_then(number) {
            return Ok(Typed::Stay(self.shown(from, PAGE)));
        }
        if let Some(n) = upper.strip_prefix('E').and_then(number) {
            let there = (1..=lines).contains(&n);
            self.slot = Some(Slot::Over(n)).filter(|_| there);
            return there
                .then(|| Typed::Stay(Vec::new()))
                .ok_or_else(|| AplError::new(ErrorKind::Defn));
        }
        let Some(n) = number(&upper) else {
            return Ok(Typed::Command);
        };
        self.slot = match n {
            0 => Some(Slot::Header),
            n if n <= lines => {
                self.defn.body.remove(n - 1);
                Some(Slot::Insert(n))
            }
            _ => return Err(AplError::new(ErrorKind::Defn)),
        };
        Ok(Typed::Stay(Vec::new()))
    }

    /// `count` lines of the function from line `from`, line 0 being the
    /// header, each behind its number.
    fn shown(&self, from: usize, count: usize) -> Vec<String> {
        let header = header_text(&self.defn);
        let all = std::iter::once(&header).chain(self.defn.body.iter());
        let numbered = all
            .enumerate()
            .map(|(n, text)| format!("{:<WIDTH$}{text}", format!("[{n}]")));
        numbered.skip(from).take(count).collect()
    }
}
