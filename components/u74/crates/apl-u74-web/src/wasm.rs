//! What the page calls. Each is a tested function of this crate,
//! passed through.

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{DECKS, holes, printout};

/// How many demo decks there are.
#[wasm_bindgen]
#[must_use]
pub fn decks() -> usize {
    DECKS.len()
}

/// Demo deck `i`'s title and cards, a title line and then a card to
/// a line; empty past the last deck.
#[wasm_bindgen]
#[must_use]
pub fn deck(i: usize) -> String {
    DECKS
        .get(i)
        .map(|d| format!("{}\n{}", d.title, d.cards.trim_end()))
        .unwrap_or_default()
}

/// The holes of each column of the card `line`.
#[wasm_bindgen]
#[must_use]
pub fn card(line: &str) -> Vec<u16> {
    holes(line)
}

/// The printout of `cards` submitted as a batch job.
#[wasm_bindgen]
#[must_use]
pub fn submit(cards: &str) -> String {
    printout(cards)
}
