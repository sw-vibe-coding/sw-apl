//! Overstrikes: the 2741 way of typing an APL glyph.
//!
//! On a 2741 you formed `⍟` by typing `○`, backspacing, and typing
//! `*` over it. The keyboard carried only the foundational
//! characters; everything else was struck from two of them.
//!
//! The table lives in `data/glyphs.toml` with the rest of the glyph
//! data and is generated into `apl-value`, so that the CLI and the
//! browser terminal read one table rather than two.

mod machine;
mod strike;

pub use machine::{BACK, BACK_LABEL, BACKSPACE, Strike, compose};
pub use strike::{read, strike};
