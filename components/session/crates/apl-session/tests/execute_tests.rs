//! Execute, which (B) '75 has and (A) '70 does not: a character
//! vector run as a line of APL in the workspace. The IBM 5110 APL
//! Reference Manual (Chapter 4) and docs/mode-b.md.

use apl_session::{Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn execute_runs_a_character_vector_as_a_line() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "⍎'1+2'"), vec!["3"]);
    assert_eq!(out(&mut b, "10×⍎'1+2'"), vec!["30"], "its value is a value");
    out(&mut b, "A←'2×3'");
    assert_eq!(out(&mut b, "⍎A"), vec!["6"]);
    assert_eq!(
        out(&mut b, "⍎'4'"),
        vec!["4"],
        "a character scalar is a line too"
    );
}

#[test]
fn an_executed_line_shows_what_it_would_show() {
    let mut b = in_mode(Mode::B);
    assert!(
        out(&mut b, "⍎'X←5'").is_empty(),
        "an assignment shows nothing"
    );
    assert_eq!(out(&mut b, "X"), vec!["5"]);
    // The manual's example: executing an empty vector does nothing.
    out(&mut b, "A←1");
    out(&mut b, "B←2");
    assert!(out(&mut b, "⍎(A=B)/'A+B'").is_empty());
    out(&mut b, "B←1");
    assert_eq!(out(&mut b, "⍎(A=B)/'A+B'"), vec!["2"]);
}

#[test]
fn a_line_with_no_value_has_none_to_give() {
    let mut b = in_mode(Mode::B);
    let reply = b.respond("1+⍎''");
    assert!(reply.error);
    assert_eq!(reply.lines[0], "VALUE ERROR", "{:?}", reply.lines);
}

#[test]
fn the_executed_lines_errors_are_its_errors() {
    let mut b = in_mode(Mode::B);
    let reply = b.respond("⍎'1 2+3 4 5'");
    assert!(reply.error);
    assert_eq!(reply.lines[0], "LENGTH ERROR", "{:?}", reply.lines);
    let reply = b.respond("⍎'NOSUCH'");
    assert_eq!(reply.lines[0], "VALUE ERROR", "{:?}", reply.lines);
}

#[test]
fn execute_takes_a_character_scalar_or_vector_only() {
    let mut b = in_mode(Mode::B);
    assert_eq!(b.respond("⍎5").lines[0], "DOMAIN ERROR");
    assert_eq!(b.respond("⍎2 2⍴'1+2 '").lines[0], "RANK ERROR");
    assert_eq!(
        b.respond("1⍎'2'").lines[0],
        "SYNTAX ERROR",
        "no dyadic form"
    );
}

#[test]
fn execute_works_inside_a_function() {
    let mut b = in_mode(Mode::B);
    for line in ["∇R←RUN T", "L:R←⍎T", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "RUN '3×4'"), vec!["12"]);
}

#[test]
fn in_70_execute_is_refused_as_it_always_was() {
    let mut a = in_mode(Mode::A);
    let reply = a.respond("⍎'1+2'");
    assert!(reply.error);
    assert_eq!(
        reply.lines[0],
        "CHARACTER ERROR: U+234E (execute, not APL\\360)"
    );
}

#[test]
fn format_is_75s_and_not_70s() {
    let mut b = in_mode(Mode::B);
    assert_eq!(
        b.respond("⍕1 2").lines[0],
        "1 2",
        "format_tests has the rest"
    );
    let mut a = in_mode(Mode::A);
    assert!(a.respond("⍕1 2").lines[0].starts_with("CHARACTER ERROR: U+2355"));
}

#[test]
fn a_workspace_using_execute_is_75_only() {
    let mut b = in_mode(Mode::B);
    for line in ["∇R←RUN T", "R←⍎T", "∇"] {
        out(&mut b, line);
    }
    let text = apl_wsfile::write(&b.ws.saved, "1.00.00 01/01/70");
    assert_eq!(text.lines().nth(1), Some("⍝!MODES (B)"));
}

/// The executed line is not on the paper, so an error in it is shown
/// against the statement, with the caret under the execute.
#[test]
fn an_error_in_the_executed_line_points_at_the_execute() {
    let mut b = in_mode(Mode::B);
    let reply = b.respond("3+⍎'1 2+3 4 5'");
    assert_eq!(reply.lines[1], "      3+⍎'1 2+3 4 5'");
    assert_eq!(reply.lines[2], "        ^", "{:?}", reply.lines);
}
