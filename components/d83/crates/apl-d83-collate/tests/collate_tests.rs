//! The manual's examples of the dyadic grades.

use apl_d83_collate::grade;
use apl_value::{Array, Data, ErrorKind, Number};

fn chars(shape: &[usize], text: &str) -> Array {
    Array {
        shape: shape.to_vec(),
        data: Data::Char(text.chars().collect()),
    }
}

fn ints(a: &Array) -> Vec<i64> {
    let Data::Num(ns) = &a.data else {
        panic!("a permutation is numbers")
    };
    ns.iter()
        .map(|n| match n {
            Number::Int(i) => *i,
            Number::Float(_) => panic!("a permutation is whole numbers"),
        })
        .collect()
}

#[test]
fn a_vector_sorts_by_where_its_characters_are_found() {
    let (l, r) = (chars(&[5], "ABCDE"), chars(&[4], "DEAL"));
    assert_eq!(ints(&grade(&l, &r, false, 1).unwrap()), [3, 1, 2, 4]);
    assert_eq!(ints(&grade(&l, &r, true, 1).unwrap()), [4, 2, 1, 3]);
    assert_eq!(ints(&grade(&l, &r, false, 0).unwrap()), [2, 0, 1, 3]);
}

#[test]
fn spelling_counts_before_case() {
    let l = chars(&[2, 5], "abcdeABCDE");
    let r = chars(&[5, 4], "dealDealdeadDeadDEED");
    assert_eq!(ints(&grade(&l, &r, true, 1).unwrap()), [5, 2, 1, 4, 3]);
    assert_eq!(ints(&grade(&l, &r, false, 1).unwrap()), [3, 4, 1, 2, 5]);
}

#[test]
fn an_empty_array_has_an_empty_permutation() {
    let (l, r) = (chars(&[3], "ABC"), chars(&[0, 4], ""));
    assert!(ints(&grade(&l, &r, false, 1).unwrap()).is_empty());
}

#[test]
fn numbers_and_scalars_are_refused() {
    let num = Array::scalar(Number::Int(1));
    let (v, s) = (chars(&[3], "ABC"), chars(&[], "A"));
    let kind = |l: &Array, r: &Array| grade(l, r, false, 1).unwrap_err().kind;
    assert_eq!(kind(&v, &num), ErrorKind::Domain);
    assert_eq!(kind(&num, &v), ErrorKind::Domain);
    assert_eq!(kind(&v, &s), ErrorKind::Rank);
    assert_eq!(kind(&s, &v), ErrorKind::Rank);
}
