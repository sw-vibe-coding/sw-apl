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

use apl_session::{Console, INDENT, Reply, Session, Shown};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as LineResult};

/// The line editor, shared between the loop and the console so a read
/// part way through a statement uses the same history.
type Editor = Rc<RefCell<DefaultEditor>>;

/// The console the interactive session reads through, over the same
/// editor the loop uses.
#[derive(Debug)]
struct Terminal(Editor);

impl Console for Terminal {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (lines, open) = shown.split();
        for line in lines {
            println!("{line}");
        }
        if !prompt.is_empty() {
            println!("{prompt}");
        }
        let at = open.unwrap_or(INDENT);
        read_line(&mut self.0.borrow_mut(), at).ok().flatten()
    }
}

/// Run the interactive loop until `)OFF` or end of input.
///
/// # Errors
/// Terminal or I/O failures from the line editor.
pub fn run_interactive() -> io::Result<()> {
    let editor = DefaultEditor::new().map_err(io::Error::other)?;
    let editor: Editor = Rc::new(RefCell::new(editor));
    let history = history_path();
    if let Some(path) = &history {
        let _ = editor.borrow_mut().load_history(path);
    }
    let mut session = Session::default();
    session.ws.console = Box::new(Terminal(Rc::clone(&editor)));
    loop {
        let prompt = session.prompt();
        match read_line(&mut editor.borrow_mut(), &prompt) {
            Ok(Some(line)) => match session.respond(&line) {
                Reply::Off => break,
                Reply::Output(output) => output.iter().for_each(|t| println!("{t}")),
            },
            Ok(None) => break,
            Err(err) => return Err(io::Error::other(err)),
        }
    }
    if let Some(path) = &history {
        let _ = editor.borrow_mut().save_history(path);
    }
    Ok(())
}

/// One edited line; `None` at end of input. A cancelled line (Ctrl-C)
/// is skipped and the prompt shown again.
fn read_line(editor: &mut DefaultEditor, prompt: &str) -> LineResult<Option<String>> {
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
