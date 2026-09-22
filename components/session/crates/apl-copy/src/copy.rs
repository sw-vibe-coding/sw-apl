//! `)COPY` and `)PCOPY`.

use apl_eval::Workspace;
use apl_library::{Stored, holds, text};
use apl_wsfile::{definitions, expand};

/// What a copy found: the lines to run, and the names it would not
/// overwrite.
pub struct Taken {
    /// The definitions to feed back through the session.
    pub feed: Vec<String>,
    /// Names left alone because this workspace already holds them.
    /// Only a protected copy keeps anything.
    pub kept: Vec<String>,
}

/// The names a copy brings, and the ones it leaves.
///
/// `protect` is `)PCOPY`: a name this workspace already holds is
/// left alone rather than overwritten. A name not asked for is not
/// kept -- it was never wanted -- so only a genuine collision is
/// reported. A system variable comes only when it is named.
///
/// # Errors
/// Whatever reading the workspace reports, and OBJECT NOT FOUND
/// when a name asked for is not in it.
pub fn take(ws: &Workspace, rest: &[&str], protect: bool) -> Result<(Stored, Taken), &'static str> {
    let stored = text(ws, rest)?;
    let asked = &rest[stored.used.min(rest.len())..];
    holds(&stored.apl, asked)?;
    // A group among the names asked for brings its members with it.
    let wanted = expand(&stored.apl, asked);
    let mut taken = Taken {
        feed: Vec::new(),
        kept: Vec::new(),
    };
    for (name, lines) in definitions(&stored.apl) {
        // Copying every object leaves the latent expression, `⎕LX`,
        // which is the workspace's and comes only when asked for.
        let chosen = wanted.contains(&name) || asked.is_empty() && !name.starts_with('⎕');
        if !chosen {
            continue;
        }
        if protect && held(ws, &name) {
            taken.kept.push(name);
        } else {
            taken.feed.extend(lines);
        }
    }
    Ok((stored, taken))
}

/// Whether this workspace already holds the name, as a variable, a
/// function or a group.
fn held(ws: &Workspace, name: &str) -> bool {
    ws.saved.groups.contains_key(name) || ws.get(name).is_some() || ws.is_function(name)
}
