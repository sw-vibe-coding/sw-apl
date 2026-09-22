//! Reduce and scan along an axis.

use apl_prims_mixed::reshape;
use apl_value::{AplResult, Array, ErrorKind, FUZZ, Number};

// At APL\360's fixed tolerance, which is what every test here
// assumes; `⎕CT` in (B) is pinned by the session tests.
fn reduce(f: char, r: &Array, k: usize) -> AplResult<Array> {
    apl_prims_ops::reduce(f, r, k, FUZZ)
}

fn scan(f: char, r: &Array, k: usize) -> AplResult<Array> {
    apl_prims_ops::scan(f, r, k, FUZZ)
}

fn outer(g: char, l: &Array, r: &Array) -> AplResult<Array> {
    apl_prims_ops::outer(g, l, r, FUZZ)
}

fn inner(f: char, g: char, l: &Array, r: &Array) -> AplResult<Array> {
    apl_prims_ops::inner(f, g, l, r, FUZZ)
}

fn v(xs: &[i64]) -> Array {
    Array::vector(xs.iter().map(|&x| Number::Int(x)).collect())
}
fn s(x: i64) -> Array {
    Array::scalar(Number::Int(x))
}
fn m(rows: i64, cols: i64, xs: &[i64]) -> Array {
    reshape(&v(&[rows, cols]), &v(xs)).unwrap()
}

#[test]
fn reduce_folds_right_to_left() {
    assert_eq!(reduce('+', &v(&[1, 2, 3, 4, 5]), 0).unwrap(), s(15));
    assert_eq!(reduce('-', &v(&[1, 2, 3]), 0).unwrap(), s(2), "1-(2-3)");
    assert_eq!(reduce('×', &v(&[1, 2, 3, 4]), 0).unwrap(), s(24));
    assert_eq!(reduce('⌈', &v(&[3, 9, 2]), 0).unwrap(), s(9));
    assert_eq!(reduce('÷', &v(&[8, 4, 2]), 0).unwrap(), s(4), "8÷(4÷2)");
    assert_eq!(reduce('∨', &v(&[0, 0, 1]), 0).unwrap(), s(1));
    assert_eq!(reduce('∧', &v(&[1, 1, 0]), 0).unwrap(), s(0));
    assert_eq!(reduce('+', &s(7), 0).unwrap(), s(7));
    assert_eq!(reduce('+', &v(&[7]), 0).unwrap(), s(7));
}

