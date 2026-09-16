//! Placeholder session: the line-in, lines-out contract that the
//! `apl-session` crate will implement. Until then every statement
//! answers `NOT IMPLEMENTED` and `)OFF` ends the session.

/// What the shell should do after handing a line to the session.
#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    /// Print these lines (may be empty) and prompt again.
    Output(Vec<String>),
    /// End the session.
    Off,
}

/// The six-space indent that precedes every input line.
pub const INDENT: &str = "      ";

/// Respond to one input line.
pub fn respond(line: &str) -> Reply {
    let text = line.trim();
    if text.eq_ignore_ascii_case(")OFF") {
        return Reply::Off;
    }
    if text.is_empty() || text.starts_with('\u{235D}') {
        return Reply::Output(Vec::new());
    }
    Reply::Output(vec!["NOT IMPLEMENTED".to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_ends_session_case_insensitively() {
        assert_eq!(respond(")OFF"), Reply::Off);
        assert_eq!(respond("  )off"), Reply::Off);
    }

    #[test]
    fn blank_and_comment_lines_are_silent() {
        assert_eq!(respond(""), Reply::Output(vec![]));
        assert_eq!(respond("\u{235D} a comment"), Reply::Output(vec![]));
    }

    #[test]
    fn statements_are_not_implemented_yet() {
        assert_eq!(
            respond("2+2"),
            Reply::Output(vec!["NOT IMPLEMENTED".to_string()])
        );
    }
}
