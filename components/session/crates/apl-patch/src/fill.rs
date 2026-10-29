//! What the reader types into a statement after its marks.

use apl_value::{AplError, AplResult, ErrorKind};

use crate::marks::cells;

/// `typed` laid on `edited` from position `from`, as on paper: a
/// blank keeps what is there, a character fills a blank, and a
/// character over a character strikes it. Typing runs on past the end
/// of the line. `also` is the mode's overstrikes beyond APL\360's.
///
/// # Errors
/// CHARACTER ERROR for a character struck over one it forms no glyph
/// with -- the manual's "illegitimate overstrike".
pub fn fill(
    edited: &str,
    from: usize,
    typed: &str,
    also: &[(char, char, char)],
) -> AplResult<String> {
    let mut line = cells(edited);
    for (i, cell) in cells(typed).into_iter().enumerate() {
        let at = from + i;
        if at < line.len() {
            line[at] = over(&line[at], cell, also)?;
        } else {
            line.resize(at, " ".to_string());
            line.push(cell);
        }
    }
    Ok(line.concat())
}

/// One position: `typed` over what `was` there.
fn over(was: &str, typed: String, also: &[(char, char, char)]) -> AplResult<String> {
    if typed == " " || typed == was {
        return Ok(was.to_string());
    }
    if was == " " {
        return Ok(typed);
    }
    let one = |s: &str| {
        let mut chars = s.chars();
        chars.next().filter(|_| chars.next().is_none())
    };
    let struck = match (one(was), one(&typed)) {
        (Some(a), Some(b)) => apl_strike::strike(a, b, also),
        _ => None,
    };
    let bad = typed.chars().next().unwrap_or(' ');
    struck.ok_or_else(|| AplError::new(ErrorKind::Character(bad)))
}
