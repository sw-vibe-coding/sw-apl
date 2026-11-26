//! Where on the card an error was found.

use apl_session::INDENT;

/// The card, and the column when it is known.
#[must_use]
pub fn located(number: usize, column: Option<usize>) -> String {
    match column {
        Some(column) => format!("CARD {number} COLUMN {column}"),
        None => format!("CARD {number}"),
    }
}

/// The card column of the caret in an error report, when the report
/// shows `statement` itself (and not a line of a function it called):
/// `columns` gives each of its characters' columns on the card.
#[must_use]
pub fn column(lines: &[String], statement: &str, columns: &[usize]) -> Option<usize> {
    let echoed = format!("{INDENT}{statement}");
    let at = lines.iter().position(|l| *l == echoed)?;
    let caret = lines.get(at + 1)?.chars().position(|c| c == '^')?;
    columns.get(caret.checked_sub(INDENT.len())?).copied()
}
