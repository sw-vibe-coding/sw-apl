//! `)WIDTH` wrapping. Continuation lines are indented six spaces.

/// Indent for continuation lines.
pub const CONTINUE: &str = "      ";

/// Lay out `cells` on lines no wider than `width`, breaking only
/// between cells (a single cell wider than the width stands alone).
pub fn wrap_cells(cells: &[String], sep: &str, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    for cell in cells {
        let candidate = if line.is_empty() {
            cell.clone()
        } else {
            format!("{line}{sep}{cell}")
        };
        if candidate.chars().count() <= width || line.is_empty() {
            line = candidate;
        } else {
            lines.push(std::mem::replace(&mut line, format!("{CONTINUE}{cell}")));
        }
    }
    lines.push(line);
    lines
}

/// The widest cell in each of `cols` columns.
pub fn column_widths(cells: &[String], cols: usize) -> Vec<usize> {
    let mut widths = vec![0; cols];
    for (i, c) in cells.iter().enumerate() {
        widths[i % cols] = widths[i % cols].max(c.chars().count());
    }
    widths
}

/// Group columns of the given widths into blocks that fit `width`
/// (later blocks leave room for the continuation indent). Returns
/// `(first_column, end_column)` ranges.
pub fn column_blocks(widths: &[usize], sep_len: usize, width: usize) -> Vec<(usize, usize)> {
    let mut blocks = Vec::new();
    let mut start = 0;
    while start < widths.len() {
        let limit = if blocks.is_empty() {
            width
        } else {
            width.saturating_sub(CONTINUE.len())
        };
        let mut end = start;
        let mut used = 0;
        while end < widths.len() {
            let next = used + widths[end] + if end > start { sep_len } else { 0 };
            if next > limit && end > start {
                break;
            }
            used = next;
            end += 1;
        }
        blocks.push((start, end));
        start = end;
    }
    blocks
}
