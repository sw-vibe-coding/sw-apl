//! Transfer files listed and dropped: `)LIB [lib] .AIO` and
//! `)DROP name.AIO`, the forms PC APL's `)LIB` and `)DROP` take for a
//! file rather than a workspace (APL Reference, May 1983, page 12-15).

use apl_d83_aio::{key, list};
use apl_library::{IMPROPER_LIBRARY, library, valid};
use apl_store::TRANSFER;
use apl_workspace::{Workspace, moment};

/// `)LIB .AIO` and `)LIB n .AIO`: the transfer files in a library, by
/// name. `None` for a `)LIB` that names no extension, which lists
/// workspaces as it always has.
pub fn list_files(ws: &Workspace, rest: &[&str]) -> Option<Vec<String>> {
    let number = match rest {
        [ext] if ext.eq_ignore_ascii_case(TRANSFER) => 0,
        [n, ext] if ext.eq_ignore_ascii_case(TRANSFER) => n.parse().ok()?,
        _ => return None,
    };
    Some(match library(&*ws.store, number) {
        Some(n) => list(&*ws.store, n),
        None => vec![IMPROPER_LIBRARY.to_string()],
    })
}

/// `)DROP name.AIO`: forget a transfer file in library 0. The reply is
/// the moment, as for a workspace; NOT FOUND when there is none.
/// `None` for a `)DROP` of a workspace.
pub fn drop_file(ws: &mut Workspace, rest: &[&str]) -> Option<Vec<String>> {
    let [file] = rest else {
        return None;
    };
    let cut = file.len().checked_sub(TRANSFER.len())?;
    let (name, ext) = (file.get(..cut)?, file.get(cut..)?);
    if !ext.eq_ignore_ascii_case(TRANSFER) || !valid(name) {
        return None;
    }
    let when = moment(&(ws.clock)());
    Some(match ws.store.write(0, &key(name), None) {
        Ok(()) => vec![when],
        Err(_) => vec!["NOT FOUND".to_string()],
    })
}
