//! Keys that edit the line rather than type into it.

use apl_keyboard::{Keyboard, Move};
use crossterm::event::KeyCode;

/// What a read keeps besides the line itself.
pub(crate) struct State {
    /// Where in the history recall has reached.
    pub history_at: usize,
    /// The line that was being typed when recall started.
    pub draft: String,
    /// Escape was pressed: the next key is literal.
    pub escaped: bool,
}

/// An editing key.
pub(crate) fn edit(code: KeyCode, keyboard: &mut Keyboard, state: &mut State, history: &[String]) {
    match code {
        KeyCode::Backspace => keyboard.backspace(),
        KeyCode::Delete => keyboard.delete(),
        KeyCode::Left => keyboard.move_cursor(Move::Left),
        KeyCode::Right => keyboard.move_cursor(Move::Right),
        KeyCode::Home => keyboard.move_cursor(Move::Home),
        KeyCode::End => keyboard.move_cursor(Move::End),
        KeyCode::Up | KeyCode::Down => recall(code == KeyCode::Up, keyboard, state, history),
        _ => special(code, keyboard, state),
    }
}

/// The keys that are neither editing nor typing: literal mode, the
/// escape that quotes one key, and the `)` a command starts with.
fn special(code: KeyCode, keyboard: &mut Keyboard, state: &mut State) {
    match code {
        KeyCode::Esc => {
            keyboard.pending = false;
            state.escaped = true;
        }
        KeyCode::F(2) => {
            keyboard.literal = !keyboard.literal;
            let mode = if keyboard.literal {
                "Literal Unicode"
            } else {
                "2741"
            };
            print!("\r\n[{mode} input]\r\n");
        }
        // F4 is the `)` a command begins with; whether it struck is the
        // keyboard's business and needs no answer here.
        KeyCode::F(4) => {
            keyboard.paste(")");
        }
        _ => (),
    }
}

/// Up and down the history, keeping the half-typed line to come
/// back to.
fn recall(previous: bool, keyboard: &mut Keyboard, state: &mut State, history: &[String]) {
    if previous && state.history_at > 0 {
        if state.history_at == history.len() {
            state.draft = keyboard.text();
        }
        state.history_at -= 1;
    } else if !previous && state.history_at < history.len() {
        state.history_at += 1;
    } else {
        return;
    }
    keyboard.clear();
    keyboard.paste(history.get(state.history_at).unwrap_or(&state.draft));
}
