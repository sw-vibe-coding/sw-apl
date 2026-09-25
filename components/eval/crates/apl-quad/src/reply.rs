//! A reply to quad input that is not an answer: a system command, or
//! a line that failed.

use apl_value::{AplError, AplResult, ErrorKind};
use apl_workspace::{Output, Workspace, error_lines};

/// Run `typed` as a system command when it is one and the workspace
/// has a session to run it. `true` when it ran and the read is to be
/// made again, with what it showed ahead of the prompt.
///
/// # Errors
/// INTERRUPT when the command abandoned the read.
pub fn command(ws: &mut Workspace, typed: &str) -> AplResult<bool> {
    let (Some(text), Some(run)) = (typed.trim_start().strip_prefix(')'), ws.command) else {
        return Ok(false);
    };
    let lines = run(ws, text).ok_or_else(|| AplError::new(ErrorKind::Interrupt))?;
    ws.output.push(Output::Lines(lines));
    Ok(true)
}

/// Report an error in a reply, ahead of the prompt that asks again.
/// A function the reply called and that failed is not left waiting:
/// the request is made again, so nothing it began is resumed.
pub fn report(ws: &mut Workspace, depth: usize, err: &AplError, typed: &str) {
    while ws.si().len() > depth {
        ws.leave();
    }
    ws.output.push(Output::Lines(error_lines(err, typed)));
}
