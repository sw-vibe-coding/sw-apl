//! Punching: the holes in each of a card's 80 columns, and how a
//! card spells what the 029 cannot punch.

use apl_u74_card::{COLUMNS, Card};
use apl_u74_code::holes as code;
use apl_u74_mnemonic::MNEMONICS;

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

/// The `$` spellings, one a line: the spelling, a space, the glyph.
#[must_use]
pub fn reference() -> String {
    let lines: Vec<String> = MNEMONICS
        .iter()
        .map(|(code, glyph)| format!("${code} {glyph}"))
        .collect();
    lines.join("\n")
}
