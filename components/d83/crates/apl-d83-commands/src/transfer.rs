//! `)IN` and `)OUT` (APL Reference, May 1983, pages 12-8 and 12-13).

use apl_d83_aio::{compose, contents, key};
use apl_d83_transfer::{Object, establish};
use apl_library::{IMPROPER_LIBRARY, Named, library, named};
use apl_workspace::{Workspace, moment};

/// The report for a transfer file, or an object, that is not there.
const NOT_FOUND: &str = "NOT FOUND";

/// `)IN [lib] name [objects]`: copy the objects named, or every one,
/// out of the transfer file into the workspace, each replacing what
/// held its name. The reply is SAVED and when the file was written.
/// Only with nothing on the state indicator.
pub fn inward(ws: &mut Workspace, rest: &[&str]) -> Vec<String> {
    if !ws.si().is_empty() {
        return vec!["NOT WITH UNCLEAR SI".to_string()];
    }
    let Named {
        library: n,
        name,
        used,
    } = match named(rest) {
        Ok(found) => found,
        Err(report) => return vec![report.to_string()],
    };
    let Some(n) = library(&*ws.store, n) else {
        return vec![IMPROPER_LIBRARY.to_string()];
    };
    let Some(text) = ws.store.read(n, &key(&name)) else {
        return vec![NOT_FOUND.to_string()];
    };
    let file = contents(&text);
    let mut lines = vec![format!("SAVED {}", file.when)];
    lines.extend(copy(ws, file.objects, &rest[used..]));
    lines
}

/// Establish the objects `asked` for, or all of them, and say what did
/// not come: the names asked for that the file does not hold, then NOT
/// FOUND; and any the workspace would not take, as NOT COPIED.
fn copy(ws: &mut Workspace, objects: Vec<Object>, asked: &[&str]) -> Vec<String> {
    let held: Vec<String> = objects.iter().map(|o| o.name().to_string()).collect();
    let missing: Vec<&str> = asked
        .iter()
        .copied()
        .filter(|a| !held.iter().any(|h| h == a))
        .collect();
    let wanted = objects
        .into_iter()
        .filter(|o| asked.is_empty() || asked.contains(&o.name()));
    let refused: Vec<String> = wanted
        .filter_map(|o| {
            let name = o.name().to_string();
            establish(ws, o).is_none().then_some(name)
        })
        .collect();
    let mut lines = Vec::new();
    if !missing.is_empty() {
        lines.extend([missing.join(" "), NOT_FOUND.to_string()]);
    }
    if !refused.is_empty() {
        lines.push(format!("NOT COPIED: {}", refused.join(" ")));
    }
    lines
}

/// `)OUT [lib] name [objects]`: write the objects named, or every one
/// and the settings, to the transfer file, replacing it. The reply is
/// when; or, when an object named has no transfer form, its name and
/// NOT FOUND, and nothing is written.
pub fn outward(ws: &mut Workspace, rest: &[&str]) -> Vec<String> {
    let Named {
        library: n,
        name,
        used,
    } = match named(rest) {
        Ok(found) => found,
        Err(report) => return vec![report.to_string()],
    };
    let when = moment(&(ws.clock)());
    let text = match compose(ws, &rest[used..], &when) {
        Ok(text) => text,
        Err(missing) => return vec![missing.join(" "), NOT_FOUND.to_string()],
    };
    match ws.store.write(n, &key(&name), Some(&text)) {
        Ok(()) => vec![when],
        Err(err) => vec![format!("NOT SAVED, {err}")],
    }
}
