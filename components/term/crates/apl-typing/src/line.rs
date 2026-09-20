//! One line, from the first keystroke to Enter.

use apl_keyboard::Keyboard;
use apl_paper::{redraw, submit};

use crate::actions;
use crate::input::{self, Outcome};
use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute, terminal,
};
use std::io;

/// Raw mode, given back however the read ends. A terminal left in
/// raw mode by a panicking client is a shell the reader has to
/// rescue, so the guard is the only way it is entered.
struct Raw;
impl Raw {
    /// Enter raw mode, with bracketed paste on.
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

/// Read one line at the keyboard, prompted with `prompt`.
///
/// `None` is Ctrl-D on an empty line: the reader hanging up, which
/// drops the link and, at the service's end, releases the session.
///
/// # Errors
/// Whatever the terminal reports.
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
        redraw(prompt, keyboard)?;
        match input::read(keyboard, &mut state, history)? {
            Outcome::Continue => (),
            Outcome::Submit => return submit(prompt, keyboard, history).map(Some),
            Outcome::Disconnect => {
                print!("\r\n");
                return Ok(None);
            }
        }
    }
}
