//! Membership and index-of.

use apl_prims_search::{index_of, membership};
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
fn chars(shape: &[usize], t: &str) -> Array {
    Array::new(shape.to_vec(), Data::Char(t.chars().collect())).unwrap()
}

#[test]
fn membership_by_element_with_the_fuzz() {
    assert_eq!(
        membership(&v(&[1, 2, 3, 4]), &v(&[2, 4, 6])),
        v(&[0, 1, 0, 1])
    );
    assert_eq!(membership(&s(5), &v(&[1, 2, 3, 4, 5])), s(1));
    assert_eq!(membership(&s(5), &v(&[1, 2, 3])), s(0));
    assert_eq!(
        membership(&chars(&[5], "hello"), &chars(&[5], "aeiou")),
        v(&[0, 1, 0, 0, 1])
    );
    assert_eq!(
        membership(&v(&[1]), &chars(&[1], "1")),
        v(&[0]),
        "numbers never match characters"
    );
    let close = Array::scalar(Number::Float(0.1 + 0.2));
    assert_eq!(membership(&close, &Array::scalar(Number::Float(0.3))), s(1));
    let m = Array::new(vec![2, 2], Data::Num(ints(&[1, 2, 3, 4]))).unwrap();
    assert_eq!(membership(&m, &v(&[2, 3])).shape, vec![2, 2]);
    assert_eq!(
        membership(&v(&[1, 2]), &m),
        v(&[1, 1]),
        "the right argument may have any rank"
    );
}

#[test]
fn index_of_returns_the_first_position_or_one_past_the_end() {
    let l = v(&[10, 20, 30]);
    assert_eq!(index_of(&l, &s(20), 1).unwrap(), s(2));
    assert_eq!(index_of(&l, &s(40), 1).unwrap(), s(4));
    assert_eq!(index_of(&l, &v(&[10, 30]), 1).unwrap(), v(&[1, 3]));
    assert_eq!(index_of(&l, &s(20), 0).unwrap(), s(1));
    assert_eq!(index_of(&l, &s(40), 0).unwrap(), s(3));
    assert_eq!(
        index_of(&v(&[5, 5]), &s(5), 1).unwrap(),
        s(1),
        "first occurrence"
    );
    assert_eq!(
        index_of(&chars(&[3], "abc"), &chars(&[2], "cz"), 1).unwrap(),
        v(&[3, 4])
    );
    assert_eq!(index_of(&l, &chars(&[1], "x"), 1).unwrap(), v(&[4]));
    let m = Array::new(vec![2, 2], Data::Num(ints(&[1, 2, 3, 4]))).unwrap();
    assert_eq!(index_of(&l, &m, 1).unwrap().shape, vec![2, 2]);
    assert_eq!(index_of(&m, &s(1), 1).unwrap_err().kind, ErrorKind::Rank);
    assert_eq!(index_of(&s(1), &s(1), 1).unwrap_err().kind, ErrorKind::Rank);
}
