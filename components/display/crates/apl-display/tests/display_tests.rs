//! APL\360 output formatting.

use apl_display::{format_array, format_number};
use apl_value::{Array, Data, Number};

const PP: usize = 10;

#[test]
fn integers_use_high_minus() {
    assert_eq!(format_number(Number::Int(42), PP), "42");
    assert_eq!(format_number(Number::Int(-5), PP), "\u{af}5");
}

#[test]
fn floats_show_significant_digits_without_trailing_zeros() {
    assert_eq!(format_number(Number::Float(2.5), PP), "2.5");
    assert_eq!(format_number(Number::Float(0.25), PP), "0.25");
    assert_eq!(format_number(Number::Float(-0.015), PP), "\u{af}0.015");
    assert_eq!(format_number(Number::Float(1.0 / 3.0), PP), "0.3333333333");
    assert_eq!(format_number(Number::Float(2.0 / 3.0), 5), "0.66667");
    assert_eq!(format_number(Number::Float(123_456.75), PP), "123456.75");
}

#[test]
fn large_and_small_floats_use_exponent_form() {
    assert_eq!(format_number(Number::Float(1e20), PP), "1E20");
    assert_eq!(format_number(Number::Float(1.5e-7), PP), "1.5E\u{af}7");
    assert_eq!(format_number(Number::Float(-2.5e12), PP), "\u{af}2.5E12");
}

#[test]
fn scalars_and_vectors_are_single_lines() {
    assert_eq!(format_array(&Array::scalar(Number::Int(7)), PP), vec!["7"]);
    let v = Array::vector(vec![Number::Int(1), Number::Int(-2), Number::Float(2.5)]);
    assert_eq!(format_array(&v, PP), vec!["1 \u{af}2 2.5"]);
}

#[test]
fn empty_vector_is_a_blank_line() {
    assert_eq!(format_array(&Array::vector(vec![]), PP), vec![""]);
}

#[test]
fn matrix_columns_are_right_aligned() {
    let m = Array::new(
        vec![2, 3],
        Data::Num(vec![
            Number::Int(1),
            Number::Int(20),
            Number::Int(3),
            Number::Int(400),
            Number::Int(5),
            Number::Int(-6),
        ]),
    )
    .unwrap();
    assert_eq!(format_array(&m, PP), vec!["  1 20  3", "400  5 \u{af}6"]);
}
