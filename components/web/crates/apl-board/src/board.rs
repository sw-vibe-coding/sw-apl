//! The keyboard as the page holds it.

use apl_keyboard::Keyboard;
use apl_modes::{Feature, Mode};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::draw::state;
use crate::press::{Act, act, trs80};

/// The keyboard for one page: a 2741's, or in (C) a TRS-80's.
#[wasm_bindgen]
pub struct Board {
    keys: Keyboard,
    /// Whether a key types its own character, case swapped, as on the
    /// TRS-80 APL80 ran on, rather than the 2741's glyph.
    trs80: bool,
}

#[wasm_bindgen]
impl Board {
    /// A keyboard with an empty line for `mode` -- "A" or "70", "B" or
    /// "75", "C" or "80", anything else being (A): the 2741 map,
    /// composing that mode's overstrikes, or in (C) the TRS-80's keys.
    /// A page switching modes starts a new session and makes a new
    /// board for it.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(mode: &str) -> Board {
        let mode = Mode::parse(mode).unwrap_or_default();
        let mut keys = Keyboard::default();
        keys.also = mode.overstrikes();
        let trs80 = mode.profile().has(Feature::Apl80Keys);
        Board { keys, trs80 }
    }

    /// Take one keystroke and hand back the line.
    ///
    /// The answer is the JSON `draw::state` describes. Enter with a
    /// strike still waiting rings instead of sending: the glyph is
    /// half typed. On the TRS-80 a key types its character, an
    /// unshifted letter the capital and a shifted one the lower-case
    /// code APL80 reads as a symbol.
    #[must_use]
    pub fn press(&mut self, key: &str, ctrl: bool, alt: bool) -> String {
        let mut bell = false;
        let mut submit = false;
        let typed = act(key, ctrl, alt);
        match if self.trs80 { trs80(typed) } else { typed } {
            Act::Type(c) => bell = !self.keys.type_key(c),
            Act::Literal(c) => bell = !self.keys.paste(&c.to_string()),
            Act::Overstrike => bell = !self.keys.overstrike(),
            Act::Backspace => self.keys.backspace(),
            Act::Delete => self.keys.delete(),
            Act::Carriage(where_to) => self.keys.move_cursor(where_to),
            Act::Clear => self.keys.clear(),
            Act::Submit if self.keys.pending => bell = true,
            Act::Submit => submit = true,
            Act::Ignore => return state(&mut self.keys, false, false, false),
        }
        state(&mut self.keys, bell, submit, true)
    }

    /// Put text in literally, which is what a paste is: Unicode
    /// already, not keystrokes to translate.
    #[must_use]
    pub fn paste(&mut self, text: &str) -> String {
        let bell = !self.keys.paste(text);
        state(&mut self.keys, bell, false, true)
    }

    /// The finished line, and a fresh one after it.
    #[must_use]
    pub fn take(&mut self) -> String {
        let line = self.keys.text();
        self.keys.clear();
        line
    }
}
