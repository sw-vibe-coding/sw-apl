//! One typed line in definition mode: a statement, a line command, or
//! BREAK.

use apl_ast::Defn;
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
                let mut header = header(text, self.glyphs, 0)?;
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
    /// system command, and not definition mode's. Its letters are
    /// capitals: on the TRS-80 a lower-case letter is a shifted one, a
    /// symbol.
    fn command(&mut self, command: &str) -> AplResult<Typed> {
        let number = |rest: &str| rest.trim().parse::<usize>().ok();
        let lines = self.defn.body.len();
        if command == "?" {
            self.slot = None;
            return Ok(Typed::Stay(self.shown(0, lines + 1)));
        }
        if let Some(from) = command.strip_prefix('P').and_then(number) {
            return Ok(Typed::Stay(self.shown(from, PAGE)));
        }
        let over = command.strip_prefix('E').and_then(number);
        let Some(n) = over.or_else(|| number(command)) else {
            return Ok(Typed::Command);
        };
        self.slot = Some(match (over.is_some(), n) {
            (false, 0) => Slot::Header,
            (_, n) if n == 0 || n > lines => return Err(AplError::new(ErrorKind::Defn)),
            (true, n) => Slot::Over(n),
            (false, n) => Slot::Insert(n),
        });
        if let Some(Slot::Insert(n)) = self.slot {
            self.defn.body.remove(n - 1);
        }
        Ok(Typed::Stay(Vec::new()))
    }

    /// `count` lines of the function from line `from`, line 0 being the
    /// header, each behind its number.
    fn shown(&self, from: usize, count: usize) -> Vec<String> {
        let header = apl_c80_keys::named(&header_text(&self.defn));
        let all = std::iter::once(&header).chain(self.defn.body.iter());
        let numbered = all
            .enumerate()
            .map(|(n, text)| format!("{:<WIDTH$}{text}", format!("[{n}]")));
        numbered.skip(from).take(count).collect()
    }
}

/// A header typed in APL80's characters, from column `at` of its line,
/// as a definition to fill.
///
/// # Errors
/// As the translation and `parse_header` give them, the caret under
/// what was typed.
pub fn header(text: &str, glyphs: &'static str, at: usize) -> AplResult<Defn> {
    let shift = |mut e: AplError| {
        e.caret = e.caret.map(|c| c + at);
        e
    };
    let typed = apl_c80_keys::glyphs(text, &|_| false).map_err(shift)?;
    parse_header(&typed.text, glyphs).map_err(|e| shift(typed.place(e)))
}
