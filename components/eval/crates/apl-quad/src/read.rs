//! The two reads.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::{Output, Run, Workspace};

use crate::reply::{command, report};

/// The APL\360 prompt for evaluated input. Quote-quad has none: it
/// carries on wherever the last `⍞←` left the line.
const QUAD: &str = "⎕:";

/// Read a line and evaluate it as a statement in the current
/// environment, so it sees the locals of whatever is running. A line
/// that yields no value -- blank, a comment, an assignment -- prompts
/// again, as APL\360 does.
///
/// The manuals, APL\360's and the 5110's alike: an invalid entry
/// gets its error report and the request is made again, and a system
/// command is executed and the request is made again, unless it
/// replaces the workspace.
///
/// # Errors
/// INTERRUPT when there is no more input, when a branch is typed,
/// which is how the reader is escaped, when a stop was asked for, and
/// when a command abandoned the read.
pub fn evaluated(ws: &mut Workspace, run: Run) -> AplResult<Array> {
    loop {
        let typed = read(ws, QUAD)?;
        if command(ws, &typed)? {
            continue;
        }
        let depth = ws.si().len();
        match run(ws, &typed) {
            Ok(Output::Value(value)) => return Ok(value),
            Ok(Output::Mixed(parts)) => {
                if let Some(value) = parts.into_iter().next() {
                    return Ok(value);
                }
            }
            Ok(Output::Branch(_)) => return Err(AplError::new(ErrorKind::Interrupt)),
            Ok(_) => {}
            Err(err) if err.kind == ErrorKind::Interrupt => return Err(err),
            Err(err) => report(ws, depth, &err, &typed),
        }
    }
}

/// Read a line as characters, with no prompt and no evaluation.
///
/// # Errors
/// INTERRUPT when there is no more input.
pub fn characters(ws: &mut Workspace) -> AplResult<Array> {
    let typed = read(ws, "")?;
    let chars: Vec<char> = typed.chars().collect();
    let shape = vec![chars.len()];
    Array::new(shape, Data::Char(chars))
}

/// Show what the statement has produced so far, then take a line.
fn read(ws: &mut Workspace, prompt: &str) -> AplResult<String> {
    let shown = ws.flush();
    ws.console
        .read(&shown, prompt)
        .ok_or_else(|| AplError::new(ErrorKind::Interrupt))
}