#[test]
fn reduce_along_either_axis_of_a_matrix() {
    let a = m(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(reduce('+', &a, 1).unwrap(), v(&[6, 15]));
    assert_eq!(reduce('+', &a, 0).unwrap(), v(&[5, 7, 9]));
    assert_eq!(reduce('-', &a, 0).unwrap(), v(&[-3, -3, -3]));
    let cube = reshape(&v(&[2, 2, 2]), &v(&[1, 2, 3, 4, 5, 6, 7, 8])).unwrap();
    assert_eq!(reduce('+', &cube, 1).unwrap(), m(2, 2, &[4, 6, 12, 14]));
    assert_eq!(reduce('+', &cube, 0).unwrap(), m(2, 2, &[6, 8, 10, 12]));
}

#[test]
fn reduce_of_empty_gives_the_identity() {
    let e = v(&[]);
    let table = [
        ('+', 0.0),
        ('-', 0.0),
        ('×', 1.0),
        ('÷', 1.0),
        ('|', 0.0),
        ('*', 1.0),
        ('!', 1.0),
        ('∧', 1.0),
        ('∨', 0.0),
        ('<', 0.0),
        ('≤', 1.0),
        ('=', 1.0),
        ('≥', 1.0),
        ('>', 0.0),
        ('≠', 0.0),
        ('⌈', f64::MIN),
        ('⌊', f64::MAX),
    ];
    for (f, want) in table {
        assert_eq!(
            reduce(f, &e, 0).unwrap(),
            Array::scalar(Number::from_f64(want)),
            "{f}"
        );
    }
    for f in ['⍟', '○', '⍲', '⍱'] {
        assert_eq!(reduce(f, &e, 0).unwrap_err().kind, ErrorKind::Domain, "{f}");
    }
    assert_eq!(reduce('+', &m(0, 3, &[]), 0).unwrap(), v(&[0, 0, 0]));
    assert_eq!(reduce('+', &m(2, 0, &[]), 1).unwrap(), v(&[0, 0]));
    assert_eq!(
        reduce('⍳', &v(&[1, 2]), 0).unwrap_err().kind,
        ErrorKind::Syntax
    );
}

#[test]
fn scan_gives_running_reductions() {
    assert_eq!(scan('+', &v(&[1, 2, 3, 4]), 0).unwrap(), v(&[1, 3, 6, 10]));
    assert_eq!(
        scan('×', &v(&[1, 2, 3, 4, 5]), 0).unwrap(),
        v(&[1, 2, 6, 24, 120])
    );
    assert_eq!(
        scan('-', &v(&[1, 2, 3]), 0).unwrap(),
        v(&[1, -1, 2]),
        "each element is a prefix reduce"
    );
    assert_eq!(
        scan('⌈', &v(&[3, 1, 4, 1, 5, 9]), 0).unwrap(),
        v(&[3, 3, 4, 4, 5, 9])
    );
    assert_eq!(
        scan('∨', &v(&[0, 0, 1, 0, 0]), 0).unwrap(),
        v(&[0, 0, 1, 1, 1])
    );
    assert_eq!(
        scan('∧', &v(&[1, 1, 1, 0, 1]), 0).unwrap(),
        v(&[1, 1, 1, 0, 0])
    );
    assert_eq!(scan('+', &s(7), 0).unwrap(), s(7));
    assert_eq!(scan('+', &v(&[]), 0).unwrap(), v(&[]));
    let a = m(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(scan('+', &a, 1).unwrap(), m(2, 3, &[1, 3, 6, 4, 9, 15]));
    assert_eq!(scan('+', &a, 0).unwrap(), m(2, 3, &[1, 2, 3, 5, 7, 9]));
    assert_eq!(scan('⍳', &v(&[1]), 0).unwrap_err().kind, ErrorKind::Syntax);
}

#[test]
fn outer_product_pairs_every_element() {
    assert_eq!(
        outer('+', &v(&[1, 2, 3]), &v(&[10, 20])).unwrap(),
        m(3, 2, &[11, 21, 12, 22, 13, 23])
    );
    assert_eq!(
        outer('×', &v(&[1, 2]), &v(&[1, 2, 3])).unwrap(),
        m(2, 3, &[1, 2, 3, 2, 4, 6])
    );
    assert_eq!(
        outer('=', &v(&[1, 2, 3]), &v(&[1, 2, 3])).unwrap(),
        reshape(&v(&[3, 3]), &v(&[1, 0, 0, 0, 1, 0, 0, 0, 1])).unwrap()
    );
    assert_eq!(outer('+', &s(1), &v(&[1, 2])).unwrap(), v(&[2, 3]));
    assert_eq!(outer('+', &s(1), &s(2)).unwrap(), s(3));
    let cube = outer('+', &m(2, 2, &[0, 0, 0, 0]), &v(&[1, 2, 3])).unwrap();
    assert_eq!(cube.shape, vec![2, 2, 3]);
    assert_eq!(outer('+', &v(&[]), &v(&[1, 2])).unwrap().shape, vec![0, 2]);
    assert_eq!(
        outer('⍳', &v(&[1]), &v(&[1])).unwrap_err().kind,
        ErrorKind::Syntax
    );
}

#[test]
fn inner_product_reduces_over_the_shared_axis() {
    assert_eq!(
        inner('+', '×', &v(&[1, 2, 3]), &v(&[4, 5, 6])).unwrap(),
        s(32)
    );
    let a = m(2, 2, &[1, 2, 3, 4]);
    let b = m(2, 2, &[5, 6, 7, 8]);
    assert_eq!(inner('+', '×', &a, &b).unwrap(), m(2, 2, &[19, 22, 43, 50]));
    assert_eq!(
        inner('+', '×', &m(2, 3, &[1, 2, 3, 4, 5, 6]), &v(&[1, 2, 3])).unwrap(),
        v(&[14, 32])
    );
    assert_eq!(
        inner('+', '×', &v(&[1, 2]), &m(2, 3, &[1, 2, 3, 4, 5, 6])).unwrap(),
        v(&[9, 12, 15])
    );
    assert_eq!(
        inner('∨', '=', &v(&[1, 2, 3, 4, 5]), &v(&[3, 3, 3, 3, 3])).unwrap(),
        s(1)
    );
    assert_eq!(
        inner('∧', '=', &v(&[1, 2, 3]), &v(&[1, 2, 4])).unwrap(),
        s(0)
    );
    assert_eq!(
        inner('+', '×', &s(2), &v(&[1, 2, 3])).unwrap(),
        s(12),
        "a scalar extends"
    );
    assert_eq!(inner('+', '×', &v(&[1, 2, 3]), &s(2)).unwrap(), s(12));
    assert_eq!(inner('+', '×', &s(2), &s(3)).unwrap(), s(6));
    assert_eq!(
        inner('-', '×', &v(&[1, 2, 3]), &v(&[1, 1, 1])).unwrap(),
        s(2),
        "reduces from the right"
    );
    assert_eq!(
        inner('+', '×', &v(&[1, 2]), &v(&[1, 2, 3]))
            .unwrap_err()
            .kind,
        ErrorKind::Length
    );
    assert_eq!(
        inner('+', '×', &a, &v(&[1, 2, 3])).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        inner('+', '×', &v(&[]), &v(&[])).unwrap(),
        s(0),
        "empty shared axis gives the identity"
    );
    let cube = reshape(&v(&[2, 2, 2]), &v(&[1, 2, 3, 4, 5, 6, 7, 8])).unwrap();
    assert_eq!(
        inner('+', '×', &cube, &v(&[1, 1])).unwrap(),
        m(2, 2, &[3, 7, 11, 15])
    );
    assert_eq!(
        inner('⍳', '×', &v(&[1]), &v(&[1])).unwrap_err().kind,
        ErrorKind::Syntax
    );
}

fn text(t: &str) -> Array {
    Array::new(
        vec![t.chars().count()],
        apl_value::Data::Char(t.chars().collect()),
    )
    .unwrap()
}

fn ints(a: &Array) -> Vec<Number> {
    match &a.data {
        apl_value::Data::Num(v) => v.clone(),
        apl_value::Data::Char(_) => panic!("chars"),
    }
}

/// Page 3.33: every scalar function extends to arrays by reduction,
/// inner product and outer product, so = and ≠ on characters do too.
#[test]
fn outer_product_compares_characters() {
    let r = outer('=', &text("ABC"), &text("AB")).unwrap();
    assert_eq!(r.shape, vec![3, 2]);
    assert_eq!(ints(&r), [1, 0, 0, 1, 0, 0].map(Number::Int));
}

#[test]
fn inner_product_matches_strings() {
    let same = inner('∧', '=', &text("CAT"), &text("CAT")).unwrap();
    assert_eq!(ints(&same), [Number::Int(1)]);
    let differ = inner('∧', '=', &text("CAT"), &text("COT")).unwrap();
    assert_eq!(ints(&differ), [Number::Int(0)]);
    let e = inner('+', '×', &text("AB"), &text("AB")).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Domain, "only = and ≠ take characters");
}

#[test]
fn reduction_by_a_relation_takes_characters() {
    assert_eq!(
        ints(&reduce('=', &text("AA"), 0).unwrap()),
        [Number::Int(1)]
    );
    assert_eq!(
        ints(&reduce('≠', &text("AB"), 0).unwrap()),
        [Number::Int(1)]
    );
    // A B A: B=A is 0, then A=0 compares a character with a number.
    assert_eq!(
        ints(&reduce('=', &text("ABA"), 0).unwrap()),
        [Number::Int(0)]
    );
    let one = reduce('=', &text("Q"), 0).unwrap();
    assert_eq!(
        one.data,
        apl_value::Data::Char(vec!['Q']),
        "one element reduces to itself"
    );
    assert_eq!(
        reduce('+', &text("AB"), 0).unwrap_err().kind,
        ErrorKind::Domain
    );
}

/// A scan of characters would begin with a character and go on with
/// numbers, which a flat array cannot hold, so it stays refused.
#[test]
fn a_scan_of_characters_is_still_a_domain_error() {
    assert_eq!(
        scan('=', &text("AAB"), 0).unwrap_err().kind,
        ErrorKind::Domain
    );
}
