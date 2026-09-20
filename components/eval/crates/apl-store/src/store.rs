//! What a library is, apart from where it is kept.

use std::fmt::Debug;

/// What a stored workspace is called on a filesystem, past its name.
///
/// It is here rather than in the filesystem store alone because a
/// browser's local storage is a flat namespace shared with whatever
/// else the page keeps, and the same suffix keeps sw-apl's own keys
/// apart from it.
pub const SUFFIX: &str = ".apl.ws";

/// Somewhere workspaces are kept, by library number and name.
///
/// Library 0 is yours, where `)SAVE` writes; library 1 is the public
/// one sw-apl ships. There is no other, and a store says so by
/// finding nothing there and refusing to write.
///
/// Errors are the words to print after `NOT SAVED, `, which is why
/// they are owned strings: a browser refusing to keep something says
/// so in its own words, and there is no useful way to know them in
/// advance.
pub trait Store: Debug {
    /// The workspace kept in `library` under `name`, if one is.
    fn read(&self, library: usize, name: &str) -> Option<String>;

    /// Keep `text` in `library` under `name` -- or, given nothing to
    /// keep, forget what is there.
    ///
    /// The two are one operation because they are one question put to
    /// the place things are kept: what stands under this name now. A
    /// store that cannot answer it says why.
    ///
    /// # Errors
    /// Whatever the store could not do: no such library, no such
    /// workspace to forget, a disc that would not take it, a browser
    /// that would not keep it.
    fn write(&mut self, library: usize, name: &str, text: Option<&str>) -> Result<(), String>;

    /// The names kept in `library`, sorted. A library that is not
    /// there and one that is empty both list nothing: `)LIB` prints
    /// no names either way, and the distinction is drawn above this,
    /// where a library number is checked before it is used.
    fn list(&self, library: usize) -> Vec<String>;
}
