//! What the session hands back for one input line.

use apl_commands::Answer;

/// What the shell should do after handing a line to the session:
/// print these lines, which may be none, and then either prompt again
/// or stop.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Reply {
    /// The transcript lines this input produced.
    pub lines: Vec<String>,
    /// Set by `)OFF`: the session has ended.
    pub off: bool,
    /// The line did not do what it said: an error was reported rather
    /// than a result. The shell does not care, but a `)LOAD` feeding
    /// a workspace file back through the session does -- it is what
    /// tells it the load failed and must be undone.
    pub error: bool,
}

impl Reply {
    /// Lines reporting an error rather than a result.
    #[must_use]
    pub fn failed(lines: Vec<String>) -> Reply {
        Reply {
            lines,
            off: false,
            error: true,
        }
    }
}

impl From<Answer> for Reply {
    fn from(answer: Answer) -> Reply {
        Reply {
            lines: answer.lines,
            off: answer.off,
            error: false,
        }
    }
}

impl From<Vec<String>> for Reply {
    fn from(lines: Vec<String>) -> Reply {
        Reply {
            lines,
            off: false,
            error: false,
        }
    }
}
