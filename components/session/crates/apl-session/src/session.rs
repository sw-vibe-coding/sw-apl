//! The session state machine (immediate execution only, so far).

use apl_display::format_array;
use apl_eval::{Workspace, eval_line};
use apl_value::AplError;

use crate::commands::system_command;

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
    pub(crate) ws: Workspace,
    /// Print precision (`)DIGITS`).
    pub(crate) digits: usize,
    /// Print width (`)WIDTH`).
    pub(crate) width: usize,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            ws: Workspace::default(),
            digits: 10,
            width: 120,
        }
    }
}

impl Session {
    /// Respond to one input line.
    pub fn respond(&mut self, line: &str) -> Reply {
        if let Some(command) = line.trim().strip_prefix(')') {
            return system_command(self, command);
        }
        let result = eval_line(&mut self.ws, line);
        let mut lines: Vec<String> = self
            .ws
            .output
            .drain(..)
            .flat_map(|v| format_array(&v, self.digits, self.width))
            .collect();
        lines.extend(match result {
            Ok(Some(value)) => format_array(&value, self.digits, self.width),
            Ok(None) => Vec::new(),
            Err(err) => error_lines(&err, line),
        });
        Reply::Output(lines)
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
