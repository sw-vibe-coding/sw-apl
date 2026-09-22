//! The keyboard as the page holds it.

use apl_keyboard::Keyboard;
use apl_modes::Mode;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::draw::state;
use crate::press::{Act, act};

/// A 2741 keyboard for one page.
#[wasm_bindgen]
pub struct Board(Keyboard);

#[wasm_bindgen]
impl Board {
    /// A keyboard with the 2741 map and an empty line, composing the
    /// overstrikes of `mode` -- "A" or "70", "B" or "75", anything
    /// else being (A). A page switching modes starts a new session and
    /// makes a new board for it.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(mode: &str) -> Board {
        let mut keyboard = Keyboard::default();
        keyboard.also = Mode::parse(mode).unwrap_or_default().overstrikes();
        Board(keyboard)
    }

    /// Take one keystroke and hand back the line.
    ///
    /// The answer is the JSON `draw::state` describes. Enter with a
    /// strike still waiting rings instead of sending: the glyph is
    /// half typed.
    #[must_use]
    pub fn press(&mut self, key: &str, ctrl: bool, alt: bool) -> String {
        let mut bell = false;
        let mut submit = false;
        match act(key, ctrl, alt) {
            Act::Type(c) => bell = !self.0.type_key(c),
            Act::Literal(c) => bell = !self.0.paste(&c.to_string()),
            Act::Overstrike => self.0.overstrike(),
            Act::Backspace => self.0.backspace(),
            Act::Delete => self.0.delete(),
            Act::Carriage(where_to) => self.0.move_cursor(where_to),
            Act::Clear => self.0.clear(),
            Act::Submit if self.0.pending => bell = true,
            Act::Submit => submit = true,
            Act::Ignore => return state(&self.0, false, false, false),
        }
        state(&self.0, bell, submit, true)
    }

    /// Put text in literally, which is what a paste is: Unicode
    /// already, not keystrokes to translate.
    #[must_use]
    pub fn paste(&mut self, text: &str) -> String {
        let bell = !self.0.paste(text);
        state(&self.0, bell, false, true)
    }

    /// The finished line, and a fresh one after it.
    #[must_use]
    pub fn take(&mut self) -> String {
        let line = self.0.text();
        self.0.clear();
        line
    }
}
