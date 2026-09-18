//! A saved workspace as the lines that would rebuild it.

use apl_scan::header_text;
use apl_workspace::Saved;

use crate::literal::literal;

/// The prefix of a line that is a comment to APL and an instruction
/// to `)LOAD`: what APL has no way of saying about itself.
pub const DIRECTIVE: &str = "⍝!";

/// What an obscured workspace begins with, before anything
/// scrambled. `)LOAD` tells the two forms apart by the first line;
/// the other two are there for the person who runs the file as a
/// program, so it says what it is and signs off instead of printing
/// a screen of CHARACTER ERRORs.
pub const PREAMBLE: [&str; 3] = [
    "⍝!OBSCURED sw-apl workspace. Rot-13, not encryption: docs/workspaces.md.",
    "'OBSCURED WORKSPACE. )LOAD IT -- IT CANNOT BE RUN AS A PROGRAM.'",
    ")OFF",
];

/// A saved workspace as APL you could have typed: the settings as the
/// commands that set them, the variables as assignments, the
/// functions as del definitions. `when` is the moment it was saved,
/// which the loader reports.
///
/// Names come out sorted, so the same workspace writes the same bytes
/// every time and a saved file is worth keeping in git.
///
/// A workspace holding a locked function is obscured instead. The
/// writer has to put a locked body into the file in full -- there is
/// nowhere else for it -- and a text file would hand it to anyone who
/// opened it, which is the one thing locking is for. See `rot13`:
/// this is obscuring and not encryption.
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
    lines.extend(objects(saved));
    let text = lines.join("\n") + "\n";
    if !saved.funcs.values().any(|f| f.locked) {
        return text;
    }
    PREAMBLE.join("\n") + "\n" + &rot13(&text)
}

/// Letters moved thirteen places, everything else left alone.
///
/// This is obscuring, not encryption, and the documentation must not
/// suggest otherwise: anyone who means to read a rot-13 file can.
/// What it stops is reading a locked function by accident or by
/// curiosity, which is what APL\360's binary workspaces stopped.
///
/// Rot-13 is its own inverse, so this one function serves both
/// directions: writing scrambles and `plain` unscrambles with the
/// same call.
#[must_use]
pub fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            let first = match c {
                'A'..='Z' => b'A',
                'a'..='z' => b'a',
                _ => return c,
            };
            // Both arms above are ASCII, so this cannot overflow.
            char::from(first + ((c as u8) - first + 13) % 26)
        })
        .collect()
}

/// What the workspace holds, in the order a reader wants it:
/// variables, then functions, then the groups as the commands that
/// would gather them again. Groups come last because a group is only
/// names -- it does not matter whether what it names is written yet.
///
/// Each kind is sorted, so the same workspace writes the same bytes
/// every time.
fn objects(saved: &Saved) -> Vec<String> {
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
        lines.extend(definition(&saved.funcs[name]));
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
fn definition(defn: &apl_ast::Defn) -> Vec<String> {
    let mut lines = vec![format!("∇{}", header_text(defn))];
    lines.extend(defn.body.iter().cloned());
    lines.push(if defn.locked { "⍫" } else { "∇" }.to_string());
    lines
}
