//! The TRS-80's screen, as (C) '80 holds and shows it (docs/mode-c.md).
//!
//! APL80's `#` turns a number into the character with that code, and
//! some codes are not printed but obeyed: they move the cursor and
//! clear the screen, draw the 2-by-3 graphics blocks, compress spaces,
//! or, negative, place the next character as BASIC's PRINT @ does.
//! (C) holds each as a character -- a control code as itself, a block
//! as the Unicode sextant that draws it, the rest in the Private Use
//! Area -- and sends it as the terminal code that does the same, which
//! a terminal obeys and the demo's 64-by-16 screen, `Screen`, obeys too.

mod codes;
mod motion;
mod screen;

pub use codes::{character, code, sent};
pub use screen::Screen;
