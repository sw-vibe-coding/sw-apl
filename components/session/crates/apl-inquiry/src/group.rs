//! The group commands: `)GROUP`, `)GRP` and `)GRPS`.
//!
//! A group gives one name to a collection of names, so that they can
//! be copied or erased together, or gathered into a larger group. Its
//! members are names, not referents: a member need not exist, and
//! dispersing a group leaves whatever its members held.
//!
//! These are APL\360's. The IBM 5100 family dropped them
//! (docs/mode-b.md), so they are '68-only.

use apl_eval::Saved;

use crate::names::{INCORRECT, listing};

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

/// The names that hold a group.
#[must_use]
pub fn groups(saved: &Saved) -> Vec<String> {
    saved.groups.keys().cloned().collect()
}

/// Answer one group command, or `None` when it is not one: `)GRPS`,
/// `)GRP name` and `)GROUP`. Their listings sort and wrap as `)FNS`
/// does.
pub fn grouping(saved: &mut Saved, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    let width = saved.print.width;
    Some(match (name, rest) {
        ("GRPS", _) => {
            let mut names = groups(saved);
            names.sort_unstable();
            listing(names, rest, width)
        }
        ("GRP", [group]) => listing(members(saved, group), &[], width),
        ("GRP", _) => vec![INCORRECT.to_string()],
        ("GROUP", _) => group(saved, rest),
        _ => return None,
    })
}
