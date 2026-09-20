use crate::Keyboard;

impl Keyboard {
    pub fn overstrike(&mut self) {
        if self.cursor > 0 {
            self.pending = true;
        }
    }

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
        self.paste(&text);
        true
    }

    fn strike(&mut self, text: &str) -> bool {
        let base = &self.cells[self.cursor - 1];
        if base.chars().count() != 1 || text.chars().count() != 1 {
            return false;
        }
        let Some(glyph) =
            apl_strike::strike(base.chars().next().unwrap(), text.chars().next().unwrap())
        else {
            return false;
        };
        self.cells[self.cursor - 1] = glyph;
        self.pending = false;
        true
    }

    pub fn paste(&mut self, text: &str) {
        self.pending = false;
        for c in text.chars().filter(|c| !c.is_control()) {
            if c == '\u{332}' && self.cursor > 0 {
                self.cells[self.cursor - 1].push(c);
            } else {
                self.cells.insert(self.cursor, c.to_string());
                self.cursor += 1;
            }
        }
    }
}
