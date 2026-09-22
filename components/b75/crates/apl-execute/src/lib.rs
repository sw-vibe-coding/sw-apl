//! Execute, `⍎`: a character scalar or vector run as a line of APL in
//! the workspace. (B) '75 has it and (A) '70 does not; the lexer only
//! lets the glyph through in (B). The IBM 5110 APL Reference Manual,
//! Chapter 4, and docs/mode-b.md.

mod execute;

pub use execute::{execute, value, whole};
