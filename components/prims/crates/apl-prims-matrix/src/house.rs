//! Least squares by Householder QR.
//!
//! The reflections are applied to the basis and to the right-hand
//! sides together, so the factorisation is never stored: what is
//! left of the basis is R, and what is left of the right-hand sides
//! is Q transposed times them, which is what the back substitution
//! wants.
//!
//! Not the normal equations. Forming `(⍉B)+.×B` and solving that is
//! shorter and squares the condition number, which loses half the
//! digits in the answer -- and the overdetermined fit, where that
//! matters most, is the reason this primitive exists.

use apl_value::{AplError, AplResult, ErrorKind};

/// Solve `basis X = sides` in the least squares sense. `basis` is
/// `rows` by `cols` and `sides` is `rows` by `wide`, both row-major;
/// the result is `cols` by `wide`.
///
/// # Errors
/// DOMAIN ERROR when the basis is rank deficient, which is what
/// "singular" means once a matrix need not be square.
pub fn solve(
    basis: &[f64],
    rows: usize,
    cols: usize,
    sides: &[f64],
    wide: usize,
) -> AplResult<Vec<f64>> {
    let (mut upper, mut turned) = (basis.to_vec(), sides.to_vec());
    for step in 0..cols {
        reflect(&mut upper, (rows, cols), &mut turned, wide, step);
    }
    back_substitute(&upper, cols, &turned, wide)
}

/// One Householder reflection: zero the column below the diagonal at
/// `step`, and turn the right-hand sides by the same reflection.
fn reflect(upper: &mut [f64], size: (usize, usize), turned: &mut [f64], wide: usize, step: usize) {
    let (rows, cols) = size;
    let at = |row: usize| row * cols + step;
    let norm = (step..rows).map(|row| upper[at(row)].powi(2)).sum::<f64>();
    let norm = norm.sqrt();
    if norm == 0.0 {
        return;
    }
    // Reflect onto whichever sign is further from the column, so
    // that forming the mirror cannot cancel its leading element.
    let along = if upper[at(step)] > 0.0 { -norm } else { norm };
    let mut mirror = vec![0.0; rows];
    mirror[step] = upper[at(step)] - along;
    for row in step + 1..rows {
        mirror[row] = upper[at(row)];
    }
    let scale = mirror.iter().map(|x| x * x).sum::<f64>();
    if scale == 0.0 {
        return;
    }
    turn(upper, (rows, cols), &mirror, scale, step);
    turn(turned, (rows, wide), &mirror, scale, 0);
}

/// `x - (2 (v⊥x) ÷ v⊥v) v` down every column from `first` on.
fn turn(cells: &mut [f64], size: (usize, usize), mirror: &[f64], scale: f64, first: usize) {
    let (rows, width) = size;
    for col in first..width {
        let at = |row: usize| row * width + col;
        let dot: f64 = (0..rows).map(|row| mirror[row] * cells[at(row)]).sum();
        let factor = 2.0 * dot / scale;
        for row in 0..rows {
            cells[at(row)] -= factor * mirror[row];
        }
    }
}

/// Solve the triangular `R X = Q'A` from the bottom up.
///
/// A diagonal entry small beside the largest is not a number the
/// factorisation computed: it is what is left of one after the
/// cancellation, and dividing by it would amplify rounding into the
/// answer. The threshold is machine epsilon scaled by the size of
/// the problem, which is the usual rank test; it is deliberately not
/// the comparison tolerance, which is about values a user typed
/// looking equal rather than about a factorisation losing a digit.
fn back_substitute(upper: &[f64], cols: usize, turned: &[f64], wide: usize) -> AplResult<Vec<f64>> {
    let diagonal = |row: usize| upper[row * cols + row].abs();
    let largest = (0..cols).map(diagonal).fold(0.0, f64::max);
    #[allow(clippy::cast_precision_loss)]
    let tolerance = f64::EPSILON * (cols.max(wide) as f64) * largest;
    if largest == 0.0 || (0..cols).any(|row| diagonal(row) <= tolerance) {
        return Err(AplError::new(ErrorKind::Domain));
    }
    let mut found = vec![0.0; cols * wide];
    for col in 0..wide {
        for row in (0..cols).rev() {
            let known: f64 = (row + 1..cols)
                .map(|k| upper[row * cols + k] * found[k * wide + col])
                .sum();
            found[row * wide + col] = (turned[row * wide + col] - known) / upper[row * cols + row];
        }
    }
    Ok(found)
}
