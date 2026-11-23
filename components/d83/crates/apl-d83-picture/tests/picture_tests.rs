//! The examples of the APL Reference (May 1983), pages 7-64 to 7-69
//! and 8-18 to 8-19: each pattern, each argument, each result.

use apl_d83_picture::{CLEAR, picture};
use apl_value::{Array, Data, ErrorKind, Number};

fn pattern(text: &str) -> Array {
    let chars: Vec<char> = text.chars().collect();
    Array {
        shape: vec![chars.len()],
        data: Data::Char(chars),
    }
}

fn numbers(values: &[f64]) -> Array {
    Array::vector(values.iter().map(|x| Number::from_f64(*x)).collect())
}

fn shown(l: &str, r: &[f64], fc: &str) -> String {
    let fc: Vec<char> = fc.chars().collect();
    let z = picture(&pattern(l), &numbers(r), &fc).expect("formats");
    let Data::Char(chars) = z.data else {
        panic!("a picture is characters")
    };
    chars.iter().collect()
}

fn clear(l: &str, r: &[f64]) -> String {
    shown(l, r, CLEAR)
}

#[test]
fn the_clear_format_control() {
    assert_eq!(CLEAR, ".,*0_");
}

#[test]
fn one_field_serves_every_number() {
    let z = clear(" 555.55", &[1.0, 0.0, 10.1, 100.0]);
    assert_eq!(z, ["   1   ", "       ", "  10.1 ", " 100   "].concat());
    assert_eq!(z.chars().count(), 28);
}

#[test]
fn several_fields_take_a_number_each() {
    assert_eq!(clear(" 5 5.5 5.55", &[1.12, 2.12, 3.12]), " 1 2.1 3.12");
    assert_eq!(
        clear(" 005 5.50 5.550", &[1.12, 2.12, 3.12]),
        " 001 2.12 3.120"
    );
    assert_eq!(
        clear("HERE: 5 5.5 ;THERE: 5.55", &[1.12, 2.12, 3.12]),
        "HERE: 1 2.1 ;THERE: 3.12"
    );
}

#[test]
fn a_field_may_hold_decorations() {
    assert_eq!(clear("05/05/05", &[70481.0]), "07/04/81");
    assert_eq!(clear("06/06/06", &[7.0, 4.0, 81.0]), "07/04/81");
}

#[test]
fn one_two_and_three_float_a_decoration_by_sign() {
    let r = [-1.0, 0.0, 10.0, -100.0];
    assert_eq!(clear(" -551.50", &r), "   -1.00     .00   10.00 -100.00");
    assert_eq!(clear("(551.50)", &r), "  (1.00)    .00   10.00 (100.00)");
    assert_eq!(clear(" +552.50", &r), "    1.00    +.00  +10.00  100.00");
    assert_eq!(clear(" $553.50", &r), "   $1.00    $.00  $10.00 $100.00");
}

#[test]
fn four_holds_a_decoration_in_place() {
    assert_eq!(
        clear(" -551.45*", &[-1.0, 0.0, 10.1, -100.0]),
        ["   -1   *", "        *", "   10.1 *", " -100   *"].concat()
    );
}

#[test]
fn seven_scales_and_eight_fills_and_nine_pads_what_is_not_zero() {
    assert_eq!(clear("1.70*00", &[12345.0]), "1.23*04");
    assert_eq!(
        clear(" 8555.50", &[1.0, 0.0, 10.0, 100.0]),
        " ***1.00 ****.00 **10.00 *100.00"
    );
    assert_eq!(
        clear(" 555.59", &[1.0, 0.0, 100.0]),
        "   1.00        100.00"
    );
}

#[test]
fn a_number_too_wide_is_an_error_or_the_overflow_fill() {
    let fc: Vec<char> = CLEAR.chars().collect();
    let err = picture(&pattern(" 555.59"), &numbers(&[1.0, 1000.0]), &fc).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Domain);
    assert_eq!(
        shown(" 555.59", &[1.0, 1000.0, 100.0], ".,*?_"),
        "   1.00 ?????? 100.00"
    );
    assert_eq!(shown("5555", &[123_456.0], ".,*?_"), "????");
}

#[test]
fn the_format_control_names_the_characters() {
    assert_eq!(shown("5.5555", &[2.5625], ",,*0_"), "2,5625");
    assert_eq!(
        shown("555,555,555", &[123_456_789.0], "..*0_"),
        "123.456.789"
    );
    assert_eq!(shown("555,555,555", &[1234.0], CLEAR), "      1,234");
    assert_eq!(shown("855555", &[1234.0], ".,00_"), "001234");
    assert_eq!(shown("$e355", &[12.0], ".,*0e"), "$  12");
}

#[test]
fn a_matrix_is_formatted_row_by_row() {
    let r = Array {
        shape: vec![2, 2],
        data: Data::Num(
            [1.5, 2.0, -3.0, 40.0]
                .iter()
                .map(|x| Number::from_f64(*x))
                .collect(),
        ),
    };
    let fc: Vec<char> = CLEAR.chars().collect();
    let z = picture(&pattern(" -15.5"), &r, &fc).expect("formats");
    assert_eq!(z.shape, vec![2, 12]);
    let Data::Char(chars) = z.data else { panic!() };
    let text: String = chars.iter().collect();
    assert_eq!(text, ["   1.5", "   2  ", "  -3  ", "  40  "].concat());
}

#[test]
fn the_arguments_are_a_pattern_and_numbers() {
    let fc: Vec<char> = CLEAR.chars().collect();
    let chars = pattern("ABC");
    assert_eq!(
        picture(&pattern("55"), &chars, &fc).unwrap_err().kind,
        ErrorKind::Domain
    );
    assert_eq!(
        picture(&pattern(" 5 5"), &numbers(&[1.0, 2.0, 3.0]), &fc)
            .unwrap_err()
            .kind,
        ErrorKind::Length
    );
    let matrix = Array {
        shape: vec![1, 2],
        data: Data::Char(vec!['5', '5']),
    };
    assert_eq!(
        picture(&matrix, &numbers(&[1.0]), &fc).unwrap_err().kind,
        ErrorKind::Rank
    );
}
