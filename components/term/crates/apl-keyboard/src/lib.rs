//! The 2741 keyboard: what a keystroke means, and the line being
//! typed.
//!
//! Base, backspace, overstrike is how an APL glyph is typed. A 2741
//! formed `⍟` from `○`, backspace, `*`, and this is where that
//! happens: the keyboard holds the line as cells, one glyph each, and
//! an overstrike replaces the cell before the carriage rather than
//! adding to it.
//!
//! Nothing here touches a terminal or a socket. The composed line is
//! what leaves, which is what lets the same state machine drive a
//! terminal and a browser from one overstrike table.

mod compose;
mod edit;
mod keys;

pub use edit::Move;
pub use keys::Keyboard;
