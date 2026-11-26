//! What an IBM 029 punches: the characters of the card code table in
//! ibm-1130-rs (`punch-card-core`, `hollerith.rs`), less the cent sign
//! it leaves out.

use crate::card::Card;

/// The 029's characters besides the letters and digits: blank and the
/// specials. No `[`, `]`, `\`, `^` or lower case.
pub const PUNCHED: &str = " &-/.<(+|!$*);¬,%_>?:#@'=\"";

/// Whether an 029 punches `c`.
#[must_use]
pub fn punchable(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit() || PUNCHED.contains(c)
}

impl Card {
    /// The first column, counted from 1, holding what an 029 cannot
    /// punch: a card from a file may, and is not read past it.
    #[must_use]
    pub fn unpunchable(&self) -> Option<usize> {
        let at = self.columns.chars().position(|c| !punchable(c));
        at.map(|at| at + 1)
    }
}
