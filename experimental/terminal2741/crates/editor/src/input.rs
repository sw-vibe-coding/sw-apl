use crate::actions::{self, State};
use aplterm_keyboard::Keyboard;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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

pub(crate) enum Outcome {
    Continue,
    Submit,
    Disconnect,
}

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

fn character(c: char, modifiers: KeyModifiers, keyboard: &mut Keyboard) {
    if modifiers.contains(KeyModifiers::ALT) {
        keyboard.paste(&c.to_string());
    } else if !keyboard.type_key(c) {
        print!("\x07");
    }
}
