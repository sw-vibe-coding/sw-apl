//! Putting a character down, moving the cursor, and clearing.

use crate::screen::{HIGH, Screen, WIDE};

impl Screen {
    /// `c` at the cursor, which moves on; past the end of a line it
    /// wraps to the next.
    pub(crate) fn put(&mut self, c: char) {
        if self.col >= WIDE {
            self.newline();
        }
        self.cells[self.row][self.col] = c;
        self.col += 1;
    }

    /// The start of the next line, scrolling up past the foot.
    pub(crate) fn newline(&mut self) {
        self.col = 0;
        if self.row + 1 < HIGH {
            self.row += 1;
            return;
        }
        self.cells.remove(0);
        self.cells.push(vec![' '; WIDE]);
    }

    /// One escape sequence, `ESC [ params last`: `H` home or to a line
    /// and column, `J` clear to the end of the screen, `K` to the end of
    /// the line, `A` to `D` up, down, right and left. Any other is
    /// ignored, as the cursor's showing and hiding are.
    pub(crate) fn escape(&mut self, params: &str, last: char) {
        let n = |i: usize| -> usize {
            params
                .split(';')
                .nth(i)
                .and_then(|p| p.parse().ok())
                .unwrap_or(1)
        };
        match last {
            'H' => (self.row, self.col) = ((n(0) - 1).min(HIGH - 1), (n(1) - 1).min(WIDE - 1)),
            'J' => self.clear(true),
            'K' => self.clear(false),
            'A' => self.row = self.row.saturating_sub(1),
            'B' => self.row = (self.row + 1).min(HIGH - 1),
            'C' => self.col = (self.col + 1).min(WIDE - 1),
            'D' => self.col = self.col.saturating_sub(1),
            _ => {}
        }
    }

    /// Blank from the cursor to the end of the line, and to the end of
    /// the screen when `below`.
    fn clear(&mut self, below: bool) {
        let (row, col) = (self.row, self.col.min(WIDE));
        self.cells[row][col..].fill(' ');
        if below {
            self.cells[row + 1..].iter_mut().for_each(|r| r.fill(' '));
        }
    }
}
