//! Arithmetic scalar functions on f64 (results demoted by the caller).

use apl_scalar_arith::{dyadic_arith, monadic_arith};
use apl_value::ErrorKind;

fn m(f: char, a: f64) -> Result<f64, ErrorKind> {
    monadic_arith(f, a)
        .expect("arith glyph")
        .map_err(|e| e.kind)
}
fn d(f: char, a: f64, b: f64) -> Result<f64, ErrorKind> {
    dyadic_arith(f, a, b)
        .expect("arith glyph")
        .map_err(|e| e.kind)
}

#[test]
fn monadic_table() {
    assert_eq!(m('+', 5.0), Ok(5.0));
    assert_eq!(m('-', 5.0), Ok(-5.0));
    assert_eq!(m('\u{d7}', -3.0), Ok(-1.0));
    assert_eq!(m('\u{d7}', 0.0), Ok(0.0));
    assert_eq!(m('\u{f7}', 4.0), Ok(0.25));
    assert_eq!(m('\u{f7}', 0.0), Err(ErrorKind::Domain));
    assert_eq!(m('|', -2.5), Ok(2.5));
    assert_eq!(m('*', 0.0), Ok(1.0));
    assert_eq!(m('\u{235f}', 1.0), Ok(0.0));
    assert!((m('\u{235f}', std::f64::consts::E).unwrap() - 1.0).abs() < 1e-12);
    assert_eq!(m('\u{235f}', 0.0), Err(ErrorKind::Domain));
    assert_eq!(m('\u{235f}', -1.0), Err(ErrorKind::Domain));
    assert_eq!(monadic_arith('!', 1.0), None, "not an arith glyph");
}

#[test]
fn floor_and_ceiling_are_tolerant() {
    assert_eq!(m('\u{230a}', 2.5), Ok(2.0));
    assert_eq!(m('\u{230a}', -2.5), Ok(-3.0));
    assert_eq!(m('\u{230a}', 3.0 - 1e-14), Ok(3.0), "within the fuzz of 3");
    assert_eq!(m('\u{2308}', 2.5), Ok(3.0));
    assert_eq!(m('\u{2308}', 2.0 + 1e-14), Ok(2.0));
}

#[test]
fn dyadic_table() {
    assert_eq!(d('+', 3.0, 4.0), Ok(7.0));
    assert_eq!(d('-', 10.0, 3.0), Ok(7.0));
    assert_eq!(d('\u{d7}', 6.0, 4.0), Ok(24.0));
    assert_eq!(d('\u{f7}', 20.0, 5.0), Ok(4.0));
    assert_eq!(d('\u{f7}', 0.0, 0.0), Ok(1.0));
    assert_eq!(d('\u{f7}', 1.0, 0.0), Err(ErrorKind::Domain));
    assert_eq!(d('\u{2308}', 1.0, 4.0), Ok(4.0));
    assert_eq!(d('\u{230a}', 1.0, 4.0), Ok(1.0));
    assert_eq!(d('*', 2.0, 10.0), Ok(1024.0));
    assert_eq!(d('*', 0.0, 0.0), Ok(1.0));
    assert_eq!(d('*', -8.0, 1.0 / 3.0), Err(ErrorKind::Domain));
    assert_eq!(d('\u{235f}', 2.0, 8.0), Ok(3.0));
    assert!((d('\u{235f}', 10.0, 1000.0).unwrap() - 3.0).abs() < 1e-12);
    assert_eq!(d('\u{235f}', 1.0, 5.0), Err(ErrorKind::Domain));
    assert_eq!(d('\u{235f}', 2.0, 0.0), Err(ErrorKind::Domain));
    assert_eq!(dyadic_arith('=', 1.0, 1.0), None, "not an arith glyph");
}

#[test]
fn residue_follows_apl_and_is_tolerant() {
    assert_eq!(d('|', 3.0, 7.0), Ok(1.0));
    assert_eq!(d('|', -3.0, 10.0), Ok(-2.0));
    assert_eq!(d('|', 3.0, -7.0), Ok(2.0));
    assert_eq!(d('|', 0.0, 7.0), Ok(7.0));
    assert_eq!(d('|', 0.5, 3.75), Ok(0.25));
    assert_eq!(
        d('|', 1.0, 3.0 - 1e-14),
        Ok(0.0),
        "tolerantly integral quotient"
    );
}
