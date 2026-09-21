//! Reading one line at a 2741 keyboard.
//!
//! Raw keystrokes stay here. What leaves is the composed line, which
//! is what the protocol carries and what `Session::respond` wants --
//! the service never learns that `⍟` was typed as `○`, a key, and
//! `*`.

mod actions;
mod input;
mod line;

pub use line::{read_line, wait};
