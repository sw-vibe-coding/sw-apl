//! Take, drop, reverse, rotate, transpose.

use apl_prims_select::{axis_index, drop, reverse, rotate, take, transpose};
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
fn take_and_drop_vectors_with_negatives_and_overtake() {
    let a = v(&[1, 2, 3, 4, 5]);
    assert_eq!(take(&s(3), &a).unwrap(), v(&[1, 2, 3]));
    assert_eq!(take(&s(-2), &a).unwrap(), v(&[4, 5]));
    assert_eq!(take(&s(7), &a).unwrap(), v(&[1, 2, 3, 4, 5, 0, 0]));
    assert_eq!(take(&s(-7), &a).unwrap(), v(&[0, 0, 1, 2, 3, 4, 5]));
    assert_eq!(take(&s(0), &a).unwrap(), v(&[]));
    assert_eq!(drop(&s(2), &a).unwrap(), v(&[3, 4, 5]));
    assert_eq!(drop(&s(-2), &a).unwrap(), v(&[1, 2, 3]));
    assert_eq!(drop(&s(9), &a).unwrap(), v(&[]));
    assert_eq!(drop(&s(0), &a).unwrap(), a);
    assert_eq!(
        take(&s(3), &s(5)).unwrap(),
        v(&[5, 0, 0]),
        "a scalar takes as a one-element vector"
    );
    assert_eq!(take(&s(3), &chars(&[2], "AB")).unwrap(), chars(&[3], "AB "));
}

#[test]
fn take_and_drop_matrices_per_axis() {
    assert_eq!(take(&v(&[1, 3]), &m9()).unwrap(), m(&[1, 3], &[1, 2, 3]));
    assert_eq!(take(&v(&[-1, 3]), &m9()).unwrap(), m(&[1, 3], &[7, 8, 9]));
    assert_eq!(take(&v(&[2, 2]), &m9()).unwrap(), m(&[2, 2], &[1, 2, 4, 5]));
    assert_eq!(
        take(&v(&[-2, -2]), &m9()).unwrap(),
        m(&[2, 2], &[5, 6, 8, 9])
    );
    assert_eq!(
        take(&v(&[2, 4]), &m9()).unwrap(),
        m(&[2, 4], &[1, 2, 3, 0, 4, 5, 6, 0])
    );
    assert_eq!(drop(&v(&[1, 1]), &m9()).unwrap(), m(&[2, 2], &[5, 6, 8, 9]));
    assert_eq!(drop(&v(&[-1, -2]), &m9()).unwrap(), m(&[2, 1], &[1, 4]));
    assert_eq!(drop(&v(&[1, 3]), &m9()).unwrap(), m(&[2, 0], &[]));
}

