//! The marks typed under a displayed statement, and what they leave.

use apl_value::UNDERSCORE;

/// A line as positions on paper. An underscored letter is a letter
/// and a combining low line: two code points, one position.
pub(crate) fn cells(text: &str) -> Vec<String> {
    let mut cells: Vec<String> = Vec::new();
    for c in text.chars() {
        match cells.last_mut() {
            Some(last) if c == UNDERSCORE => last.push(c),
            _ => cells.push(c.to_string()),
        }
    }
    cells
}

/// `shown` with `marks` applied, the marks typed from position `at`
/// on; and the position of the first blank they put in, where the
/// carriage stops, if they put any in. A mark past the end of the
/// statement is under nothing, and does nothing.
#[must_use]
pub fn mark(shown: &str, at: usize, marks: &str) -> (String, Option<usize>) {
    let marks = cells(marks);
    let (mut edited, mut column, mut gap) = (String::new(), 0, None);
    for (i, cell) in cells(shown).into_iter().enumerate() {
        let under = i.checked_sub(at).and_then(|j| marks.get(j));
        let mut chars = under.map(|m| m.chars()).into_iter().flatten();
        match (chars.next(), chars.next()) {
            (Some('/'), None) => continue,
            (Some(m), None) if blanks(m) > 0 => {
                gap.get_or_insert(column);
                edited.push_str(&" ".repeat(blanks(m)));
                column += blanks(m);
            }
            _ => {}
        }
        edited.push_str(&cell);
        column += 1;
    }
    (edited, gap)
}

/// The blanks a mark puts before the character above it: K for a
/// digit K, 5×R for a letter, R its place in the alphabet.
fn blanks(mark: char) -> usize {
    match mark.to_ascii_uppercase() {
        d @ '0'..='9' => d as usize - '0' as usize,
        r @ 'A'..='Z' => 5 * (r as usize - 'A' as usize + 1),
        _ => 0,
    }
}
