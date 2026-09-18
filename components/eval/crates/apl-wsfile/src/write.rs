//! A saved workspace as the lines that would rebuild it.

use apl_scan::header_text;
use apl_workspace::Saved;

use crate::literal::literal;

/// The prefix of a line that is a comment to APL and an instruction
/// to `)LOAD`: what APL has no way of saying about itself.
pub const DIRECTIVE: &str = "⍝!";

/// A saved workspace as APL you could have typed: the settings as the
/// commands that set them, the variables as assignments, the
/// functions as del definitions. `when` is the moment it was saved,
/// which the loader reports.
///
/// Names come out sorted, so the same workspace writes the same bytes
/// every time and a saved file is worth keeping in git.
#[must_use]
pub fn write(saved: &Saved, when: &str) -> String {
    let mut lines = vec![
        "⍝ sw-apl workspace. Re-executable APL: loading it runs it.".to_string(),
        format!("{DIRECTIVE}SAVED {when}"),
        format!("{DIRECTIVE}LINK {}", saved.env.link),
    ];
    if let Some(id) = &saved.id {
        lines.push(format!(")WSID {id}"));
    }
    lines.push(format!(")ORIGIN {}", saved.env.io));
    lines.push(format!(")DIGITS {}", saved.print.digits));
    lines.push(format!(")WIDTH {}", saved.print.width));
    let mut names: Vec<&String> = saved.vars.keys().collect();
    names.sort();
    for name in names {
        lines.push(format!("{name}←{}", literal(&saved.vars[name])));
    }
    let mut functions: Vec<&String> = saved.funcs.keys().collect();
    functions.sort();
    for name in functions {
        lines.extend(definition(&saved.funcs[name]));
    }
    lines.extend(gatherings(saved));
    lines.join("\n") + "\n"
}

/// The groups, as the commands that would gather them again. They
/// come last because a group is only names: it does not matter
/// whether what it names has been written yet.
fn gatherings(saved: &Saved) -> Vec<String> {
    let mut names: Vec<&String> = saved.groups.keys().collect();
    names.sort();
    names
        .iter()
        .map(|name| format!(")GROUP {name} {}", saved.groups[*name].join(" ")))
        .collect()
}

/// One function in del form, closed the way it was closed: del-tilde
/// when it is locked, which is the only way a file can carry a locked
/// function at all. A text workspace therefore shows what the editor
/// will not; locking guards the editor, not the file.
fn definition(defn: &apl_ast::Defn) -> Vec<String> {
    let mut lines = vec![format!("∇{}", header_text(defn))];
    lines.extend(defn.body.iter().cloned());
    lines.push(if defn.locked { "⍫" } else { "∇" }.to_string());
    lines
}
