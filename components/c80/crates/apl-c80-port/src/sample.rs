//! A sample, ported: what it types, as APL80 types it.

use crate::port::typed;

/// `text`, a sample in glyphs, for (C): its modes line `(C)` -- added
/// to an (A) sample, which has none -- its notes as they are, and every
/// line it types in APL80's characters.
#[must_use]
pub fn sample(text: &str) -> String {
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| match line {
            l if l.starts_with("⍝!MODES") => "⍝!MODES (C)".to_string(),
            l if l.starts_with('⍝') => l.to_string(),
            l => typed(l),
        })
        .collect();
    if !text.starts_with("⍝!MODES") {
        lines.insert(0, "⍝!MODES (C)".to_string());
    }
    lines.join("\n") + "\n"
}
