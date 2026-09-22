//! Which stored workspace a mode means by a name.

use apl_store::Store;

use apl_modes::{Mode, Modes, modes};

/// What separates a name from the modes its key was made for, when
/// two workspaces share a name: `BIRDS` and `BIRDS@B`. A workspace
/// name is letters and digits, so it can never contain one.
pub const APART: char = '@';

/// A stored workspace under one of a name's keys.
pub struct Entry {
    /// The key the store holds it under.
    pub key: String,
    /// What is stored there.
    pub text: String,
    /// The modes its line names.
    pub modes: Modes,
}

/// Every stored workspace answering to `name`, whatever its modes.
pub fn entries(store: &dyn Store, library: usize, name: &str) -> Vec<Entry> {
    let ours = |key: &String| key.split(APART).next() == Some(name);
    let keys = store.list(library).into_iter().filter(ours);
    keys.filter_map(|key| {
        let text = store.read(library, &key)?;
        let modes = modes(&text);
        Some(Entry { key, text, modes })
    })
    .collect()
}

/// The workspace `mode` finds under `name` in `library`, if it runs
/// there.
#[must_use]
pub fn read(store: &dyn Store, mode: Mode, library: usize, name: &str) -> Option<String> {
    let found = entries(store, library, name).into_iter();
    found.filter(|e| e.modes.has(mode)).map(|e| e.text).next()
}

/// The names `mode` lists in `library`: the workspaces that run in
/// it, each once, sorted.
#[must_use]
pub fn list(store: &dyn Store, mode: Mode, library: usize) -> Vec<String> {
    let runs = |key: &String| {
        store
            .read(library, key)
            .is_some_and(|t| modes(&t).has(mode))
    };
    let keys = store.list(library).into_iter().filter(runs);
    let mut names: Vec<String> = keys
        .filter_map(|k| Some(k.split(APART).next()?.to_string()))
        .collect();
    names.sort();
    names.dedup();
    names
}
