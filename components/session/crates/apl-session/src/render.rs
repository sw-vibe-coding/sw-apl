//! Turning what a statement produced into transcript lines.

use apl_display::format_array;
use apl_eval::Output;
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

/// APL\360 error display: name, echoed statement, caret line.
pub fn error_lines(err: &AplError, line: &str) -> Vec<String> {
    let caret = err.caret.unwrap_or(0);
    vec![
        err.kind.to_string(),
        format!("{INDENT}{line}"),
        format!("{INDENT}{}^", " ".repeat(caret)),
    ]
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
