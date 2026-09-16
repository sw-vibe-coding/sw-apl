//! Whole arrays to transcript lines.

use apl_value::{Array, Data};

use crate::number::format_number;
use crate::wrap::{CONTINUE, column_blocks, column_widths, wrap_cells};

/// Format an array as the lines the terminal prints, with `digits`
/// significant digits and lines no wider than `width`.
#[must_use]
pub fn format_array(a: &Array, digits: usize, width: usize) -> Vec<String> {
    if a.data.count() == 0 {
        return vec![String::new()];
    }
    let (cells, sep) = cells(a, digits);
    if a.shape.len() <= 1 {
        return wrap_cells(&cells, sep, width);
    }
    let widths = column_widths(&cells, a.shape[a.shape.len() - 1]);
    let mut lines = Vec::new();
    for (b, (lo, hi)) in column_blocks(&widths, sep.len(), width)
        .into_iter()
        .enumerate()
    {
        let indent = if b == 0 { "" } else { CONTINUE };
        lines.extend(block_lines(
            &cells,
            &a.shape,
            &widths[lo..hi],
            (lo, hi),
            sep,
            indent,
        ));
    }
    lines
}

/// Every element as text, plus the separator between columns.
fn cells(a: &Array, digits: usize) -> (Vec<String>, &'static str) {
    match &a.data {
        Data::Num(v) => (v.iter().map(|&n| format_number(n, digits)).collect(), " "),
        Data::Char(v) => (v.iter().map(char::to_string).collect(), ""),
    }
}

/// The rows of one column block, with blank lines between planes.
fn block_lines(
    cells: &[String],
    shape: &[usize],
    widths: &[usize],
    (lo, hi): (usize, usize),
    sep: &str,
    indent: &str,
) -> Vec<String> {
    let cols = shape[shape.len() - 1];
    let mut lines = Vec::new();
    for (r, row) in cells.chunks(cols).enumerate() {
        lines.extend(std::iter::repeat_n(String::new(), plane_gap(r, shape)));
        let text = row[lo..hi]
            .iter()
            .zip(widths)
            .map(|(c, w)| format!("{c:>w$}"))
            .collect::<Vec<_>>()
            .join(sep);
        lines.push(format!("{indent}{text}"));
    }
    lines
}

/// Blank lines before row `r`: one per higher axis that starts over
/// at this row.
fn plane_gap(r: usize, shape: &[usize]) -> usize {
    if r == 0 {
        return 0;
    }
    let mut span = 1;
    let mut gap = 0;
    for &n in shape[1..shape.len() - 1].iter().rev() {
        span *= n;
        if r.is_multiple_of(span) {
            gap += 1;
        }
    }
    gap
}
