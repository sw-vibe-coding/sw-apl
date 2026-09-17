//! iota, rho, ravel, catenate.

use apl_prims_mixed::{iota, ravel, reshape, shape};
use apl_value::{Array, Data, ErrorKind, Number};

fn ints(xs: &[i64]) -> Vec<Number> {
    xs.iter().map(|&x| Number::Int(x)).collect()
}
fn v(xs: &[i64]) -> Array {
    Array::vector(ints(xs))
}
fn s(x: i64) -> Array {
    Array::scalar(Number::Int(x))
}

#[test]
fn iota_counts_from_the_index_origin() {
    assert_eq!(iota(&s(5), 1).unwrap(), v(&[1, 2, 3, 4, 5]));
    assert_eq!(iota(&s(3), 0).unwrap(), v(&[0, 1, 2]));
    assert_eq!(iota(&s(0), 1).unwrap(), v(&[]));
    assert_eq!(
        iota(&Array::scalar(Number::Float(2.0)), 1).unwrap(),
        v(&[1, 2])
    );
}

#[test]
fn iota_rejects_vectors_negatives_and_fractions() {
    assert_eq!(iota(&v(&[3]), 1).unwrap_err().kind, ErrorKind::Rank);
    assert_eq!(iota(&s(-1), 1).unwrap_err().kind, ErrorKind::Domain);
    assert_eq!(
        iota(&Array::scalar(Number::Float(2.5)), 1)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
}

#[test]
fn shape_of_scalar_vector_matrix() {
    assert_eq!(shape(&s(42)), v(&[]));
    assert_eq!(shape(&v(&[1, 2, 3])), v(&[3]));
    let m = reshape(&v(&[2, 3]), &v(&[1, 2, 3, 4, 5, 6])).unwrap();
    assert_eq!(shape(&m), v(&[2, 3]));
}

#[test]
fn reshape_cycles_and_truncates() {
    assert_eq!(reshape(&s(5), &v(&[1, 2, 3])).unwrap(), v(&[1, 2, 3, 1, 2]));
    assert_eq!(reshape(&s(2), &v(&[1, 2, 3])).unwrap(), v(&[1, 2]));
    let m = reshape(&v(&[2, 3]), &v(&[1, 2, 3, 4, 5, 6])).unwrap();
    assert_eq!(m.shape, vec![2, 3]);
    assert_eq!(m.data, Data::Num(ints(&[1, 2, 3, 4, 5, 6])));
    assert_eq!(reshape(&v(&[]), &v(&[7])).unwrap(), s(7));
    assert_eq!(
        reshape(&s(3), &v(&[])).unwrap(),
        v(&[0, 0, 0]),
        "empty fills with zero"
    );
    assert_eq!(reshape(&s(0), &v(&[1, 2])).unwrap(), v(&[]));
}

#[test]
fn reshape_rejects_bad_shapes() {
    assert_eq!(
        reshape(&s(-1), &v(&[1])).unwrap_err().kind,
        ErrorKind::Domain
    );
    let m = reshape(&v(&[1, 1]), &s(2)).unwrap();
    assert_eq!(reshape(&m, &s(1)).unwrap_err().kind, ErrorKind::Rank);
}

#[test]
fn ravel_flattens() {
    let m = reshape(&v(&[2, 2]), &v(&[1, 2, 3, 4])).unwrap();
    assert_eq!(ravel(&m), v(&[1, 2, 3, 4]));
    assert_eq!(ravel(&s(9)), v(&[9]));
}
