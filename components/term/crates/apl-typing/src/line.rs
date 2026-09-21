//! One line, from the first keystroke to Enter.

use apl_keyboard::Keyboard;
use apl_paper::{redraw, submit};

use crate::actions;
use crate::input::{self, Outcome};
use crossterm::{
    event::{
        self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEventKind,
        KeyModifiers,
    },
    execute, terminal,
};
use std::io;
use std::time::Duration;

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

/// While the service is working: hold the keyboard until `ready` has
/// something, and call `attn` whenever ATTN is pressed.
///
/// The keyboard is locked, as a 2741's was while the computer worked,
/// with only ATTN live. Raw mode is held for the whole wait, not taken
/// and dropped between looks: a key typed in cooked mode would be echoed
/// by the terminal and its Return turned into a newline the line editor
/// does not take for Enter, so there is no gap for one to fall into.
/// Every key is read; ATTN keys raise ATTN and the rest are dropped.
///
/// Escape is ATTN, and Ctrl-[, the same byte. So is Ctrl-C, the
/// interrupt key at the CLI: in raw mode it is only a key, and left
/// alone it would do nothing while a loop ran.
///
/// # Errors
/// Whatever the terminal reports, or `attn` does.
pub fn wait<T>(
    mut ready: impl FnMut() -> Option<T>,
    mut attn: impl FnMut() -> io::Result<()>,
) -> io::Result<T> {
    let _raw = Raw::enter()?;
    loop {
        if let Some(done) = ready() {
            return Ok(done);
        }
        if !event::poll(LOOK)? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let is_attn =
            key.code == KeyCode::Esc || (ctrl && matches!(key.code, KeyCode::Char('[' | 'c')));
        if key.kind != KeyEventKind::Release && is_attn {
            attn()?;
        }
    }
}

/// How long a wait looks at the keyboard before looking for its answer
/// again: the most a finished answer waits to be noticed.
const LOOK: Duration = Duration::from_millis(10);
