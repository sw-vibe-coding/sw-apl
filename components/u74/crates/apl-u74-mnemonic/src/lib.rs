//! The `$` spellings of (U) '74: a dollar and two letters for each APL
//! character an IBM 029 cannot punch (docs/mode-u.md, "The `$`
//! mnemonics"). Cards hold the spellings; the interpreter sees glyphs.

mod spell;
mod table;

pub use spell::{Spelled, spell};
pub use table::{MNEMONICS, glyph, mnemonic};
