//! The interactive session: a line editor with history behind the
//! six-space prompt (the bracketed line number in definition mode).
//! Up arrow recalls earlier input for editing or re-submission;
//! Ctrl-C cancels the line; Ctrl-D ends like `)OFF`.
//!
//! The same editor answers a statement that reads, so `⎕` and `⍞`
//! prompt the way any other line does.

use std::cell::RefCell;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;

use apl_session::{Reply, Session};
use apl_strike::{BACK, read as struck};
use rustyline::error::ReadlineError;
use rustyline::{
    Cmd, DefaultEditor, EventHandler, KeyCode, KeyEvent, Modifiers, Result as LineResult,
};

use crate::host::{Editor, Terminal, catch_interrupt};
use crate::shell::show;

/// Run the interactive loop until `)OFF` or end of input. `ws` is
/// the workspace size in bytes and the directory the libraries are
/// under.
///
/// # Errors
/// Terminal or I/O failures from the line editor.
pub fn run_interactive(ws: (usize, PathBuf)) -> io::Result<()> {
    let mut editor = DefaultEditor::new().map_err(io::Error::other)?;
    // The overstrike key inserts the marker that `compose` reads, so
    // rustyline goes on editing an ordinary line and the strike is
    // formed when the line is done. Backspace keeps deleting, which
    // a line editor needs and a 2741 never had.
    editor.bind_sequence(
        KeyEvent(KeyCode::Char(']'), Modifiers::CTRL),
        EventHandler::Simple(Cmd::Insert(1, BACK.to_string())),
    );
    let editor: Editor = Rc::new(RefCell::new(editor));
    let history = history_path();
    if let Some(path) = &history {
        let _ = editor.borrow_mut().load_history(path);
    }
    let mut session = Session::attached(Box::new(Terminal(Rc::clone(&editor))));
    (session.ws.quota, session.ws.libraries) = ws;
    catch_interrupt();
    prompt_loop(&mut session, &editor)?;
    if let Some(path) = &history {
        let _ = editor.borrow_mut().save_history(path);
    }
    Ok(())
}

/// Prompt and answer until `)OFF` or end of input.
///
/// A line `⍞←` left open is where the carriage is, so it is what the
/// reader prompts with: typing continues it, as at a terminal. End
/// of input is Ctrl-D, which signs off as `)OFF` does and ends the
/// loop either way. Every line read is composed first, so an
/// overstrike typed with the key bound above becomes its glyph
/// before the session sees it.
fn prompt_loop(session: &mut Session, editor: &Editor) -> io::Result<()> {
    let mut open = String::new();
    loop {
        let prompt = if open.is_empty() {
            session.prompt()
        } else {
            std::mem::take(&mut open)
        };
        let read = match read_line(&mut editor.borrow_mut(), &prompt) {
            Ok(read) => read,
            Err(err) => return Err(io::Error::other(err)),
        };
        let ending = read.is_none();
        let typed = read.unwrap_or_else(|| ")OFF".to_string());
        let reply = match struck(&typed) {
            Ok(line) => session.respond(&line),
            Err(report) => Reply::failed(vec![report]),
        };
        open = show(&reply);
        if reply.off || ending {
            return Ok(());
        }
    }
}

/// One edited line; `None` at end of input. A cancelled line (Ctrl-C)
/// is skipped and the prompt shown again.
pub fn read_line(editor: &mut DefaultEditor, prompt: &str) -> LineResult<Option<String>> {
    loop {
        match editor.readline(prompt) {
            Ok(line) => {
                let _ = editor.add_history_entry(&line);
                return Ok(Some(line));
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {
                println!();
                return Ok(None);
            }
            Err(err) => return Err(err),
        }
    }
}

/// `$HOME/.sw-apl_history`, when a home directory is known.
fn history_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".sw-apl_history"))
}
