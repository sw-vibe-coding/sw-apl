//! Numeric literals: digits, optional point, optional `E` exponent,
//! high minus (U+00AF) as the negative sign on mantissa and exponent.

use apl_value::{AplError, AplResult, ErrorKind, Number};

/// Scan a literal starting at `start`. Returns the number and the
/// index just past it.
pub fn lex_number(chars: &[char], start: usize) -> AplResult<(Number, usize)> {
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
        Ok(x) if !bad_tail && x.is_finite() => Ok((Number::from_f64(x), end)),
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
