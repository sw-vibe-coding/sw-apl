//! Saving and dropping in one mode without disturbing another.

use apl_modes::{Mode, Modes, modes, retag};
use apl_store::Store;

use crate::find::{Entry, entries};

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
    let key = key_for(name, claimed);
    store.write(library, &key, Some(&retag(text, claimed)))?;
    match mine.filter(|e| e.key != key) {
        Some(e) => narrow(store, library, name, &e, e.modes.minus(claimed)),
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
    narrow(store, library, name, &entry, left)
}

/// `entry` keeps only the modes `left`, under the key that says so,
/// or goes.
fn narrow(
    store: &mut dyn Store,
    library: usize,
    name: &str,
    entry: &Entry,
    left: Modes,
) -> Result<(), String> {
    if left == Modes::NONE {
        return store.write(library, &entry.key, None);
    }
    let key = key_for(name, left);
    store.write(library, &key, Some(&retag(&entry.text, left)))?;
    if key == entry.key {
        return Ok(());
    }
    store.write(library, &entry.key, None)
}

/// The key a workspace running in `modes` is kept under: the name
/// alone when it runs in both, and the name marked with its one mode
/// when not, so a list of the files shows which mode each is for
/// (owner, 2026-09-22).
fn key_for(name: &str, modes: Modes) -> String {
    if modes == Modes::only(Mode::A) {
        format!("{name}.a-70")
    } else if modes == Modes::only(Mode::B) {
        format!("{name}.b-75")
    } else {
        name.to_string()
    }
}
