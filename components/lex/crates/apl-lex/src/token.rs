//! Token vocabulary and the accepted glyph set.

use apl_value::{AplError, AplResult, Array, ErrorKind, UNDERSCORE};

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

pub use apl_value::PRIMITIVES;

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

/// True when `c` may stand in a name at this point: `prev` is the
/// character before it, or `None` where the name would start.
///
/// Letters are the ASCII ones plus delta and delta-underbar, and
/// digits follow but do not start. The combining low line makes the
/// letter before it one of the underscored alphabet -- A̲ through Z̲,
/// characters of the APL\360 set in their own right -- so it
/// continues a name, but only directly on a letter, which is the
/// only thing it can underscore. Alone, on a digit, or on another
/// low line it is a CHARACTER ERROR.
#[must_use]
pub fn is_name_char(c: char, prev: Option<char>) -> bool {
    let letter = c.is_ascii_alphabetic() || c == '\u{2206}' || c == '\u{2359}';
    let follows = |ok: fn(char) -> bool| prev.is_some_and(ok);
    letter
        || (c.is_ascii_digit() && prev.is_some())
        || (c == UNDERSCORE && follows(|p| p.is_ascii_alphabetic()))
}
