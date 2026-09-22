//! Catenate along an axis, laminate, axis resolution.

use apl_prims_join::{Axis, catenate, resolve_axis};
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
fn cat(l: &Array, r: &Array) -> Array {
    catenate(l, r, Axis::Last).unwrap()
}

#[test]
fn vectors_and_scalars_along_the_last_axis() {
    assert_eq!(cat(&v(&[1, 2]), &v(&[3])), v(&[1, 2, 3]));
    assert_eq!(cat(&s(1), &s(2)), v(&[1, 2]));
    assert_eq!(cat(&s(0), &v(&[1, 2])), v(&[0, 1, 2]));
    assert_eq!(cat(&v(&[]), &v(&[1, 2, 3])), v(&[1, 2, 3]));
}

#[test]
fn matrices_along_the_last_axis() {
    let a = m(&[2, 2], &[1, 2, 3, 4]);
    let b = m(&[2, 2], &[5, 6, 7, 8]);
    assert_eq!(cat(&a, &b), m(&[2, 4], &[1, 2, 5, 6, 3, 4, 7, 8]));
    assert_eq!(cat(&a, &v(&[9, 8])), m(&[2, 3], &[1, 2, 9, 3, 4, 8]));
    assert_eq!(cat(&v(&[9, 8]), &a), m(&[2, 3], &[9, 1, 2, 8, 3, 4]));
    assert_eq!(cat(&a, &s(0)), m(&[2, 3], &[1, 2, 0, 3, 4, 0]));
    assert_eq!(cat(&s(7), &a), m(&[2, 3], &[7, 1, 2, 7, 3, 4]));
}

#[test]
fn matrices_along_the_first_axis() {
    let a = m(&[2, 2], &[1, 2, 3, 4]);
    let b = m(&[2, 2], &[5, 6, 7, 8]);
    assert_eq!(
        catenate(&a, &b, Axis::At(0)).unwrap(),
        m(&[4, 2], &[1, 2, 3, 4, 5, 6, 7, 8])
    );
    assert_eq!(
        catenate(&a, &v(&[9, 8]), Axis::At(0)).unwrap(),
        m(&[3, 2], &[1, 2, 3, 4, 9, 8])
    );
    assert_eq!(
        catenate(&a, &s(0), Axis::At(0)).unwrap(),
        m(&[3, 2], &[1, 2, 3, 4, 0, 0])
    );
    let cube = m(&[2, 1, 2], &[1, 2, 3, 4]);
    assert_eq!(
        catenate(&cube, &cube, Axis::At(1)).unwrap(),
        m(&[2, 2, 2], &[1, 2, 1, 2, 3, 4, 3, 4])
    );
}

