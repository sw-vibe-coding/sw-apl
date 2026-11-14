//! The port, line by line.

use apl_c80_keys::shown;

/// `text`, a workspace file in glyphs, as (C) would save it: the
/// heading for (C) alone, with its digits and width; each definition
/// as `)DEF`, its lines, and the empty line that stands for BREAK;
/// everything else in APL80's characters.
#[must_use]
pub fn port(text: &str) -> String {
    let mut open = false;
    let mut lines = Vec::new();
    for (n, line) in text.lines().enumerate() {
        lines.push(match (n, line) {
            (0, _) => "c SW-APL WORKSPACE, AS APL80 TYPES IT: LOADING IT RUNS IT.".to_string(),
            (_, l) if l.starts_with("⍝!") => heading(l),
            (_, "∇") if open => {
                open = false;
                String::new()
            }
            (_, l) if !open && l.starts_with('∇') => {
                open = true;
                format!(")DEF {}", typed(&l['∇'.len_utf8()..]))
            }
            (_, l) => typed(l),
        });
    }
    crate::branch::branches(lines).join("\n") + "\n"
}

/// A directive, with (C)'s modes, digits and width in place of the
/// file's.
fn heading(line: &str) -> String {
    let (word, _) = line.split_once(' ').unwrap_or((line, ""));
    match word {
        "⍝!MODES" => "⍝!MODES (C)".to_string(),
        "⍝!DIGITS" => "⍝!DIGITS 6".to_string(),
        "⍝!WIDTH" => "⍝!WIDTH 64".to_string(),
        _ => line.to_string(),
    }
}

/// A line as APL80 types it: a `∆` that begins a name as `D.`, then
/// every glyph as APL80's character.
pub(crate) fn typed(line: &str) -> String {
    let mut renamed = String::new();
    let mut after_name = false;
    for c in line.chars() {
        if c == '∆' && !after_name {
            renamed.push_str("D⍙");
        } else {
            renamed.push(c);
        }
        after_name = c.is_alphanumeric() || matches!(c, '∆' | '⍙' | '_' | '¯');
    }
    shown(&brackets(&renamed))
}

/// Brackets as APL80 writes them: an index in round brackets, `A(I)`,
/// and an axis after an operator or a function as `d` before the
/// argument, `+/1dM` for `+/[1]M`.
fn brackets(line: &str) -> String {
    let mut out = String::new();
    let mut axis = Vec::new();
    let mut last = ' ';
    for c in line.chars() {
        match c {
            '[' => {
                let is_axis = matches!(last, '/' | '\\' | '⌿' | '⍀' | '⌽' | '⊖' | ',' | '↑' | '↓');
                axis.push(is_axis);
                if !is_axis {
                    out.push('(');
                }
            }
            ']' => out.push(if axis.pop() == Some(true) { 'd' } else { ')' }),
            c => out.push(c),
        }
        last = c;
    }
    out
}
