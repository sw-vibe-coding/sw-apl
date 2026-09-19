//! How wide a line of glyphs prints.

use crate::UNDERSCORE;

/// The columns `text` occupies on paper.
///
/// Every glyph sw-apl knows prints in one position, so this is a
/// count of characters -- except the combining low line of an
/// underscored letter, which lands on the letter before it and takes
/// no column of its own. Counting code points would take every line
/// holding such a name out by one.
#[must_use]
pub fn columns(text: &str) -> usize {
    text.chars().filter(|&c| c != UNDERSCORE).count()
}

/// `text` padded with spaces to `width` columns, left-justified.
///
/// `format!("{text:width$}")` cannot do this: it pads to a count of
/// characters, so a name carrying a low line comes out a space short.
#[must_use]
pub fn pad(text: &str, width: usize) -> String {
    let fill = width.saturating_sub(columns(text));
    format!("{text}{:fill$}", "")
}
