//! The reduce operator along the last axis.

use apl_prims_mixed::reshape;
use apl_prims_ops::reduce;
use apl_value::{Array, ErrorKind, Number};

fn v(xs: &[i64]) -> Array {
    Array::vector(xs.iter().map(|&x| Number::Int(x)).collect())
}
fn s(x: i64) -> Array {
    Array::scalar(Number::Int(x))
}

#[test]
fn reduce_folds_right_to_left() {
    assert_eq!(reduce('+', &v(&[1, 2, 3, 4, 5])).unwrap(), s(15));
    assert_eq!(reduce('-', &v(&[1, 2, 3])).unwrap(), s(2), "1-(2-3)");
    assert_eq!(reduce('×', &v(&[1, 2, 3, 4])).unwrap(), s(24));
    assert_eq!(reduce('⌈', &v(&[3, 9, 2])).unwrap(), s(9));
    assert_eq!(reduce('÷', &v(&[8, 4, 2])).unwrap(), s(4), "8÷(4÷2)");
}

#[test]
fn reduce_of_scalar_and_one_element() {
    assert_eq!(reduce('+', &s(7)).unwrap(), s(7));
    assert_eq!(reduce('+', &v(&[7])).unwrap(), s(7));
}

#[test]
fn reduce_of_empty_gives_the_identity() {
    assert_eq!(reduce('+', &v(&[])).unwrap(), s(0));
    assert_eq!(reduce('×', &v(&[])).unwrap(), s(1));
    assert_eq!(
        reduce('⌈', &v(&[])).unwrap(),
        Array::scalar(Number::Float(f64::MIN))
    );
    assert_eq!(
        reduce('⌊', &v(&[])).unwrap(),
        Array::scalar(Number::Float(f64::MAX))
    );
    assert_eq!(reduce('-', &v(&[])).unwrap(), s(0));
}

#[test]
fn reduce_along_the_last_axis_of_a_matrix() {
    let m = reshape(&v(&[2, 3]), &v(&[1, 2, 3, 4, 5, 6])).unwrap();
    assert_eq!(reduce('+', &m).unwrap(), v(&[6, 15]));
}

#[test]
fn reduce_with_a_non_scalar_function_is_not_implemented() {
    assert_eq!(
        reduce('⍳', &v(&[1, 2])).unwrap_err().kind,
        ErrorKind::NotImplemented
    );
}
