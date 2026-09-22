//! Compress and expand.

use apl_prims_mask::{compress, expand};
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
fn m(shape: &[usize], xs: &[i64]) -> Array {
    Array::new(shape.to_vec(), Data::Num(ints(xs))).unwrap()
}
fn chars(shape: &[usize], t: &str) -> Array {
    Array::new(shape.to_vec(), Data::Char(t.chars().collect())).unwrap()
}
fn m9() -> Array {
    m(&[3, 3], &[1, 2, 3, 4, 5, 6, 7, 8, 9])
}

#[test]
fn compress_vectors_and_scalars() {
    assert_eq!(
        compress(&v(&[1, 0, 1]), &v(&[10, 20, 30]), 0).unwrap(),
        v(&[10, 30])
    );
    assert_eq!(compress(&v(&[0, 0, 0]), &v(&[4, 5, 6]), 0).unwrap(), v(&[]));
    assert_eq!(compress(&s(1), &v(&[4, 5, 6]), 0).unwrap(), v(&[4, 5, 6]));
    assert_eq!(compress(&s(0), &v(&[4, 5, 6]), 0).unwrap(), v(&[]));
    // The APL\360 User's Manual, page 3.41: a scalar or one-element
    // left argument extends to the right one, but "a scalar right
    // argument is not extended" -- it is one element, and the left
    // argument must be one too.
    assert_eq!(
        compress(&v(&[1, 1, 0]), &s(7), 0).unwrap_err().kind,
        ErrorKind::Length,
        "a scalar right argument does not extend"
    );
    assert_eq!(
        compress(&s(1), &s(7), 0).unwrap(),
        v(&[7]),
        "a vector in every case"
    );
    assert_eq!(compress(&v(&[0]), &s(7), 0).unwrap(), v(&[]));
    assert_eq!(
        compress(&v(&[1]), &v(&[4, 5, 6]), 0).unwrap(),
        v(&[4, 5, 6])
    );
    assert_eq!(
        compress(&v(&[1, 1, 0]), &chars(&[3], "abc"), 0).unwrap(),
        chars(&[2], "ab")
    );
}

#[test]
fn compress_matrices_along_either_axis() {
    assert_eq!(
        compress(&v(&[1, 0, 1]), &m9(), 1).unwrap(),
        m(&[3, 2], &[1, 3, 4, 6, 7, 9])
    );
    assert_eq!(
        compress(&v(&[0, 1, 1]), &m9(), 0).unwrap(),
        m(&[2, 3], &[4, 5, 6, 7, 8, 9])
    );
    assert_eq!(compress(&s(1), &m9(), 0).unwrap(), m9());
}

#[test]
fn compress_errors() {
    assert_eq!(
        compress(&v(&[1, 0]), &v(&[1, 2, 3]), 0).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        compress(&v(&[1, 2, 1]), &v(&[1, 2, 3]), 0)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
    assert_eq!(
        compress(&m(&[1, 3], &[1, 1, 1]), &v(&[1, 2, 3]), 0)
            .unwrap_err()
            .kind,
        ErrorKind::Rank
    );
    assert_eq!(
        compress(&chars(&[1], "1"), &v(&[1]), 0).unwrap_err().kind,
        ErrorKind::Domain
    );
}

#[test]
fn expand_inserts_fill_at_zeros() {
    assert_eq!(
        expand(&v(&[1, 0, 1, 0, 1]), &v(&[1, 2, 3]), 0).unwrap(),
        v(&[1, 0, 2, 0, 3])
    );
    assert_eq!(expand(&v(&[0, 1]), &v(&[9]), 0).unwrap(), v(&[0, 9]));
    assert_eq!(
        expand(&v(&[1, 0, 1]), &chars(&[2], "ab"), 0).unwrap(),
        chars(&[3], "a b")
    );
    assert_eq!(
        expand(&v(&[1, 0, 1]), &s(4), 0).unwrap(),
        v(&[4, 0, 4]),
        "a scalar extends"
    );
    assert_eq!(
        expand(&v(&[1, 0, 1]), &m(&[2, 2], &[1, 2, 3, 4]), 1).unwrap(),
        m(&[2, 3], &[1, 0, 2, 3, 0, 4])
    );
    assert_eq!(
        expand(&v(&[0, 1, 1]), &m(&[2, 2], &[1, 2, 3, 4]), 0).unwrap(),
        m(&[3, 2], &[0, 0, 1, 2, 3, 4])
    );
    assert_eq!(
        expand(&v(&[1, 0]), &v(&[1, 2]), 0).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        expand(&v(&[1, 2]), &v(&[1, 2]), 0).unwrap_err().kind,
        ErrorKind::Domain
    );
}
