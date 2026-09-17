//! Reduce and scan along an axis.

use apl_prims_mixed::reshape;
use apl_prims_ops::{reduce, scan};
use apl_value::{Array, ErrorKind, Number};

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
        ErrorKind::NotImplemented
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
    assert_eq!(
        scan('⍳', &v(&[1]), 0).unwrap_err().kind,
        ErrorKind::NotImplemented
    );
}
