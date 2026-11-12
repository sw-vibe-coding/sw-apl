//! The obscured form: what a workspace holding a locked function is
//! written as, and how it is read back.

use apl_modes::{modes, retag};

use crate::write::DIRECTIVE;

/// What an obscured workspace begins with, before anything
/// scrambled. `)LOAD` tells the two forms apart by the first line;
/// the other two are there for the person who runs the file as a
/// program, so it says what it is and signs off instead of printing
/// a screen of CHARACTER ERRORs.
pub const PREAMBLE: [&str; 3] = [
    "⍝!OBSCURED sw-apl workspace. Rot-13, not encryption: docs/workspaces.md.",
    "'OBSCURED WORKSPACE. )LOAD IT -- IT CANNOT BE RUN AS A PROGRAM.'",
    ")OFF",
];

/// A workspace file as APL, whichever way it was written. A plain
/// file is itself; an obscured one -- which is what a workspace
/// holding a locked function is written as -- is what lies under the
/// preamble, unscrambled. The two are told apart by the first line.
///
/// Everything that reads a workspace goes through here, so nothing
/// above it needs to know the two forms exist.
#[must_use]
pub fn plain(text: &str) -> String {
    if !text.starts_with(PREAMBLE[0]) {
        return text.to_string();
    }
    // The modes line follows the preamble in the clear, and goes back
    // where a plain file has it.
    let mut body = text.lines().skip(PREAMBLE.len()).peekable();
    let clear = body.next_if(|l| l.starts_with(&format!("{DIRECTIVE}MODES")));
    let body: Vec<&str> = body.collect();
    let revealed = rot13(&(body.join("\n") + "\n"));
    match clear {
        Some(_) => retag(&revealed, modes(text)),
        None => revealed,
    }
}

/// Letters moved thirteen places, everything else left alone.
///
/// This is obscuring, not encryption, and the documentation must not
/// suggest otherwise: anyone who means to read a rot-13 file can.
/// What it stops is reading a locked function by accident or by
/// curiosity, which is what APL\360's binary workspaces stopped.
///
/// Rot-13 is its own inverse, so this one function serves both
/// directions: writing scrambles and `plain` unscrambles with the
/// same call.
#[must_use]
pub fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            let first = match c {
                'A'..='Z' => b'A',
                'a'..='z' => b'a',
                _ => return c,
            };
            // Both arms above are ASCII, so this cannot overflow.
            char::from(first + ((c as u8) - first + 13) % 26)
        })
        .collect()
}
