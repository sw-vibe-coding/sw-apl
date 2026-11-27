//! (U) '74 in a browser. The page picks a demo deck, shows its cards
//! one at a time, and submits it: what it asks of the WebAssembly is
//! here, plain Rust and tested natively, and only the exports are
//! wasm's.

mod decks;
mod punch;

pub use decks::{DECKS, Deck, catalog, printout};
pub use punch::{holes, reference};

#[cfg(target_arch = "wasm32")]
mod wasm;
