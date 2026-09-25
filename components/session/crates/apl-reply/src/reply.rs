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
    /// The last line is not finished: `⍞←` wrote to it without
    /// ending it, so whatever comes next belongs on the same line.
    /// A shell that ignores this prints one line where APL\360
    /// printed part of one.
    pub open: bool,
}

impl Reply {
    /// The finished lines, and the last one on its own when `⍞←`
    /// left it open. A shell prints the finished ones as lines and
    /// the open one without ending it, so what comes next carries
    /// on. Mirrors `Shown::split`, which is where the flag starts.
    #[must_use]
    pub fn split(&self) -> (&[String], Option<&str>) {
        match self.lines.split_last() {
            Some((last, rest)) if self.open => (rest, Some(last.as_str())),
            _ => (&self.lines, None),
        }
    }

    /// Lines reporting an error rather than a result.
    #[must_use]
    pub fn failed(lines: Vec<String>) -> Reply {
        Reply {
            lines,
            off: false,
            error: true,
            open: false,
        }
    }
}

impl From<Answer> for Reply {
    fn from(answer: Answer) -> Reply {
        Reply {
            lines: answer.lines,
            off: answer.off,
            error: false,
            open: false,
        }
    }
}

impl From<Vec<String>> for Reply {
    fn from(lines: Vec<String>) -> Reply {
        Reply {
            lines,
            off: false,
            error: false,
            open: false,
        }
    }
}
