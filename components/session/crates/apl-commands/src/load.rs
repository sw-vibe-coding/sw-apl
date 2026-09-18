//! Reading a workspace back, and taking names out of one.
//!
//! A workspace file is APL, so loading it is running it. Copying is
//! not: running the file would apply its settings, and the index
//! origin would change under code already written. Copying therefore
//! runs the definitions and leaves the commands alone. See
//! `docs/index-origin-considerations.md`.

use std::fs;

use apl_eval::Workspace;
use apl_wsfile::{DIRECTIVE, definitions, expand, plain};

use crate::command::{Answer, INCORRECT};
use crate::save::library;

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
    let Some((text, _)) = read(ws, rest) else {
        return Answer {
            lines: vec![INCORRECT.to_string()],
            ..Answer::default()
        };
    };
    let when = text
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{DIRECTIVE}SAVED ")));
    let mut feed = vec![")CLEAR".to_string()];
    feed.extend(text.lines().map(String::from));
    Answer {
        lines: vec![format!("SAVED {}", when.unwrap_or_default())],
        feed,
        off: false,
    }
}

/// `)COPY [lib] name [objects]` and `)PCOPY`: bring names out of a
/// saved workspace into this one, running its definitions but not its
/// commands. `protect` leaves alone any name already here.
pub fn copy(ws: &Workspace, rest: &[&str], protect: bool) -> Answer {
    let Some((text, used)) = read(ws, rest) else {
        return Answer {
            lines: vec![INCORRECT.to_string()],
            ..Answer::default()
        };
    };
    let asked = &rest[used.min(rest.len())..];
    // A group among the names asked for brings its members with it.
    let wanted = expand(&text, asked);
    let mut feed = Vec::new();
    for (name, lines) in definitions(&text) {
        let unwanted = !asked.is_empty() && !wanted.contains(&name);
        let taken = ws.saved.groups.contains_key(&name);
        let held = protect && (taken || ws.get(&name).is_some() || ws.is_function(&name));
        if !unwanted && !held {
            feed.extend(lines);
        }
    }
    Answer {
        feed,
        ..Answer::default()
    }
}

/// `)LIB [n]`: the workspaces in one library, by name.
pub fn lib(ws: &Workspace, rest: &[&str]) -> Vec<String> {
    let number = match rest {
        [] => Some(0),
        [n] => n.parse().ok(),
        _ => None,
    };
    let Some(dir) = number.and_then(|n| library(ws, n)) else {
        return vec![INCORRECT.to_string()];
    };
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|e| {
            e.file_name()
                .to_str()?
                .strip_suffix(".apl.ws")
                .map(String::from)
        })
        .collect();
    names.sort();
    names
}

/// The text of the workspace the first word or two of `rest` names,
/// from its library, with how many of those words it took: a library
/// number and a name, or just a name.
fn read(ws: &Workspace, rest: &[&str]) -> Option<(String, usize)> {
    let (number, name, used) = match rest {
        [number, name, ..] if number.parse::<usize>().is_ok() => (number.parse().ok()?, *name, 2),
        [name, ..] => (0, *name, 1),
        [] => return None,
    };
    let path = library(ws, number)?.join(format!("{name}.apl.ws"));
    // A workspace holding a locked function was written obscured;
    // everything above here works on the APL, not on the file.
    Some((plain(&fs::read_to_string(path).ok()?), used))
}
