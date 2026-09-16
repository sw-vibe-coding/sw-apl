//! Token vocabulary and the accepted glyph set.

use apl_value::Number;

/// One lexical unit.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A numeric literal (strands are joined by the parser).
    Number(Number),
    /// A user name.
    Name(String),
    /// A primitive function or operator glyph.
    Prim(char),
    /// Left arrow: assignment.
    Assign,
    LParen,
    RParen,
}

/// A token with its character offset in the line.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

/// Every primitive glyph in `docs/glyphs.txt` that stands alone as a
/// function or operator token.
pub const PRIMITIVES: &str = "+-×÷⌈⌊*⍟|!○~∧∨⍲⍱<≤=≥>≠?⍳⍴,⌽⊖⍉↑↓/⌿\\⍀⊥⊤∊⍋⍒⌹⍎⍕.∘";

/// Letters allowed in names: ASCII letters plus delta and delta-underbar.
#[must_use]
pub fn is_name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '∆' || c == '⍙'
}
