//! Which workspace a command means, and what to say when there is
//! not one.

use apl_eval::Workspace;
use apl_wsfile::{DIRECTIVE, definitions, plain};

use crate::name::valid;
use crate::report::{IMPROPER_LIBRARY, INCORRECT, OBJECT_NOT_FOUND, WS_NOT_FOUND};

/// The library a number names. Library 0 is yours, where `)SAVE`
/// writes; library 1 is the public one sw-apl ships. `None` when
/// there is no such library, which is a different thing from an
/// empty one.
///
/// Which libraries exist is the interpreter's rule and not the
/// store's: a browser and a filesystem keep the same two.
#[must_use]
pub fn library(number: usize) -> Option<usize> {
    match number {
        0 | 1 => Some(number),
        _ => None,
    }
}

/// A library and a name, and how many words of the command they
/// took.
pub struct Named {
    /// The library number, 0 when the command did not give one.
    pub library: usize,
    /// The workspace name, already checked.
    pub name: String,
    /// How many words of the command the library and name took.
    pub used: usize,
}

/// The workspace a `[lib] name` names: a library number and a name,
/// or just a name. The error is the report to print, which is why
/// this returns one rather than `None` -- a missing library and a
/// missing name are not the same fault and must not read the same.
///
/// A name is checked before it is used for anything. It becomes a
/// filename under the filesystem store, and one that is not a name
/// could name a directory, a parent, or a file outside the library
/// entirely.
///
/// # Errors
/// INCORRECT COMMAND when no name was given or what was given is
/// not a name, and IMPROPER LIBRARY REFERENCE when the number names
/// no library.
pub fn named(rest: &[&str]) -> Result<Named, &'static str> {
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
    let library = library(number).ok_or(IMPROPER_LIBRARY)?;
    Ok(Named {
        library,
        name: name.to_string(),
        used,
    })
}

/// A stored workspace, read.
pub struct Stored {
    /// Its contents as APL, revealed if it was stored obscured.
    pub apl: String,
    /// When it was stored, as the workspace records it. `)LOAD` and
    /// `)COPY` both report this, which is why it is read here
    /// rather than by each of them.
    pub when: String,
    /// How many words of the command the library and name took.
    pub used: usize,
}

/// The workspace a `[lib] name` names, read out of the store. A
/// workspace holding a locked function was stored obscured, so this
/// reveals it: everything above works on the APL, not on what was
/// kept.
///
/// # Errors
/// As `named`, and WS NOT FOUND when no workspace of that name is
/// stored there.
pub fn text(ws: &Workspace, rest: &[&str]) -> Result<Stored, &'static str> {
    let Named {
        library,
        name,
        used,
    } = named(rest)?;
    let read = apl_shelves::read(&*ws.store, ws.mode, library, &name).ok_or(WS_NOT_FOUND)?;
    let apl = plain(&read);
    let stamp = format!("{DIRECTIVE}SAVED ");
    let when = apl.lines().find_map(|l| l.strip_prefix(&stamp));
    let when = when.unwrap_or_default().to_string();
    Ok(Stored { apl, when, used })
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
