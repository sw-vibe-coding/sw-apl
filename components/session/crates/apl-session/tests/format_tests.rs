//! Format, which (B) '75 has and (A) '70 does not: the IBM 5110 APL
//! Reference Manual, "The ⍕ Function: Format". The examples here are
//! the manual's own.

use apl_session::{Mode, Session};

fn b() -> Session {
    let mut s = Session::default();
    s.ws.mode = Mode::B;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    let reply = s.respond(line);
    assert!(!reply.error, "{line}: {:?}", reply.lines);
    reply.lines
}

fn error(s: &mut Session, line: &str) -> String {
    let reply = s.respond(line);
    assert!(reply.error, "{line}: {:?}", reply.lines);
    reply.lines[0].clone()
}

/// The manual's B: a 3 by 2 matrix of mixed numbers.
fn with_b() -> Session {
    let mut s = b();
    out(&mut s, "B←3 2⍴12.34 ¯34.567 0 12 ¯0.26 ¯123.45");
    s
}

#[test]
fn monadic_format_is_the_display_as_characters() {
    let mut s = b();
    out(&mut s, "B←3 4⍴⍳12");
    out(&mut s, "X←⍕B");
    assert_eq!(
        out(&mut s, "X"),
        out(&mut s, "B"),
        "identical in appearance"
    );
    assert_eq!(out(&mut s, "⍴X"), vec!["3 10"]);
    assert_eq!(out(&mut s, "⍴⍕123"), vec!["3"], "a scalar gives a vector");
    assert_eq!(out(&mut s, "⍴⍕1 2 3"), vec!["5"]);
    assert_eq!(out(&mut s, "⍕÷3"), vec!["0.33333"], "at ⎕PP");
    assert_eq!(
        out(&mut s, "⍕'ABC'"),
        vec!["ABC"],
        "characters are themselves"
    );
    assert_eq!(out(&mut s, "⍴⍕⍳0"), vec!["0"]);
}

#[test]
fn a_width_and_a_positive_precision_give_decimal_form() {
    let mut s = with_b();
    assert_eq!(
        out(&mut s, "9 2⍕B"),
        vec![
            "    12.34   ¯34.57",
            "      .00    12.00",
            "     ¯.26  ¯123.45",
        ]
    );
    assert_eq!(out(&mut s, "⍴9 2⍕B"), vec!["3 18"]);
}

#[test]
fn a_negative_precision_gives_scaled_form() {
    let mut s = with_b();
    assert_eq!(
        out(&mut s, "9 ¯2⍕B"),
        vec![
            "   1.2E01  ¯3.5E01",
            "  0.0E¯01   1.2E01",
            " ¯2.6E¯01  ¯1.2E02",
        ]
    );
}

#[test]
fn a_width_of_zero_leaves_one_space_between_numbers() {
    let mut s = with_b();
    let wide = vec!["   12.34  ¯34.57", "     .00   12.00", "    ¯.26 ¯123.45"];
    assert_eq!(out(&mut s, "0 2⍕B"), wide);
    assert_eq!(out(&mut s, "2⍕B"), wide, "one number is a precision");
}

#[test]
fn each_column_may_have_a_pair_of_its_own() {
    let mut s = with_b();
    assert_eq!(
        out(&mut s, "6 2 6 1⍕B"),
        vec![" 12.34 ¯34.6", "   .00  12.0", "  ¯.26¯123.4"]
    );
}

#[test]
fn the_sign_is_kept_when_the_digits_are_not() {
    let mut s = b();
    assert_eq!(out(&mut s, "4 2⍕¯.0004"), vec!["¯.00"]);
    assert_eq!(
        out(&mut s, "5 0⍕3.7 ¯2"),
        vec!["    4   ¯2"],
        "no point at 0"
    );
    assert_eq!(
        out(&mut s, "3⍕9.9996"),
        vec![" 10.000"],
        "a carry adds a digit"
    );
    assert_eq!(out(&mut s, "0 ¯3⍕9.9996"), vec![" 1.00E01"]);
}

#[test]
fn what_format_refuses() {
    let mut s = with_b();
    assert_eq!(error(&mut s, "3 2⍕B"), "DOMAIN ERROR", "too narrow");
    assert_eq!(error(&mut s, "9 2 9⍕B"), "LENGTH ERROR");
    assert_eq!(error(&mut s, "9 2⍕'AB'"), "DOMAIN ERROR");
    assert_eq!(error(&mut s, "9.5 2⍕B"), "DOMAIN ERROR");
}

#[test]
fn format_is_not_in_70() {
    let mut a = Session::default();
    assert_eq!(
        a.respond("⍕3").lines[0],
        "CHARACTER ERROR: U+2355 (format, not APL\\360)"
    );
}
