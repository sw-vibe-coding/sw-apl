//! A card's face: the holes in each of its 80 columns.

use apl_u74_card::{COLUMNS, Card};
use apl_u74_code::holes as code;

/// The holes of each column of the card punched from `line`; a
/// character an 029 cannot punch, which a file may hold, is all ones.
#[must_use]
pub fn holes(line: &str) -> Vec<u16> {
    let card = Card::new(1, line);
    let mut columns: Vec<u16> = card
        .columns
        .chars()
        .map(|c| code(c).map_or(u16::MAX, |h| h.0))
        .collect();
    columns.resize(COLUMNS, 0);
    columns
}
