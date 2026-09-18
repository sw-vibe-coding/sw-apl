//! The session state machine: immediate execution and definition mode.

use apl_call::{clear, resume, suspend};
use apl_editor::Definition;
use apl_eval::{Console, INDENT, Output, Workspace, error_lines, eval_line, render_all, system};
use apl_value::AplResult;

use crate::commands::{definition_line, open_definition, run_command};
use crate::reply::Reply;

/// An interactive APL session.
#[derive(Debug, Default)]
pub struct Session {
    /// The workspace this session owns: its variables, functions,
    /// print settings, and the console it reads a line through.
    pub ws: Workspace,
    /// The function open in the del editor, while definition mode
    /// is open.
    pub(crate) defining: Option<Definition>,
}

impl Session {
    /// A session attached to a terminal: statements read through
    /// `console`, and the I-beams read the real clock rather than the
    /// stopped one a bare workspace starts with. Sign-on is the
    /// moment this is called, which is what `⌶24` reports.
    #[must_use]
    pub fn attached(console: Box<dyn Console>) -> Session {
        let mut session = Session::default();
        session.ws.console = console;
        session.ws.clock = system;
        session.ws.signed_on = system().now;
        session
    }

    /// The prompt for the next input line: six spaces in immediate
    /// execution, the bracketed line number in definition mode.
    #[must_use]
    pub fn prompt(&self) -> String {
        self.defining
            .as_ref()
            .map_or_else(|| INDENT.to_string(), Definition::prompt)
    }

    /// Evaluate one statement. A branch typed here is not a branch at
    /// all: a bare arrow clears the top of the state indicator, and a
    /// line number takes the suspended function up again there. An
    /// error that reaches this far settles which function it left
    /// suspended.
    fn run(&mut self, line: &str) -> AplResult<Output> {
        // Every path comes back through this match rather than using
        // `?`: an error has to reach `suspend` below, which is what
        // settles the starred entry in the state indicator.
        let shown = match eval_line(&mut self.ws, line) {
            Ok(Output::Branch(Some(n))) if !self.ws.si().is_empty() => {
                resume(&mut self.ws, n, eval_line)
            }
            Ok(Output::Branch(None)) => {
                clear(&mut self.ws);
                Ok(Output::Nothing)
            }
            Ok(Output::Branch(Some(_))) => Ok(Output::Nothing),
            other => other,
        };
        shown.inspect_err(|_| suspend(&mut self.ws))
    }

    /// Respond to one input line.
    pub fn respond(&mut self, line: &str) -> Reply {
        if self.defining.is_some() {
            return definition_line(self, line);
        }
        let trimmed = line.trim();
        if let Some(command) = trimmed.strip_prefix(')') {
            return run_command(self, command);
        }
        if let Some(header) = trimmed.strip_prefix('∇') {
            return open_definition(self, header);
        }
        let result = self.run(line);
        // Anything a read showed already comes first, then whatever
        // the statement produced after it.
        let mut lines = self.ws.console.take();
        lines.extend(self.ws.flush().lines);
        let mut reply = match result {
            Ok(out) => Reply::from(render_all(&[out], self.ws.saved.print).lines),
            Err(err) => Reply::failed(error_lines(&err, line)),
        };
        lines.append(&mut reply.lines);
        reply.lines = lines;
        reply
    }
}
