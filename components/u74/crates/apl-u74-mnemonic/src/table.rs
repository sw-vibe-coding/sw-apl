//! The table: the owner's notes' spellings, NAND and NOR on the right
//! glyphs, and ours for what APL\360 has and the notes do not give.

/// Each spelling and the glyph it stands for.
pub const MNEMONICS: [(&str, char); 45] = [
    // The notes'.
    ("AL", '←'),
    ("RA", '→'),
    ("DL", '∇'),
    ("IO", '⍳'),
    ("RH", '⍴'),
    ("TA", '↑'),
    ("DR", '↓'),
    ("RV", '⌽'),
    ("TR", '⍉'),
    ("MU", '×'),
    ("DI", '÷'),
    ("EP", '∊'),
    ("DM", '⌹'),
    ("CE", '⌈'),
    ("FL", '⌊'),
    ("AB", '|'),
    ("LO", '⍟'),
    ("CI", '○'),
    ("FA", '!'),
    ("RO", '?'),
    ("LE", '≤'),
    ("GE", '≥'),
    ("NE", '≠'),
    ("AN", '∧'),
    ("OR", '∨'),
    ("NT", '~'),
    ("NG", '¯'),
    ("QD", '⎕'),
    ("QQ", '⍞'),
    ("CM", '⍝'),
    ("SO", '∘'),
    ("IB", '⌶'),
    // The notes' names, corrected: they put NAND on ⊼ and NOR on ⍲.
    ("NA", '⍲'),
    ("NO", '⍱'),
    // Ours.
    ("LB", '['),
    ("RB", ']'),
    ("BS", '\\'),
    ("SB", '⌿'),
    ("BB", '⍀'),
    ("RF", '⊖'),
    ("DC", '⊥'),
    ("EC", '⊤'),
    ("GU", '⍋'),
    ("GD", '⍒'),
    ("DT", '∆'),
];

/// The glyph `code` spells, as `RH` spells `⍴`.
#[must_use]
pub fn glyph(code: &str) -> Option<char> {
    MNEMONICS.iter().find(|(c, _)| *c == code).map(|(_, g)| *g)
}

/// How a card spells `glyph`.
#[must_use]
pub fn mnemonic(glyph: char) -> Option<&'static str> {
    MNEMONICS.iter().find(|(_, g)| *g == glyph).map(|(c, _)| *c)
}
