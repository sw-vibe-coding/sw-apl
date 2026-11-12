//! The memory: a screen and the bytes poked above it.

use std::collections::BTreeMap;
use std::ops::RangeInclusive;

use apl_c80_screen::Screen;

/// The screen's addresses: 64 by 16 characters, a line at a time.
pub const SCREEN: RangeInclusive<i64> = 15360..=16383;
/// The RAM's addresses: from the screen's end to the top of 48K.
pub const RAM: RangeInclusive<i64> = 16384..=65535;

/// The modeled memory.
#[derive(Debug, Clone, Default)]
pub struct Machine {
    /// What is on the screen: everything (C) has shown, the lines typed
    /// and what a poke put there.
    pub screen: Screen,
    /// The bytes poked into RAM; every other byte there is 0.
    pub ram: BTreeMap<i64, u8>,
}

impl Machine {
    /// The byte at `at`: on the screen, the code of the character shown
    /// there; in RAM, what was poked, or 0. `None` for an address the
    /// model has not got.
    #[must_use]
    pub fn read(&self, at: i64) -> Option<u8> {
        if RAM.contains(&at) {
            return Some(self.ram.get(&at).copied().unwrap_or(0));
        }
        let pos = usize::try_from(at - SCREEN.start())
            .ok()
            .filter(|_| SCREEN.contains(&at))?;
        let row = &self.screen.rows()[pos / 64];
        Some(code(row.chars().nth(pos % 64).unwrap_or(' ')))
    }
}

/// The TRS-80's code for a character on the screen, which shows APL80's
/// characters: ASCII as itself, the arrows on their codes, a graphics
/// block on its own. Anything else is shown as a blank.
fn code(c: char) -> u8 {
    let block = || apl_c80_screen::code(c).and_then(|n| u8::try_from(n).ok());
    match c {
        '↑' => 91,
        '↓' => 92,
        '←' => 93,
        '→' => 94,
        c => u8::try_from(c)
            .ok()
            .filter(u8::is_ascii)
            .or_else(block)
            .unwrap_or(b' '),
    }
}
