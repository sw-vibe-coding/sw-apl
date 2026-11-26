//! One card.

/// How many columns a card has.
pub const COLUMNS: usize = 80;

/// How many of them hold the statement; the rest are the sequence
/// number, which only the listing reads.
pub const STATEMENT: usize = 72;

/// One card: its place in the deck, counted from 1, and its columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    /// Its place in the deck.
    pub number: usize,
    /// Its columns as punched, at most 80, trailing blanks dropped.
    pub columns: String,
}

impl Card {
    /// Card `number` holding `line`: what lies past column 80 is not
    /// on it.
    #[must_use]
    pub fn new(number: usize, line: &str) -> Card {
        let columns: String = line.chars().take(COLUMNS).collect();
        let columns = columns.trim_end().to_string();
        Card { number, columns }
    }

    /// The statement: columns 1-72, trailing blanks dropped.
    #[must_use]
    pub fn statement(&self) -> String {
        let text: String = self.columns.chars().take(STATEMENT).collect();
        text.trim_end().to_string()
    }

    /// The word of a control card, one with `@` in column 1: `RUN`,
    /// `APL`, `FIN`. `None` for a card of APL.
    #[must_use]
    pub fn control(&self) -> Option<&str> {
        let rest = self.columns.strip_prefix('@')?;
        let end = rest.find([',', ' ']).unwrap_or(rest.len());
        Some(&rest[..end])
    }
}
