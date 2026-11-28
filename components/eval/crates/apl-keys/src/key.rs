//! Keys and file names.

/// What a stored workspace is called on a filesystem, past its name.
///
/// It is here rather than in the filesystem store alone because a
/// browser's local storage is a flat namespace shared with whatever
/// else the page keeps, and the same suffix keeps sw-apl's own keys
/// apart from it.
pub const SUFFIX: &str = ".apl.ws";

/// What a transfer file is called past its name: (D)'s `)OUT` writes
/// `NAME.AIO`, as IBM PC APL did, and `)IN` reads it. A store keeps
/// one under that key, beside the workspaces; a workspace key never
/// ends so, since its modes are written in lower case.
pub const TRANSFER: &str = ".AIO";

/// What a DOS file is called past its name: (D)'s AP210 keeps the
/// file `B:FILE.EXT` as `FILE.EXT.DOS`, beside the workspaces, every
/// drive being library 0. Upper case, as a transfer file's is.
pub const DOS: &str = ".DOS";

/// Whether `key` is a workspace's, not a transfer file's or a DOS
/// file's: those have no modes line, and would pass for (A)'s.
#[must_use]
pub fn workspace(key: &str) -> bool {
    !key.ends_with(TRANSFER) && !key.ends_with(DOS)
}

/// The file a key is kept in on a filesystem: a workspace's name and
/// [`SUFFIX`], or a transfer or DOS file's own name.
#[must_use]
pub fn file_of(key: &str) -> String {
    if !workspace(key) {
        return key.to_string();
    }
    format!("{key}{SUFFIX}")
}

/// The key a file on a filesystem is kept under, if it is one a
/// library holds: a workspace, a transfer file or a DOS file.
#[must_use]
pub fn key_of(file: &str) -> Option<String> {
    let key = file
        .strip_suffix(SUFFIX)
        .or((!workspace(file)).then_some(file))?;
    Some(key.to_string())
}
