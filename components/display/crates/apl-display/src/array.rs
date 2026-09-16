//! Whole arrays to transcript lines.

use apl_value::{Array, Data};

use crate::number::format_number;

/// Format an array as the lines the terminal prints.
#[must_use]
pub fn format_array(a: &Array, pp: usize) -> Vec<String> {
    let cells: Vec<String> = match &a.data {
        Data::Num(v) => v.iter().map(|&n| format_number(n, pp)).collect(),
        Data::Char(v) => v.iter().map(char::to_string).collect(),
    };
    match a.shape.len() {
        0 | 1 => vec![cells.join(" ")],
        _ => format_matrix(&cells, &a.shape, matches!(a.data, Data::Char(_))),
    }
}

/// Rows of right-aligned columns (character matrices are not padded).
fn format_matrix(cells: &[String], shape: &[usize], chars: bool) -> Vec<String> {
    let cols = *shape.last().unwrap_or(&0);
    if cols == 0 {
        return vec![String::new(); shape.iter().product::<usize>()];
    }
    let widths = column_widths(cells, cols);
    cells
        .chunks(cols)
        .map(|row| {
            row.iter()
                .zip(&widths)
                .map(|(c, w)| if chars { c.clone() } else { format!("{c:>w$}") })
                .collect::<Vec<_>>()
                .join(if chars { "" } else { " " })
        })
        .collect()
}

fn column_widths(cells: &[String], cols: usize) -> Vec<usize> {
    let mut widths = vec![0; cols];
    for (i, c) in cells.iter().enumerate() {
        widths[i % cols] = widths[i % cols].max(c.chars().count());
    }
    widths
}
