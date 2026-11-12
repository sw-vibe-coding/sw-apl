//! APL80's character codes: the TRS-80's, with the characters APL80
//! puts on them.

/// Each character APL80 shows beyond the ones it shares with ASCII,
/// with its code: the substitutes, the TRS-80's arrows, and the
/// shifted letters, which are the lower-case codes. The manual's
/// Character Set, as `mode-c.md` reads it.
const APL80: [(char, u8); 43] = [
    ('∘', 34),
    ('≠', 36),
    ('÷', 37),
    ('∧', 38),
    ('¯', 45),
    ('⍟', 64),
    ('↑', 91),
    ('↓', 92),
    ('←', 93),
    ('→', 94),
    ('-', 95),
    ('⍲', 97),
    ('⊥', 98),
    ('⍝', 99),
    ('∊', 101),
    ('⍕', 102),
    ('⍉', 103),
    ('⌈', 104),
    ('⍳', 105),
    ('|', 106),
    ('≤', 107),
    ('⌊', 108),
    ('⍞', 109),
    ('~', 110),
    ('○', 111),
    ('⍴', 112),
    ('⎕', 113),
    ('⌽', 114),
    ('⌶', 115),
    ('⊤', 116),
    ('∨', 118),
    ('\\', 119),
    ('×', 120),
    ('⍱', 121),
    ('≥', 122),
    ('!', 33),
    ('\'', 39),
    ('(', 40),
    (')', 41),
    ('*', 42),
    ('+', 43),
    (',', 44),
    ('.', 46),
];

/// The ASCII characters APL80 keeps as they are, beyond the ones in
/// the table: the blank, `#`, the digits, `/ : ; < = > ?`, and the
/// capital letters.
fn plain(c: char) -> bool {
    matches!(c, ' ' | '#' | '/' | '0'..='9' | ':'..='?' | 'A'..='Z')
}

/// The code APL80 gives `c`, if it has the character at all.
#[must_use]
pub fn code(c: char) -> Option<u8> {
    if plain(c) {
        return u8::try_from(c).ok();
    }
    APL80.iter().find(|(g, _)| *g == c).map(|(_, n)| *n)
}

/// The character APL80 shows for code `n`, if it is one it prints.
#[must_use]
pub fn glyph(n: u8) -> Option<char> {
    let c = char::from(n);
    if plain(c) {
        return Some(c);
    }
    APL80.iter().find(|(_, k)| *k == n).map(|(g, _)| *g)
}
