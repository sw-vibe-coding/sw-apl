//! Scalar primitives with scalar extension.

use apl_prims_scalar::{dyadic, monadic};
use apl_value::{Array, Data, ErrorKind, Number};

fn nums(a: &Array) -> Vec<Number> {
    match &a.data {
        Data::Num(v) => v.clone(),
        Data::Char(_) => panic!("chars"),
    }
}
fn v(xs: &[f64]) -> Array {
    Array::vector(xs.iter().map(|&x| Number::from_f64(x)).collect())
}
fn s(x: f64) -> Array {
    Array::scalar(Number::from_f64(x))
}

#[test]
fn dyadic_arithmetic_on_scalars() {
    assert_eq!(
        nums(&dyadic('+', &s(3.0), &s(4.0)).unwrap()),
        [Number::Int(7)]
    );
    assert_eq!(
        nums(&dyadic('-', &s(10.0), &s(3.0)).unwrap()),
        [Number::Int(7)]
    );
    assert_eq!(
        nums(&dyadic('\u{d7}', &s(6.0), &s(4.0)).unwrap()),
        [Number::Int(24)]
    );
    assert_eq!(
        nums(&dyadic('\u{f7}', &s(20.0), &s(5.0)).unwrap()),
        [Number::Int(4)]
    );
    assert_eq!(
        nums(&dyadic('\u{f7}', &s(1.0), &s(4.0)).unwrap()),
        [Number::Float(0.25)]
    );
    assert_eq!(
        nums(&dyadic('\u{2308}', &s(1.0), &s(4.0)).unwrap()),
        [Number::Int(4)]
    );
    assert_eq!(
        nums(&dyadic('\u{230a}', &s(1.0), &s(4.0)).unwrap()),
        [Number::Int(1)]
    );
    assert_eq!(
        nums(&dyadic('*', &s(2.0), &s(10.0)).unwrap()),
        [Number::Int(1024)]
    );
    assert_eq!(
        nums(&dyadic('|', &s(3.0), &s(10.0)).unwrap()),
        [Number::Int(1)]
    );
    assert_eq!(
        nums(&dyadic('|', &s(-3.0), &s(10.0)).unwrap()),
        [Number::Int(-2)]
    );
    assert_eq!(
        nums(&dyadic('|', &s(0.0), &s(7.0)).unwrap()),
        [Number::Int(7)]
    );
}

#[test]
fn divide_by_zero_rules() {
    assert_eq!(
        nums(&dyadic('\u{f7}', &s(0.0), &s(0.0)).unwrap()),
        [Number::Int(1)]
    );
    assert_eq!(
        dyadic('\u{f7}', &s(1.0), &s(0.0)).unwrap_err().kind,
        ErrorKind::Domain
    );
}

#[test]
fn scalar_extension_and_agreement() {
    assert_eq!(
        nums(&dyadic('+', &v(&[1.0, 2.0, 3.0]), &s(10.0)).unwrap()),
        [Number::Int(11), Number::Int(12), Number::Int(13)]
    );
    assert_eq!(
        nums(&dyadic('\u{d7}', &s(2.0), &v(&[1.0, 2.0, 3.0])).unwrap()),
        [Number::Int(2), Number::Int(4), Number::Int(6)]
    );
    let r = dyadic('+', &v(&[1.0, 2.0, 3.0]), &v(&[10.0, 20.0, 30.0])).unwrap();
    assert_eq!(r.shape, vec![3]);
    assert_eq!(
        nums(&r),
        [Number::Int(11), Number::Int(22), Number::Int(33)]
    );
    assert_eq!(
        dyadic('+', &v(&[1.0, 2.0]), &v(&[1.0, 2.0, 3.0]))
            .unwrap_err()
            .kind,
        ErrorKind::Length
    );
    let m = Array::new(vec![1, 2], Data::Num(vec![Number::Int(1); 2])).unwrap();
    assert_eq!(
        dyadic('+', &m, &v(&[1.0, 2.0])).unwrap_err().kind,
        ErrorKind::Rank
    );
}

#[test]
fn one_element_vector_is_not_a_scalar_for_extension() {
    assert_eq!(
        dyadic('+', &v(&[1.0]), &v(&[1.0, 2.0])).unwrap_err().kind,
        ErrorKind::Length
    );
}

#[test]
fn empty_vectors_agree() {
    let r = dyadic('+', &v(&[]), &v(&[])).unwrap();
    assert_eq!(r.shape, vec![0]);
}

