//! Reading a workspace back, and taking names out of one.
//!
//! A saved workspace carries its settings as directives -- comments to
//! APL, instructions to sw-apl -- rather than as the commands that set
//! them, because only '70 has those commands. A workspace file is also
//! a program, so a directive takes effect wherever the line comes
//! from: a `)LOAD`, a file run with `-f`, or the keyboard.
//!
//! A workspace file is APL, so loading it is running it. Copying is
//! not: running the file would apply its settings, and the index
//! origin would change under code already written. Copying therefore
//! runs the definitions and leaves the commands alone. See
//! `docs/index-origin-considerations.md`.

use apl_copy::take;
use apl_eval::{Saved, Workspace};
use apl_library::{INCORRECT, Stored, text};

use apl_settings::setting;

use crate::command::Answer;

/// `)LOAD [lib] name`: replace the workspace with a saved one. A
/// stored workspace is APL, and loading it is typing it, so its
/// lines come back to be fed through the session -- the evaluator
/// alone would miss the `)` commands and the del definitions.
///
/// The reply is SAVED and the moment the file records, as APL\360
/// replied, and nothing else: typing DESCRIBE is the reader's move.
///
/// The clear that replaces the old workspace is the first line fed
/// back rather than something done here, so that it is undone with
/// the rest if a line of the file will not fit: the session puts the
/// old workspace aside before it runs any of them.
pub fn load(ws: &Workspace, rest: &[&str]) -> Answer {
    // A word `)LOAD` did not account for is a command it does not
    // take, which is a different fault from a name it cannot find.
    // `used` is 1 or 2 and never more than `rest` has, so what is
    // left over is what the command was given and cannot use.
    let whole = |found: Stored| match rest.len() - found.used {
        0 => Ok(found),
        _ => Err(INCORRECT),
    };
    let found = match text(ws, rest).and_then(whole) {
        Ok(found) => found,
        Err(report) => return trouble(report),
    };
    let mut feed = vec![")CLEAR".to_string()];
    feed.extend(found.apl.lines().map(String::from));
    Answer {
        lines: vec![format!("SAVED {}", found.when)],
        feed,
        off: false,
    }
}

/// A command that did nothing, and the report saying why.
fn trouble(report: &str) -> Answer {
    Answer {
        lines: vec![report.to_string()],
        ..Answer::default()
    }
}

/// `)COPY [lib] name [objects]` and `)PCOPY`: bring names out of a
/// stored workspace into this one.
///
/// The reply is the SAVED line the source records, as APL\360 gave
/// it, and for a protected copy the names it would not overwrite.
pub fn copy(ws: &Workspace, rest: &[&str], protect: bool) -> Answer {
    let (stored, taken) = match take(ws, rest, protect) {
        Ok(found) => found,
        Err(report) => return trouble(report),
    };
    let mut lines = vec![format!("SAVED {}", stored.when)];
    if !taken.kept.is_empty() {
        lines.push(format!("NOT COPIED: {}", taken.kept.join(" ")));
    }
    Answer {
        lines,
        feed: taken.feed,
        off: false,
    }
}

/// Where the random link must lie: a Lehmer generator's state is
/// never zero and always below its modulus.
const LINKS: std::ops::Range<u64> = 1..2_147_483_647;

/// Apply `line` if it is a settings directive, and say whether it
/// was one. A value out of range is ignored exactly as the command
/// would refuse it, and says nothing: a directive is still a comment.
pub fn directive(saved: &mut Saved, line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix("⍝!") else {
        return false;
    };
    let mut words = rest.split_whitespace();
    match (words.next(), words.next(), words.next()) {
        (Some(name @ ("ORIGIN" | "DIGITS" | "WIDTH")), Some(value), None) => {
            setting(saved, name, value);
            true
        }
        (Some("LINK"), Some(value), None) => {
            let state = value.parse().ok().filter(|n| LINKS.contains(n));
            saved.env.link = state.unwrap_or(saved.env.link);
            true
        }
        _ => false,
    }
}