#[test]
fn take_and_drop_errors() {
    assert_eq!(take(&s(2), &m9()).unwrap_err().kind, ErrorKind::Length);
    assert_eq!(
        take(&v(&[1, 1, 1]), &m9()).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        take(&Array::scalar(Number::Float(1.5)), &v(&[1]))
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
    assert_eq!(
        take(&m(&[1, 1], &[1]), &v(&[1])).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        drop(&chars(&[], "A"), &v(&[1])).unwrap_err().kind,
        ErrorKind::Domain
    );
}

#[test]
fn reverse_along_either_axis() {
    assert_eq!(reverse(&v(&[1, 2, 3]), 0), v(&[3, 2, 1]));
    assert_eq!(reverse(&m9(), 1), m(&[3, 3], &[3, 2, 1, 6, 5, 4, 9, 8, 7]));
    assert_eq!(reverse(&m9(), 0), m(&[3, 3], &[7, 8, 9, 4, 5, 6, 1, 2, 3]));
    assert_eq!(reverse(&s(7), 0), s(7));
    assert_eq!(reverse(&v(&[]), 0), v(&[]));
    assert_eq!(reverse(&chars(&[3], "ABC"), 0), chars(&[3], "CBA"));
}

#[test]
fn rotate_by_scalar_and_by_vector() {
    let a = v(&[1, 2, 3, 4, 5]);
    assert_eq!(rotate(&s(2), &a, 0).unwrap(), v(&[3, 4, 5, 1, 2]));
    assert_eq!(rotate(&s(-1), &a, 0).unwrap(), v(&[5, 1, 2, 3, 4]));
    assert_eq!(rotate(&s(0), &a, 0).unwrap(), a);
    assert_eq!(rotate(&s(5), &a, 0).unwrap(), a);
    assert_eq!(
        rotate(&s(1), &m9(), 1).unwrap(),
        m(&[3, 3], &[2, 3, 1, 5, 6, 4, 8, 9, 7])
    );
    assert_eq!(
        rotate(&s(1), &m9(), 0).unwrap(),
        m(&[3, 3], &[4, 5, 6, 7, 8, 9, 1, 2, 3])
    );
    assert_eq!(
        rotate(&v(&[0, 1, 2]), &m9(), 1).unwrap(),
        m(&[3, 3], &[1, 2, 3, 5, 6, 4, 9, 7, 8])
    );
    assert_eq!(
        rotate(&v(&[1, 0, -1]), &m9(), 0).unwrap(),
        m(&[3, 3], &[4, 2, 9, 7, 5, 3, 1, 8, 6])
    );
    assert_eq!(
        rotate(&s(2), &chars(&[5], "abcde"), 0).unwrap(),
        chars(&[5], "cdeab")
    );
    assert_eq!(rotate(&s(1), &s(9), 0).unwrap(), s(9));
    assert_eq!(
        rotate(&v(&[1, 2]), &m9(), 1).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(rotate(&m9(), &m9(), 1).unwrap_err().kind, ErrorKind::Rank);
    assert_eq!(
        rotate(&Array::scalar(Number::Float(0.5)), &a, 0)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
}

#[test]
fn monadic_transpose_reverses_the_axes() {
    assert_eq!(
        transpose(None, &m(&[2, 3], &[1, 2, 3, 4, 5, 6]), 1).unwrap(),
        m(&[3, 2], &[1, 4, 2, 5, 3, 6])
    );
    assert_eq!(transpose(None, &v(&[1, 2, 3]), 1).unwrap(), v(&[1, 2, 3]));
    assert_eq!(transpose(None, &s(42), 1).unwrap(), s(42));
    let cube = m(&[2, 1, 3], &[1, 2, 3, 4, 5, 6]);
    assert_eq!(
        transpose(None, &cube, 1).unwrap(),
        m(&[3, 1, 2], &[1, 4, 2, 5, 3, 6])
    );
}

#[test]
fn dyadic_transpose_permutes_and_takes_diagonals() {
    let a = m(&[2, 3], &[1, 2, 3, 4, 5, 6]);
    assert_eq!(
        transpose(Some(&v(&[2, 1])), &a, 1).unwrap(),
        transpose(None, &a, 1).unwrap()
    );
    assert_eq!(transpose(Some(&v(&[1, 2])), &a, 1).unwrap(), a);
    assert_eq!(
        transpose(Some(&v(&[1, 1])), &m9(), 1).unwrap(),
        v(&[1, 5, 9])
    );
    assert_eq!(transpose(Some(&v(&[0, 1])), &a, 0).unwrap(), a);
    assert_eq!(
        transpose(Some(&v(&[1, 1])), &a, 1).unwrap(),
        v(&[1, 5]),
        "diagonal of a 2 by 3"
    );
    assert_eq!(
        transpose(Some(&v(&[1])), &a, 1).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        transpose(Some(&v(&[1, 3])), &a, 1).unwrap_err().kind,
        ErrorKind::Domain
    );
    assert_eq!(
        transpose(Some(&v(&[2, 2])), &a, 1).unwrap_err().kind,
        ErrorKind::Domain,
        "axis 1 missing"
    );
}

#[test]
fn axis_index_picks_last_or_first_by_default() {
    let ax = |x: f64| Array::scalar(Number::from_f64(x));
    assert_eq!(axis_index(None, false, 2, 1).unwrap(), 1);
    assert_eq!(axis_index(None, true, 2, 1).unwrap(), 0);
    assert_eq!(axis_index(Some(&ax(1.0)), false, 2, 1).unwrap(), 0);
    assert_eq!(axis_index(Some(&ax(1.0)), true, 2, 1).unwrap(), 0);
    assert_eq!(axis_index(Some(&ax(0.0)), false, 2, 0).unwrap(), 0);
    assert_eq!(
        axis_index(Some(&ax(1.5)), false, 2, 1).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        axis_index(Some(&ax(3.0)), false, 2, 1).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(axis_index(None, false, 0, 1).unwrap(), 0);
}
