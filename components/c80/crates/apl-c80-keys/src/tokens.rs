//! The tokens of more than one character: strings, comments, names and
//! numbers.

use apl_value::AplResult;

use crate::typed::{Last, Out};
use apl_c80_chars::name;

impl Out<'_> {
    /// A string, each character in it as itself; a doubled quote is
    /// one. Where it ends.
    ///
    /// # Errors
    /// CHARACTER ERROR for a character APL80 has not got.
    pub(crate) fn quoted(&mut self, chars: &[char], at: usize) -> AplResult<usize> {
        self.push('\'', at);
        let mut i = at + 1;
        while i < chars.len() {
            let c = chars[i];
            self.datum(c, i)?;
            i += 1;
            if c == '\'' && chars.get(i) != Some(&'\'') {
                break;
            }
            if c == '\'' {
                self.push('\'', i);
                i += 1;
            }
        }
        // A string, like a variable, may be indexed: `' *'(1+G)`.
        self.last = Last::Variable;
        Ok(i)
    }

    /// A comment, `c` and the rest of the line, each character as
    /// itself.
    ///
    /// # Errors
    /// CHARACTER ERROR for a character APL80 has not got.
    pub(crate) fn comment(&mut self, chars: &[char], at: usize) -> AplResult<usize> {
        self.push('⍝', at);
        for (i, c) in chars.iter().enumerate().skip(at + 1) {
            self.datum(*c, i)?;
        }
        Ok(chars.len())
    }

    /// A name: a capital, then capitals, digits, `.` and `-`.
    pub(crate) fn word(&mut self, chars: &[char], at: usize) -> usize {
        let mut i = at;
        while i < chars.len() && matches!(chars[i], 'A'..='Z' | '0'..='9' | '.' | '-') {
            i += 1;
        }
        let typed: String = chars[at..i].iter().collect();
        let held = name(&typed);
        self.token = self.glyphs.len();
        for (k, g) in held.chars().enumerate() {
            self.push(g, at + k);
        }
        self.last = if (self.function)(&held) {
            Last::Function
        } else {
            Last::Variable
        };
        i
    }

    /// A number, its minus the negative sign, an exponent's too.
    pub(crate) fn number(&mut self, chars: &[char], at: usize) -> usize {
        self.token = self.glyphs.len();
        let mut i = at;
        while i < chars.len() {
            let c = chars[i];
            let exponent = i > at && chars[i - 1] == 'E';
            let part =
                c.is_ascii_digit() || c == '.' || c == 'E' || (c == '-' && (i == at || exponent));
            if !part {
                break;
            }
            self.push(if c == '-' { '¯' } else { c }, i);
            i += 1;
        }
        self.last = Last::Value;
        i
    }
}
