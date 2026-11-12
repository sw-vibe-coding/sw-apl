//! The screen: 16 lines of 64 characters and a cursor.

/// How many characters a line holds.
pub(crate) const WIDE: usize = 64;
/// How many lines the screen shows.
pub(crate) const HIGH: usize = 16;

/// The TRS-80's screen: what is on it, and where the next character
/// goes. Printing past the foot scrolls it up a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub(crate) cells: Vec<Vec<char>>,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            cells: vec![vec![' '; WIDE]; HIGH],
            row: 0,
            col: 0,
        }
    }
}

impl Screen {
    /// Print `text`, obeying the terminal codes in it: a new line, a
    /// return to the start of the line, a backspace, and the escape
    /// sequences `sent` gives the TRS-80's codes as.
    pub fn print(&mut self, text: &str) {
        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\n' => self.newline(),
                '\r' => self.col = 0,
                '\x08' => self.col = self.col.saturating_sub(1),
                '\x1b' if chars.peek() == Some(&'[') => {
                    chars.next();
                    let mut params = String::new();
                    while let Some(&p) = chars.peek().filter(|p| !p.is_ascii_alphabetic()) {
                        params.push(p);
                        chars.next();
                    }
                    let last = chars.next().unwrap_or(' ');
                    self.escape(&params, last);
                }
                c if !c.is_control() => self.put(c),
                _ => {}
            }
        }
    }

    /// The lines on the screen, top to foot, each 64 characters.
    #[must_use]
    pub fn rows(&self) -> Vec<String> {
        self.cells.iter().map(|r| r.iter().collect()).collect()
    }

    /// Where the next character goes: the line and the column.
    #[must_use]
    pub fn cursor(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}
