//! Writing a workspace out, and the directories they live in.

use std::fs;

use apl_eval::{Workspace, hms};
use apl_library::{INCORRECT, WS_NOT_FOUND, file, not_saved};
use apl_wsfile::write;

use crate::command::CLEAR;

/// `)SAVE [name]`: write the workspace into library 0, under the name
/// given or the one it already answers to. The reply is the moment it
/// was written and the name it was written under, as APL\360 replied.
/// Library 0 is made if it is not there, rather than refusing.
pub fn save(ws: &mut Workspace, name: Option<&str>) -> Vec<String> {
    let active = ws.saved.id.clone();
    let Some(id) = name.map(ToString::to_string).or_else(|| active.clone()) else {
        return vec![not_saved(CLEAR)];
    };
    let Ok((path, _)) = file(ws, &[id.as_str()]) else {
        return vec![INCORRECT.to_string()];
    };
    // Report 13: naming a stored workspace that is not this one does
    // not overwrite it. `)SAVE` with no name re-stores this one, and
    // that is always allowed.
    if path.exists() && active.as_deref() != Some(id.as_str()) {
        return vec![not_saved(active.as_deref().unwrap_or(CLEAR))];
    }
    ws.saved.id = Some(id.clone());
    let when = moment(ws);
    let made = path.parent().map_or(Ok(()), fs::create_dir_all);
    if let Err(err) = made {
        return vec![format!("NOT SAVED, {err}")];
    }
    match fs::write(path, write(&ws.saved, &when)) {
        Ok(()) => vec![format!("{when} {id}")],
        Err(err) => vec![format!("NOT SAVED, {err}")],
    }
}

/// `)DROP name`: forget a stored workspace. The reply is the moment
/// it was dropped, and nothing else -- APL\360's `)DROP WSID` prints
/// the time and the date, so an echo of the name would be output it
/// never produced. It takes no library number: library 0 is the only
/// one you can write to.
pub fn drop_workspace(ws: &Workspace, rest: &[&str]) -> Vec<String> {
    let [name] = rest else {
        return vec![INCORRECT.to_string()];
    };
    let path = match file(ws, &[*name]) {
        Ok((path, _)) => path,
        Err(report) => return vec![report.to_string()],
    };
    match fs::remove_file(path) {
        Ok(()) => vec![moment(ws)],
        Err(_) => vec![WS_NOT_FOUND.to_string()],
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
