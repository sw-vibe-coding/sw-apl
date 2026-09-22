//! Saving and dropping in one mode without disturbing another.

use apl_modes::{Mode, Modes, modes, render, retag};
use apl_store::Store;

use crate::find::{APART, Entry, entries};

/// Keep `text` as `mode`'s workspace `name`.
///
/// It replaces the workspace this mode finds under the name, in every
/// mode that one was listed in and this one runs in; where it no
/// longer runs, the old one stays listed. It is also listed in any
/// other mode it runs in that has no workspace of that name. What
/// another mode keeps under the name as its own is never touched,
/// which is how two workspaces come to share a name.
///
/// The new text is written before anything else is touched, so a
/// store that refuses it has lost nothing.
///
/// # Errors
/// Whatever the store could not do.
pub fn keep(
    store: &mut dyn Store,
    mode: Mode,
    library: usize,
    name: &str,
    text: &str,
) -> Result<(), String> {
    let (mine, theirs): (Vec<Entry>, Vec<Entry>) = entries(store, library, name)
        .into_iter()
        .partition(|e| e.modes.has(mode));
    let held = |left: Modes, e: &Entry| left.minus(e.modes);
    let claimed = theirs.iter().fold(modes(text), held);
    let claimed = Modes::ALL.minus(Modes::ALL.minus(claimed).minus(Modes::only(mode)));
    let mine = mine.into_iter().next();
    let key = key_for(name, claimed, mine.as_ref(), &theirs);
    store.write(library, &key, Some(&retag(text, claimed)))?;
    match mine.filter(|e| e.key != key) {
        Some(e) => narrow(store, library, &e.key, &e.text, e.modes.minus(claimed)),
        None => Ok(()),
    }
}

/// Forget `mode`'s workspace `name`. One that runs in other modes too
/// stays for them.
///
/// # Errors
/// When `mode` has no workspace of that name, and whatever the store
/// could not do.
pub fn forget(store: &mut dyn Store, mode: Mode, library: usize, name: &str) -> Result<(), String> {
    let found = entries(store, library, name)
        .into_iter()
        .find(|e| e.modes.has(mode));
    let entry = found.ok_or("there is no such workspace")?;
    let left = entry.modes.minus(Modes::only(mode));
    narrow(store, library, &entry.key, &entry.text, left)
}

/// What is under `key` keeps only the modes `left`, or goes.
fn narrow(
    store: &mut dyn Store,
    library: usize,
    key: &str,
    text: &str,
    left: Modes,
) -> Result<(), String> {
    if left == Modes::NONE {
        return store.write(library, key, None);
    }
    store.write(library, key, Some(&retag(text, left)))
}

/// The key a save claiming `claimed` goes under: the name itself when
/// nothing that stays is there, and the name marked with its modes
/// when something is. What stays is another mode's own workspace, and
/// this mode's old one where it keeps modes of its own.
fn key_for(name: &str, claimed: Modes, mine: Option<&Entry>, theirs: &[Entry]) -> String {
    let stays = mine.filter(|e| e.modes.minus(claimed) != Modes::NONE);
    if theirs.iter().chain(stays).all(|e| e.key != name) {
        return name.to_string();
    }
    format!("{name}{APART}{}", render(claimed).replace(['(', ')'], ""))
}
