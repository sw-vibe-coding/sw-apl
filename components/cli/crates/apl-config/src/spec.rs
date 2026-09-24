//! One configured library, as a flag gives it, and adding the lot to
//! a store.

use std::path::PathBuf;

use apl_libraries::{Added, Source};
use apl_store::Store;

/// A library to add: its number (2 and up), the name `)LIBS` gives
/// it, and the directory it is kept in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySpec {
    /// 2 and up: 0 and 1 are sw-apl's own.
    pub number: usize,
    /// Capitals, as `)LIBS` prints it.
    pub name: String,
    /// The directory of `NAME.apl.ws` files.
    pub path: PathBuf,
}

/// `N=DIR[,NAME]`, as `--lib` takes it. The name is optional, LIBN
/// when it is not given, and is kept in capitals.
///
/// # Errors
/// What is wrong with it, in words for the command line.
pub fn parse_lib(arg: &str) -> Result<LibrarySpec, String> {
    let (number, rest) = arg.split_once('=').ok_or("expected N=DIR[,NAME]")?;
    let number: usize = number
        .parse()
        .map_err(|_| format!("{number} is not a library number"))?;
    let (path, name) = rest
        .split_once(',')
        .map_or((rest, None), |(p, n)| (p, Some(n)));
    let name = name.map_or(format!("LIB{number}"), str::to_ascii_uppercase);
    check(number, &name)?;
    if path.is_empty() {
        return Err("a library needs a directory".to_string());
    }
    let path = PathBuf::from(path);
    Ok(LibrarySpec { number, name, path })
}

/// A number sw-apl will add, and a name `)LIBS` can print.
///
/// # Errors
/// Library 0 or 1, or a name that is not letters and digits.
pub(crate) fn check(number: usize, name: &str) -> Result<(), String> {
    if number < 2 {
        return Err(format!(
            "library {number} is sw-apl's own; configure 2 and up"
        ));
    }
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!(
            "{name:?} is not a library name: letters and digits"
        ));
    }
    Ok(())
}

/// `store` with `libraries` added, each read-only from its directory.
/// A directory that is not there is added all the same, listing
/// nothing, and said so on stderr: a session is not refused for it.
#[must_use]
pub fn attach(store: Box<dyn Store>, libraries: &[LibrarySpec]) -> Added {
    let mut added = Added::new(store);
    for l in libraries {
        if !l.path.is_dir() {
            eprintln!(
                "sw-apl: library {} ({}): no such directory {}",
                l.number,
                l.name,
                l.path.display()
            );
        }
        let place = l.path.display().to_string();
        added = added.with(l.number, &l.name, &place, Source::Dir(l.path.clone()));
    }
    added
}
