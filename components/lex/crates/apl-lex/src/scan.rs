//! The scanner: one pass over the characters of a line.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};

use crate::number::lex_number;
use crate::token::{PRIMITIVES, Token, TokenKind, is_name_start};

/// Tokenize one line. A line whose first non-blank character is a
/// right parenthesis is one `SystemCommand` token; a lamp ends the
/// line.
///
/// # Errors
/// CHARACTER ERROR for any character outside the accepted set;
/// SYNTAX ERROR for a malformed number or an unterminated quote.
/// The caret is set.
pub fn tokenize(line: &str) -> AplResult<Vec<Token>> {
    let chars: Vec<char> = line.chars().collect();
    let first = chars.iter().position(|&c| c != ' ');
    if let Some(pos) = first.filter(|&p| chars[p] == ')') {
        let text: String = chars[pos + 1..].iter().collect();
        let kind = TokenKind::SystemCommand(text.trim().to_string());
        return Ok(vec![Token { kind, pos }]);
    }
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
    if let Some(kind) = TokenKind::punctuation(c) {
        return Ok((kind, i + 1));
    }
    Ok(match c {
        '\'' => lex_string(chars, i)?,
        _ if c.is_ascii_digit() || c == '¯' => {
            let (n, end) = lex_number(chars, i)?;
            (TokenKind::Number(n), end)
        }
        _ if is_name_start(c) => lex_name(chars, i),
        _ if PRIMITIVES.contains(c) => (TokenKind::Prim(c), i + 1),
        _ => return Err(AplError::new(ErrorKind::Character(c)).at(i)),
    })
}

/// A name: a name-start letter followed by letters and digits.
fn lex_name(chars: &[char], start: usize) -> (TokenKind, usize) {
    let mut end = start + 1;
    while end < chars.len() && (is_name_start(chars[end]) || chars[end].is_ascii_digit()) {
        end += 1;
    }
    (TokenKind::Name(chars[start..end].iter().collect()), end)
}

/// A quoted literal; two quotes inside stand for one. Any character
/// is data inside quotes. One character makes a scalar.
fn lex_string(chars: &[char], start: usize) -> AplResult<(TokenKind, usize)> {
    let mut text = Vec::new();
    let mut i = start + 1;
    loop {
        match chars.get(i) {
            Some('\'') if chars.get(i + 1) == Some(&'\'') => {
                text.push('\'');
                i += 2;
            }
            Some('\'') => break,
            Some(&c) => {
                text.push(c);
                i += 1;
            }
            None => return Err(AplError::new(ErrorKind::Syntax).at(start)),
        }
    }
    let shape = if text.len() == 1 {
        vec![]
    } else {
        vec![text.len()]
    };
    let array = Array::new(shape, Data::Char(text)).map_err(|e| e.at(start))?;
    Ok((TokenKind::Chars(array), i + 1))
}
