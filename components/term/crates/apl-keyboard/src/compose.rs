//! Forming a glyph: base, overstrike key, overstrike.

use crate::keys::Keyboard;

impl Keyboard {
    /// The overstrike key: hold the cell the carriage just passed, so
    /// that the next key strikes it rather than adding a cell. On a
    /// 2741 this was backspace; a line editor needs backspace for
    /// deleting, so it is a key of its own here.
    pub fn overstrike(&mut self) {
        if self.cursor > 0 {
            self.pending = true;
        }
    }

    /// Type one key. `false` when it was meant to strike the cell
    /// before the carriage and the pair forms no glyph, which is a
    /// 2741 ringing its bell and leaving the base where it was.
    pub fn type_key(&mut self, c: char) -> bool {
        let text = if self.literal {
            c.to_string()
        } else {
            self.map
                .get(&c)
                .cloned()
                .unwrap_or_else(|| c.to_ascii_uppercase().to_string())
        };
        if self.pending {
            return self.strike(&text);
        }
        self.paste(&text)
    }

    /// Replace the cell before the carriage with the glyph the pair
    /// forms. Only single characters strike: an already overstruck
    /// cell is not a base for a third.
    fn strike(&mut self, text: &str) -> bool {
        let base = &self.cells[self.cursor - 1];
        if base.chars().count() != 1 || text.chars().count() != 1 {
            return false;
        }
        let (Some(base), Some(over)) = (base.chars().next(), text.chars().next()) else {
            return false;
        };
        let Some(glyph) = apl_strike::strike(base, over) else {
            return false;
        };
        self.cells[self.cursor - 1] = glyph;
        self.pending = false;
        true
    }

    /// Put text in at the carriage: a combining low line joins the cell
    /// before it, since an underscored letter is one glyph, and control
    /// characters are dropped because a 2741 has no keys for them.
    ///
    /// One character arriving while a strike is pending strikes, as a
    /// typed key would, and `false` is a pair that forms no glyph -- the
    /// bell, with the base left as it was. The strike was waiting for
    /// the next input and this is it, however it came: the on-screen
    /// board sends every glyph by paste, and so does an expander, and
    /// both used to drop the strike and put the glyph beside its base.
    /// More than one character is not the second key of an overstrike,
    /// so it abandons the strike and goes in as typed.
    pub fn paste(&mut self, text: &str) -> bool {
        if self.pending {
            let mut chars = text.chars();
            if let (Some(c), None) = (chars.next(), chars.next()) {
                return self.strike(&c.to_string());
            }
        }
        self.pending = false;
        for c in text.chars().filter(|c| !c.is_control()) {
            if c == '\u{332}' && self.cursor > 0 {
                self.cells[self.cursor - 1].push(c);
            } else {
                self.cells.insert(self.cursor, c.to_string());
                self.cursor += 1;
            }
        }
        true
    }
}
