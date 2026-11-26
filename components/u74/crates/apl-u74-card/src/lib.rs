//! A punched card as (U) '74 reads it: 80 columns, the statement in
//! 1-72 and a sequence number in 73-80, punched on an IBM 029
//! (docs/mode-u.md). A deck is a text file, one card a line.

mod card;
mod deck;
mod punch;

pub use card::{COLUMNS, Card, STATEMENT};
pub use deck::deck;
pub use punch::{PUNCHED, punchable};
