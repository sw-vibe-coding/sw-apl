//! (D) '83's extensions to (B)'s language, as the May 1983 APL
//! Reference gives them: ambivalent defined functions, `⎕EA`, the
//! dyadic grades with a collating sequence, and `)RESET`.

use apl_session::{Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn define(s: &mut Session, lines: &[&str]) {
    for line in lines {
        out(s, line);
    }
}

/// The manual's ROOT: the square root when no left argument is given.
const ROOT: &[&str] = &["∇Z←N ROOT A", "→(0≠⎕NC 'N')/RN", "N←2", "RN:Z←A*÷N", "∇"];

const BAD: &[&str] = &["∇BAD", "1÷0", "∇"];

#[test]
fn a_dyadic_function_called_monadically_has_no_left_argument_in_d() {
    let mut d = in_mode(Mode::D);
    define(&mut d, ROOT);
    assert_eq!(out(&mut d, "3 ROOT 27"), ["3"]);
    assert_eq!(out(&mut d, "ROOT 16"), ["4"]);
    assert_eq!(out(&mut d, "1+ROOT 16"), ["5"], "and for its value");
    out(&mut d, "N←5");
    assert_eq!(out(&mut d, "ROOT 16"), ["4"], "the global N is shadowed");
    assert!(out(&mut d, ")SI").is_empty());
}

#[test]
fn b_still_refuses_a_dyadic_function_called_monadically() {
    let mut b = in_mode(Mode::B);
    define(&mut b, ROOT);
    assert_eq!(out(&mut b, "ROOT 16")[0], "SYNTAX ERROR");
}

#[test]
fn execute_alternate_runs_the_right_and_on_an_error_the_left() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "'0' ⎕EA '1 2 3'"), ["1 2 3"]);
    assert_eq!(out(&mut d, "'0' ⎕EA '1÷0'"), ["0"]);
    assert_eq!(out(&mut d, "'''NO''' ⎕EA '1 2+1 2 3'"), ["NO"]);
    assert_eq!(out(&mut d, "2×'0' ⎕EA '1+2'"), ["6"], "for its value");
    assert_eq!(out(&mut d, "'1÷0' ⎕EA '2÷0'")[0], "DOMAIN ERROR");
}

#[test]
fn execute_alternate_with_no_value_shows_nothing() {
    let mut d = in_mode(Mode::D);
    assert!(out(&mut d, "'X←1' ⎕EA 'X←2'").is_empty());
    assert_eq!(out(&mut d, "X"), ["2"]);
    assert!(out(&mut d, "'X←1' ⎕EA 'X←Y'").is_empty());
    assert_eq!(out(&mut d, "X"), ["1"], "Y has no value: the alternate");
}

#[test]
fn an_error_in_a_function_the_right_calls_is_trapped_and_unwound() {
    let mut d = in_mode(Mode::D);
    define(&mut d, BAD);
    assert_eq!(out(&mut d, "'7' ⎕EA 'BAD'"), ["7"]);
    assert!(out(&mut d, ")SI").is_empty(), "BAD is not left suspended");
}

#[test]
fn execute_alternate_takes_character_vectors_only() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "'0' ⎕EA 1")[0], "DOMAIN ERROR");
    assert_eq!(out(&mut d, "'0' ⎕EA 1 1⍴'1'")[0], "RANK ERROR");
    assert_eq!(out(&mut d, "0 ⎕EA '1'")[0], "DOMAIN ERROR");
}

#[test]
fn b_has_no_execute_alternate() {
    let mut b = in_mode(Mode::B);
    assert!(out(&mut b, "'0' ⎕EA '1'")[0].ends_with("ERROR"));
}

#[test]
fn the_dyadic_grades_sort_by_a_collating_sequence() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "'ABCDE' ⍒ 'DEAL'"), ["4 2 1 3"]);
    assert_eq!(out(&mut d, "'ABCDE' ⍋ 'DEAL'"), ["3 1 2 4"]);
    out(&mut d, "R←5 4⍴'DEALLEADDEADDEEDDALE'");
    assert_eq!(out(&mut d, "'ABCDE' ⍋ R"), ["5 3 1 4 2"]);
    assert_eq!(out(&mut d, "'ABCDE' ⍒ R"), ["2 4 1 3 5"]);
}

#[test]
fn the_last_axis_of_the_sequence_counts_most() {
    let mut d = in_mode(Mode::D);
    out(&mut d, "R←5 4⍴'dealDealdeadDeadDEED'");
    out(&mut d, "L←2 5⍴'abcdeABCDE'");
    assert_eq!(out(&mut d, "L⍒R"), ["5 2 1 4 3"]);
    assert_eq!(out(&mut d, "L⍋R"), ["3 4 1 2 5"]);
    out(&mut d, "⎕IO←0");
    assert_eq!(out(&mut d, "'ABCDE' ⍋ 'DEAL'"), ["2 0 1 3"]);
}

#[test]
fn the_dyadic_grades_take_characters_above_a_scalar() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "'ABC' ⍋ 3 1 2")[0], "DOMAIN ERROR");
    assert_eq!(out(&mut d, "1 2 3 ⍋ 'CAB'")[0], "DOMAIN ERROR");
    assert_eq!(out(&mut d, "'ABC' ⍋ 'A'")[0], "RANK ERROR");
    assert_eq!(out(&mut d, "'A' ⍋ 'CAB'")[0], "RANK ERROR");
    let mut b = in_mode(Mode::B);
    assert!(
        out(&mut b, "'ABC' ⍋ 'CAB'")[0].ends_with("ERROR"),
        "not in (B)"
    );
}

#[test]
fn reset_and_si_clear_clear_the_state_indicator() {
    for command in [")RESET", ")SI CLEAR"] {
        let mut d = in_mode(Mode::D);
        define(&mut d, BAD);
        out(&mut d, "BAD");
        out(&mut d, "BAD");
        assert_eq!(out(&mut d, ")SI").len(), 2, "two suspensions");
        assert!(out(&mut d, command).is_empty(), "{command}");
        assert!(out(&mut d, ")SI").is_empty(), "{command}");
    }
}

#[test]
fn b_has_no_reset() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, ")RESET"), ["INCORRECT COMMAND"]);
    assert_eq!(out(&mut b, ")SI CLEAR"), ["INCORRECT COMMAND"]);
}
