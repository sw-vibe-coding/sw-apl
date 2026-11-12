//! One character at a time: APL80's to the glyph it stands for, and
//! back.

/// The shifted letters, typed as the lower-case codes: each with the
/// glyph it stands for. `d`, the axis, and `u`, peek and poke, stand
/// for none.
pub(crate) const SHIFTED: [(char, char); 24] = [
    ('a', '⍲'),
    ('b', '⊥'),
    ('c', '⍝'),
    ('e', '∊'),
    ('f', '⍕'),
    ('g', '⍉'),
    ('h', '⌈'),
    ('i', '⍳'),
    ('j', '|'),
    ('k', '≤'),
    ('l', '⌊'),
    ('m', '⍞'),
    ('n', '~'),
    ('o', '○'),
    ('p', '⍴'),
    ('q', '⎕'),
    ('r', '⌽'),
    ('s', '⌶'),
    ('t', '⊤'),
    ('v', '∨'),
    ('w', '\\'),
    ('x', '×'),
    ('y', '⍱'),
    ('z', '≥'),
];

/// The other characters that stand for something else: the
/// substitutes, the minus signs, and the TRS-80's codes for its arrows,
/// which an ASCII keyboard types as `[ \ ] ^`.
pub(crate) const OTHER: [(char, char); 11] = [
    ('%', '÷'),
    ('@', '⍟'),
    ('$', '≠'),
    ('&', '∧'),
    ('"', '∘'),
    ('-', '¯'),
    ('_', '-'),
    ('[', '↑'),
    ('\\', '↓'),
    (']', '←'),
    ('^', '→'),
];

/// The glyph a character of APL80's stands for, as data: in a string
/// or a comment, where a character is only itself. `None` for one it
/// has not got.
#[must_use]
pub fn data(c: char) -> Option<char> {
    let table = SHIFTED.iter().chain(OTHER.iter());
    if let Some((_, g)) = table.into_iter().find(|(k, _)| *k == c) {
        return Some(*g);
    }
    let plain = c.is_ascii_graphic() || c == ' ' || matches!(c, '↑' | '↓' | '←' | '→');
    (plain && !matches!(c, '`' | '{' | '|' | '}' | '~')).then_some(c)
}

/// A name as APL80 writes it, as the glyphs hold it: `.` as `⍙`, `-`
/// as `∆`.
#[must_use]
pub fn name(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '.' => '⍙',
            '-' => '∆',
            c => c,
        })
        .collect()
}

/// Characters typed as data -- in reply to quote-quad -- each as the
/// character the glyphs hold for it.
#[must_use]
pub fn text(typed: &str) -> String {
    typed.chars().map(|c| data(c).unwrap_or(c)).collect()
}
