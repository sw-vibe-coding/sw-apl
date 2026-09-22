//! Encode and decode.

use apl_prims_radix::{decode, encode};
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

#[test]
fn decode_mixed_radix() {
    assert_eq!(decode(&v(&[2, 2, 2]), &v(&[1, 0, 1])).unwrap(), s(5));
    assert_eq!(decode(&v(&[24, 60, 60]), &v(&[1, 1, 1])).unwrap(), s(3661));
    assert_eq!(decode(&v(&[10, 10, 10]), &v(&[1, 2, 3])).unwrap(), s(123));
    assert_eq!(
        decode(&s(2), &v(&[1, 0, 1])).unwrap(),
        s(5),
        "a scalar radix extends"
    );
    assert_eq!(
        decode(&v(&[2, 2, 2]), &s(1)).unwrap(),
        s(7),
        "a scalar digit extends"
    );
    assert_eq!(decode(&s(10), &s(7)).unwrap(), s(7));
    assert_eq!(
        decode(&v(&[2, 2]), &m(&[2, 3], &[1, 0, 1, 1, 1, 0])).unwrap(),
        v(&[3, 1, 2]),
        "columns decode"
    );
    assert_eq!(
        decode(&v(&[2, 2]), &v(&[1, 0, 1])).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        decode(&m(&[1, 1], &[2]), &v(&[1])).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        decode(&Array::scalar(Number::Float(0.5)), &v(&[1, 1])).unwrap(),
        Array::scalar(Number::Float(1.5))
    );
}

#[test]
fn encode_mixed_radix() {
    assert_eq!(encode(&v(&[2, 2, 2]), &s(5)).unwrap(), v(&[1, 0, 1]));
    assert_eq!(encode(&v(&[24, 60, 60]), &s(3661)).unwrap(), v(&[1, 1, 1]));
    assert_eq!(encode(&v(&[10, 10, 10]), &s(123)).unwrap(), v(&[1, 2, 3]));
    assert_eq!(
        encode(&v(&[2, 2, 2]), &s(13)).unwrap(),
        v(&[1, 0, 1]),
        "high digits are lost"
    );
    assert_eq!(
        encode(&v(&[0, 2, 2]), &s(13)).unwrap(),
        v(&[3, 0, 1]),
        "a zero radix keeps the rest"
    );
    assert_eq!(encode(&s(10), &s(123)).unwrap(), s(3));
    assert_eq!(
        encode(&v(&[2, 2]), &v(&[1, 2, 3])).unwrap(),
        m(&[2, 3], &[0, 1, 1, 1, 0, 1])
    );
    assert_eq!(
        encode(&v(&[10, 10]), &s(-3)).unwrap(),
        v(&[9, 7]),
        "APL residue for negatives"
    );
    assert_eq!(
        encode(&v(&[1, 10]), &Array::scalar(Number::Float(2.5))).unwrap(),
        Array::vector(vec![Number::Int(0), Number::Float(2.5)])
    );
    assert_eq!(
        encode(&m(&[1, 1], &[2]), &s(1)).unwrap_err().kind,
        ErrorKind::Rank
    );
}

#[test]
fn encode_then_decode_round_trips() {
    let digits = encode(&v(&[2, 2, 2, 2]), &s(13)).unwrap();
    assert_eq!(decode(&v(&[2, 2, 2, 2]), &digits).unwrap(), s(13));
}

/// The APL\360 User's Manual, page 3.42: the arguments of decode
/// must be of the same dimension, except that either may be a scalar
/// or a one-element vector.
#[test]
fn decode_takes_a_one_element_vector_as_a_scalar() {
    let digits = v(&[1, 7, 7, 6]);
    assert_eq!(decode(&v(&[10]), &digits).unwrap(), s(1776), "on the left");
    assert_eq!(
        decode(&v(&[10, 10, 10]), &v(&[5])).unwrap(),
        s(555),
        "on the right"
    );
    assert_eq!(
        decode(&v(&[10, 10]), &digits).unwrap_err().kind,
        ErrorKind::Length
    );
}
