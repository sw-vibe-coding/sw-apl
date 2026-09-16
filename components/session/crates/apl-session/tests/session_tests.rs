//! The session turns one input line into transcript lines.

use apl_session::{Reply, Session};

fn out(s: &mut Session, line: &str) -> Vec<String> {
    match s.respond(line) {
        Reply::Output(v) => v,
        Reply::Off => panic!("unexpected )OFF"),
    }
}

#[test]
fn evaluates_and_displays() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "3 + 4"), vec!["7"]);
    assert_eq!(out(&mut s, "1 2 3 + 10 20 30"), vec!["11 22 33"]);
    assert_eq!(out(&mut s, "20 \u{f7} 8"), vec!["2.5"]);
}

#[test]
fn assignment_is_silent_and_remembered() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "A \u{2190} 5"), Vec::<String>::new());
    assert_eq!(out(&mut s, "A + 3"), vec!["8"]);
}

#[test]
fn blank_and_comment_lines_print_nothing() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, ""), Vec::<String>::new());
    assert_eq!(out(&mut s, "\u{235d} hello"), Vec::<String>::new());
}

#[test]
fn errors_print_name_statement_and_caret() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "2 3+4 5 6"),
        vec!["LENGTH ERROR", "      2 3+4 5 6", "         ^"]
    );
    assert_eq!(
        out(&mut s, "1+XYZ"),
        vec!["VALUE ERROR", "      1+XYZ", "        ^"]
    );
    assert_eq!(
        out(&mut s, "\u{3c1}5"),
        vec!["CHARACTER ERROR: U+03C1", "      \u{3c1}5", "      ^"]
    );
}

#[test]
fn system_commands() {
    let mut s = Session::default();
    assert_eq!(s.respond(")OFF"), Reply::Off);
    assert_eq!(s.respond("  )off"), Reply::Off);
    assert_eq!(out(&mut s, ")FOO"), vec!["INCORRECT COMMAND"]);
}

#[test]
fn mvp_transcript() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "+/\u{2373}10"), vec!["55"]);
    assert_eq!(out(&mut s, "2 3\u{2374}\u{2373}6"), vec!["1 2 3", "4 5 6"]);
    assert_eq!(out(&mut s, "\u{2374}42"), vec![""]);
    assert_eq!(out(&mut s, "\u{2395}\u{2190}\u{2373}3"), vec!["1 2 3"]);
    assert_eq!(out(&mut s, "\u{2395}IO\u{2190}0"), Vec::<String>::new());
    assert_eq!(out(&mut s, "\u{2373}3"), vec!["0 1 2"]);
    assert_eq!(out(&mut s, "1+\u{2395}\u{2190}5"), vec!["5", "6"]);
}

#[test]
fn quad_output_before_an_error_still_prints() {
    let mut s = Session::default();
    let lines = out(&mut s, "XYZ+\u{2395}\u{2190}5");
    assert_eq!(lines[0], "5");
    assert_eq!(lines[1], "VALUE ERROR");
}