#[test]
fn monadic_functions() {
    assert_eq!(
        nums(&monadic('-', &v(&[1.0, -2.0])).unwrap()),
        [Number::Int(-1), Number::Int(2)]
    );
    assert_eq!(nums(&monadic('+', &s(5.0)).unwrap()), [Number::Int(5)]);
    assert_eq!(
        nums(&monadic('\u{d7}', &v(&[-3.0, 0.0, 9.0])).unwrap()),
        [Number::Int(-1), Number::Int(0), Number::Int(1)]
    );
    assert_eq!(
        nums(&monadic('\u{f7}', &s(4.0)).unwrap()),
        [Number::Float(0.25)]
    );
    assert_eq!(
        monadic('\u{f7}', &s(0.0)).unwrap_err().kind,
        ErrorKind::Domain
    );
    assert_eq!(
        nums(&monadic('\u{2308}', &s(2.5)).unwrap()),
        [Number::Int(3)]
    );
    assert_eq!(
        nums(&monadic('\u{230a}', &s(-2.5)).unwrap()),
        [Number::Int(-3)]
    );
    assert_eq!(nums(&monadic('|', &s(-2.5)).unwrap()), [Number::Float(2.5)]);
    assert_eq!(nums(&monadic('*', &s(0.0)).unwrap()), [Number::Int(1)]);
}

#[test]
fn unknown_glyph_is_not_implemented() {
    assert_eq!(
        dyadic('\u{2373}', &s(1.0), &s(1.0)).unwrap_err().kind,
        ErrorKind::NotImplemented
    );
    assert_eq!(
        monadic('\u{2374}', &s(1.0)).unwrap_err().kind,
        ErrorKind::NotImplemented
    );
}

#[test]
fn integer_results_stay_exact_beyond_2_to_53() {
    let big = Array::scalar(Number::Int(9_007_199_254_740_993));
    assert_eq!(
        nums(&dyadic('+', &big, &s(0.0)).unwrap()),
        [Number::Int(9_007_199_254_740_993)]
    );
    assert_eq!(
        nums(&dyadic('\u{d7}', &big, &s(2.0)).unwrap()),
        [Number::Int(18_014_398_509_481_986)]
    );
    let max = Array::scalar(Number::Int(i64::MAX));
    assert!(matches!(
        nums(&dyadic('+', &max, &s(1.0)).unwrap())[0],
        Number::Float(_)
    ));
}

#[test]
fn the_complete_scalar_family_dispatches() {
    let b = |x: f64| Number::from_f64(x);
    assert_eq!(
        nums(&monadic('~', &v(&[0.0, 1.0])).unwrap()),
        [b(1.0), b(0.0)]
    );
    assert_eq!(nums(&monadic('!', &s(5.0)).unwrap()), [b(120.0)]);
    assert_eq!(nums(&monadic('\u{235f}', &s(1.0)).unwrap()), [b(0.0)]);
    assert!(
        matches!(nums(&monadic('\u{25cb}', &s(1.0)).unwrap())[0], Number::Float(x) if (x - std::f64::consts::PI).abs() < 1e-12)
    );
    assert_eq!(
        nums(&dyadic('=', &v(&[1.0, 2.0, 3.0]), &v(&[1.0, 0.0, 3.0])).unwrap()),
        [b(1.0), b(0.0), b(1.0)]
    );
    assert_eq!(
        nums(&dyadic('\u{2265}', &v(&[1.0, 2.0, 3.0]), &s(2.0)).unwrap()),
        [b(0.0), b(1.0), b(1.0)]
    );
    assert_eq!(
        nums(&dyadic('\u{2227}', &v(&[0.0, 1.0]), &s(1.0)).unwrap()),
        [b(0.0), b(1.0)]
    );
    assert_eq!(
        nums(&dyadic('!', &s(2.0), &v(&[3.0, 4.0, 5.0, 6.0])).unwrap()),
        [b(3.0), b(6.0), b(10.0), b(15.0)]
    );
    assert_eq!(
        nums(&dyadic('\u{235f}', &s(2.0), &s(8.0)).unwrap()),
        [b(3.0)]
    );
    assert_eq!(
        nums(&dyadic('\u{25cb}', &s(2.0), &s(0.0)).unwrap()),
        [b(1.0)]
    );
    assert_eq!(
        dyadic('\u{2227}', &s(2.0), &s(1.0)).unwrap_err().kind,
        ErrorKind::Domain
    );
    assert_eq!(monadic('~', &s(2.0)).unwrap_err().kind, ErrorKind::Domain);
}

#[test]
fn unknown_glyphs_are_not_implemented_even_on_empty_arguments() {
    assert_eq!(
        monadic('\u{233d}', &v(&[])).unwrap_err().kind,
        ErrorKind::NotImplemented
    );
    assert_eq!(
        dyadic('\u{2373}', &v(&[]), &v(&[])).unwrap_err().kind,
        ErrorKind::NotImplemented
    );
    assert_eq!(
        monadic('?', &v(&[])).unwrap_err().kind,
        ErrorKind::NotImplemented,
        "roll needs the random link; dispatched above the scalar family"
    );
}
