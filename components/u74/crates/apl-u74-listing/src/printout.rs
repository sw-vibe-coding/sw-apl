//! The banner and the cards.

use apl_session::INDENT;
use apl_u74_card::Card;

use crate::locate::located;

/// What the processor prints first.
pub const BANNER: &str = "UNIVERSITY OF MARYLAND APL";

/// The banner, with the date and time when there is a clock, and the
/// clear workspace the run starts with.
#[must_use]
pub fn banner(when: Option<&str>) -> Vec<String> {
    let first = when.map_or_else(|| BANNER.to_string(), |w| format!("{BANNER}   {w}"));
    vec![first, "CLEAR WS".to_string()]
}

/// A card as punched: its number in four columns and two blanks, so
/// that its column 1 is under the echoed statement's first character.
#[must_use]
pub fn card(card: &Card) -> String {
    format!("{:04}  {}", card.number, card.columns)
}

/// A CHARACTER ERROR at `column` of `card`: a character no glyph
/// stands for there, the card's text with a caret under it.
#[must_use]
pub fn character(card: &Card, column: usize) -> Vec<String> {
    vec![
        "CHARACTER ERROR".to_string(),
        format!("{INDENT}{}", card.statement()),
        format!("{INDENT}{}^", " ".repeat(column - 1)),
        located(card.number, Some(column)),
    ]
}
