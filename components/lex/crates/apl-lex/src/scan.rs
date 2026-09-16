//! The scanner: one pass over the characters of a line.

use apl_value::{AplError, AplResult, ErrorKind};

use crate::number::lex_number;
use crate::token::{PRIMITIVES, Token, TokenKind, is_name_start};

/// Tokenize one statement line. A lamp ends the line.
///
/// # Errors
/// CHARACTER ERROR for any character outside the accepted set;
/// SYNTAX ERROR for a malformed number. The caret is set.
pub fn tokenize(line: &str) -> AplResult<Vec<Token>> {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let (kind, next) = match c {
            ' ' => {
                i += 1;
                continue;
            }
            '⍝' => break,
            _ => classify(&chars, i, c)?,
        };
        tokens.push(Token { kind, pos: i });
        i = next;
    }
    Ok(tokens)
}

/// Decide what token starts at `i` and where it ends.
fn classify(chars: &[char], i: usize, c: char) -> AplResult<(TokenKind, usize)> {
    Ok(match c {
        '(' => (TokenKind::LParen, i + 1),
        ')' => (TokenKind::RParen, i + 1),
        '←' => (TokenKind::Assign, i + 1),
        '⎕' => lex_sysname(chars, i),
        _ if c.is_ascii_digit() || c == '¯' => {
            let (n, end) = lex_number(chars, i)?;
            (TokenKind::Number(n), end)
        }
        _ if is_name_start(c) => lex_name(chars, i),
        _ if PRIMITIVES.contains(c) => (TokenKind::Prim(c), i + 1),
        _ => return Err(AplError::new(ErrorKind::Character(c)).at(i)),
    })
}

/// A quad name (`⎕IO`) when letters follow the quad, else the bare quad.
fn lex_sysname(chars: &[char], start: usize) -> (TokenKind, usize) {
    let mut end = start + 1;
    while end < chars.len() && chars[end].is_ascii_alphabetic() {
        end += 1;
    }
    if end == start + 1 {
        return (TokenKind::Quad, end);
    }
    (
        TokenKind::SysName(chars[start + 1..end].iter().collect()),
        end,
    )
}

/// A name: a name-start letter followed by letters and digits.
fn lex_name(chars: &[char], start: usize) -> (TokenKind, usize) {
    let mut end = start + 1;
    while end < chars.len() && (is_name_start(chars[end]) || chars[end].is_ascii_digit()) {
        end += 1;
    }
    (TokenKind::Name(chars[start..end].iter().collect()), end)
}