#[test]
fn shape_rank_and_axis_errors() {
    let a = m(&[2, 2], &[1, 2, 3, 4]);
    let b = m(&[3, 2], &[1, 2, 3, 4, 5, 6]);
    assert_eq!(
        catenate(&a, &b, Axis::Last).unwrap_err().kind,
        ErrorKind::Length
    );
    assert_eq!(
        catenate(&a, &v(&[1, 2, 3]), Axis::Last).unwrap_err().kind,
        ErrorKind::Length
    );
    let cube = m(&[1, 2, 2], &[1, 2, 3, 4]);
    assert_eq!(
        catenate(&a, &cube, Axis::Last).unwrap_err().kind,
        ErrorKind::Length,
        "ranks differing by one conform"
    );
    assert_eq!(
        catenate(&v(&[1, 2]), &cube, Axis::Last).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        catenate(&v(&[1, 2]), &a, Axis::Laminate(0))
            .unwrap_err()
            .kind,
        ErrorKind::Rank
    );
    assert_eq!(
        catenate(&a, &a, Axis::At(2)).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        catenate(&v(&[1]), &chars(&[1], "A"), Axis::Last)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
}

#[test]
fn laminate_makes_a_new_axis_of_length_two() {
    let (a, b) = (v(&[1, 2, 3]), v(&[4, 5, 6]));
    assert_eq!(
        catenate(&a, &b, Axis::Laminate(0)).unwrap(),
        m(&[2, 3], &[1, 2, 3, 4, 5, 6])
    );
    assert_eq!(
        catenate(&a, &b, Axis::Laminate(1)).unwrap(),
        m(&[3, 2], &[1, 4, 2, 5, 3, 6])
    );
    assert_eq!(
        catenate(&v(&[1, 2]), &s(0), Axis::Laminate(0)).unwrap(),
        m(&[2, 2], &[1, 2, 0, 0])
    );
    assert_eq!(
        catenate(&s(1), &s(2), Axis::Laminate(0)).unwrap(),
        v(&[1, 2])
    );
    assert_eq!(
        catenate(&a, &v(&[1, 2]), Axis::Laminate(0))
            .unwrap_err()
            .kind,
        ErrorKind::Length
    );
    assert_eq!(
        catenate(&a, &b, Axis::Laminate(2)).unwrap_err().kind,
        ErrorKind::Index
    );
}

#[test]
fn character_data_and_empties() {
    assert_eq!(
        cat(&chars(&[2], "AB"), &chars(&[2], "CD")),
        chars(&[4], "ABCD")
    );
    assert_eq!(
        cat(&chars(&[2, 2], "ABCD"), &chars(&[2], "XY")),
        chars(&[2, 3], "ABXCDY")
    );
    assert_eq!(
        cat(&chars(&[0], ""), &v(&[1, 2])),
        v(&[1, 2]),
        "an empty is typeless"
    );
    assert_eq!(cat(&v(&[]), &chars(&[1], "A")), chars(&[1], "A"));
}

#[test]
fn axis_resolution_honours_the_origin() {
    let ax = |x: f64| Array::scalar(Number::from_f64(x));
    assert_eq!(resolve_axis(None, 1, 2).unwrap(), Axis::Last);
    assert_eq!(resolve_axis(Some(&ax(1.0)), 1, 2).unwrap(), Axis::At(0));
    assert_eq!(resolve_axis(Some(&ax(2.0)), 1, 2).unwrap(), Axis::At(1));
    assert_eq!(resolve_axis(Some(&ax(0.0)), 0, 2).unwrap(), Axis::At(0));
    assert_eq!(
        resolve_axis(Some(&ax(0.5)), 1, 1).unwrap(),
        Axis::Laminate(0)
    );
    assert_eq!(
        resolve_axis(Some(&ax(1.5)), 1, 1).unwrap(),
        Axis::Laminate(1)
    );
    assert_eq!(
        resolve_axis(Some(&ax(-0.5)), 0, 1).unwrap(),
        Axis::Laminate(0)
    );
    assert_eq!(
        resolve_axis(Some(&ax(3.0)), 1, 2).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        resolve_axis(Some(&ax(0.0)), 1, 2).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        resolve_axis(Some(&ax(2.5)), 1, 1).unwrap_err().kind,
        ErrorKind::Index
    );
    assert_eq!(
        resolve_axis(Some(&v(&[1, 2])), 1, 2).unwrap_err().kind,
        ErrorKind::Rank
    );
    assert_eq!(
        resolve_axis(Some(&chars(&[], "A")), 1, 2).unwrap_err().kind,
        ErrorKind::Domain
    );
}

/// The notes to Table 3.8 of the APL\360 User's Manual: a one-element
/// array may replace any scalar -- the axis in brackets among them.
#[test]
fn an_axis_may_be_a_one_element_array() {
    let one =
        |shape: Vec<usize>, x: i64| Array::new(shape, Data::Num(vec![Number::Int(x)])).unwrap();
    assert_eq!(
        resolve_axis(Some(&one(vec![1], 1)), 1, 2).unwrap(),
        Axis::At(0)
    );
    assert_eq!(
        resolve_axis(Some(&one(vec![1, 1], 2)), 1, 2).unwrap(),
        Axis::At(1)
    );
    let two = Array::vector(vec![Number::Int(1), Number::Int(2)]);
    assert_eq!(
        resolve_axis(Some(&two), 1, 2).unwrap_err().kind,
        ErrorKind::Rank
    );
}
