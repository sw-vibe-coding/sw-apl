//! Which file a command means, and what to say when there is not one.

use std::fs;
use std::path::PathBuf;

use apl_eval::Workspace;
use apl_wsfile::{definitions, plain};

use crate::name::valid;
use crate::report::{IMPROPER_LIBRARY, INCORRECT, OBJECT_NOT_FOUND, WS_NOT_FOUND};

/// The directory a library number names, under the session's library
/// root. Library 0 is yours, where `)SAVE` writes; the numbered ones
/// are public. `None` when there is no such library, which is a
/// different thing from an empty one.
#[must_use]
pub fn root(ws: &Workspace, number: usize) -> Option<PathBuf> {
    let under = |path: &str| Some(ws.libraries.join(path));
    match number {
        0 => under("work"),
        1 => under("ws/lib1"),
        _ => None,
    }
}

/// The file a `[lib] name` names, and how many words that took: a
/// library number and a name, or just a name. The error is the
/// report to print, which is why this returns one rather than
/// `None` -- a missing library and a missing name are not the same
/// fault and must not read the same.
///
/// A name is checked before it is joined to anything. It becomes a
/// filename, and one that is not a name could name a directory, a
/// parent, or a file outside the library entirely.
///
/// # Errors
/// INCORRECT COMMAND when no name was given or what was given is
/// not a name, and IMPROPER LIBRARY REFERENCE when the number names
/// no library.
pub fn file(ws: &Workspace, rest: &[&str]) -> Result<(PathBuf, usize), &'static str> {
    let (number, name, used) = match rest {
        [number, name, ..] if number.parse::<usize>().is_ok() => {
            (number.parse().unwrap_or_default(), *name, 2)
        }
        [name, ..] => (0, *name, 1),
        [] => return Err(INCORRECT),
    };
    if !valid(name) {
        return Err(INCORRECT);
    }
    let dir = root(ws, number).ok_or(IMPROPER_LIBRARY)?;
    Ok((dir.join(format!("{name}.apl.ws")), used))
}

/// The APL in the file a `[lib] name` names, and how many words that
/// took. A workspace holding a locked function was written obscured,
/// so this reveals it: everything above works on the APL, not on the
/// file.
///
/// # Errors
/// As `file`, and WS NOT FOUND when no workspace of that name is
/// stored there.
pub fn text(ws: &Workspace, rest: &[&str]) -> Result<(String, usize), &'static str> {
    let (path, used) = file(ws, rest)?;
    let read = fs::read_to_string(path).map_err(|_| WS_NOT_FOUND)?;
    Ok((plain(&read), used))
}

/// Whether the workspace in `apl` holds every name asked for.
///
/// One name it does not hold is enough to refuse the lot: a copy
/// that half worked cannot be told from one that worked, and the
/// manual's report is about the command rather than about each name.
///
/// # Errors
/// OBJECT NOT FOUND when a name is not there.
pub fn holds(apl: &str, asked: &[&str]) -> Result<(), &'static str> {
    let there: Vec<String> = definitions(apl).into_iter().map(|(name, _)| name).collect();
    if asked.iter().all(|n| there.contains(&(*n).to_string())) {
        return Ok(());
    }
    Err(OBJECT_NOT_FOUND)
}
