//! What each mode adds to APL\360's characters, and what it can do.
//!
//! One place answers it, so the lexer, the evaluator and every front
//! end that composes an overstrike cannot disagree about a mode.

use apl_value::{MODE_B, OVERSTRIKE_B};

use apl_profile::{A70, B75, C80, Profile};

use crate::mode::Mode;

impl Mode {
    /// The later glyphs this mode takes as primitives beyond
    /// APL\360's: none in (A), execute and format in (B) -- and in
    /// (B) quad, which there begins a system name before a letter --
    /// and in (C) its conversion and Call, `#`, and its peek and poke,
    /// the shifted U, held as `⌷`; its format is a statement, not a glyph.
    #[must_use]
    pub fn glyphs(self) -> &'static str {
        match self {
            Mode::A => "",
            Mode::C => "#⌷",
            Mode::B => MODE_B,
        }
    }

    /// The overstrikes this mode forms beyond APL\360's: none in (A),
    /// the 5100's pairs for execute and format in (B), none in (C),
    /// whose keyboard types every character it has without one.
    #[must_use]
    pub fn overstrikes(self) -> &'static [(char, char, char)] {
        match self {
            Mode::A | Mode::C => &[],
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
            Mode::C => 'C',
        }
    }
}

impl Mode {
    /// What this mode can do (`apl-profile`).
    #[must_use]
    pub fn profile(self) -> Profile {
        match self {
            Mode::A => A70,
            Mode::B => B75,
            Mode::C => C80,
        }
    }
}
