//! Editing the line: what a 2741 could not do, and a reader expects.

use crate::keys::Keyboard;

/// Where the carriage is asked to go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    /// One cell back.
    Left,
    /// One cell on.
    Right,
    /// To the start of the line.
    Home,
    /// To the end of it.
    End,
}

impl Keyboard {
    /// Delete the cell before the carriage -- a whole glyph,
    /// underscore and all. A strike waiting to be made is cancelled
    /// instead, which is how a reader backs out of one.
    pub fn backspace(&mut self) {
        if self.pending {
            self.pending = false;
        } else if self.cursor > 0 {
            self.cursor -= 1;
            self.cells.remove(self.cursor);
        }
    }

    /// Delete the cell the carriage is on.
    pub fn delete(&mut self) {
        self.pending = false;
        if self.cursor < self.cells.len() {
            self.cells.remove(self.cursor);
        }
    }

    /// Move the carriage, which cancels a waiting strike: the base
    /// it was to strike is no longer the cell before it.
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
