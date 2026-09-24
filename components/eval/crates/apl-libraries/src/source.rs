//! Where a configured library's workspaces come from.

use std::fs;
use std::path::PathBuf;

use apl_store::{SUFFIX, Shelf};

/// A configured library's workspaces.
#[derive(Debug, Clone)]
pub enum Source {
    /// A directory of `NAME.apl.ws` files, read when asked.
    Dir(PathBuf),
    /// Workspaces already in hand, by key: what a browser fetched.
    Kept(Shelf),
}

impl Source {
    /// The workspace kept under `key`, if one is.
    #[must_use]
    pub fn read(&self, key: &str) -> Option<String> {
        match self {
            Source::Dir(dir) => fs::read_to_string(dir.join(format!("{key}{SUFFIX}"))).ok(),
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
                .filter_map(|e| Some(e.file_name().to_str()?.strip_suffix(SUFFIX)?.to_string()))
                .collect(),
        };
        keys.sort();
        keys
    }
}
