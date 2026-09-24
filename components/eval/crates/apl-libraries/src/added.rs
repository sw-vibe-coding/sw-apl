//! A store with configured libraries added to it.

use std::collections::BTreeMap;

use apl_store::{Library, READ_ONLY, Store};

use crate::source::Source;

/// The store a session had, and the libraries configured beyond it.
#[derive(Debug)]
pub struct Added {
    /// Libraries 0 and 1, as they were.
    pub(crate) inner: Box<dyn Store>,
    /// The rest, by number.
    pub(crate) more: BTreeMap<usize, (Library, Source)>,
}

impl Store for Added {
    fn read(&self, library: usize, name: &str) -> Option<String> {
        match self.more.get(&library) {
            Some((_, source)) => source.read(name),
            None => self.inner.read(library, name),
        }
    }

    fn write(&mut self, library: usize, name: &str, text: Option<&str>) -> Result<(), String> {
        if self.more.contains_key(&library) {
            return Err(READ_ONLY.to_string());
        }
        self.inner.write(library, name, text)
    }

    fn list(&self, library: usize) -> Vec<String> {
        match self.more.get(&library) {
            Some((_, source)) => source.list(),
            None => self.inner.list(library),
        }
    }

    fn libraries(&self) -> Vec<Library> {
        let mut all = self.inner.libraries();
        all.retain(|l| !self.more.contains_key(&l.number));
        all.extend(self.more.values().map(|(l, _)| l.clone()));
        all.sort_by_key(|l| l.number);
        all
    }
}
