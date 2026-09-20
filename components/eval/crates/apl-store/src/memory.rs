//! Libraries as text held in memory, which is what a browser has.
//!
//! A browser worker cannot reach local storage and cannot wait for
//! the page to answer one, so the workspaces are held here and the
//! page is told, through `kept`, whenever library 0 changes. What
//! the page does with them -- keep them, or find it has nowhere to
//! -- does not change what the session sees, which is why a refusal
//! to keep something is not an error here: the workspace is saved,
//! and only its survival past the tab is in doubt.
//!
//! Library 1 is whatever the host put in it, and read only: sw-apl
//! ships those workspaces and `)SAVE` has never written there.

use std::collections::BTreeMap;

use crate::store::Store;

/// What a library holds: each workspace's text, under its name.
pub type Shelf = BTreeMap<String, String>;

/// What a store says when asked to write where it may not.
pub const READ_ONLY: &str = "THAT LIBRARY IS READ ONLY";

/// Libraries with no filesystem under them.
#[derive(Debug, Default, Clone)]
pub struct Memory {
    /// Library 0, which `)SAVE` writes and `)DROP` forgets.
    pub work: Shelf,
    /// Library 1, which the host fills and nothing here changes.
    pub lib1: Shelf,
    /// Told whenever library 0 changes, so a host that has somewhere
    /// to keep it can. `None` when nothing is keeping it, which is
    /// what a test wants and what the trait tests use.
    pub kept: Option<fn(&Shelf)>,
}

impl Memory {
    /// The shelf a library number names, or `None` for no library.
    fn shelf(&self, library: usize) -> Option<&Shelf> {
        match library {
            0 => Some(&self.work),
            1 => Some(&self.lib1),
            _ => None,
        }
    }
}

impl Store for Memory {
    fn read(&self, library: usize, name: &str) -> Option<String> {
        self.shelf(library)?.get(name).cloned()
    }

    fn write(&mut self, library: usize, name: &str, text: Option<&str>) -> Result<(), String> {
        if library != 0 {
            return Err(READ_ONLY.to_string());
        }
        match text {
            Some(text) => {
                self.work.insert(name.to_string(), text.to_string());
            }
            None if self.work.remove(name).is_some() => {}
            None => return Err("there is no such workspace".to_string()),
        }
        if let Some(keep) = self.kept {
            keep(&self.work);
        }
        Ok(())
    }

    fn list(&self, library: usize) -> Vec<String> {
        self.shelf(library)
            .map(|shelf| shelf.keys().cloned().collect())
            .unwrap_or_default()
    }
}
