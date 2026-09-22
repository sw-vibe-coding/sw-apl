//! The eight system values.

use apl_ibeam::{Time, argument, ibeam};
use apl_value::{Array, Data, ErrorKind, Number};

fn at(now: i64, cpu: i64, date: i64) -> Time {
    Time { now, cpu, date }
}

/// The free space every case here is run with, when it is not the
/// value under test: a figure no other I-beam can be confused with.
const FREE: i64 = 4096;

fn one(n: i64, time: Time, signed_on: i64, lines: &[i64]) -> i64 {
    let got = ibeam(n, time, signed_on, lines, FREE).expect("a system value");
    assert!(got.shape.is_empty(), "{n} is a scalar");
    match got.data {
        Data::Num(v) => match v[..] {
            [Number::Int(i)] => i,
            _ => panic!("{n} is one whole number"),
        },
        Data::Char(_) => panic!("{n} is a number"),
    }
}

#[test]
fn each_value_reports_what_its_row_promises() {
    let time = at(1000, 250, 91_726);
    assert_eq!(one(20, time, 7, &[]), 1000, "time of day");
    assert_eq!(one(21, time, 7, &[]), 250, "processor time");
    assert_eq!(one(22, time, 7, &[]), FREE, "space available");
    assert_eq!(one(23, time, 7, &[]), 1, "terminals connected");
    assert_eq!(one(24, time, 7, &[]), 7, "time of sign-on");
    assert_eq!(one(25, time, 7, &[]), 91_726, "today's date");
}

#[test]
fn the_line_values_read_the_state_indicator() {
    let time = Time::default();
    // Innermost first, so 26 is the line now executing.
    assert_eq!(one(26, time, 0, &[4, 8]), 4);
    let lines = ibeam(27, time, 0, &[4, 8], FREE).unwrap();
    assert_eq!(lines.shape, [2]);
    assert_eq!(lines.data, Data::Num(vec![Number::Int(4), Number::Int(8)]));
    // With nothing running, 26 is zero and 27 is empty.
    assert_eq!(one(26, time, 0, &[]), 0);
    assert_eq!(ibeam(27, time, 0, &[], FREE).unwrap().shape, [0]);
}

#[test]
fn any_other_argument_is_domain_error() {
    let time = Time::default();
    for n in [-1, 0, 1, 19, 28, 100] {
        assert_eq!(
            ibeam(n, time, 0, &[], FREE).unwrap_err().kind,
            ErrorKind::Domain,
            "{n}"
        );
    }
}

#[test]
fn the_argument_must_be_one_whole_number() {
    assert_eq!(argument(&Array::scalar(Number::Int(20))).unwrap(), 20);
    for bad in [
        Array::vector(vec![Number::Int(20), Number::Int(21)]),
        Array::vector(Vec::new()),
        Array::scalar(Number::Float(20.5)),
    ] {
        assert_eq!(argument(&bad).unwrap_err().kind, ErrorKind::Domain);
    }
    let chars = Array::new(vec![1], Data::Char(vec!['A'])).unwrap();
    assert_eq!(argument(&chars).unwrap_err().kind, ErrorKind::Domain);
}

/// 22 reports what it is told and counts nothing itself: the
/// workspace does the counting, so a change to the accounting cannot
/// reach in here.
#[test]
fn the_space_available_is_whatever_the_workspace_says() {
    let time = Time::default();
    for free in [0, 1, 4096, i64::MAX] {
        assert_eq!(
            ibeam(22, time, 0, &[], free).unwrap().data,
            Data::Num(vec![Number::Int(free)]),
            "{free}"
        );
    }
}
