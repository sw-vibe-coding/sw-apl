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
}

impl From<Answer> for Reply {
    fn from(answer: Answer) -> Reply {
        Reply {
            lines: answer.lines,
            off: answer.off,
        }
    }
}

impl From<Vec<String>> for Reply {
    fn from(lines: Vec<String>) -> Reply {
        Reply { lines, off: false }
    }
}
