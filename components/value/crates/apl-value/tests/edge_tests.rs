//! The boundaries: where an integer stops being one, and how wide
//! the fuzz actually is.
//!
//! The manual is specific about the fuzz and vague about its form:
//! "For operations such as floor and ceiling, and in comparisons, a
//! 'fuzz' of about 1E¯13 is applied in order to avoid anomalous
//! results that might otherwise be engendered by doing decimal
//! arithmetic on a binary machine." About 1E¯13, and nothing about
//! relative or absolute. That it is relative is ours, and these
//! tests are where that choice is written down.

use apl_value::{FUZZ, Number};

fn int(i: i64) -> Number {
    Number::Int(i)
}

fn float(x: f64) -> Number {
    Number::Float(x)
}

#[test]
fn exact_arithmetic_stops_at_the_edge_of_i64() {
    // Adding one to the largest i64 cannot be exact, so there is no
    // exact answer to give and the caller takes the floating path.
    assert_eq!(Number::exact_int('+', int(i64::MAX), int(1)), None);
    assert_eq!(Number::exact_int('-', int(i64::MIN), int(1)), None);
    assert_eq!(Number::exact_int('×', int(i64::MAX), int(2)), None);
    // One short of the edge is still exact.
    assert_eq!(
        Number::exact_int('+', int(i64::MAX - 1), int(1)),
        Some(int(i64::MAX))
    );
    assert_eq!(
        Number::exact_int('-', int(i64::MIN + 1), int(1)),
        Some(int(i64::MIN))
    );
}

#[test]
fn only_two_integers_take_the_exact_path() {
    assert_eq!(Number::exact_int('+', int(1), float(2.0)), None);
    assert_eq!(Number::exact_int('+', float(1.0), int(2)), None);
    // And only the three functions that have an exact answer.
    assert_eq!(Number::exact_int('÷', int(4), int(2)), None);
    assert_eq!(Number::exact_int('*', int(2), int(3)), None);
}

#[test]
fn a_float_demotes_to_an_integer_only_below_two_to_the_fifty_three() {
    // 2^53 is the last integer f64 represents exactly, so above it
    // "this is a whole number" stops meaning what it says.
    let limit = 9_007_199_254_740_992.0_f64;
    assert_eq!(Number::from_f64(limit - 1.0), int(9_007_199_254_740_991));
    assert!(matches!(Number::from_f64(limit), Number::Float(_)));
    assert!(matches!(Number::from_f64(-limit), Number::Float(_)));
    assert_eq!(
        Number::from_f64(-(limit - 1.0)),
        int(-9_007_199_254_740_991)
    );
    // A fraction stays a fraction however small.
    assert!(matches!(Number::from_f64(0.5), Number::Float(_)));
}

#[test]
fn the_fuzz_is_relative_so_it_grows_with_the_numbers() {
    // At 1, the fuzz is the fuzz.
    assert!(int(1).tolerant_eq(float(1.0 + FUZZ / 2.0), FUZZ));
    assert!(!int(1).tolerant_eq(float(1.0 + FUZZ * 10.0), FUZZ));
    // At a million, it is a million times wider, which is the point
    // of a relative tolerance: the same number of significant
    // digits agree.
    let big = 1e6;
    assert!(float(big).tolerant_eq(float(big * (1.0 + FUZZ / 2.0)), FUZZ));
    assert!(!float(big).tolerant_eq(float(big * (1.0 + FUZZ * 10.0)), FUZZ));
}

#[test]
fn zero_has_no_slack_at_all() {
    // The tolerance is a fraction of the larger magnitude, and both
    // are zero here, so nothing but zero equals zero. Without this,
    // every small number would be zero and division would surprise.
    assert!(!float(1e-300).tolerant_eq(int(0), FUZZ));
    assert!(!float(f64::MIN_POSITIVE).tolerant_eq(int(0), FUZZ));
    assert!(int(0).tolerant_eq(float(0.0), FUZZ));
    assert!(
        int(0).tolerant_eq(float(-0.0), FUZZ),
        "and so does minus zero"
    );
}

#[test]
fn two_integers_compare_exactly_whatever_the_tolerance() {
    // Integers hold their value exactly, so a tolerance would only
    // make two different ones equal.
    assert!(!int(i64::MAX).tolerant_eq(int(i64::MAX - 1), FUZZ));
    assert!(!int(1).tolerant_eq(int(2), FUZZ));
    assert!(int(7).tolerant_eq(int(7), 0.0));
}

#[test]
fn the_fuzz_is_the_one_the_manual_names() {
    assert!((FUZZ - 1e-13).abs() < f64::EPSILON, "about 1E¯13");
}
