mod actions;
mod input;
mod screen;

use aplterm_keyboard::Keyboard;
use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute, terminal,
};
use input::Outcome;
use std::io;

struct Raw;
impl Raw {
    fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let guard = Self;
        execute!(io::stdout(), EnableBracketedPaste)?;
        Ok(guard)
    }
}
impl Drop for Raw {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableBracketedPaste);
        let _ = terminal::disable_raw_mode();
    }
}

/// Raw keystrokes stay local; only completed, composed lines are returned.
pub fn read_line(
    prompt: &str,
    keyboard: &mut Keyboard,
    history: &mut Vec<String>,
) -> io::Result<Option<String>> {
    let _raw = Raw::enter()?;
    keyboard.clear();
    let mut state = actions::State {
        history_at: history.len(),
        draft: String::new(),
        escaped: false,
    };
    loop {
        screen::redraw(prompt, keyboard)?;
        match input::read(keyboard, &mut state, history)? {
            Outcome::Continue => (),
            Outcome::Submit => return screen::submit(prompt, keyboard, history).map(Some),
            Outcome::Disconnect => {
                print!("\r\n");
                return Ok(None);
            }
        }
    }
}
