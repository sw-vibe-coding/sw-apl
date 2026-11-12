//! A saved workspace as the lines that would rebuild it.

use apl_ast::Defn;
use apl_modes::{Modes, render};
use apl_scan::header_text;
use apl_uses::runs_in;
use apl_value::FUZZ;
use apl_workspace::Saved;

use crate::literal::literal;
use crate::obscure::{PREAMBLE, rot13};

/// The prefix of a line that is a comment to APL and an instruction
/// to `)LOAD`: what APL has no way of saying about itself.
pub const DIRECTIVE: &str = "⍝!";

/// A saved workspace as APL you could have typed: the variables as
/// assignments, the functions as del definitions, and what APL cannot
/// say for itself as directives -- the modes it runs in second, where
/// a reader and `)LIB` look for them, then when it was saved, its
/// random link and its settings. A latent expression is the `⎕LX`
/// assignment that sets it, which only (B) runs, as only (B) loads a
/// workspace that has one. `when` is the moment it was saved, which
/// the loader reports.
///
/// A file holds the globals, so it is written from the workspace
/// unwound: under a suspension a local sits where the global it hid
/// belongs. A comparison tolerance other than the fixed one is a
/// `CT` directive, which only (B) can have set.
///
/// Names come out sorted, so the same workspace writes the same bytes
/// every time and a saved file is worth keeping in git.
///
/// A workspace holding a locked function is obscured instead. The
/// writer has to put a locked body into the file in full -- there is
/// nowhere else for it -- and a text file would hand it to anyone who
/// opened it, which is the one thing locking is for. See `rot13`:
/// this is obscuring and not encryption. The modes line stays in the
/// clear, after the preamble, so the workspace is listed in the right
/// modes without being revealed.
#[must_use]
pub fn write(saved: &Saved, when: &str) -> String {
    let saved = &saved.unwound();
    let mut lines = heading(saved, when, runs_in(saved));
    lines.extend(saved.latent.iter().map(|lx| format!("⎕LX←{}", literal(lx))));
    lines.extend(objects(saved, definition));
    if !saved.funcs.values().any(|f| f.locked) {
        return lines.join("\n") + "\n";
    }
    let modes = lines.remove(1);
    let hidden = rot13(&(lines.join("\n") + "\n"));
    PREAMBLE.join("\n") + "\n" + &modes + "\n" + &hidden
}

/// What a file says first, of a workspace that runs in `modes`: what
/// it is, the modes line, when it was saved, its random link and its
/// settings as directives, a tolerance other than the fixed one, and
/// the name it was saved under.
#[must_use]
pub fn heading(saved: &Saved, when: &str, modes: Modes) -> Vec<String> {
    let mut lines = vec![
        "⍝ sw-apl workspace. Re-executable APL: loading it runs it.".to_string(),
        format!("{DIRECTIVE}MODES {}", render(modes)),
        format!("{DIRECTIVE}SAVED {when}"),
        format!("{DIRECTIVE}LINK {}", saved.env.link),
        format!("{DIRECTIVE}ORIGIN {}", saved.env.io),
        format!("{DIRECTIVE}DIGITS {}", saved.print.digits),
        format!("{DIRECTIVE}WIDTH {}", saved.print.width),
    ];
    let ct = saved.env.ct;
    lines.extend((ct.to_bits() != FUZZ.to_bits()).then(|| format!("{DIRECTIVE}CT {ct}")));
    lines.extend(saved.id.iter().map(|id| format!(")WSID {id}")));
    lines
}

/// What the workspace holds, in the order a reader wants it:
/// variables, then functions, each written by `define`, then the
/// groups as the commands that would gather them again. Groups come
/// last because a group is only names -- it does not matter whether
/// what it names is written yet.
///
/// Each kind is sorted, so the same workspace writes the same bytes
/// every time.
#[must_use]
pub fn objects(saved: &Saved, define: fn(&Defn) -> Vec<String>) -> Vec<String> {
    let mut lines = Vec::new();
    let mut names: Vec<&String> = saved.vars.keys().collect();
    names.sort();
    lines.extend(
        names
            .iter()
            .map(|n| format!("{n}←{}", literal(&saved.vars[*n]))),
    );
    let mut functions: Vec<&String> = saved.funcs.keys().collect();
    functions.sort();
    for name in functions {
        lines.extend(define(&saved.funcs[name]));
    }
    let mut groups: Vec<&String> = saved.groups.keys().collect();
    groups.sort();
    let gathered = groups.iter();
    lines.extend(gathered.map(|n| format!(")GROUP {n} {}", saved.groups[*n].join(" "))));
    lines
}

/// One function in del form, closed the way it was closed: del-tilde
/// when it is locked, which is the only way a file can carry a locked
/// function at all. A text workspace therefore shows what the editor
/// will not; locking guards the editor, not the file.
fn definition(defn: &Defn) -> Vec<String> {
    let mut lines = vec![format!("∇{}", header_text(defn))];
    lines.extend(defn.body.iter().cloned());
    lines.push(if defn.locked { "⍫" } else { "∇" }.to_string());
    lines
}
