//! Turning what a statement produced into transcript lines.

use apl_display::format_array;
use apl_value::{AplError, Array};

use crate::console::INDENT;
use crate::output::{Output, Print, Shown};

/// One statement's output as transcript lines.
#[must_use]
pub fn render(out: &Output, print: Print) -> Vec<String> {
    match out {
        Output::Nothing | Output::Branch(_) => Vec::new(),
        Output::Value(value) => format_array(value, print.digits, print.width),
        Output::Mixed(parts) => mixed_lines(parts, print),
        Output::Bare(text) => vec![text.clone()],
    }
}

/// A run of outputs as transcript lines. Characters written with `⍞←`
/// leave the line open, so whatever is shown next carries on where
/// they left off instead of starting a line of its own -- including
/// an answer typed at a read, which is why the flag comes back too.
#[must_use]
pub fn render_all(outs: &[Output], print: Print) -> Shown {
    let mut lines: Vec<String> = Vec::new();
    let mut open = false;
    for out in outs {
        let mut next = render(out, print).into_iter().peekable();
        if next.peek().is_none() {
            continue; // Shows nothing, so it neither joins nor closes.
        }
        // A let chain, not a tuple: `next.next()` must not run unless
        // the line before it was left open, or it swallows a line.
        if open
            && let Some(last) = lines.last_mut()
            && let Some(first) = next.next()
        {
            last.push_str(&first);
        }
        lines.extend(next);
        open = matches!(out, Output::Bare(_));
    }
    Shown { lines, open }
}

/// Mixed output: single-line parts are printed side by side with no
/// separator; when any part spans lines, the parts follow each other.
fn mixed_lines(parts: &[Array], print: Print) -> Vec<String> {
    let blocks: Vec<Vec<String>> = parts
        .iter()
        .map(|p| format_array(p, print.digits, print.width))
        .collect();
    if blocks.iter().all(|b| b.len() == 1) {
        return vec![blocks.iter().map(|b| b[0].as_str()).collect()];
    }
    blocks.concat()
}

/// APL\360 error display: the error's name, the statement it came
/// from, and a caret under the point of detection. A statement inside
/// a defined function is headed by the function and the line instead
/// of the six-space indent.
#[must_use]
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
