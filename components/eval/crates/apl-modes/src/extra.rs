//! What each mode adds to APL\360's characters.
//!
//! One place answers it, so the lexer, the evaluator and every front
//! end that composes an overstrike cannot disagree about a mode.

use apl_value::{MODE_B, OVERSTRIKE_B};

use crate::mode::Mode;

impl Mode {
    /// The later glyphs this mode takes as primitives beyond
    /// APL\360's: none in (A), execute and format in (B) -- and in
    /// (B) quad, which there begins a system name before a letter.
    #[must_use]
    pub fn glyphs(self) -> &'static str {
        match self {
            Mode::A => "",
            Mode::B => MODE_B,
        }
    }

    /// The overstrikes this mode forms beyond APL\360's: none in (A),
    /// the 5100's pairs for execute and format in (B).
    #[must_use]
    pub fn overstrikes(self) -> &'static [(char, char, char)] {
        match self {
            Mode::A => &[],
            Mode::B => &OVERSTRIKE_B,
        }
    }

    /// The mode's letter, as a workspace's modes line and the frame a
    /// service sends its terminal write it.
    #[must_use]
    pub fn letter(self) -> char {
        match self {
            Mode::A => 'A',
            Mode::B => 'B',
        }
    }
}
