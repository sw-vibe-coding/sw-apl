//! Characters as rows: the names or lines a character array spells,
//! and lines made into a matrix.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};

/// The rows of a character array, blanks either side removed: a
/// scalar or vector is one row.
///
/// # Errors
/// DOMAIN ERROR unless `r` is characters, RANK ERROR past a matrix.
pub fn rows(r: &Array) -> AplResult<Vec<String>> {
    let (count, width) = match r.shape.as_slice() {
        [] => (1, 1),
        [n] => (1, *n),
        [m, n] => (*m, *n),
        _ => return Err(AplError::new(ErrorKind::Rank)),
    };
    let Data::Char(chars) = &r.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let row = |i: usize| chars[i * width..(i + 1) * width].iter().collect::<String>();
    Ok((0..count).map(|i| row(i).trim().to_string()).collect())
}

/// Lines as a character matrix, each padded with blanks to the
/// longest. No lines is a matrix of no rows and no columns.
#[must_use]
pub fn matrix(lines: &[String]) -> Array {
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let pad = |l: &String| {
        l.chars()
            .chain(std::iter::repeat(' '))
            .take(width)
            .collect::<Vec<_>>()
    };
    Array {
        shape: vec![lines.len(), width],
        data: Data::Char(lines.iter().flat_map(pad).collect()),
    }
}
