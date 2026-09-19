//! Forming a glyph by striking one character over another.

use apl_value::{OVERSTRIKE, UNDERBAR, UNDERSCORE, UNDERSCORED};

/// The glyph `base` and `over` form when struck together, in either
/// order, or `None` when they form none.
///
/// Text rather than a character: the underscored alphabet is a
/// letter and a combining low line, two code points printing in one
/// position, because Unicode has no precomposed underscored letter.
/// `data/glyphs.toml` records why that representation was chosen.
///
/// Either order, because on paper there is none: backspace only
/// positions the carriage, so both impressions land on one spot and
/// `⎕` over `'` looks exactly like `'` over `⎕`. Whether APL\360
/// accepted both is not stated in its manuals; it is unambiguous
/// here because no two pairs use the same two characters, which
/// `no_two_overstrikes_use_the_same_two_characters` checks.
#[must_use]
pub fn strike(base: char, over: char) -> Option<String> {
    let same = |(a, b): (char, char)| (a == base && b == over) || (a == over && b == base);
    let found = OVERSTRIKE.iter().find(|(_, a, b)| same((*a, *b)));
    match found {
        Some((glyph, _, _)) => Some(glyph.to_string()),
        None => underscored(base, over),
    }
}

/// A letter struck with the underbar: one of the twenty-six further
/// characters of the APL\360 set, written as the letter and a
/// combining low line.
///
/// A rule rather than twenty-six rows in `OVERSTRIKE`, because "a
/// letter and an underbar" is what the 2741 did. It adds no
/// ambiguity: the one table pair using the underbar strikes it over
/// `∆`, which is no letter.
fn underscored(base: char, over: char) -> Option<String> {
    let ((UNDERBAR, letter) | (letter, UNDERBAR)) = (base, over) else {
        return None;
    };
    UNDERSCORED
        .contains(letter)
        .then(|| format!("{letter}{UNDERSCORE}"))
}

/// A line with its overstrikes formed, or the error an illegitimate
/// one earns.
///
/// Every front end reads a line this way -- the CLI's reader, the
/// CLI's batch runner, and the browser terminal through the same
/// crate -- so the wording cannot differ between them.
///
/// # Errors
/// The CHARACTER ERROR text. The manual gives "Illegitimate
/// overstrike" as a cause of one, and sw-apl's CHARACTER ERROR
/// names what it objected to, so this names both characters.
pub fn read(line: &str) -> Result<String, String> {
    crate::machine::compose(line).map_err(|(base, over)| {
        format!(
            "CHARACTER ERROR: U+{:04X} struck over U+{:04X} forms no glyph",
            u32::from(over),
            u32::from(base)
        )
    })
}
