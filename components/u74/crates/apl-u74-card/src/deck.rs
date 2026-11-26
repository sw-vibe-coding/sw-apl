//! A deck: a text file, one card a line.

use crate::card::Card;

/// The cards of `text`, one a line (LF or CRLF), numbered from 1. A
/// blank line is a blank card.
#[must_use]
pub fn deck(text: &str) -> Vec<Card> {
    text.lines()
        .enumerate()
        .map(|(at, line)| Card::new(at + 1, line))
        .collect()
}
