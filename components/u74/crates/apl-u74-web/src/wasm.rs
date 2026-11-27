//! What the page calls. Each is a tested function of this crate,
//! passed through.

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{catalog, holes, printout, reference};

/// Every demo deck: a title line and a card to a line, a form feed
/// between decks.
#[wasm_bindgen]
#[must_use]
pub fn decks() -> String {
    catalog()
}

/// The holes of each column of the card `line`.
#[wasm_bindgen]
#[must_use]
pub fn card(line: &str) -> Vec<u16> {
    holes(line)
}

/// The `$` spellings, one a line: the spelling, a space, the glyph.
#[wasm_bindgen]
#[must_use]
pub fn spellings() -> String {
    reference()
}

/// The printout of `cards` submitted as a batch job.
#[wasm_bindgen]
#[must_use]
pub fn submit(cards: &str) -> String {
    printout(cards)
}
