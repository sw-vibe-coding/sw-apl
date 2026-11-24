//! Where transfer files are kept: beside the workspaces, under keys
//! of their own.

use apl_store::{Store, TRANSFER};

/// The key the transfer file `name` is kept under: `NAME.AIO`.
#[must_use]
pub fn key(name: &str) -> String {
    format!("{name}{TRANSFER}")
}

/// The names of the transfer files in `library`, sorted.
#[must_use]
pub fn list(store: &dyn Store, library: usize) -> Vec<String> {
    let keys = store.list(library).into_iter();
    let mut names: Vec<String> = keys
        .filter_map(|k| k.strip_suffix(TRANSFER).map(String::from))
        .collect();
    names.sort();
    names
}
