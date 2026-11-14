//! A branch on an indexed list of labels, which APL80 cannot write:
//! round brackets index a variable or a string, not a parenthesized
//! expression (docs/mode-c.md), so the list goes through a local.

/// `lines`, ported, with each `→(A,B,C)(I)` as `D.B←A,B,C` and then
/// `→D.B(I)`, and `D.B` made local to every function that does it.
pub(crate) fn branches(lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut def = None;
    let mut local = false;
    for line in lines {
        if line.starts_with(")DEF ") {
            localize(&mut out, def, local);
            (def, local) = (Some(out.len()), false);
            out.push(line);
        } else if let Some((label, list, index)) = split(&line) {
            out.push(format!("{label}D.B←{list}"));
            out.push(format!("→D.B{index}"));
            local = true;
        } else {
            out.push(line);
        }
    }
    localize(&mut out, def, local);
    out
}

/// Make `D.B` local to the definition at `def`, if it branched so.
fn localize(out: &mut [String], def: Option<usize>, local: bool) {
    if let (true, Some(at)) = (local, def) {
        out[at].push_str(";D.B");
    }
}

/// A line `LABEL:→(LIST)(INDEX)` as its label (with the colon, or
/// nothing), the list, and the index in its brackets.
fn split(line: &str) -> Option<(&str, &str, &str)> {
    let at = line.find("→(")?;
    let label = &line[..at];
    if !(label.is_empty() || label.ends_with(':')) {
        return None;
    }
    let body = &line[at + '→'.len_utf8()..];
    let close = closing(body)?;
    let index = &body[close + 1..];
    (index.starts_with('(') && closing(index)? == index.len() - 1).then_some((
        label,
        &body[1..close],
        index,
    ))
}

/// Where the bracket that opens `text` closes.
fn closing(text: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in text.char_indices() {
        depth += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if depth == 0 {
            return Some(i);
        }
    }
    None
}
