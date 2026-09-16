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
    /// Bare quad: evaluated input/output.
    Quad,
    /// Quad followed by letters: a system variable or function.
    SysName(String),
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

impl TokenKind {
    /// True when the token can end an operand: a glyph immediately
    /// to its right has an array on its left and is dyadic.
    #[must_use]
    pub fn ends_operand(&self) -> bool {
        matches!(
            self,
            TokenKind::Number(_)
                | TokenKind::Name(_)
                | TokenKind::RParen
                | TokenKind::Quad
                | TokenKind::SysName(_)
        )
    }
}

/// Letters allowed in names: ASCII letters plus delta and delta-underbar.
#[must_use]
pub fn is_name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '∆' || c == '⍙'
}
