//! Defining the fields.

use apl_value::{Array, Data, Number};

use crate::screen::{Done, ROWS, Screen};

impl Screen {
    /// Command 1: the fields, a matrix of six columns (or one field as
    /// six numbers), at most 50; with `f`, field `f` alone. 2 for a
    /// field that is not whole numbers or does not fit on the screen.
    pub fn define(&mut self, f: Option<i64>, given: &Array) -> Done {
        let rows = rows(given).filter(|r| r.iter().all(|r| fits(r, self.cols)));
        match (rows, f.map(usize::try_from)) {
            (Some(rows), None) if rows.len() <= 50 => self.fields = rows,
            (Some(rows), Some(Ok(i @ 1..))) if rows.len() == 1 && i <= self.fields.len() => {
                self.fields[i - 1] = rows[0];
            }
            _ => return (2, None),
        }
        (0, None)
    }
}

/// The rows of six whole numbers `given` holds, if that is what it is.
fn rows(given: &Array) -> Option<Vec<[usize; 6]>> {
    let Data::Num(ns) = &given.data else {
        return None;
    };
    if given.shape.len() > 2 || given.shape.last() != Some(&6) {
        return None;
    }
    let whole = |n: &Number| match Number::from_f64(n.as_f64()) {
        Number::Int(i) => usize::try_from(i).ok(),
        Number::Float(_) => None,
    };
    let ns: Vec<usize> = ns.iter().map(whole).collect::<Option<_>>()?;
    ns.chunks(6)
        .map(|r| <[usize; 6]>::try_from(r).ok())
        .collect()
}

/// Whether a field is on a screen of `cols` characters a line: inside
/// it, or for a field of one line, running on no further than its end.
fn fits(&[row, col, high, wide, kind, attr]: &[usize; 6], cols: usize) -> bool {
    let sound = row * col * high * wide > 0 && (kind == 0 || kind == 2) && attr <= 255;
    let run_on = high == 1 && (row - 1) * cols + col - 1 + wide <= ROWS * cols;
    sound && (run_on || (row + high - 1 <= ROWS && col + wide - 1 <= cols))
}
