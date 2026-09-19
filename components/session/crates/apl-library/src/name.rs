//! What a workspace may be called.
//!
//! A workspace name becomes a filename, which is why this exists at
//! all: before it did, `)SAVE A/B` made a directory called `A` and
//! `)SAVE ../../X` wrote outside the library altogether. A name that
//! has to be safe to join to a path is a name that has to be
//! checked, and checking it against APL's own rule is both the
//! historically right answer and the one that closes every path
//! question at once -- no slash, no dot, no colon, nothing to
//! traverse with.
//!
//! The rule is the lexer's, so a workspace is named the way a
//! variable is. That includes lower case, which sw-apl accepts
//! everywhere and APL\360 had not got.

/// Whether `name` is a name: a letter, delta or delta-underbar,
/// then letters, deltas and digits.
#[must_use]
pub fn valid(name: &str) -> bool {
    let letter = |c: char| c.is_ascii_alphabetic() || c == '\u{2206}' || c == '\u{2359}';
    let mut chars = name.chars();
    chars.next().is_some_and(letter) && chars.all(|c| letter(c) || c.is_ascii_digit())
}
