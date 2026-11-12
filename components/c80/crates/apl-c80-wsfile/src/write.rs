//! The (C) form of a workspace file.

use apl_ast::Defn;
use apl_c80_keys::{named, shown};
use apl_modes::{Mode, Modes};
use apl_scan::header_text;
use apl_value::{Array, Data};
use apl_workspace::Saved;
use apl_wsfile::{heading, literal};

/// `saved` as a file for `mode` alone, saved at `when`, in APL80's
/// characters: each variable as its assignment, each function as
/// `)DEF`, its header, its lines as they were typed, and BREAK; the
/// latent expression as `)AUTO`. APL80 has no locking, so nothing in
/// it is obscured.
#[must_use]
pub fn write(saved: &Saved, when: &str, mode: Mode) -> String {
    let saved = &saved.unwound();
    let mut lines = heading(saved, when, Modes::only(mode));
    // What it is, as a comment APL80 can type: `c`, in capitals.
    lines[0] = "c SW-APL WORKSPACE, AS APL80 TYPES IT: LOADING IT RUNS IT.".to_string();
    lines.extend(saved.latent.iter().map(|lx| format!(")AUTO {}", text(lx))));
    lines.extend(variables(saved));
    let mut functions: Vec<&String> = saved.funcs.keys().collect();
    functions.sort();
    for name in functions {
        lines.extend(definition(&saved.funcs[name]));
    }
    lines.join("\n") + "\n"
}

/// Each variable as the assignment that sets it, sorted by name.
fn variables(saved: &Saved) -> Vec<String> {
    let mut names: Vec<&String> = saved.vars.keys().collect();
    names.sort();
    let assigned = names
        .iter()
        .map(|n| format!("{n}←{}", literal(&saved.vars[*n])));
    assigned.map(|line| shown(&line)).collect()
}

/// One function as it is typed: `)DEF`, the header, the lines, and
/// BREAK.
fn definition(defn: &Defn) -> Vec<String> {
    let mut lines = vec![format!(")DEF {}", named(&header_text(defn)))];
    lines.extend(defn.body.iter().cloned());
    lines.push(String::new());
    lines
}

/// A character vector as its text, as it was typed.
fn text(a: &Array) -> String {
    match &a.data {
        Data::Char(cs) => cs.iter().collect(),
        Data::Num(_) => String::new(),
    }
}
