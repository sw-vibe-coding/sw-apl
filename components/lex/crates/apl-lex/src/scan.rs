//! The scanner: one pass over the characters of a line.

use apl_value::{AplError, AplResult, ErrorKind, RESERVED};

use crate::literal::{join_strands, lex_number, lex_string};
use crate::token::{PRIMITIVES, Token, TokenKind, check_balance, is_name_char};

/// Tokenize one line. A line whose first non-blank character is a
/// right parenthesis is one `SystemCommand` token; a lamp ends the
/// line; adjacent numbers become one strand token.
///
/// `also` is the later glyphs this line may use as primitives beyond
/// APL\360's: the mode's, which is empty in (A) and `MODE_B` in (B).
/// Quad among them means a quad before a letter begins a system name.
///
/// # Errors
/// CHARACTER ERROR for any character outside the accepted set;
/// SYNTAX ERROR for a reserved character outside quotes, a malformed number, an unterminated quote, or
/// unbalanced brackets. The caret is set.
pub fn tokenize(line: &str, also: &str) -> AplResult<Vec<Token>> {
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
        let (kind, next) = classify(&chars, i, also)?;
        tokens.push(Token { kind, pos: i });
        i = next;
    }
    join_strands(&mut pending, &mut tokens);
    check_balance(&tokens)?;
    Ok(tokens)
}

/// Decide what non-numeric token starts at `i` and where it ends. A
/// quad directly before a letter begins a system name, `⎕IO`, when
/// the mode's glyphs include quad; otherwise it is quad.
fn classify(chars: &[char], i: usize, also: &str) -> AplResult<(TokenKind, usize)> {
    let c = chars[i];
    let named = chars.get(i + 1).is_some_and(char::is_ascii_alphabetic);
    if c == '⎕' && named && also.contains(c) {
        return Ok(lex_name(chars, i));
    }
    if let Some(kind) = TokenKind::punctuation(c) {
        return Ok((kind, i + 1));
    }
    Ok(match c {
        '\'' => lex_string(chars, i)?,
        _ if is_name_char(c, None) => lex_name(chars, i),
        _ if PRIMITIVES.contains(c) || also.contains(c) => (TokenKind::Prim(c), i + 1),
        // A character of the set with no meaning: not a character
        // error, which is for one outside the set.
        _ if RESERVED.iter().any(|(r, _)| *r == c) => {
            return Err(AplError::new(ErrorKind::Syntax).at(i));
        }
        _ => return Err(AplError::new(ErrorKind::Character(c)).at(i)),
    })
}

/// A name: a name-start letter, or a system name's quad, followed by
/// letters, digits, and the low lines that underscore them.
fn lex_name(chars: &[char], start: usize) -> (TokenKind, usize) {
    let mut end = start + 1;
    while end < chars.len() && is_name_char(chars[end], Some(chars[end - 1])) {
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
