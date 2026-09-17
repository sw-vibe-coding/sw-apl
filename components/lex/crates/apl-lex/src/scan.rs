//! The scanner: one pass over the characters of a line.

use apl_value::{AplError, AplResult, ErrorKind};

use crate::literal::{join_strands, lex_number, lex_string};
use crate::token::{PRIMITIVES, Token, TokenKind, check_balance, is_name_start};

/// Tokenize one line. A line whose first non-blank character is a
/// right parenthesis is one `SystemCommand` token; a lamp ends the
/// line; adjacent numbers become one strand token.
///
/// # Errors
/// CHARACTER ERROR for any character outside the accepted set;
/// SYNTAX ERROR for a malformed number, an unterminated quote, or
/// unbalanced brackets. The caret is set.
pub fn tokenize(line: &str) -> AplResult<Vec<Token>> {
    let chars: Vec<char> = line.chars().collect();
    if let Some(command) = system_command(&chars) {
        return Ok(vec![command]);
    }
    let (mut tokens, mut pending) = (Vec::new(), Vec::new());
    let mut i = 0;
    while i < chars.len() && chars[i] != '⍝' {
        if chars[i] == ' ' {
            i += 1;
            continue;
        }
        if chars[i].is_ascii_digit() || chars[i] == '¯' {
            i = lex_number(&chars, i, &mut pending)?;
            continue;
        }
        join_strands(&mut pending, &mut tokens);
        let (kind, next) = classify(&chars, i)?;
        tokens.push(Token { kind, pos: i });
        i = next;
    }
    join_strands(&mut pending, &mut tokens);
    check_balance(&tokens)?;
    Ok(tokens)
}

/// Decide what non-numeric token starts at `i` and where it ends.
fn classify(chars: &[char], i: usize) -> AplResult<(TokenKind, usize)> {
    let c = chars[i];
    if let Some(kind) = TokenKind::punctuation(c) {
        return Ok((kind, i + 1));
    }
    Ok(match c {
        '\'' => lex_string(chars, i)?,
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

/// A line whose first non-blank character is a right parenthesis is
/// one system command token holding the text after it.
fn system_command(chars: &[char]) -> Option<Token> {
    let pos = chars
        .iter()
        .position(|&c| c != ' ')
        .filter(|&p| chars[p] == ')')?;
    let text: String = chars[pos + 1..].iter().collect();
    let kind = TokenKind::SystemCommand(text.trim().to_string());
    Some(Token { kind, pos })
}
