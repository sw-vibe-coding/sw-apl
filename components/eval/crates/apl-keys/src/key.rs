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

/// The file a key is kept in on a filesystem: a workspace's name and
/// [`SUFFIX`], or a transfer file's own name.
#[must_use]
pub fn file_of(key: &str) -> String {
    if key.ends_with(TRANSFER) {
        return key.to_string();
    }
    format!("{key}{SUFFIX}")
}

/// The key a file on a filesystem is kept under, if it is one a
/// library holds: a workspace or a transfer file.
#[must_use]
pub fn key_of(file: &str) -> Option<String> {
    let key = file
        .strip_suffix(SUFFIX)
        .or(file.ends_with(TRANSFER).then_some(file))?;
    Some(key.to_string())
}
