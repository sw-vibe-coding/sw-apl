//! Scalar primitives with scalar extension.

use apl_value::{AplResult, Array, Data, ErrorKind, FUZZ, Number};

// At APL\360's fixed tolerance, which is what every test here
// assumes; `⎕CT` in (B) is pinned by the session tests.
fn monadic(f: char, r: &Array) -> AplResult<Array> {
    apl_prims_scalar::monadic(f, r, FUZZ)
}

fn dyadic(f: char, l: &Array, r: &Array) -> AplResult<Array> {
    apl_prims_scalar::dyadic(f, l, r, FUZZ)
}

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

/// This test once said the opposite, pinning sw-apl's scalar-only
/// rule. The manual (page 3.33) extends a one-element array too.
#[test]
fn one_element_vector_extends_like_a_scalar() {
    let r = dyadic('+', &v(&[1.0]), &v(&[1.0, 2.0])).unwrap();
    assert_eq!(nums(&r), nums(&v(&[2.0, 3.0])));
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
fn a_glyph_with_no_such_form_is_a_syntax_error() {
    assert_eq!(
        dyadic('\u{2373}', &s(1.0), &s(1.0)).unwrap_err().kind,
        ErrorKind::Syntax
    );
    assert_eq!(
        monadic('\u{2374}', &s(1.0)).unwrap_err().kind,
        ErrorKind::Syntax
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
fn a_glyph_with_no_such_form_is_a_syntax_error_on_empty_arguments_too() {
    assert_eq!(
        monadic('\u{233d}', &v(&[])).unwrap_err().kind,
        ErrorKind::Syntax
    );
    assert_eq!(
        dyadic('\u{2373}', &v(&[]), &v(&[])).unwrap_err().kind,
        ErrorKind::Syntax
    );
    assert_eq!(
        monadic('?', &v(&[])).unwrap_err().kind,
        ErrorKind::Syntax,
        "roll needs the random link; dispatched above the scalar family"
    );
}

/// A one-element array of any shape, holding `x`.
fn one(shape: &[usize], x: f64) -> Array {
    Array::new(shape.to_vec(), Data::Num(vec![Number::from_f64(x)])).unwrap()
}

/// The APL\360 User's Manual, page 3.33 (1968 and 1970 alike): if the
/// arguments of a dyadic scalar function differ in size, it is a
/// length or rank error unless one of them is a scalar or a
/// one-element array, whose single element then goes with each
/// element of the other.
#[test]
fn a_one_element_array_extends_like_a_scalar() {
    let r = dyadic('+', &v(&[1.0, 2.0, 3.0]), &one(&[1], 5.0)).unwrap();
    assert_eq!(
        (r.shape.clone(), nums(&r)),
        (vec![3], nums(&v(&[6.0, 7.0, 8.0])))
    );
    let r = dyadic('+', &one(&[1], 5.0), &v(&[1.0, 2.0, 3.0])).unwrap();
    assert_eq!(r.shape, vec![3]);
    let r = dyadic('+', &one(&[1, 1], 5.0), &v(&[1.0, 2.0, 3.0])).unwrap();
    assert_eq!(r.shape, vec![3], "any rank: the other argument's shape");
    let m = Array::new(vec![2, 2], Data::Num((1..=4).map(Number::Int).collect())).unwrap();
    let r = dyadic('×', &m, &one(&[1, 1, 1], 10.0)).unwrap();
    assert_eq!(
        (r.shape.clone(), nums(&r)[3]),
        (vec![2, 2], Number::Int(40))
    );
}

#[test]
fn a_one_element_array_against_an_empty_one_gives_an_empty_one() {
    let r = dyadic('+', &one(&[1], 5.0), &v(&[])).unwrap();
    assert_eq!(r.shape, vec![0]);
}

/// Two one-element arrays of different shapes: the manual does not
/// say whose shape the result takes. sw-apl gives it the shape of the
/// one of higher rank -- a guess, and the one that keeps a scalar
/// with a one-element vector as it always was.
#[test]
fn two_one_element_arrays_take_the_higher_rank() {
    let r = dyadic('+', &one(&[1], 5.0), &one(&[1, 1], 3.0)).unwrap();
    assert_eq!(r.shape, vec![1, 1]);
    let r = dyadic('+', &one(&[1, 1], 3.0), &one(&[1], 5.0)).unwrap();
    assert_eq!(r.shape, vec![1, 1]);
    let r = dyadic('+', &s(5.0), &one(&[1], 3.0)).unwrap();
    assert_eq!(r.shape, vec![1]);
}

#[test]
fn arrays_of_more_than_one_element_must_still_agree() {
    let two = v(&[1.0, 2.0]);
    assert_eq!(
        dyadic('+', &v(&[1.0, 2.0, 3.0]), &two).unwrap_err().kind,
        ErrorKind::Length
    );
    let m = Array::new(vec![1, 2], Data::Num(vec![Number::Int(1), Number::Int(2)])).unwrap();
    assert_eq!(dyadic('+', &m, &two).unwrap_err().kind, ErrorKind::Rank);
}

fn text(t: &str) -> Array {
    Array::new(vec![t.chars().count()], Data::Char(t.chars().collect())).unwrap()
}

/// The APL\360 User's Manual, page 3.8: of the scalar functions,
/// = and ≠ are defined on characters as well as on numbers.
#[test]
fn equal_and_not_equal_compare_characters() {
    let r = dyadic(
        '=',
        &text("MISSISSIPPI"),
        &Array::new(vec![], Data::Char(vec!['S'])).unwrap(),
    )
    .unwrap();
    assert_eq!(nums(&r).iter().filter(|n| **n == Number::Int(1)).count(), 4);
    let r = dyadic('≠', &text("AB"), &text("AC")).unwrap();
    assert_eq!(nums(&r), [Number::Int(0), Number::Int(1)]);
    let r = dyadic('=', &text("S"), &text("MISS")).unwrap();
    assert_eq!(r.shape, vec![4], "a one-element vector extends");
}

/// The manual does not say what a character compared with a number
/// gives; sw-apl says they are never equal rather than refusing.
#[test]
fn a_character_never_equals_a_number() {
    let r = dyadic('=', &text("1"), &s(1.0)).unwrap();
    assert_eq!(nums(&r), [Number::Int(0)]);
    let r = dyadic('≠', &s(1.0), &text("1")).unwrap();
    assert_eq!(nums(&r), [Number::Int(1)]);
}

#[test]
fn the_other_scalar_functions_still_refuse_characters() {
    for f in ['+', '<', '≤', '∧', '⌈'] {
        let e = dyadic(f, &text("AB"), &text("AB")).unwrap_err();
        assert_eq!(e.kind, ErrorKind::Domain, "{f}");
    }
    assert_eq!(
        dyadic('+', &text(""), &v(&[])).unwrap_err().kind,
        ErrorKind::Domain,
        "even empty"
    );
    assert_eq!(
        dyadic('=', &text("AB"), &text("ABC")).unwrap_err().kind,
        ErrorKind::Length
    );
}
