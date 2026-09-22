//! The line being typed, as the carriage holds it.

use std::collections::BTreeMap;

/// A 2741 keyboard part way through a line.
///
/// The line is cells, not characters: an overstruck glyph and an
/// underscored letter are each one cell, so that backspace deletes a
/// glyph rather than half of one and the carriage counts positions
/// the way the paper does.
#[derive(Debug)]
pub struct Keyboard {
    /// The line, one glyph per cell.
    pub(crate) cells: Vec<String>,
    /// Which cell the carriage is on.
    pub cursor: usize,
    /// Set by the overstrike key: the next key strikes the cell
    /// before the carriage instead of adding one.
    pub pending: bool,
    /// Take keys as the Unicode they already are, translating
    /// nothing. For a keyboard that sends APL glyphs itself.
    pub literal: bool,
    /// What each key sends, which `keymap.json` supplies and a
    /// reader may replace to match their own keycaps.
    pub map: BTreeMap<char, String>,
    /// The overstrikes the session's mode forms beyond APL\360's:
    /// none in (A), the 5100's in (B). Set from what the service says
    /// the mode is; a keyboard does not choose it.
    pub also: &'static [(char, char, char)],
}

impl Default for Keyboard {
    /// A keyboard with the default 2741 map and an empty line.
    fn default() -> Keyboard {
        Keyboard {
            cells: Vec::new(),
            cursor: 0,
            pending: false,
            literal: false,
            map: serde_json::from_str(include_str!("../keymap.json"))
                .expect("the built-in keymap is valid JSON"),
            also: &[],
        }
    }
}

impl Keyboard {
    /// The line as it would go on the wire: composed glyphs, in
    /// order, and nothing of how they were typed.
    #[must_use]
    pub fn text(&self) -> String {
        self.cells.concat()
    }

    /// What the carriage has passed, which is what says where on the
    /// page it is.
    #[must_use]
    pub fn before_cursor(&self) -> String {
        self.cells[..self.cursor].concat()
    }

    /// Start a fresh line.
    pub fn clear(&mut self) {
        self.cells.clear();
        self.cursor = 0;
        self.pending = false;
    }
}
