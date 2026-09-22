//! Comparisons with the fuzz and boolean functions.

use apl_scalar_logic::{boolean, not};
use apl_value::{ErrorKind, FUZZ, Number};

// At APL\360's fixed tolerance, which is what every test here
// assumes; `⎕CT` in (B) is pinned by the session tests.
fn compare(f: char, l: Number, r: Number) -> Option<bool> {
    apl_scalar_logic::compare(f, l, r, FUZZ)
}

fn i(v: i64) -> Number {
    Number::Int(v)
}

#[test]
fn six_comparisons() {
    let (a, b) = (i(3), i(5));
    assert_eq!(compare('<', a, b), Some(true));
    assert_eq!(compare('\u{2264}', a, b), Some(true));
    assert_eq!(compare('=', a, b), Some(false));
    assert_eq!(compare('\u{2265}', a, b), Some(false));
    assert_eq!(compare('>', a, b), Some(false));
    assert_eq!(compare('\u{2260}', a, b), Some(true));
    assert_eq!(compare('=', a, a), Some(true));
    assert_eq!(compare('\u{2264}', a, a), Some(true));
    assert_eq!(compare('+', a, a), None, "not a comparison");
}

#[test]
fn comparisons_use_the_fuzz() {
    let x = Number::Float(0.1 + 0.2);
    let y = Number::Float(0.3);
    assert_eq!(compare('=', x, y), Some(true));
    assert_eq!(
        compare('<', x, y),
        Some(false),
        "tolerantly equal is not less"
    );
    assert_eq!(compare('\u{2265}', y, x), Some(true));
    assert_eq!(compare('\u{2260}', x, y), Some(false));
}

#[test]
fn boolean_functions_accept_only_0_and_1() {
    assert!(boolean('\u{2227}', 1.0, 1.0).unwrap().unwrap());
    assert!(!boolean('\u{2227}', 1.0, 0.0).unwrap().unwrap());
    assert!(boolean('\u{2228}', 0.0, 1.0).unwrap().unwrap());
    assert!(!boolean('\u{2372}', 1.0, 1.0).unwrap().unwrap());
    assert!(boolean('\u{2371}', 0.0, 0.0).unwrap().unwrap());
    assert_eq!(
        boolean('\u{2227}', 2.0, 1.0).unwrap().unwrap_err().kind,
        ErrorKind::Domain
    );
    assert!(boolean('+', 1.0, 1.0).is_none());
    assert!(not(0.0).unwrap());
    assert!(!not(1.0).unwrap());
    assert_eq!(not(0.5).unwrap_err().kind, ErrorKind::Domain);
}
