//! Literals: numbers (digits, optional point, optional `E` exponent,
//! high minus as the negative sign on mantissa and exponent), strands,
//! and quoted character literals.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::token::{Token, TokenKind};

/// Join runs of adjacent numbers into one `Numbers` token (a scalar
/// for a single number, a vector for a strand), positioned at the
/// first number.
pub fn join_strands(pending: &mut Vec<(Number, usize)>, tokens: &mut Vec<Token>) {
    if pending.is_empty() {
        return;
    }
    let pos = pending[0].1;
    let nums: Vec<Number> = pending.drain(..).map(|(n, _)| n).collect();
    let array = if nums.len() == 1 {
        Array::scalar(nums[0])
    } else {
        Array::vector(nums)
    };
    tokens.push(Token {
        kind: TokenKind::Numbers(array),
        pos,
    });
}

/// Scan a numeric literal starting at `start`, push it (with its
/// position) onto the pending strand, and return the index past it.
pub fn lex_number(
    chars: &[char],
    start: usize,
    pending: &mut Vec<(Number, usize)>,
) -> AplResult<usize> {
    let mut end = start;
    let mut text = String::new();
    push_signed_digits(chars, &mut end, &mut text, true);
    if chars.get(end) == Some(&'E') {
        end += 1;
        text.push('e');
        push_signed_digits(chars, &mut end, &mut text, false);
    }
    let bad_tail = matches!(chars.get(end), Some(c) if c.is_ascii_alphanumeric() || *c == '.');
    match text.parse::<f64>() {
        Ok(x) if !bad_tail && x.is_finite() => {
            pending.push((Number::from_f64(x), start));
            Ok(end)
        }
        _ => Err(AplError::new(ErrorKind::Syntax).at(start)),
    }
}

/// Append `[¯]digits[.digits]` (point allowed only when `with_point`)
/// to `text`, advancing `end`.
fn push_signed_digits(chars: &[char], end: &mut usize, text: &mut String, with_point: bool) {
    if chars.get(*end) == Some(&'¯') {
        text.push('-');
        *end += 1;
    }
    let mut seen_point = false;
    while let Some(&c) = chars.get(*end) {
        if c.is_ascii_digit() {
            text.push(c);
        } else if c == '.' && with_point && !seen_point {
            seen_point = true;
            text.push(c);
        } else {
            break;
        }
        *end += 1;
    }
}

/// A quoted literal; two quotes inside stand for one. Any character
/// is data inside quotes. One character makes a scalar.
pub fn lex_string(chars: &[char], start: usize) -> AplResult<(TokenKind, usize)> {
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
