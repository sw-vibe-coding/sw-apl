//! A file of records in a store, under the key its DOS name gives.

use apl_d83_records::Record;
use apl_store::{DOS, Store};

use crate::line::{decode, encode};

/// What a file's first line says, so an empty file is told from none.
const HEAD: &str = "AP210 FILE";

/// The records of the file kept under `key` in library 0, None for a
/// record never written; None when there is no such file.
#[must_use]
pub fn load(store: &dyn Store, key: &str) -> Option<Vec<Option<Record>>> {
    let text = store.read(0, key)?;
    let mut lines = text.lines();
    (lines.next()? == HEAD).then(|| lines.map(decode).collect())
}

/// Keep `records` as the file under `key` in library 0.
///
/// # Errors
/// What the store could not do.
pub fn save(store: &mut dyn Store, key: &str, records: &[Option<Record>]) -> Result<(), String> {
    let mut text = format!("{HEAD}\n");
    for r in records {
        text.push_str(&r.as_ref().map(encode).unwrap_or_default());
        text.push('\n');
    }
    store.write(0, key, Some(&text))
}

/// The key `spec`, `[d:]filename[.ext]`, is kept under in library 0,
/// or None when it is no DOS file name: up to eight letters, digits
/// and DOS's other name characters, then up to three after a dot.
#[must_use]
pub fn key(spec: &str) -> Option<String> {
    let spec = spec.trim().to_ascii_uppercase();
    let name = match spec.split_once(':') {
        Some((drive, rest))
            if drive.len() == 1 && drive.chars().all(|c| c.is_ascii_alphabetic()) =>
        {
            rest
        }
        Some(_) => return None,
        None => &spec,
    };
    let (stem, ext) = name.split_once('.').unwrap_or((name, ""));
    let ok = |s: &str, most: usize| {
        s.len() <= most
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || "$#&@!%'()-_{}~^`".contains(c))
    };
    (!stem.is_empty() && ok(stem, 8) && ok(ext, 3)).then(|| format!("{name}{DOS}"))
}
