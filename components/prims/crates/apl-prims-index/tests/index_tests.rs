//! Bracket indexing and indexed assignment.

use apl_prims_index::{index, indexed_assign};
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
fn some(xs: &[Array]) -> Vec<Option<Array>> {
    xs.iter().cloned().map(Some).collect()
}

#[test]
fn vector_indexing_takes_the_shape_of_the_index() {
    let a = v(&[10, 20, 30, 40, 50]);
    assert_eq!(index(&a, &some(&[s(2)]), 1).unwrap(), s(20));
    assert_eq!(index(&a, &some(&[v(&[5, 1])]), 1).unwrap(), v(&[50, 10]));
    assert_eq!(
        index(&a, &some(&[m(&[2, 2], &[1, 2, 3, 4])]), 1).unwrap(),
        m(&[2, 2], &[10, 20, 30, 40])
    );
    assert_eq!(index(&a, &some(&[v(&[])]), 1).unwrap(), v(&[]));
    assert_eq!(index(&a, &some(&[s(0)]), 0).unwrap(), s(10));
    assert_eq!(
        index(&chars(&[5], "ABCDE"), &some(&[v(&[3, 1])]), 1).unwrap(),
        chars(&[2], "CA")
    );
    assert_eq!(index(&a, &[None], 1).unwrap(), a);
}

#[test]
fn matrix_indexing_with_elided_axes() {
    assert_eq!(index(&m9(), &some(&[s(2), s(3)]), 1).unwrap(), s(6));
    assert_eq!(
        index(&m9(), &[Some(v(&[1, 3])), None], 1).unwrap(),
        m(&[2, 3], &[1, 2, 3, 7, 8, 9])
    );
    assert_eq!(index(&m9(), &[None, Some(s(2))], 1).unwrap(), v(&[2, 5, 8]));
    assert_eq!(
        index(&m9(), &[Some(s(2)), Some(v(&[1, 3]))], 1).unwrap(),
        v(&[4, 6])
    );
    assert_eq!(
        index(&m9(), &[Some(v(&[1, 2])), Some(v(&[1, 2]))], 1).unwrap(),
        m(&[2, 2], &[1, 2, 4, 5])
    );
    assert_eq!(index(&m9(), &[None, None], 1).unwrap(), m9());
    let cube = m(&[2, 2, 2], &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        index(&cube, &[Some(s(2)), None, Some(s(1))], 1).unwrap(),
        v(&[5, 7])
    );
}

#[test]
fn indexing_errors() {
    let a = v(&[10, 20, 30]);
    assert_eq!(
        index(&a, &some(&[s(4)]), 1).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        index(&a, &some(&[s(0)]), 1).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        index(&a, &some(&[s(3)]), 0).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        index(&a, &some(&[s(1), s(1)]), 1).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        index(&m9(), &some(&[s(1)]), 1).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        index(&s(5), &some(&[s(1)]), 1).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        index(&a, &some(&[Array::scalar(Number::Float(1.5))]), 1)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
    assert_eq!(
        index(&a, &some(&[chars(&[], "A")]), 1).unwrap_err().kind,
        ErrorKind::Domain
    );
}

#[test]
fn indexed_assignment_with_scalar_extension() {
    let a = v(&[10, 20, 30, 40, 50]);
    assert_eq!(
        indexed_assign(&a, &some(&[s(2)]), &s(99), 1).unwrap(),
        v(&[10, 99, 30, 40, 50])
    );
    assert_eq!(
        indexed_assign(&a, &some(&[v(&[1, 5])]), &v(&[1, 5]), 1).unwrap(),
        v(&[1, 20, 30, 40, 5])
    );
    assert_eq!(
        indexed_assign(&a, &some(&[v(&[1, 5])]), &s(0), 1).unwrap(),
        v(&[0, 20, 30, 40, 0])
    );
    assert_eq!(
        indexed_assign(&m9(), &[Some(s(2)), None], &s(0), 1).unwrap(),
        m(&[3, 3], &[1, 2, 3, 0, 0, 0, 7, 8, 9])
    );
    assert_eq!(
        indexed_assign(&m9(), &[Some(s(1)), Some(v(&[1, 2]))], &v(&[7, 8]), 1).unwrap(),
        m(&[3, 3], &[7, 8, 3, 4, 5, 6, 7, 8, 9])
    );
    assert_eq!(
        indexed_assign(&chars(&[3], "ABC"), &some(&[s(2)]), &chars(&[], "x"), 1).unwrap(),
        chars(&[3], "AxC")
    );
    assert_eq!(
        indexed_assign(&a, &some(&[v(&[1, 2, 3])]), &v(&[1, 2]), 1)
            .unwrap_err()
            .kind,
        ErrorKind::Length
    );
    assert_eq!(
        indexed_assign(&a, &some(&[v(&[1, 2])]), &m(&[1, 2], &[1, 2]), 1)
            .unwrap_err()
            .kind,
        ErrorKind::Rank
    );
    assert_eq!(
        indexed_assign(&a, &some(&[s(1)]), &chars(&[], "x"), 1)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
    assert_eq!(
        indexed_assign(&a, &some(&[s(9)]), &s(1), 1)
            .unwrap_err()
            .kind,
        ErrorKind::Index
    );
}
