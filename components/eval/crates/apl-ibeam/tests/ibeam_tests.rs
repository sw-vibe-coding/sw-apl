//! The eight system values, and the clock behind the three that vary.

use apl_ibeam::{Time, argument, ibeam, stopped, system};
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

#[test]
fn a_stopped_clock_does_not_move() {
    assert_eq!(stopped(), Time::default());
    assert_eq!(stopped(), stopped(), "a transcript made with it repeats");
}

/// The real clock cannot be pinned to a value, so this pins its
/// shape: the ranges each field must fall in, and that time advances.
#[test]
fn the_system_clock_reports_a_plausible_day() {
    let time = system();
    assert!(
        (0..24 * 60 * 60 * 60).contains(&time.now),
        "sixtieths since midnight: {}",
        time.now
    );
    assert!(time.cpu >= 0, "processor time: {}", time.cpu);
    let (month, day, year) = (time.date / 10_000, (time.date / 100) % 100, time.date % 100);
    assert!((1..=12).contains(&month), "month of {}", time.date);
    assert!((1..=31).contains(&day), "day of {}", time.date);
    assert!((0..=99).contains(&year), "year of {}", time.date);
    // The same day, read twice, is the same day.
    assert_eq!(system().date, time.date);
}

#[test]
fn a_duration_prints_as_hours_minutes_and_seconds() {
    assert_eq!(apl_ibeam::hms(0), "0.00.00");
    assert_eq!(apl_ibeam::hms(60), "0.00.01");
    assert_eq!(apl_ibeam::hms(60 * 59), "0.00.59");
    assert_eq!(apl_ibeam::hms(60 * 60), "0.01.00");
    assert_eq!(apl_ibeam::hms(60 * 3600), "1.00.00");
    assert_eq!(
        apl_ibeam::hms(60 * ((12 * 3600) + (34 * 60) + 56)),
        "12.34.56"
    );
    // A clock that has not been set cannot make a negative duration.
    assert_eq!(apl_ibeam::hms(-1), "0.00.00");
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
