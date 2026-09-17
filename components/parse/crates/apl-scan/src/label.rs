//! Labels: a name and a colon at the start of a body line. A label is
//! a local constant holding the number of the line it names.

use std::borrow::Cow;

use apl_lex::{TokenKind, tokenize};

/// Every label in a body, with the line number it names. Line numbers
/// start at 1 and never depend on the index origin.
#[must_use]
pub fn labels(body: &[String]) -> Vec<(String, usize)> {
    body.iter()
        .enumerate()
        .filter_map(|(i, line)| Some((label_end(line)?.0, i + 1)))
        .collect()
}

/// A body line with any leading `LABEL:` replaced by blanks, so every
/// character after it keeps the position it had and carets still
/// point true.
#[must_use]
pub fn without_label(line: &str) -> Cow<'_, str> {
    let Some((_, end)) = label_end(line) else {
        return Cow::Borrowed(line);
    };
    let blanked = line
        .chars()
        .enumerate()
        .map(|(i, c)| if i <= end { ' ' } else { c })
        .collect();
    Cow::Owned(blanked)
}

/// The label a line starts with and the position of its colon.
fn label_end(line: &str) -> Option<(String, usize)> {
    let tokens = tokenize(line).ok()?;
    let name = match tokens.first()?.kind {
        TokenKind::Name(ref n) => n.clone(),
        _ => return None,
    };
    let colon = tokens.get(1)?;
    (colon.kind == TokenKind::Colon).then_some((name, colon.pos))
}
