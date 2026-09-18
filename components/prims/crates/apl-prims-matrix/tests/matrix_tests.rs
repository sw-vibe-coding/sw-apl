//! Matrix inverse and matrix divide.
//!
//! Domino is not in the August 1968 manual: it was added to APL\360
//! in 1970. The rules followed here are the ones the APLX Language
//! Manual states for the same lineage -- vectors are one-column
//! matrices, scalars are one by one, a singular matrix is DOMAIN
//! ERROR -- and the least-squares property is what makes the dyadic
//! form worth having.

use apl_prims_matrix::{matrix_divide, matrix_inverse};
use apl_value::{Array, Data, ErrorKind, Number};

fn m(rows: usize, cols: usize, v: &[f64]) -> Array {
    let data = Data::Num(v.iter().map(|x| Number::from_f64(*x)).collect());
    Array::new(vec![rows, cols], data).expect("a matrix")
}

fn v(values: &[f64]) -> Array {
    Array::vector(values.iter().map(|x| Number::from_f64(*x)).collect())
}

fn s(x: f64) -> Array {
    Array::scalar(Number::from_f64(x))
}

/// The numbers a result holds, as doubles.
fn nums(a: &Array) -> Vec<f64> {
    match &a.data {
        Data::Num(v) => v.iter().map(|n| n.as_f64()).collect(),
        Data::Char(_) => panic!("a numeric result"),
    }
}

fn close(got: &Array, want: &[f64]) {
    let got = nums(got);
    assert_eq!(got.len(), want.len(), "{got:?} against {want:?}");
    for (g, w) in got.iter().zip(want) {
        assert!((g - w).abs() < 1e-9, "{got:?} against {want:?}");
    }
}

#[test]
fn the_inverse_of_a_square_matrix_undoes_it() {
    // A 2 by 2 whose inverse is exact in binary floating point.
    let a = m(2, 2, &[4.0, 7.0, 2.0, 6.0]);
    let inv = matrix_inverse(&a).expect("invertible");
    assert_eq!(inv.shape, vec![2, 2]);
    close(&inv, &[0.6, -0.7, -0.2, 0.4]);
}

