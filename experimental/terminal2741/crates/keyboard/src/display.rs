/// Circled capitals are a presentation only; the wire keeps combining low lines.
pub fn display(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_uppercase() && chars.peek() == Some(&'\u{332}') {
            chars.next();
            out.push(char::from_u32(0x24b6 + c as u32 - 'A' as u32).unwrap());
        } else if !c.is_control() || c == '\n' {
            out.push(c);
        }
    }
    out
}
