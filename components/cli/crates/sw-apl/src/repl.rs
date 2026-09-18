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

use apl_session::Session;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as LineResult};

use crate::host::{Editor, Terminal, catch_interrupt};

/// Run the interactive loop until `)OFF` or end of input, in a
/// workspace of `size` bytes.
///
/// # Errors
/// Terminal or I/O failures from the line editor.
pub fn run_interactive(size: usize) -> io::Result<()> {
    let editor = DefaultEditor::new().map_err(io::Error::other)?;
    let editor: Editor = Rc::new(RefCell::new(editor));
    let history = history_path();
    if let Some(path) = &history {
        let _ = editor.borrow_mut().load_history(path);
    }
    let mut session = Session::attached(Box::new(Terminal(Rc::clone(&editor))));
    session.ws.quota = size;
    catch_interrupt();
    prompt_loop(&mut session, &editor)?;
    if let Some(path) = &history {
        let _ = editor.borrow_mut().save_history(path);
    }
    Ok(())
}

/// Prompt and answer until `)OFF` or end of input.
fn prompt_loop(session: &mut Session, editor: &Editor) -> io::Result<()> {
    loop {
        let prompt = session.prompt();
        // End of input is Ctrl-D, which signs off as `)OFF` does. It
        // ends the loop either way: in definition mode `)OFF` is a
        // body line, and there would be nothing left to close it.
        let read = match read_line(&mut editor.borrow_mut(), &prompt) {
            Ok(read) => read,
            Err(err) => return Err(io::Error::other(err)),
        };
        let ending = read.is_none();
        let reply = session.respond(&read.unwrap_or_else(|| ")OFF".to_string()));
        for text in &reply.lines {
            println!("{text}");
        }
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
