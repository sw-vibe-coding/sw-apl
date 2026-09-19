//! An output line left open by `⍞←`.
//!
//! `⍞←` writes without ending the line, so that what comes next
//! carries on it. That is the whole of the prompt idiom: `⍞←'NAME: '`
//! and then a read, answered on the same line.
//!
//! Inside a function this always worked, because the statements of
//! one call render together. Across statements in immediate
//! execution it did not: a `Reply` carried finished lines and had no
//! way to say the last one was not finished.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn quote_quad_output_leaves_the_line_open() {
    let mut s = Session::default();
    let reply = s.respond("\u{235e}\u{2190}'P'");
    assert_eq!(reply.lines, vec!["P"]);
    assert!(reply.open, "the line is not finished");
}

#[test]
fn an_open_line_survives_into_the_next_statement() {
    let mut s = Session::default();
    assert!(s.respond("\u{235e}\u{2190}'P'").open);
    let next = s.respond("\u{235e}\u{2190}'Q'");
    assert_eq!(next.lines, vec!["Q"], "its own text, for the shell to join");
    assert!(next.open);
}

#[test]
fn ordinary_output_closes_the_line() {
    let mut s = Session::default();
    assert!(!s.respond("2+2").open);
    assert!(!s.respond("'TEXT'").open);
    assert!(
        !s.respond("\u{2395}\u{2190}'QUAD'").open,
        "quad ends its line"
    );
}

/// `open` describes this reply's own lines: the last one is
/// unfinished. A reply with no lines says nothing about the state of
/// the terminal, and cannot: it printed nothing, so whatever an
/// earlier reply left open is still open. The shell closes a line
/// only by printing one.
#[test]
fn a_statement_that_shows_nothing_says_nothing_about_the_line() {
    let mut s = Session::default();
    assert!(s.respond("\u{235e}\u{2190}'P'").open);
    let quiet = s.respond("A\u{2190}1");
    assert!(quiet.lines.is_empty(), "nothing to print, nothing to close");
    assert!(!quiet.open);
}

#[test]
fn an_error_closes_the_line_before_reporting() {
    let mut s = Session::default();
    s.respond("\u{235e}\u{2190}'P'");
    let failed = s.respond("NOSUCH");
    assert_eq!(failed.lines[0], "VALUE ERROR");
    assert!(!failed.open, "a report starts on a line of its own");
}

#[test]
fn a_function_still_joins_its_own_statements() {
    let mut s = Session::default();
    for line in [
        "\u{2207}F",
        "\u{235e}\u{2190}'P'",
        "\u{235e}\u{2190}'Q'",
        "\u{2207}",
    ] {
        out(&mut s, line);
    }
    let reply = s.respond("F");
    assert_eq!(reply.lines, vec!["PQ"], "rendered together, as before");
    assert!(reply.open);
}

#[test]
fn a_command_reply_starts_its_own_line() {
    let mut s = Session::default();
    s.respond("\u{235e}\u{2190}'P'");
    let reply = s.respond(")WSID");
    assert_eq!(reply.lines, vec!["CLEAR WS"]);
    assert!(!reply.open);
}
