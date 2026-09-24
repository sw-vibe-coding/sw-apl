//! `)LIBS`: the libraries this session reaches.

use apl_eval::Store;

/// One line per library, by number, with the name it is known by:
/// 0 USER, which is yours; 1 CORE, what sw-apl ships; and whatever
/// is configured beyond them, read-only.
#[must_use]
pub fn libs(store: &dyn Store) -> Vec<String> {
    let all = store.libraries();
    all.iter()
        .map(|l| format!("{} {}", l.number, l.name))
        .collect()
}
