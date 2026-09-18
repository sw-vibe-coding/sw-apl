//! Groups, and erasing.
//!
//! A group gives one name to a collection of names, so that they can
//! be copied or erased together, or gathered into a larger group. Its
//! members are names, not referents: a member need not exist, and
//! dispersing a group leaves whatever its members held.

use apl_eval::{Saved, Workspace};

use crate::names::INCORRECT;

/// The reply when the first name of a `)GROUP` already holds a
/// function or a variable, as the manual's trouble-report table
/// words it.
const IN_USE: &str = "NOT GROUPED, NAME IN USE";

/// `)GROUP name [members]`: the first name becomes a group with the
/// rest as members, superseding any group of that name. The group
/// name used again among the members adds to what it already held.
/// One name alone disperses a group of that name. The reply is
/// nothing, unless the name is a function's or a variable's.
pub fn group(saved: &mut Saved, rest: &[&str]) -> Vec<String> {
    let [name, members @ ..] = rest else {
        return vec![INCORRECT.to_string()];
    };
    if saved.vars.contains_key(*name) || saved.funcs.contains_key(*name) {
        return vec![IN_USE.to_string()];
    }
    if members.is_empty() {
        saved.groups.remove(*name);
        return Vec::new();
    }
    let was = saved.groups.get(*name).cloned().unwrap_or_default();
    let gathered = members
        .iter()
        .flat_map(|m| {
            if m == name {
                was.clone()
            } else {
                vec![(*m).to_string()]
            }
        })
        .collect();
    saved.groups.insert((*name).to_string(), gathered);
    Vec::new()
}

/// The names a group holds, or nothing when no group holds that
/// name. A group listed among the members is not expanded: `)GRP`
/// shows what was written.
#[must_use]
pub fn members(saved: &Saved, name: &str) -> Vec<String> {
    saved.groups.get(name).cloned().unwrap_or_default()
}

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
        wanted.extend(members(&ws.saved, name));
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
