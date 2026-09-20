use crate::Keyboard;

pub enum Move {
    Left,
    Right,
    Home,
    End,
}

impl Keyboard {
    pub fn backspace(&mut self) {
        if self.pending {
            self.pending = false;
        } else if self.cursor > 0 {
            self.cursor -= 1;
            self.cells.remove(self.cursor);
        }
    }

    pub fn delete(&mut self) {
        self.pending = false;
        if self.cursor < self.cells.len() {
            self.cells.remove(self.cursor);
        }
    }

    pub fn move_cursor(&mut self, direction: Move) {
        self.pending = false;
        self.cursor = match direction {
            Move::Left => self.cursor.saturating_sub(1),
            Move::Right => (self.cursor + 1).min(self.cells.len()),
            Move::Home => 0,
            Move::End => self.cells.len(),
        };
    }
}
