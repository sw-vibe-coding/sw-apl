//! Turning what a statement produced into transcript lines.

use apl_display::format_array;
use apl_eval::{Activation, Output};
use apl_value::{AplError, Array};

use crate::session::INDENT;

/// One statement's output as transcript lines.
pub fn render(out: &Output, digits: usize, width: usize) -> Vec<String> {
    match out {
        Output::Nothing | Output::Branch(_) => Vec::new(),
        Output::Value(value) => format_array(value, digits, width),
        Output::Mixed(parts) => mixed_lines(parts, digits, width),
    }
}

/// APL\360 error display: the error's name, the statement it came
/// from, and a caret under the point of detection. A statement inside
/// a defined function is headed by the function and the line instead
/// of the six-space indent.
pub fn error_lines(err: &AplError, line: &str) -> Vec<String> {
    let caret = err.caret.unwrap_or(0);
    let (head, statement) = match &err.context {
        Some(context) => (
            format!("{}[{}]  ", context.function, context.line),
            context.statement.as_str(),
        ),
        None => (INDENT.to_string(), line),
    };
    let indent = " ".repeat(head.chars().count() + caret);
    vec![
        err.kind.to_string(),
        format!("{head}{statement}"),
        format!("{indent}^"),
    ]
}

/// The state indicator, innermost first: each function with the line
/// it stopped on, starred when it is the one the user can take up
/// again rather than a caller waiting on it. `verbose` adds the names
/// it made local, which is what `)SIV` shows.
pub fn si_lines(stack: &[Activation], verbose: bool) -> Vec<String> {
    let entries: Vec<(String, &[String])> = stack
        .iter()
        .rev()
        .map(|a| {
            let star = if a.suspended { "*" } else { "" };
            (format!("{}[{}]{star}", a.name, a.line), a.locals.as_slice())
        })
        .collect();
    let column = entries.iter().map(|(e, _)| e.chars().count()).max();
    entries
        .iter()
        .map(|(entry, locals)| match column {
            Some(width) if verbose && !locals.is_empty() => {
                format!("{entry:width$}  {}", locals.join(" "))
            }
            _ => entry.clone(),
        })
        .collect()
}

/// Mixed output: single-line parts are printed side by side with no
/// separator; when any part spans lines, the parts follow each other.
fn mixed_lines(parts: &[Array], digits: usize, width: usize) -> Vec<String> {
    let blocks: Vec<Vec<String>> = parts
        .iter()
        .map(|p| format_array(p, digits, width))
        .collect();
    if blocks.iter().all(|b| b.len() == 1) {
        return vec![blocks.iter().map(|b| b[0].as_str()).collect()];
    }
    blocks.concat()
}
