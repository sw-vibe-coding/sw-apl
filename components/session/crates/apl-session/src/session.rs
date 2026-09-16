//! The session state machine (immediate execution only, so far).

use apl_display::format_array;
use apl_eval::{Workspace, eval_line};
use apl_value::AplError;

/// The six-space indent that precedes every input line.
pub const INDENT: &str = "      ";

/// What the shell should do after handing a line to the session.
#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    /// Print these lines (may be empty) and prompt again.
    Output(Vec<String>),
    /// End the session.
    Off,
}

/// An interactive APL session.
#[derive(Debug)]
pub struct Session {
    ws: Workspace,
    /// Print precision (quad-PP).
    pp: usize,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            ws: Workspace::default(),
            pp: 10,
        }
    }
}

impl Session {
    /// Respond to one input line.
    pub fn respond(&mut self, line: &str) -> Reply {
        let text = line.trim();
        if let Some(command) = text.strip_prefix(')') {
            return system_command(command);
        }
        Reply::Output(match eval_line(&mut self.ws, line) {
            Ok(Some(value)) => format_array(&value, self.pp),
            Ok(None) => Vec::new(),
            Err(err) => error_lines(&err, line),
        })
    }
}

/// APL\360 error display: name, echoed statement, caret line.
fn error_lines(err: &AplError, line: &str) -> Vec<String> {
    let caret = err.caret.unwrap_or(0);
    vec![
        err.kind.to_string(),
        format!("{INDENT}{line}"),
        format!("{INDENT}{}^", " ".repeat(caret)),
    ]
}

/// System commands: only `)OFF` so far.
fn system_command(command: &str) -> Reply {
    if command.trim().eq_ignore_ascii_case("OFF") {
        Reply::Off
    } else {
        Reply::Output(vec!["INCORRECT COMMAND".to_string()])
    }
}
