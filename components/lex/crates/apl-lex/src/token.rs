//! Token vocabulary and the accepted glyph set.

use apl_value::{AplError, AplResult, Array, ErrorKind};

/// One lexical unit.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A numeric literal or strand, already an array (scalar for one
    /// number, vector otherwise).
    Numbers(Array),
    /// A quoted character literal, already an array (scalar for one
    /// character, vector otherwise).
    Chars(Array),
    /// A user name.
    Name(String),
    /// A primitive function or operator glyph.
    Prim(char),
    /// Left arrow: assignment.
    Assign,
    /// Right arrow: branch.
    Branch,
    /// Quad: evaluated input/output.
    Quad,
    /// Quote-quad: character input/output.
    QuoteQuad,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Semicolon,
    Colon,
    /// Del: open or close a function definition.
    Del,
    /// Del-tilde: close a definition locked.
    DelTilde,
    /// A whole line starting with a right parenthesis (text after it).
    SystemCommand(String),
}

/// A token with its character offset in the line.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

/// Every primitive glyph in `docs/glyphs.txt` that stands alone as a
/// function or operator token.
pub const PRIMITIVES: &str = "+-×÷⌈⌊*⍟|!○~∧∨⍲⍱<≤=≥>≠?⍳⍴,⌽⊖⍉↑↓/⌿\\⍀⊥⊤∊⍋⍒⌹⌶.∘";

impl TokenKind {
    /// True when the token can end an operand: a glyph immediately
    /// to its right has an array on its left and is dyadic.
    #[must_use]
    pub fn ends_operand(&self) -> bool {
        matches!(
            self,
            TokenKind::Numbers(_)
                | TokenKind::Chars(_)
                | TokenKind::Name(_)
                | TokenKind::RParen
                | TokenKind::RBracket
                | TokenKind::Quad
                | TokenKind::QuoteQuad
        )
    }

    /// The single-character punctuation and sentinel tokens.
    #[must_use]
    pub fn punctuation(c: char) -> Option<TokenKind> {
        Some(match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            '←' => TokenKind::Assign,
            '→' => TokenKind::Branch,
            '⎕' => TokenKind::Quad,
            '⍞' => TokenKind::QuoteQuad,
            '∇' => TokenKind::Del,
            '⍫' => TokenKind::DelTilde,
            _ => return None,
        })
    }
}

/// SYNTAX ERROR at the first unmatched opener or closer.
pub fn check_balance(tokens: &[Token]) -> AplResult<()> {
    let mut stack: Vec<&Token> = Vec::new();
    for tok in tokens {
        let want = match tok.kind {
            TokenKind::LParen | TokenKind::LBracket => {
                stack.push(tok);
                continue;
            }
            TokenKind::RParen => TokenKind::LParen,
            TokenKind::RBracket => TokenKind::LBracket,
            _ => continue,
        };
        if stack.pop().is_none_or(|open| open.kind != want) {
            return Err(AplError::new(ErrorKind::Syntax).at(tok.pos));
        }
    }
    match stack.last() {
        Some(open) => Err(AplError::new(ErrorKind::Syntax).at(open.pos)),
        None => Ok(()),
    }
}

/// Letters allowed in names: ASCII letters plus delta and delta-underbar.
#[must_use]
pub fn is_name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '∆' || c == '⍙'
}
