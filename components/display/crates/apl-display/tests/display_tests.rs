//! APL\360 output formatting.

use apl_display::{Precision, format_array, format_number};
use apl_value::{Array, Data, Number};

const PP: usize = 10;
const W: usize = 120;

fn ints(xs: &[i64]) -> Array {
    Array::vector(xs.iter().map(|&x| Number::Int(x)).collect())
}
fn shaped(shape: &[usize], xs: &[i64]) -> Array {
    Array::new(
        shape.to_vec(),
        Data::Num(xs.iter().map(|&x| Number::Int(x)).collect()),
    )
    .unwrap()
}
fn chars(shape: &[usize], s: &str) -> Array {
    Array::new(shape.to_vec(), Data::Char(s.chars().collect())).unwrap()
}

#[test]
fn integers_use_high_minus() {
    assert_eq!(format_number(Number::Int(42), PP), "42");
    assert_eq!(format_number(Number::Int(-5), PP), "\u{af}5");
}

#[test]
fn integers_beyond_digits_use_exponent_form() {
    assert_eq!(
        format_number(Number::Int(1_099_511_627_776), PP),
        "1.099511628E12"
    );
    assert_eq!(format_number(Number::Int(10_000_000_000), PP), "1E10");
    assert_eq!(format_number(Number::Int(9_999_999_999), PP), "9999999999");
    assert_eq!(format_number(Number::Int(-123_456), 4), "\u{af}1.235E5");
    assert_eq!(format_number(Number::Int(1234), 4), "1234");
}

#[test]
fn floats_show_significant_digits_without_trailing_zeros() {
    let table = [
        (2.5, PP, "2.5"),
        (0.25, PP, "0.25"),
        (-0.015, PP, "\u{af}0.015"),
        (1.0 / 3.0, PP, "0.3333333333"),
        (2.0 / 3.0, 5, "0.66667"),
        (123_456.75, PP, "123456.75"),
        (1e-5, PP, "0.00001"),
        (99.999_999_999_99, PP, "100"),
    ];
    for (x, pp, want) in table {
        assert_eq!(format_number(Number::Float(x), pp), want, "{x} at {pp}");
    }
}

#[test]
fn large_and_small_floats_use_exponent_form() {
    let table = [
        (1e20, PP, "1E20"),
        (1.5e-7, PP, "1.5E\u{af}7"),
        (-2.5e12, PP, "\u{af}2.5E12"),
        (1e-6, PP, "1E\u{af}6"),
        (1e10, PP, "1E10"),
        (1e10, 11, "10000000000"), // 11 digits fit at )DIGITS 11
        (123_456_789_012.0, 4, "1.235E11"),
    ];
    for (x, pp, want) in table {
        assert_eq!(format_number(Number::Float(x), pp), want, "{x} at {pp}");
    }
}

#[test]
fn scalars_and_vectors_are_single_lines() {
    assert_eq!(
        format_array(&Array::scalar(Number::Int(7)), PP, W),
        vec!["7"]
    );
    let v = Array::vector(vec![Number::Int(1), Number::Int(-2), Number::Float(2.5)]);
    assert_eq!(format_array(&v, PP, W), vec!["1 \u{af}2 2.5"]);
}

#[test]
fn empty_arrays_of_any_shape_print_one_blank_line() {
    for shape in [vec![0], vec![0, 3], vec![2, 0], vec![0, 0, 4]] {
        let a = Array::new(shape.clone(), Data::Num(vec![])).unwrap();
        assert_eq!(format_array(&a, PP, W), vec![""], "{shape:?}");
    }
    assert_eq!(format_array(&chars(&[0], ""), PP, W), vec![""]);
}

#[test]
fn matrix_columns_are_right_aligned_per_element() {
    let m = shaped(&[2, 3], &[1, 20, 3, 400, 5, -6]);
    assert_eq!(format_array(&m, PP, W), vec!["  1 20  3", "400  5 \u{af}6"]);
    let mixed = Array::new(
        vec![2, 2],
        Data::Num(vec![
            Number::Int(1),
            Number::Float(2.5),
            Number::Int(3),
            Number::Int(4),
        ]),
    )
    .unwrap();
    assert_eq!(format_array(&mixed, PP, W), vec!["1 2.5", "3   4"]);
}

