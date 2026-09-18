//! Writing a workspace out, and the directories they live in.

use std::fs;
use std::path::PathBuf;

use apl_eval::{Workspace, hms};
use apl_wsfile::write;

use crate::command::{CLEAR, INCORRECT};

/// The directory a library number names, under the session's library
/// root. Library 0 is yours, where `)SAVE` writes; the numbered ones
/// are public. `None` when there is no such library.
#[must_use]
pub fn library(ws: &Workspace, number: usize) -> Option<PathBuf> {
    let under = |path: &str| Some(ws.libraries.join(path));
    match number {
        0 => under("work"),
        1 => under("ws/lib1"),
        _ => None,
    }
}

/// `)SAVE [name]`: write the workspace into library 0, under the name
/// given or the one it already answers to. The reply is the moment it
/// was written and the name it was written under, as APL\360 replied.
/// Library 0 is made if it is not there, rather than refusing.
pub fn save(ws: &mut Workspace, name: Option<&str>) -> Vec<String> {
    let id = name
        .map(ToString::to_string)
        .or_else(|| ws.saved.id.clone());
    // Comma, not colon: the APL\360 manual's trouble-report table
    // gives `NOT SAVED, THIS WS IS wsid`, beside `NOT SAVED, WS
    // QUOTA USED UP` and `NOT GROUPED, NAME IN USE`. A comma
    // introduces a reason; a colon introduces a list, as in `NOT
    // COPIED:` and `NOT ERASED:`. Both forms are used below.
    let Some(id) = id else {
        return vec![format!("NOT SAVED, THIS WS IS {CLEAR}")];
    };
    ws.saved.id = Some(id.clone());
    let when = moment(ws);
    let Some(dir) = library(ws, 0) else {
        return vec![INCORRECT.to_string()];
    };
    if let Err(err) = fs::create_dir_all(&dir) {
        return vec![format!("NOT SAVED, {err}")];
    }
    match fs::write(dir.join(format!("{id}.apl.ws")), write(&ws.saved, &when)) {
        Ok(()) => vec![format!("{when} {id}")],
        Err(err) => vec![format!("NOT SAVED, {err}")],
    }
}

/// `)DROP name`: forget a saved workspace. The reply is the moment it
/// was dropped.
pub fn drop_workspace(ws: &Workspace, rest: &[&str]) -> Vec<String> {
    let [name] = rest else {
        return vec![INCORRECT.to_string()];
    };
    let Some(path) = library(ws, 0).map(|dir| dir.join(format!("{name}.apl.ws"))) else {
        return vec![INCORRECT.to_string()];
    };
    match fs::remove_file(path) {
        Ok(()) => vec![format!("DROPPED {name}")],
        Err(_) => vec![format!("NOT FOUND {name}")],
    }
}

/// A moment in the form APL\360 stamped one: the time, then the
/// date. It heads a `)SAVE` reply and opens a sign-off.
#[must_use]
pub fn moment(ws: &Workspace) -> String {
    let time = (ws.clock)();
    let date = time.date;
    format!(
        "{} {:02}/{:02}/{:02}",
        hms(time.now),
        date / 10_000,
        (date / 100) % 100,
        date % 100
    )
}
