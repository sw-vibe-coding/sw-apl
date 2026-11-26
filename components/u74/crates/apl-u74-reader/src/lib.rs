//! The card reader of a (U) '74 run: the cards not yet read, and the
//! printout so far, shared between the run and the console a running
//! statement reads through. `⎕` and `⍞` read the next card of the
//! deck as data (docs/mode-u.md, "Input").

mod reader;
mod tray;

pub use reader::Reader;
pub use tray::{Shared, Tray};
