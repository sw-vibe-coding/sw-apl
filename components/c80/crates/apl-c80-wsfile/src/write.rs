//! The (C) form of a workspace file.

use apl_ast::Defn;
use apl_modes::{Mode, Modes};
use apl_scan::header_text;
use apl_value::{Array, Data};
use apl_workspace::Saved;
use apl_wsfile::{heading, objects};

/// `saved` as a file for `mode` alone, saved at `when`. APL80 has no
/// locking, so nothing in it is obscured.
#[must_use]
pub fn write(saved: &Saved, when: &str, mode: Mode) -> String {
    let saved = &saved.unwound();
    let mut lines = heading(saved, when, Modes::only(mode));
    lines.extend(saved.latent.iter().map(|lx| format!(")AUTO {}", text(lx))));
    lines.extend(objects(saved, definition));
    lines.join("\n") + "\n"
}

/// One function as it is typed: `)DEF`, the header, the lines, and
/// BREAK.
fn definition(defn: &Defn) -> Vec<String> {
    let mut lines = vec![format!(")DEF {}", header_text(defn))];
    lines.extend(defn.body.iter().cloned());
    lines.push(String::new());
    lines
}

/// A character vector as its text.
fn text(a: &Array) -> String {
    match &a.data {
        Data::Char(cs) => cs.iter().collect(),
        Data::Num(_) => String::new(),
    }
}
