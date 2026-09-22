//! The del header: the line that opens a function definition, read
//! from text and written back out again.

use apl_ast::Defn;
use apl_lex::{Token, TokenKind, tokenize};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::scan::segments;

/// The header that follows an opening del: `NAME`, `NAME B`, or
/// `A NAME B`, each optionally with `R←` in front, then `;LOCAL`
/// names.
///
/// # Errors
/// Lexical errors, and DEFN ERROR for any other shape.
pub fn parse_header(text: &str) -> AplResult<Defn> {
    let tokens = tokenize(text, "")?;
    let mut ranges = segments(&tokens, 0, tokens.len()).into_iter();
    let (lo, hi) = ranges.next().unwrap_or((0, 0));
    let head = &tokens[lo..hi];
    let (result, head) = match head {
        [_, arrow, rest @ ..] if arrow.kind == TokenKind::Assign => {
            (Some(header_names(&head[..1])?.0), rest)
        }
        _ => (None, head),
    };
    let (name, left, right) = header_names(head)?;
    let locals = ranges
        .map(|(lo, hi)| Ok(header_names(&tokens[lo..hi])?.0))
        .collect::<AplResult<Vec<_>>>()?;
    Ok(Defn {
        name,
        result,
        left,
        right,
        locals,
        body: Vec::new(),
        locked: false,
    })
}

/// The header as it would be typed: the inverse of `parse_header`,
/// so a function can be shown or written out in del form.
#[must_use]
pub fn header_text(defn: &Defn) -> String {
    let mut out = String::new();
    if let Some(result) = &defn.result {
        out.push_str(result);
        out.push('←');
    }
    if let Some(left) = &defn.left {
        out.push_str(left);
        out.push(' ');
    }
    out.push_str(&defn.name);
    if let Some(right) = &defn.right {
        out.push(' ');
        out.push_str(right);
    }
    for local in &defn.locals {
        out.push(';');
        out.push_str(local);
    }
    out
}

/// The names in a header segment, as `(name, left, right)`: one name
/// is niladic, two are monadic, three dyadic.
fn header_names(head: &[Token]) -> AplResult<(String, Option<String>, Option<String>)> {
    let names = head
        .iter()
        .map(|t| match &t.kind {
            TokenKind::Name(n) => Ok(n.clone()),
            _ => Err(AplError::new(ErrorKind::Defn).at(t.pos)),
        })
        .collect::<AplResult<Vec<_>>>()?;
    match names.as_slice() {
        [n] => Ok((n.clone(), None, None)),
        [n, b] => Ok((n.clone(), None, Some(b.clone()))),
        [a, n, b] => Ok((n.clone(), Some(a.clone()), Some(b.clone()))),
        _ => Err(AplError::new(ErrorKind::Defn)),
    }
}
