//! Where a configured library's workspaces come from.

use std::fs;
use std::path::PathBuf;

use apl_store::{Shelf, file_of, key_of};

/// A configured library's workspaces.
#[derive(Debug, Clone)]
pub enum Source {
    /// A directory of `NAME.apl.ws` files, and `NAME.AIO` transfer
    /// files, read when asked.
    Dir(PathBuf),
    /// Workspaces already in hand, by key: what a browser fetched.
    Kept(Shelf),
}

impl Source {
    /// The workspace kept under `key`, if one is.
    #[must_use]
    pub fn read(&self, key: &str) -> Option<String> {
        match self {
            Source::Dir(dir) => fs::read_to_string(dir.join(file_of(key))).ok(),
            Source::Kept(held) => held.get(key).cloned(),
        }
    }

    /// Every key kept, sorted. A directory that is not there keeps
    /// nothing.
    #[must_use]
    pub fn list(&self) -> Vec<String> {
        let mut keys: Vec<String> = match self {
            Source::Kept(held) => held.keys().cloned().collect(),
            Source::Dir(dir) => fs::read_dir(dir)
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .filter_map(|e| key_of(e.file_name().to_str()?))
                .collect(),
        };
        keys.sort();
        keys
    }
}
