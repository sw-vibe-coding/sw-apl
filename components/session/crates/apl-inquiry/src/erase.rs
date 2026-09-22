//! `)ERASE`: expunging global objects by name.

use apl_eval::Workspace;

use crate::names::INCORRECT;

/// `)ERASE names`: expunge the global objects named. A group name
/// takes its members with it, which is the point of a group. A
/// function on the state indicator is left alone -- it is waiting to
/// be taken up again -- and named in the reply.
pub fn erase(ws: &mut Workspace, rest: &[&str]) -> Vec<String> {
    if rest.is_empty() {
        return vec![INCORRECT.to_string()];
    }
    let mut wanted: Vec<String> = Vec::new();
    for name in rest {
        wanted.extend(ws.saved.groups.get(*name).cloned().unwrap_or_default());
        wanted.push((*name).to_string());
    }
    let running: Vec<String> = ws.si().iter().map(|a| a.name.clone()).collect();
    let mut refused: Vec<String> = Vec::new();
    for name in wanted {
        if running.contains(&name) {
            refused.push(name);
            continue;
        }
        ws.saved.groups.remove(&name);
        ws.erase(&name);
    }
    if refused.is_empty() {
        return Vec::new();
    }
    refused.dedup();
    vec![format!("NOT ERASED: {}", refused.join(" "))]
}
