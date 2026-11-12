//! A line as typed in APL80's characters, into glyphs.

use apl_value::{AplError, AplResult};

/// A line in glyphs, and for each glyph where in the typed line it came
/// from, so an error's caret can be put back under what was typed.
#[derive(Debug, PartialEq, Eq)]
pub struct Typed {
    pub text: String,
    pub from: Vec<usize>,
}

/// What came last, which settles what comes next: an arrow is take or
/// grade, a round bracket indexes or groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Last {
    /// Nothing yet, or something an argument follows.
    Start,
    /// A number, a string, a closing bracket, quad input.
    Value,
    /// A name that is not a function.
    Variable,
    /// A function or an operator.
    Function,
}

/// The glyphs so far.
pub(crate) struct Out<'a> {
    pub(crate) glyphs: Vec<char>,
    pub(crate) from: Vec<usize>,
    pub(crate) last: Last,
    /// Where the last name or number began, in the glyphs.
    pub(crate) token: usize,
    /// For each round bracket open, whether it indexes.
    pub(crate) opened: Vec<bool>,
    pub(crate) function: &'a dyn Fn(&str) -> bool,
}

/// `line`, typed in APL80's characters, in glyphs. `function` says
/// whether a name, as the glyphs hold it, is a function now.
///
/// # Errors
/// CHARACTER ERROR for a character APL80 has not got, NONCE ERROR for
/// peek and poke, SYNTAX ERROR for an axis with nothing before it; the
/// caret is under the character typed.
pub fn glyphs(line: &str, function: &dyn Fn(&str) -> bool) -> AplResult<Typed> {
    let chars: Vec<char> = line.chars().collect();
    // A workspace file's directives, `⍝!MODES (C)` and the rest, are
    // comments no keyboard of APL80's could type: the machine's lines,
    // passed as they are.
    if line.trim_start().starts_with('⍝') {
        return Ok(Typed {
            text: line.to_string(),
            from: (0..chars.len()).collect(),
        });
    }
    let mut out = Out {
        glyphs: Vec::new(),
        from: Vec::new(),
        last: Last::Start,
        token: 0,
        opened: Vec::new(),
        function,
    };
    let mut i = 0;
    while i < chars.len() {
        i = out.step(&chars, i)?;
    }
    Ok(Typed {
        text: out.glyphs.iter().collect(),
        from: out.from,
    })
}

impl Out<'_> {
    /// Put `g` down, typed at `at`.
    pub(crate) fn push(&mut self, g: char, at: usize) {
        self.glyphs.push(g);
        self.from.push(at);
    }

    /// The token at `i`, put down; where the next one starts.
    fn step(&mut self, chars: &[char], i: usize) -> AplResult<usize> {
        let c = chars[i];
        match c {
            '\'' => return self.quoted(chars, i),
            'c' => return self.comment(chars, i),
            'A'..='Z' => return Ok(self.word(chars, i)),
            '0'..='9' | '-' => return Ok(self.number(chars, i)),
            _ => {}
        }
        if !self.mark(c, i)? {
            self.datum(c, i)?;
            if c != ' ' {
                self.last = if matches!(c, 'q' | 'm') {
                    Last::Value
                } else {
                    Last::Function
                };
            }
        }
        Ok(i + 1)
    }
}

impl Typed {
    /// `err`, its caret moved from the glyphs back to where it was
    /// typed. An error a function's line raised keeps its caret: that
    /// line was typed, and placed, on its own.
    #[must_use]
    pub fn place(&self, mut err: AplError) -> AplError {
        if err.context.is_none() {
            let end = self.from.last().map_or(0, |at| at + 1);
            err.caret = err.caret.map(|c| self.from.get(c).copied().unwrap_or(end));
        }
        err
    }
}
