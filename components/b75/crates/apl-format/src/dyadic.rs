//! Dyadic format: each number in a field of a given width.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use apl_numeral::layout;

/// `l⍕r`. `l` is a pair, width and precision, for every column of
/// `r`, or a pair for each column, or a precision alone, which is a
/// pair with width 0. A positive precision is decimal form with that
/// many places, a negative one scaled form with that many digits. A
/// width of 0 is the narrowest that leaves a space between numbers.
/// The result is characters: `r`'s shape, its last axis times the
/// field width (a scalar is a vector of one field).
///
/// # Errors
/// DOMAIN ERROR for characters, a width or precision that is not a
/// whole number, a negative width, or a number too wide for its
/// field; LENGTH ERROR for a left argument that is neither one number,
/// one pair, nor a pair for each column.
pub fn dyadic(l: &Array, r: &Array) -> AplResult<Array> {
    let domain = || AplError::new(ErrorKind::Domain);
    let Data::Num(values) = &r.data else {
        return Err(domain());
    };
    let cols = r.shape.last().copied().unwrap_or(1);
    let pairs = pairs(l, cols)?;
    let cells: Vec<String> = (0..values.len())
        .map(|i| layout(values[i], pairs[i % cols.max(1)].1))
        .collect();
    let widths = widths(&cells, &pairs, cols);
    let text = fields(&cells, &widths)?;
    let mut shape = if r.shape.is_empty() {
        vec![1]
    } else {
        r.shape.clone()
    };
    let last = shape.len() - 1;
    shape[last] = widths.iter().sum();
    Array::new(shape, Data::Char(text))
}

/// The (width, precision) for each column.
fn pairs(l: &Array, cols: usize) -> AplResult<Vec<(usize, i64)>> {
    let domain = || AplError::new(ErrorKind::Domain);
    let Data::Num(ns) = &l.data else {
        return Err(domain());
    };
    let whole = |n: &Number| match Number::from_f64(n.as_f64()) {
        Number::Int(i) => Ok(i),
        Number::Float(_) => Err(domain()),
    };
    let ns = ns.iter().map(whole).collect::<AplResult<Vec<i64>>>()?;
    let pair = |w: i64, p: i64| usize::try_from(w).map(|w| (w, p)).map_err(|_| domain());
    match ns.as_slice() {
        [p] => Ok(vec![(0, *p); cols.max(1)]),
        [w, p] => Ok(vec![pair(*w, *p)?; cols.max(1)]),
        many if many.len() == 2 * cols => many.chunks(2).map(|c| pair(c[0], c[1])).collect(),
        _ => Err(AplError::new(ErrorKind::Length)),
    }
}

/// The field width of each column. A width of 0 is one more than the
/// widest number it takes: the widest in the column when each column
/// has a pair of its own, the widest in the array when one pair is
/// for every column.
fn widths(cells: &[String], pairs: &[(usize, i64)], cols: usize) -> Vec<usize> {
    let cols = cols.max(1);
    let shared = pairs.windows(2).all(|w| w[0] == w[1]);
    let widest = |column: Option<usize>| {
        let mine = |i: &usize| column.is_none_or(|c| i % cols == c);
        (0..cells.len())
            .filter(mine)
            .map(|i| cells[i].chars().count())
            .max()
            .unwrap_or(0)
    };
    (0..cols)
        .map(|c| match pairs[c].0 {
            0 => widest(if shared { None } else { Some(c) }) + 1,
            w => w,
        })
        .collect()
}

/// Each cell right-justified in its column's field.
///
/// # Errors
/// DOMAIN ERROR for a cell wider than its field.
fn fields(cells: &[String], widths: &[usize]) -> AplResult<Vec<char>> {
    let mut text = Vec::new();
    for (i, cell) in cells.iter().enumerate() {
        let w = widths[i % widths.len().max(1)];
        let pad = w.checked_sub(cell.chars().count());
        let pad = pad.ok_or_else(|| AplError::new(ErrorKind::Domain))?;
        text.extend(std::iter::repeat_n(' ', pad).chain(cell.chars()));
    }
    Ok(text)
}
