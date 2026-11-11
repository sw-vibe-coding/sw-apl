//! A clear workspace, whose settings differ by mode.

use apl_modes::Mode;
use apl_workspace::{Print, Saved};

/// A clear workspace in `mode`. (B)'s prints five digits in a line of
/// 64, which follow from the 5110's screen of 64 characters, and a
/// whole number of up to ten digits in full (IBM 5110 APL Reference
/// Manual, Chapter 5); (A)'s prints ten digits in a line of 120, as it
/// always has. (C)'s prints six digits, APL80's `)PS`, in the 64
/// columns of the TRS-80's screen; the manual does not say how a long
/// whole number prints, so it prints as (A)'s does. All start with
/// origin 1 and link 16807.
#[must_use]
pub fn clear(mode: Mode) -> Saved {
    let print = match mode {
        Mode::A => Print::default(),
        Mode::B => Print {
            digits: 5,
            width: 64,
            whole: 10,
        },
        Mode::C => Print {
            digits: 6,
            width: 64,
            whole: 0,
        },
    };
    Saved {
        print,
        ..Saved::default()
    }
}
