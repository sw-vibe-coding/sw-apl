//! A card's statement spelled out in glyphs.

use crate::table::glyph;

/// A statement in glyphs, and the card column each glyph came from,
/// so that an error can be pointed at on the card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spelled {
    /// The statement as the interpreter reads it.
    pub text: String,
    /// For each character of `text`, its column on the card, from 1.
    pub columns: Vec<usize>,
}

/// `statement` in glyphs: each `$` and two letters its glyph, `$$` a
/// dollar, the 029's `¬` a tilde, anything else itself -- inside
/// quotes too, so that a character string can hold a glyph.
///
/// # Errors
/// The column of a `$` that begins no spelling.
pub fn spell(statement: &str) -> Result<Spelled, usize> {
    let chars: Vec<char> = statement.chars().collect();
    let mut spelled = Spelled {
        text: String::new(),
        columns: Vec::new(),
    };
    let mut at = 0;
    while at < chars.len() {
        let (c, width) = one(&chars[at..]).ok_or(at + 1)?;
        spelled.text.push(c);
        spelled.columns.push(at + 1);
        at += width;
    }
    Ok(spelled)
}

/// The glyph at the start of `rest` and how many columns it took.
fn one(rest: &[char]) -> Option<(char, usize)> {
    match rest {
        ['$', '$', ..] => Some(('$', 2)),
        ['$', a, b, ..] => glyph(&format!("{a}{b}")).map(|g| (g, 3)),
        ['¬', ..] => Some(('~', 1)),
        [c, ..] if *c != '$' => Some((*c, 1)),
        _ => None,
    }
}
