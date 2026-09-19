//! Typing an overstruck glyph, one keystroke at a time.
//!
//! A 2741 had no delete: backspace positioned the carriage and the
//! next character struck over what was there. A line editor needs
//! backspace for deleting, so sw-apl puts the carriage move on a key
//! of its own -- see `BACK_LABEL` -- and this is what that key and
//! the characters around it drive.

use crate::strike::strike;

/// The keystroke that takes the last character back to be struck
/// over, named once so that no help text can name a different key
/// from the one bound.
///
/// `Ctrl-]`, and not `Ctrl-H`: `Ctrl-H` *is* `0x08`, byte-identical
/// to backspace, so a terminal cannot tell them apart, and rustyline
/// binds it besides. `0x1D` is bound by neither rustyline nor the
/// terminal's line discipline -- SIGQUIT is `Ctrl-\`, `0x1C` -- and
/// by no browser shortcut known. `Ctrl-^` (`0x1E`) is the fallback
/// if a platform turns out to claim it.
pub const BACK: char = '\u{1d}';

/// How to say it. The label and the binding come from here together,
/// so the screen cannot promise a key that does nothing.
pub const BACK_LABEL: &str = "Ctrl-]";

/// The state of typing an overstrike: what was last put on the
/// line, and what is being struck over.
#[derive(Debug, Default)]
pub struct Strike {
    last: Option<char>,
    held: Option<char>,
    refused: Option<(char, char)>,
}

impl Strike {
    /// One character typed: what to put on the line, or `None` when
    /// it struck nothing, in which case `refused` says what was
    /// typed so the caller can report it.
    ///
    /// Text, not a character: an underscored letter is a letter and
    /// a combining low line. `last` keeps the final code point, so
    /// the key below takes back the low line and a third impression
    /// on one position forms nothing -- which is what it is.
    pub fn typed(&mut self, c: char) -> Option<String> {
        self.refused = None;
        let Some(held) = self.held.take() else {
            self.last = Some(c);
            return Some(c.to_string());
        };
        let struck = strike(held, c);
        match &struck {
            Some(glyph) => self.last = glyph.chars().next_back(),
            None => self.refused = Some((held, c)),
        }
        struck
    }

    /// The overstrike key: take the last character back, so the next
    /// one strikes over it rather than following it. The character
    /// taken back is returned, for the caller to remove from the
    /// line it is showing.
    ///
    /// `None` when there is nothing to take back -- the start of a
    /// line, or a second press, which is where a carriage would meet
    /// the left margin.
    pub fn back(&mut self) -> Option<char> {
        self.held = self.last.take();
        self.held
    }

    /// The two characters of a strike that formed no glyph. The
    /// manual calls that an illegitimate overstrike and gives it as
    /// the cause of a CHARACTER error.
    #[must_use]
    pub fn refused(&self) -> Option<(char, char)> {
        self.refused
    }
}

/// The character a 2741 actually transmitted to move the carriage
/// back: `0x08`. A file, or another terminal, writes this.
pub const BACKSPACE: char = '\u{8}';

/// A whole line with its overstrikes formed.
///
/// Either marker moves the carriage back one: `BACK`, which is what
/// sw-apl's own key inserts because a line editor needs `0x08` for
/// deleting, and `BACKSPACE` itself, which is what a 2741 sent and
/// what a file written elsewhere will hold.
///
/// # Errors
/// The two characters of a strike that forms no glyph -- an
/// illegitimate overstrike, which the caller reports as a CHARACTER
/// error.
pub fn compose(line: &str) -> Result<String, (char, char)> {
    if !line.contains(BACK) && !line.contains(BACKSPACE) {
        return Ok(line.to_string());
    }
    let mut out = String::new();
    let mut back = false;
    for c in line.chars() {
        if c == BACK || c == BACKSPACE {
            back = true;
            continue;
        }
        let held = if back { out.pop() } else { None };
        back = false;
        match held {
            Some(held) => out.push_str(&strike(held, c).ok_or((held, c))?),
            None => out.push(c),
        }
    }
    Ok(out)
}
