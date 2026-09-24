//! Writing a workspace out, forgetting one, and listing a library.

use apl_eval::{Workspace, hms};
use apl_library::{IMPROPER_LIBRARY, INCORRECT, WS_NOT_FOUND, library, named, not_saved};
use apl_wsfile::write;

use crate::command::CLEAR;

/// `)SAVE [name]`: write the workspace into library 0, under the name
/// given or the one it already answers to. The reply is the moment it
/// was written and the name it was written under, as APL\360 replied.
/// Library 0 is made if it is not there, rather than refusing, and
/// what "made" means is the store's business: a directory on a disc,
/// a key in a browser.
pub fn save(ws: &mut Workspace, name: Option<&str>) -> Vec<String> {
    let active = ws.saved.id.clone();
    let Some(id) = name.map(ToString::to_string).or_else(|| active.clone()) else {
        return vec![not_saved(CLEAR)];
    };
    if named(&[id.as_str()]).is_err() {
        return vec![INCORRECT.to_string()];
    }
    // Report 13: naming a stored workspace that is not this one does
    // not overwrite it. `)SAVE` with no name re-stores this one, and
    // that is always allowed.
    let there = apl_shelves::read(&*ws.store, ws.mode, 0, &id).is_some();
    if there && active.as_deref() != Some(id.as_str()) {
        return vec![not_saved(active.as_deref().unwrap_or(CLEAR))];
    }
    ws.saved.id = Some(id.clone());
    let when = moment(ws);
    let text = write(&ws.saved, &when);
    match apl_shelves::keep(&mut *ws.store, ws.mode, 0, &id, &text) {
        Ok(()) => vec![format!("{when} {id}")],
        Err(err) => vec![format!("NOT SAVED, {err}")],
    }
}

/// `)DROP name`: forget a stored workspace. The reply is the moment
/// it was dropped, and nothing else -- APL\360's `)DROP WSID` prints
/// the time and the date, so an echo of the name would be output it
/// never produced. It takes no library number: library 0 is the only
/// one you can write to.
pub fn drop_workspace(ws: &mut Workspace, rest: &[&str]) -> Vec<String> {
    let [name] = rest else {
        return vec![INCORRECT.to_string()];
    };
    let found = match named(&[*name]) {
        Ok(found) => found,
        Err(report) => return vec![report.to_string()],
    };
    let when = moment(ws);
    match apl_shelves::forget(&mut *ws.store, ws.mode, 0, &found.name) {
        Ok(()) => vec![when],
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

/// `)LIB [n]`: the workspaces in one library, by name.
pub fn lib(ws: &Workspace, rest: &[&str]) -> Vec<String> {
    // A word that is not a number is a command `)LIB` does not take;
    // a number naming no library is a reference that is not a
    // library. The manual keeps those apart and so does this.
    let numbered = |n: usize| library(&*ws.store, n).ok_or(IMPROPER_LIBRARY);
    let found = match rest {
        [] => numbered(0),
        [n] => n.parse().map_err(|_| INCORRECT).and_then(numbered),
        _ => Err(INCORRECT),
    };
    match found {
        Ok(number) => apl_shelves::list(&*ws.store, ws.mode, number),
        Err(report) => vec![report.to_string()],
    }
}
