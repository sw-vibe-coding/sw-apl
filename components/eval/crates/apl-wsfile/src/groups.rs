//! Taking a workspace file apart again.
//!
//! `)LOAD` does not need this -- a workspace file is APL, so loading
//! it is running it. `)COPY` does: it takes the definitions and
//! leaves the commands, because running the file would apply its
//! settings and change the index origin under code already written.

use apl_modes::{modes, retag};

use crate::write::{DIRECTIVE, PREAMBLE, rot13};

/// A workspace file as APL, whichever way it was written. A plain
/// file is itself; an obscured one -- which is what a workspace
/// holding a locked function is written as -- is what lies under the
/// preamble, unscrambled. The two are told apart by the first line.
///
/// Everything that reads a workspace goes through here, so nothing
/// above it needs to know the two forms exist.
#[must_use]
pub fn plain(text: &str) -> String {
    if !text.starts_with(PREAMBLE[0]) {
        return text.to_string();
    }
    // The modes line follows the preamble in the clear, and goes back
    // where a plain file has it.
    let mut body = text.lines().skip(PREAMBLE.len()).peekable();
    let clear = body.next_if(|l| l.starts_with(&format!("{DIRECTIVE}MODES")));
    let body: Vec<&str> = body.collect();
    let revealed = rot13(&(body.join("\n") + "\n"));
    match clear {
        Some(_) => retag(&revealed, modes(text)),
        None => revealed,
    }
}

/// The file's definitions, each as the name it defines and the lines
/// that define it, in the order they appear. Commands and directives
/// are left out, which is what makes copying different from loading.
#[must_use]
pub fn definitions(text: &str) -> Vec<(String, Vec<String>)> {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    let mut open = false;
    for line in text.lines() {
        if open {
            if let Some(group) = groups.last_mut() {
                group.1.push(line.to_string());
            }
            open = !matches!(line.trim(), "∇" | "⍫");
        } else if let Some(header) = line.strip_prefix('∇') {
            groups.push((function_name(header), vec![line.to_string()]));
            open = true;
        } else if let Some(rest) = line.strip_prefix(")GROUP ") {
            let name = rest.split_whitespace().next().unwrap_or_default();
            groups.push((name.to_string(), vec![line.to_string()]));
        } else if let Some((name, _)) = line.split_once('←') {
            groups.push((name.trim().to_string(), vec![line.to_string()]));
        }
    }
    groups
}

/// The names to copy, with the members of any group among them
/// added. Copying a group brings what it gathers -- that is what a
/// group is for -- and the file's own `)GROUP` line is where its
/// members are written down.
#[must_use]
pub fn expand(text: &str, wanted: &[&str]) -> Vec<String> {
    let mut names: Vec<String> = wanted.iter().map(|n| (*n).to_string()).collect();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix(")GROUP ") else {
            continue;
        };
        let mut words = rest.split_whitespace();
        let group = words.next().unwrap_or_default();
        if names.iter().any(|n| n == group) {
            names.extend(words.map(String::from));
        }
    }
    names
}

/// The name a del header defines: the one before the right argument,
/// after any result and left argument.
fn function_name(header: &str) -> String {
    let head = header.split(';').next().unwrap_or(header);
    let head = head.split('←').next_back().unwrap_or(head);
    // The name is the first word of a niladic or monadic header and
    // the second of a dyadic one: NAME, NAME B, or A NAME B.
    let names: Vec<&str> = head.split_whitespace().collect();
    let name = match names.len() {
        1 | 2 => names.first(),
        3 => names.get(1),
        _ => None,
    };
    name.unwrap_or(&"").to_string()
}
