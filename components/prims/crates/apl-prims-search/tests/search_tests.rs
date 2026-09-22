//! Membership and index-of.

use apl_value::{AplResult, Array, Data, ErrorKind, FUZZ, Number};

// At APL\360's fixed tolerance, which is what every test here
// assumes; `⎕CT` in (B) is pinned by the session tests.
fn membership(l: &Array, r: &Array) -> Array {
    apl_prims_search::membership(l, r, FUZZ)
}

fn index_of(l: &Array, r: &Array, io: i64) -> AplResult<Array> {
    apl_prims_search::index_of(l, r, io, FUZZ)
}

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

#[test]
fn grade_up_and_down_are_stable_and_origin_aware() {
    use apl_prims_search::grade;
    assert_eq!(grade(&v(&[30, 10, 20]), false, 1).unwrap(), v(&[2, 3, 1]));
    assert_eq!(grade(&v(&[30, 10, 20]), true, 1).unwrap(), v(&[1, 3, 2]));
    assert_eq!(
        grade(&v(&[5, 3, 1, 4, 2]), false, 1).unwrap(),
        v(&[3, 5, 2, 4, 1])
    );
    assert_eq!(grade(&v(&[30, 10, 20]), false, 0).unwrap(), v(&[1, 2, 0]));
    assert_eq!(
        grade(&v(&[2, 1, 2, 1]), false, 1).unwrap(),
        v(&[2, 4, 1, 3]),
        "ties keep their order"
    );
    assert_eq!(
        grade(&v(&[2, 1, 2, 1]), true, 1).unwrap(),
        v(&[1, 3, 2, 4]),
        "ties keep their order going down too"
    );
    assert_eq!(grade(&v(&[]), false, 1).unwrap(), v(&[]));
    assert_eq!(
        grade(
            &Array::vector(vec![Number::Float(1.5), Number::Int(1)]),
            false,
            1
        )
        .unwrap(),
        v(&[2, 1])
    );
    assert_eq!(grade(&s(1), false, 1).unwrap_err().kind, ErrorKind::Rank);
    assert_eq!(
        grade(&chars(&[2], "ab"), false, 1).unwrap_err().kind,
        ErrorKind::Domain
    );
}
