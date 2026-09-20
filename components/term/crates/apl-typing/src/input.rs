//! One keystroke at a time.

use apl_keyboard::Keyboard;

use crate::actions::{self, State};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Take the next terminal event and act on it. A paste arrives as
/// one event and goes in literally: it is Unicode already, not
/// keystrokes to translate.
pub(crate) fn read(
    keyboard: &mut Keyboard,
    state: &mut State,
    history: &[String],
) -> std::io::Result<Outcome> {
    Ok(match event::read()? {
        Event::Paste(text) => {
            keyboard.paste(&text);
            Outcome::Continue
        }
        Event::Key(key) if key.kind != KeyEventKind::Release => {
            handle(key, keyboard, state, history)
        }
        _ => Outcome::Continue,
    })
}

/// What the keystroke means for the line being read.
pub(crate) enum Outcome {
    /// Go on reading.
    Continue,
    /// Enter: send the line.
    Submit,
    /// Ctrl-D on an empty line: hang up.
    Disconnect,
}

/// What one key does. Enter with a strike still waiting rings the
/// bell instead of sending: the glyph is half typed.
pub(crate) fn handle(
    key: KeyEvent,
    keyboard: &mut Keyboard,
    state: &mut State,
    history: &[String],
) -> Outcome {
    if std::mem::take(&mut state.escaped) {
        if let KeyCode::Char(c) = key.code {
            keyboard.paste(&c.to_string());
        }
    } else if key.modifiers.contains(KeyModifiers::CONTROL) {
        return control(key.code, keyboard);
    } else {
        match key.code {
            KeyCode::Enter if !keyboard.pending => return Outcome::Submit,
            KeyCode::Enter => print!("\x07"),
            KeyCode::Char(c) => character(c, key.modifiers, keyboard),
            code => actions::edit(code, keyboard, state, history),
        }
    }
    Outcome::Continue
}

/// A key held with Ctrl.
fn control(code: KeyCode, keyboard: &mut Keyboard) -> Outcome {
    match code {
        // Crossterm decodes the legacy Ctrl-] byte (0x1D) as Ctrl-5.
        KeyCode::Char(']' | '5' | '\u{1d}') => keyboard.overstrike(),
        KeyCode::Char('c' | 'u') => keyboard.clear(),
        KeyCode::Char('d') if keyboard.text().is_empty() => return Outcome::Disconnect,
        KeyCode::Char('d') => keyboard.delete(),
        _ => (),
    }
    Outcome::Continue
}

/// An ordinary key: translated by the keymap, or literal when held
/// with Alt. A pair that forms no glyph rings the bell.
fn character(c: char, modifiers: KeyModifiers, keyboard: &mut Keyboard) {
    if modifiers.contains(KeyModifiers::ALT) {
        keyboard.paste(&c.to_string());
    } else if !keyboard.type_key(c) {
        print!("\x07");
    }
}