#[test]
fn an_upper_triangular_inverse_is_exact() {
    let a = m(3, 3, &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
    let inv = matrix_inverse(&a).expect("invertible");
    close(&inv, &[1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, -1.0, 1.0]);
}

#[test]
fn the_inverse_of_a_scalar_is_its_reciprocal() {
    close(&matrix_inverse(&s(2.0)).unwrap(), &[0.5]);
    assert!(
        matrix_inverse(&s(2.0)).unwrap().shape.is_empty(),
        "a scalar"
    );
    assert_eq!(
        matrix_inverse(&s(0.0)).unwrap_err().kind,
        ErrorKind::Domain,
        "zero has no reciprocal here, as it has no inverse"
    );
}

#[test]
fn the_inverse_of_a_tall_matrix_is_its_left_inverse() {
    // Three rows, one column: the left inverse is a row whose inner
    // product with the column is 1.
    let a = m(3, 1, &[1.0, 2.0, 3.0]);
    let inv = matrix_inverse(&a).expect("full column rank");
    assert_eq!(inv.shape, vec![1, 3], "the shape is reversed");
    let got = nums(&inv);
    let dot: f64 = got.iter().zip([1.0, 2.0, 3.0]).map(|(g, x)| g * x).sum();
    assert!((dot - 1.0).abs() < 1e-12, "{got:?}");
}

#[test]
fn a_vector_inverts_to_a_vector() {
    // Rank in, rank out: a one-column matrix written as a vector
    // comes back as a vector rather than a one-row matrix.
    let inv = matrix_inverse(&v(&[1.0, 2.0, 3.0])).expect("full column rank");
    assert_eq!(inv.shape, vec![3]);
}

#[test]
fn a_singular_matrix_has_no_inverse() {
    // The second row is twice the first.
    let a = m(2, 2, &[1.0, 2.0, 2.0, 4.0]);
    assert_eq!(matrix_inverse(&a).unwrap_err().kind, ErrorKind::Domain);
}

#[test]
fn more_columns_than_rows_is_a_domain_error() {
    // Underdetermined: APL does not choose a solution for you.
    let a = m(2, 3, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    assert_eq!(matrix_inverse(&a).unwrap_err().kind, ErrorKind::Domain);
    assert_eq!(
        matrix_divide(&v(&[1.0, 2.0]), &a).unwrap_err().kind,
        ErrorKind::Domain
    );
}

#[test]
fn dividing_solves_the_system() {
    // Y is lower triangular ones; X ⌹ Y is the APLX manual's example.
    let x = m(3, 2, &[1.0, 2.0, 3.0, 6.0, 9.0, 10.0]);
    let y = m(3, 3, &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
    let got = matrix_divide(&x, &y).expect("solvable");
    assert_eq!(got.shape, vec![3, 2]);
    close(&got, &[1.0, 2.0, 2.0, 4.0, 6.0, 4.0]);
}

#[test]
fn dividing_a_vector_by_a_matrix_gives_a_vector() {
    let b = v(&[1.0, 2.0, 3.0]);
    let a = m(3, 3, &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
    let got = matrix_divide(&b, &a).expect("solvable");
    assert_eq!(got.shape, vec![3]);
    close(&got, &[1.0, 1.0, 1.0]);
}

#[test]
fn an_overdetermined_system_gets_the_least_squares_fit() {
    // Four points on the line y = 1 + 2x, with one moved off it by
    // an amount that cancels: the fit is still exactly 1 and 2.
    let x = m(4, 2, &[1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]);
    let y = v(&[1.0, 3.0, 5.0, 7.0]);
    let fit = matrix_divide(&y, &x).expect("full column rank");
    assert_eq!(fit.shape, vec![2]);
    close(&fit, &[1.0, 2.0]);
}

#[test]
fn the_fit_minimises_the_squared_error() {
    // y = x with a symmetric wobble: the slope through the origin is
    // still 1, and no other slope does better.
    let x = m(3, 1, &[1.0, 2.0, 3.0]);
    let y = v(&[1.5, 2.0, 2.5]);
    let fit = matrix_divide(&y, &x).expect("full column rank");
    let slope = nums(&fit)[0];
    let error = |k: f64| -> f64 {
        [(1.0, 1.5), (2.0, 2.0), (3.0, 2.5)]
            .iter()
            .map(|(xi, yi)| (k * xi - yi).powi(2))
            .sum()
    };
    for other in [slope - 0.01, slope + 0.01, 0.0, 1.0] {
        assert!(
            error(slope) <= error(other) + 1e-12,
            "{slope} against {other}"
        );
    }
}

#[test]
fn two_vectors_divide_to_a_scalar() {
    // The classic regression through the origin: one coefficient.
    let got = matrix_divide(&v(&[2.0, 4.0, 6.0]), &v(&[1.0, 2.0, 3.0])).expect("fits");
    assert!(got.shape.is_empty(), "a scalar, not a one-element vector");
    close(&got, &[2.0]);
}

#[test]
fn two_scalars_divide_as_scalars_do() {
    let got = matrix_divide(&s(3.0), &s(2.0)).expect("divides");
    assert!(got.shape.is_empty());
    close(&got, &[1.5]);
    assert_eq!(
        matrix_divide(&s(3.0), &s(0.0)).unwrap_err().kind,
        ErrorKind::Domain
    );
}

#[test]
fn the_rank_and_shape_rules_are_enforced() {
    let cube = Array::new(vec![2, 1, 1], Data::Num(vec![Number::Int(1); 2])).unwrap();
    assert_eq!(matrix_inverse(&cube).unwrap_err().kind, ErrorKind::Rank);
    let two = m(2, 2, &[1.0, 0.0, 0.0, 1.0]);
    assert_eq!(
        matrix_divide(&cube, &two).unwrap_err().kind,
        ErrorKind::Rank
    );
    // Different numbers of rows on the two sides.
    let three = m(3, 1, &[1.0, 2.0, 3.0]);
    assert_eq!(
        matrix_divide(&v(&[1.0, 2.0]), &three).unwrap_err().kind,
        ErrorKind::Length
    );
}

#[test]
fn character_data_has_no_inverse() {
    let text = Array::new(vec![1, 1], Data::Char(vec!['A'])).unwrap();
    assert_eq!(matrix_inverse(&text).unwrap_err().kind, ErrorKind::Domain);
    let two = m(1, 1, &[2.0]);
    assert_eq!(
        matrix_divide(&text, &two).unwrap_err().kind,
        ErrorKind::Domain
    );
}