#[test]
fn character_arrays_print_without_quotes_or_spacing() {
    assert_eq!(format_array(&chars(&[], "A"), PP, W), vec!["A"]);
    assert_eq!(format_array(&chars(&[5], "HELLO"), PP, W), vec!["HELLO"]);
    assert_eq!(
        format_array(&chars(&[2, 3], "ABCDEF"), PP, W),
        vec!["ABC", "DEF"]
    );
}

#[test]
fn rank_three_and_four_separate_planes_with_blank_lines() {
    let cube = shaped(&[2, 2, 3], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    assert_eq!(
        format_array(&cube, PP, W),
        vec![" 1  2  3", " 4  5  6", "", " 7  8  9", "10 11 12"]
    );
    let four = shaped(&[2, 1, 2, 2], &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        format_array(&four, PP, W),
        vec!["1 2", "3 4", "", "", "5 6", "7 8"]
    );
}

#[test]
fn rank_three_columns_align_across_planes() {
    let cube = shaped(&[2, 1, 2], &[1, 2, 100, 200]);
    assert_eq!(format_array(&cube, PP, W), vec!["  1   2", "", "100 200"]);
}

#[test]
fn long_vectors_wrap_between_elements_with_six_space_continuation() {
    let v = ints(&(1..=30).collect::<Vec<_>>());
    let lines = format_array(&v, PP, 40);
    assert_eq!(
        lines,
        vec![
            "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16",
            "      17 18 19 20 21 22 23 24 25 26 27",
            "      28 29 30",
        ]
    );
    for line in &lines {
        assert!(line.chars().count() <= 40);
    }
}

#[test]
fn character_vectors_wrap_at_the_width() {
    let text: String = "ABCDEFGHIJ".repeat(5);
    let lines = format_array(&chars(&[50], &text), PP, 30);
    assert_eq!(lines[0], "ABCDEFGHIJABCDEFGHIJABCDEFGHIJ");
    assert_eq!(lines[1], "      ABCDEFGHIJABCDEFGHIJ");
}

#[test]
fn wide_matrices_wrap_in_column_blocks() {
    let m = shaped(&[2, 12], &(1..=24).collect::<Vec<_>>());
    assert_eq!(
        format_array(&m, PP, 30),
        vec![
            " 1  2  3  4  5  6  7  8  9 10",
            "13 14 15 16 17 18 19 20 21 22",
            "      11 12",
            "      23 24",
        ]
    );
}

#[test]
fn an_element_wider_than_the_width_still_prints() {
    let v = Array::vector(vec![Number::Float(1.0 / 3.0), Number::Float(2.0 / 3.0)]);
    assert_eq!(format_array(&v, PP, 30), vec!["0.3333333333 0.6666666667"]);
    assert_eq!(
        format_array(&v, PP, 8),
        vec!["0.3333333333", "      0.6666666667"]
    );
}

#[test]
fn a_combining_low_line_takes_no_column() {
    // An underscored letter prints in one position, as it did on
    // paper: the low line lands on the letter. `)WIDTH` must count
    // it as nothing, or a line holding one wraps a character early.
    let text = chars(&[8], "A\u{332}B\u{332}C\u{332}D\u{332}");
    assert_eq!(
        format_array(&text, PP, 4),
        vec!["A\u{332}B\u{332}C\u{332}D\u{332}"]
    );
    assert_eq!(
        format_array(&text, PP, 3),
        vec!["A\u{332}B\u{332}C\u{332}", "      D\u{332}"]
    );
}

#[test]
fn a_whole_number_can_be_shown_in_full_past_the_precision() {
    // The 5110's rule: five digits, but a whole number of up to ten
    // is shown in full, and only a longer one in E form.
    let b = Precision {
        digits: 5,
        whole: 10,
    };
    assert_eq!(format_number(Number::Int(1_048_576), b), "1048576");
    assert_eq!(format_number(Number::Int(1_234_567_890), b), "1234567890");
    assert_eq!(format_number(Number::Int(12_345_678_901), b), "1.2346E10");
    assert_eq!(format_number(Number::Float(1.0 / 3.0), b), "0.33333");
    // APL\360's: a whole number longer than the digits is E form.
    assert_eq!(format_number(Number::Int(1_048_576), 5), "1.0486E6");
}
