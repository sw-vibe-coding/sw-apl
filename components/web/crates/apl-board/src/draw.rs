//! The line as a page needs to draw it.

use apl_keyboard::Keyboard;
use serde_json::json;

/// What one keystroke leaves behind, as JSON.
///
/// `text` is the whole line and `before` is what the carriage has
/// passed, so the page knows where to draw it -- a count of
/// characters would not do, because a glyph may be two code points
/// and an overstrike changes one already typed. `pending` is a strike
/// waiting for its second key, `bell` a pair that forms no glyph,
/// `submit` a finished line, and `acted` whether the keystroke was
/// ours at all: a chord we do not claim has to reach the browser, so
/// that copy still copies and reload still reloads.
#[must_use]
pub fn state(keyboard: &Keyboard, bell: bool, submit: bool, acted: bool) -> String {
    json!({
        "text": keyboard.text(),
        "before": keyboard.before_cursor(),
        "pending": keyboard.pending,
        "bell": bell,
        "submit": submit,
        "acted": acted,
    })
    .to_string()
}
