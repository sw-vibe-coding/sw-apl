//! `⎕DL`, on a thread with no session to stop it: no wait is taken,
//! and the one asked for is given back.

use apl_value::{Array, Data, ErrorKind, Number};

fn num(ns: &[f64]) -> Array {
    Array::vector(ns.iter().map(|n| Number::from_f64(*n)).collect())
}

#[test]
fn a_delay_gives_back_the_seconds() {
    let got = apl_d83_delay::delay(&Array::scalar(Number::from_f64(0.25))).unwrap();
    assert_eq!(got, Array::scalar(Number::from_f64(0.25)));
    let one = apl_d83_delay::delay(&num(&[2.0])).unwrap();
    assert_eq!(one, Array::scalar(Number::Int(2)));
}

#[test]
fn a_delay_takes_one_number() {
    let kind = |r: &Array| apl_d83_delay::delay(r).unwrap_err().kind;
    assert_eq!(kind(&num(&[1.0, 2.0])), ErrorKind::Length);
    let chars = Array {
        shape: vec![],
        data: Data::Char(vec!['A']),
    };
    assert_eq!(kind(&chars), ErrorKind::Domain);
    let matrix = Array {
        shape: vec![1, 1],
        data: Data::Num(vec![Number::Int(1)]),
    };
    assert_eq!(kind(&matrix), ErrorKind::Rank);
}
