//! What is shown, translated back to APL80's characters.

use crate::tables::{OTHER, SHIFTED};

/// What is shown, in APL80's characters: each glyph as the character
/// APL80 has for it, a name's `⍙` and `∆` as `.` and `-`, a grade as
/// its arrow, a code the screen obeys as the terminal code for it.
#[must_use]
pub fn shown(text: &str) -> String {
    let back = |g: char| {
        let table = SHIFTED.iter().chain(OTHER[..7].iter());
        table.into_iter().find(|(_, k)| *k == g).map(|(c, _)| *c)
    };
    let mut shown = String::new();
    for g in text.chars() {
        // A code the screen obeys is sent as the terminal code for it.
        if let Some(sent) = apl_c80_screen::sent(g) {
            shown.push_str(&sent);
            continue;
        }
        shown.push(match g {
            '⍋' => '↑',
            '⍒' => '↓',
            '∆' => '-',
            '⍙' => '.',
            g => back(g).unwrap_or(g),
        });
    }
    shown
}

/// What a command shows, which is prose and names: each name's `⍙`
/// and `∆` as the `.` and `-` it was typed with, and nothing else
/// changed.
#[must_use]
pub fn named(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '⍙' => '.',
            '∆' => '-',
            c => c,
        })
        .collect()
}
