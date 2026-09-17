//! The session state machine: immediate execution and definition mode.

use apl_editor::Definition;
use apl_eval::{Workspace, eval_line};

use crate::commands::{definition_line, open_definition, system_command};
use crate::render::{error_lines, render};

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
    /// The function open in the del editor, while definition mode
    /// is open.
    pub(crate) defining: Option<Definition>,
    /// Print precision (`)DIGITS`).
    pub(crate) digits: usize,
    /// Print width (`)WIDTH`).
    pub(crate) width: usize,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            ws: Workspace::default(),
            defining: None,
            digits: 10,
            width: 120,
        }
    }
}

impl Session {
    /// The prompt for the next input line: six spaces in immediate
    /// execution, the bracketed line number in definition mode.
    #[must_use]
    pub fn prompt(&self) -> String {
        self.defining
            .as_ref()
            .map_or_else(|| INDENT.to_string(), Definition::prompt)
    }

    /// Respond to one input line.
    pub fn respond(&mut self, line: &str) -> Reply {
        if self.defining.is_some() {
            return definition_line(self, line);
        }
        let trimmed = line.trim();
        if let Some(command) = trimmed.strip_prefix(')') {
            return system_command(self, command);
        }
        if let Some(header) = trimmed.strip_prefix('∇') {
            return open_definition(self, header);
        }
        let result = eval_line(&mut self.ws, line);
        let shown: Vec<_> = self.ws.output.drain(..).collect();
        let mut lines: Vec<String> = shown
            .iter()
            .flat_map(|o| render(o, self.digits, self.width))
            .collect();
        lines.extend(match result {
            Ok(out) => render(&out, self.digits, self.width),
            Err(err) => error_lines(&err, line),
        });
        Reply::Output(lines)
    }
}
