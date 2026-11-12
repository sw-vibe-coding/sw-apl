//! The characters whose meaning depends on what came before: round
//! brackets, the arrows, and the axis.

use apl_value::{AplError, AplResult, ErrorKind};

use crate::typed::{Last, Out};
use apl_c80_chars::data;

impl Out<'_> {
    /// `c`, typed at `at`, if it is one of these: put down, and `true`.
    ///
    /// # Errors
    /// NONCE ERROR for peek and poke; SYNTAX ERROR for an axis with no
    /// number or name before it.
    pub(crate) fn mark(&mut self, c: char, at: usize) -> AplResult<bool> {
        let argued = matches!(self.last, Last::Value | Last::Variable);
        // A closing bracket closes the last one opened, which indexed or grouped.
        let closes = c == ')' && self.opened.pop() == Some(true);
        let (g, last) = match c {
            '(' if self.last == Last::Variable => ('[', Last::Start),
            '(' => ('(', Last::Start),
            ')' => (if closes { ']' } else { ')' }, Last::Value),
            '↑' | '[' => (if argued { '↑' } else { '⍋' }, Last::Function),
            '↓' | '\\' => (if argued { '↓' } else { '⍒' }, Last::Function),
            '←' | ']' => ('←', Last::Start),
            '→' | '^' => ('→', Last::Start),
            ';' | ':' => (c, Last::Start),
            'd' => return self.axis(at).map(|()| true),
            'u' => return Err(AplError::new(ErrorKind::Nonce).at(at)),
            _ => return Ok(false),
        };
        if c == '(' {
            self.opened.push(g == '[');
        }
        self.push(g, at);
        self.last = last;
        Ok(true)
    }

    /// `kd`: the number or name just put down becomes the axis, `[k]`.
    fn axis(&mut self, at: usize) -> AplResult<()> {
        if !matches!(self.last, Last::Value | Last::Variable) {
            return Err(AplError::new(ErrorKind::Syntax).at(at));
        }
        let from = self.from[self.token];
        self.glyphs.insert(self.token, '[');
        self.from.insert(self.token, from);
        self.push(']', at);
        self.last = Last::Function;
        Ok(())
    }
}

impl Out<'_> {
    /// `c`, typed at `at`, as the glyph the glyphs hold for it.
    ///
    /// # Errors
    /// CHARACTER ERROR for a character APL80 has not got.
    pub(crate) fn datum(&mut self, c: char, at: usize) -> AplResult<()> {
        let g = data(c).ok_or_else(|| AplError::new(ErrorKind::Character(c)).at(at))?;
        self.push(g, at);
        Ok(())
    }
}
