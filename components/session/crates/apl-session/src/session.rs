//! The session state machine: immediate execution and definition mode.

use apl_call::{clear, resume, suspend};
use apl_editor::Definition;
use apl_eval::{Console, Host, INDENT, Output, Workspace, error_lines, eval_line, system};
use apl_settings::clear as clear_workspace;
use apl_value::AplResult;

use crate::commands::dispatch;
use crate::quad::abandon;
use apl_reply::Reply;

/// An interactive APL session.
#[derive(Debug)]
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
    /// moment this is called, which is what `⌶24` reports. The host's
    /// decisions -- the quota, where the libraries are kept, the mode
    /// -- are made here too, once, as the session begins, and the
    /// workspace is the clear one that mode starts with.
    #[must_use]
    pub fn attached(console: Box<dyn Console>, host: Host) -> Session {
        let mut session = Session::default();
        session.ws.console = console;
        session.ws.clock = system;
        session.ws.signed_on = system().now;
        session.ws.quota = host.quota;
        session.ws.store = host.store;
        session.ws.mode = host.mode;
        session.ws.saved = clear_workspace(host.mode);
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
    ///
    /// A command is looked for before an open definition takes the
    /// line, because a system command entered during a definition is
    /// never a statement in it. `run_command` says which are refused
    /// outright and which run at once.
    ///
    /// Anything a read showed comes first. What the statement
    /// produced goes back into the workspace's pending output rather
    /// than being rendered on its own, so that one flush decides
    /// whether the last line was left open -- a statement showing
    /// nothing must not close a line an earlier one wrote to. An
    /// error report starts on a line of its own.
    pub fn respond(&mut self, line: &str) -> Reply {
        if let Some(reply) = dispatch(self, line) {
            return reply;
        }
        let result = self.run(line);
        if let Some(command) = self.ws.abandoned.take() {
            return abandon(self, &command);
        }
        let mut lines = self.ws.console.take();
        let failed = result.map(|out| self.ws.output.push(out)).err();
        let shown = self.ws.flush();
        lines.extend(shown.lines);
        let Some(err) = failed else {
            return Reply {
                lines,
                open: shown.open,
                ..Reply::default()
            };
        };
        lines.extend(error_lines(&err, line));
        Reply::failed(lines)
    }
}
