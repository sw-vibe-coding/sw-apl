//! The atomic vector, `⎕AV`: every character the system has, 256 of
//! them, in the order the IBM 5110 APL Reference Manual lists them
//! (Appendix B).

use apl_value::{Array, Data};

/// How many characters the atomic vector holds.
const LENGTH: usize = 256;

/// The positions the manual gives a character sw-apl has, as runs:
/// each string's characters stand one after another from the 1-origin
/// position beside it.
///
/// Position 63 the manual names "circle shoe" but pictures as circle
/// and slope, and nowhere else lists transpose, so sw-apl reads it as
/// `⍉`. Execute is at 79 and format at 78, which the manual names
/// base null and top null for the characters struck to form them.
const RUNS: [(usize, &str); 9] = [
    (15, "[]();/\\←→"),
    (
        26,
        "¨+-×÷*⌈⌊|∧∨<≤=≥>≠⍺∊⍳⍴⍵,!⌽⊥⊤○?~↑↓⊂⊃∩∪_⍉⌶∘⎕⍞⍟⍲⍱⍝⍋⍒⊖⌿⍀⌹⍕⍎&@#$",
    ),
    (87, "ABCDEFGHIJKLMNOPQRSTUVWXYZ∆"),
    (140, "⍙0123456789.¯ ':∇"),
    (161, "⍫"),
    (171, "¬\"%"),
    (184, "¢"),
    (187, "abcdefghijklmnopqrstuvwxyz{}"),
    (221, "`"),
];

/// The atomic vector.
///
/// A position whose character sw-apl cannot hold as one character, or
/// does not have, holds one from Unicode's private use area instead,
/// U+E000 plus its 0-origin position, so that every position is a
/// character of its own and `⎕AV⍳⎕AV` is every index. Those are the
/// positions the manual marks reserved or unused; the underscored
/// letters, which sw-apl writes as a letter and a combining low line;
/// the trace and stop characters; the control characters -- cursor
/// return, backspace, line feed -- whose effect on output is not
/// implemented; and the graphics kept for maintenance.
#[must_use]
pub fn atomic_vector() -> Array {
    let private = |i: usize| char::from_u32(0xE000 + u32::try_from(i).unwrap_or(0));
    let mut chars: Vec<char> = (0..LENGTH).filter_map(private).collect();
    for (from, run) in RUNS {
        for (i, c) in run.chars().enumerate() {
            chars[from - 1 + i] = c;
        }
    }
    Array {
        shape: vec![LENGTH],
        data: Data::Char(chars),
    }
}
