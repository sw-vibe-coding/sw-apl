//! Reading a workspace back, and taking names out of one.
//!
//! A workspace file is APL, so loading it is running it. Copying is
//! not: running the file would apply its settings, and the index
//! origin would change under code already written. Copying therefore
//! runs the definitions and leaves the commands alone. See
//! `docs/index-origin-considerations.md`.

use std::fs;

use apl_copy::take;
use apl_eval::Workspace;
use apl_library::{IMPROPER_LIBRARY, INCORRECT, Stored, root, text};

use crate::command::Answer;

/// `)LOAD [lib] name`: replace the workspace with a saved one. The
/// file is APL, and loading it is typing it, so its lines come back
/// to be fed through the session -- the evaluator alone would miss
/// the `)` commands and the del definitions.
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

/// `)LIB [n]`: the workspaces in one library, by name.
pub fn lib(ws: &Workspace, rest: &[&str]) -> Vec<String> {
    // A word that is not a number is a command `)LIB` does not take;
    // a number naming no library is a reference that is not a
    // library. The manual keeps those apart and so does this.
    let numbered = |n: usize| root(ws, n).ok_or(IMPROPER_LIBRARY);
    let found = match rest {
        [] => numbered(0),
        [n] => n.parse().map_err(|_| INCORRECT).and_then(numbered),
        _ => Err(INCORRECT),
    };
    let dir = match found {
        Ok(dir) => dir,
        Err(report) => return vec![report.to_string()],
    };
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let stem = |e: fs::DirEntry| Some(e.file_name().to_str()?.strip_suffix(".apl.ws")?.to_string());
    let mut found: Vec<String> = entries.filter_map(Result::ok).filter_map(stem).collect();
    found.sort();
    found
}
