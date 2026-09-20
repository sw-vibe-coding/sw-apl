//! What a keystroke means, decided before any browser is involved.
//!
//! A browser names its keys and a terminal sends bytes, so the two
//! clients cannot share the code that reads a keystroke -- but they
//! can share what the keystrokes mean, and this is that. It is
//! ordinary Rust and is tested natively.

use apl_keyboard::Move;

/// What one keystroke asks the keyboard to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    /// Type this key, translated by the keymap.
    Type(char),
    /// Put this character in as it is, translating nothing.
    Literal(char),
    /// Hold the cell before the carriage for an overstrike.
    Overstrike,
    /// Delete backwards, or cancel a waiting strike.
    Backspace,
    /// Delete forwards.
    Delete,
    /// Move the carriage.
    Carriage(Move),
    /// Abandon the line.
    Clear,
    /// Send it.
    Submit,
    /// A key with no meaning here: a function key, a modifier on its
    /// own, or a chord the browser has already dealt with.
    Ignore,
}

/// The one character a browser key name stands for, if it is one.
/// A browser names printable keys by the character they produce and
/// everything else by a word, so a name of length one is the whole
/// test.
fn printable(key: &str) -> Option<char> {
    let mut chars = key.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

/// What a keystroke means. `ctrl` and `alt` are the modifiers the
/// browser reports.
///
/// Ctrl-] is the overstrike key, as it is at the CLI and for the same
/// reason: backspace is needed for deleting, and Ctrl-H *is*
/// backspace. Alt holds a key back from the keymap, which is how a
/// reader types the ASCII a key is painted with.
#[must_use]
pub fn act(key: &str, ctrl: bool, alt: bool) -> Act {
    if ctrl {
        return match key {
            "]" => Act::Overstrike,
            "c" | "u" => Act::Clear,
            _ => Act::Ignore,
        };
    }
    match key {
        "Enter" => Act::Submit,
        "Backspace" => Act::Backspace,
        "Delete" => Act::Delete,
        "ArrowLeft" => Act::Carriage(Move::Left),
        "ArrowRight" => Act::Carriage(Move::Right),
        "Home" => Act::Carriage(Move::Home),
        "End" => Act::Carriage(Move::End),
        _ => match printable(key) {
            Some(c) if alt => Act::Literal(c),
            Some(c) => Act::Type(c),
            None => Act::Ignore,
        },
    }
}
