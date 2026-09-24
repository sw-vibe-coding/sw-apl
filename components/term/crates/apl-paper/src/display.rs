//! Underscored capitals, as a font will actually draw them.

/// Text as the paper shows it: an underscored capital as the circled
/// one, and no control characters. The wire keeps the capital and its
/// combining low line, which is the interpreter's own representation;
/// this is only what a reader sees.
#[must_use]
pub fn display(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_uppercase() && chars.peek() == Some(&'\u{332}') {
            chars.next();
            out.push(char::from_u32(0x24b6 + c as u32 - 'A' as u32).unwrap_or(c));
        } else if ('\u{e000}'..='\u{f8ff}').contains(&c) {
            // A Private Use Area character: (B)'s atomic vector holds
            // U+E000 plus its index where the 5110 had a character
            // sw-apl cannot hold as one. No font draws those.
            out.push('·');
        } else if !c.is_control() || c == '\n' {
            out.push(c);
        }
    }
    out
}
