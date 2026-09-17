//! The interactive session: a line editor with history behind the
//! six-space prompt. Up arrow recalls earlier input for editing or
//! re-submission; Ctrl-C cancels the line; Ctrl-D ends like `)OFF`.

use std::io;
use std::path::PathBuf;

use apl_session::{INDENT, Reply, Session};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as LineResult};

/// Run the interactive loop until `)OFF` or end of input.
///
/// # Errors
/// Terminal or I/O failures from the line editor.
pub fn run_interactive() -> io::Result<()> {
    let mut editor = DefaultEditor::new().map_err(io::Error::other)?;
    let history = history_path();
    if let Some(path) = &history {
        let _ = editor.load_history(path);
    }
    let mut session = Session::default();
    loop {
        match read_line(&mut editor) {
            Ok(Some(line)) => match session.respond(&line) {
                Reply::Off => break,
                Reply::Output(output) => output.iter().for_each(|t| println!("{t}")),
            },
            Ok(None) => break,
            Err(err) => return Err(io::Error::other(err)),
        }
    }
    if let Some(path) = &history {
        let _ = editor.save_history(path);
    }
    Ok(())
}

/// One edited line; `None` at end of input. A cancelled line (Ctrl-C)
/// is skipped and the prompt shown again.
fn read_line(editor: &mut DefaultEditor) -> LineResult<Option<String>> {
    loop {
        match editor.readline(INDENT) {
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
