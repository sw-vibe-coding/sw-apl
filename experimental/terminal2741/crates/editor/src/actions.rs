use aplterm_keyboard::{Keyboard, Move};
use crossterm::event::KeyCode;

pub(crate) struct State {
    pub history_at: usize,
    pub draft: String,
    pub escaped: bool,
}

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
        KeyCode::F(4) => keyboard.paste(")"),
        _ => (),
    }
}

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
