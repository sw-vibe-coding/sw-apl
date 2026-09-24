//! Adding libraries to a store.

use std::collections::BTreeMap;

use apl_store::{Library, Store};

use crate::added::Added;
use crate::source::Source;

impl Added {
    /// `inner`, with no library added yet.
    #[must_use]
    pub fn new(inner: Box<dyn Store>) -> Added {
        Added {
            inner,
            more: BTreeMap::new(),
        }
    }

    /// Add library `number`, called `name`, kept in `source` and
    /// described by `place`. Libraries 0 and 1 are the store's own and
    /// cannot be replaced: adding one of them does nothing.
    #[must_use]
    pub fn with(mut self, number: usize, name: &str, place: &str, source: Source) -> Added {
        if number > 1 {
            let library = Library {
                number,
                name: name.to_string(),
                place: place.to_string(),
            };
            self.more.insert(number, (library, source));
        }
        self
    }
}
