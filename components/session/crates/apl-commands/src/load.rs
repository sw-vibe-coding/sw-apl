//! Reading a workspace back, and taking names out of one.
//!
//! A workspace file is APL, so loading it is running it. Copying is
//! not: running the file would apply its settings, and the index
//! origin would change under code already written. Copying therefore
//! runs the definitions and leaves the commands alone. See
//! `docs/index-origin-considerations.md`.

use std::fs;

use apl_eval::{Saved, Workspace};
use apl_wsfile::{DIRECTIVE, definitions};

use crate::command::{Answer, INCORRECT};
use crate::save::library;

/// `)LOAD [lib] name`: replace the workspace with a saved one. The
/// file is APL, and loading it is typing it, so its lines come back
/// to be fed through the session -- the evaluator alone would miss
/// the `)` commands and the del definitions.
///
/// The reply is SAVED and the moment the file records, as APL\360
/// replied, and nothing else: typing DESCRIBE is the reader's move.
pub fn load(ws: &mut Workspace, rest: &[&str]) -> Answer {
    let Some((text, _)) = read(ws, rest) else {
        return Answer {
            lines: vec![INCORRECT.to_string()],
            ..Answer::default()
        };
    };
    ws.saved = Saved::default();
    let when = text
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{DIRECTIVE}SAVED ")));
    Answer {
        lines: vec![format!("SAVED {}", when.unwrap_or_default())],
        feed: text.lines().map(String::from).collect(),
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
    let wanted = &rest[used.min(rest.len())..];
    let mut feed = Vec::new();
    for (name, lines) in definitions(&text) {
        let unwanted = !wanted.is_empty() && !wanted.contains(&name.as_str());
        let held = protect && (ws.get(&name).is_some() || ws.is_function(&name));
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
    Some((fs::read_to_string(path).ok()?, used))
}
