//! (C) '80, APL80 on the TRS-80: what its manual's "Limitations" leave
//! out of the APL the core runs, each beside (A), which keeps it
//! (docs/mode-c.md). (C) is typed in APL80's characters, (A) in
//! glyphs. The manual names no error for what is not implemented; (C)
//! answers NONCE ERROR, as (B) does for what the 5110 dropped, DOMAIN
//! ERROR for an argument of the wrong kind, SYNTAX ERROR for a
//! statement that specifies twice, and DEFN ERROR past the limits on
//! functions and lines. What APL80 has no character for cannot be
//! typed at all.

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

/// The first line a session in `mode` answers `line` with.
fn first(mode: Mode, line: &str) -> String {
    out(&mut in_mode(mode), line)
        .first()
        .cloned()
        .unwrap_or_default()
}

/// The first line `c` answers in (C), typed in APL80's characters.
fn c(line: &str) -> String {
    first(Mode::C, line)
}

/// The first line `a` answers in (A), typed in glyphs.
fn a(line: &str) -> String {
    first(Mode::A, line)
}

#[test]
fn c_is_named_and_starts_with_apl80s_settings() {
    let mut s = in_mode(Mode::C);
    assert_eq!(out(&mut s, ")DIALECT"), vec!["(C) '80"]);
    assert_eq!(out(&mut s, "%3"), vec!["0.333333"], "six digits, as )PS");
    assert_eq!(s.ws.saved.print.width, 64, "the TRS-80's 64 columns");
    assert_eq!(
        out(&mut s, "+w1 2 3 4"),
        vec!["1 3 6 10"],
        "the disk version scans"
    );
}

#[test]
fn domino_cannot_be_typed() {
    assert_eq!(c("⌹2 2p4 0 0 2"), "CHARACTER ERROR: U+2339");
    assert_eq!(a("⌹2 2⍴4 0 0 2"), "0.25   0");
}

#[test]
fn lamination_is_not_implemented() {
    assert_eq!(c("1 2,0.5d3 4"), "NONCE ERROR");
    assert_eq!(a("1 2,[0.5]3 4"), "1 2");
    assert_eq!(c("1 2,1d3 4"), "1 2 3 4", "catenation stays");
}

#[test]
fn the_diagonal_transpose_is_not_implemented() {
    assert_eq!(c("1 1g2 2pi4"), "NONCE ERROR");
    assert_eq!(a("1 1⍉2 2⍴⍳4"), "1 4");
    assert_eq!(
        c("2 1g2 2pi4"),
        "1 3",
        "a transpose that names each axis once stays"
    );
}

#[test]
fn the_hyperbolic_functions_are_not_implemented() {
    assert_eq!(c("5o0"), "NONCE ERROR");
    assert_eq!(a("5○0"), "0");
    assert_eq!(c("-7o0"), "NONCE ERROR");
    assert_eq!(c("5 1\".o0"), "NONCE ERROR", "nor through a product");
    assert_eq!(c("1o0"), "0", "the circular functions stay");
    assert_eq!(c("o/1 0"), "0", "and a reduction that does not reach one");
}

#[test]
fn factorial_and_binomial_take_whole_numbers() {
    assert_eq!(c("!2.5"), "DOMAIN ERROR");
    assert_eq!(c("2.5!5"), "DOMAIN ERROR");
    assert_eq!(c("!/2.5 3"), "DOMAIN ERROR", "nor through a reduction");
    assert_eq!(c("!5"), "120");
    assert_eq!(c("2!5"), "10");
    assert_eq!(a("!2.5"), "3.32335097");
}

#[test]
fn encode_and_decode_take_vectors() {
    assert_eq!(c("10 10t2 2pi4"), "NONCE ERROR");
    assert_eq!(c("(2 2p10)b1 2"), "NONCE ERROR");
    assert_eq!(c("10b1 2 3"), "123", "a scalar and a vector stay");
    assert_eq!(c("10 10 10t123"), "1 2 3");
}

#[test]
fn a_statement_specifies_once_and_not_as_a_value() {
    assert_eq!(c("X←Y←9"), "SYNTAX ERROR");
    assert_eq!(a("X←Y←9"), "");
    assert_eq!(c("%X←4"), "SYNTAX ERROR", "a specification used as a value");
    assert_eq!(c("(X←3)+1"), "SYNTAX ERROR");
    let mut s = in_mode(Mode::C);
    assert!(out(&mut s, "X←3").is_empty());
    assert_eq!(out(&mut s, "X"), vec!["3"]);
}

/// Open `name` the mode's way -- `)DEF` in (C), the del elsewhere --
/// and what the line that ends it is.
fn opening(s: &Session, name: &str) -> (String, &'static str) {
    if s.ws.mode == Mode::C {
        (format!(")DEF {name}"), "")
    } else {
        (format!("∇{name}"), "∇")
    }
}

/// Define `name`, one line long, and what ending it showed.
fn define(s: &mut Session, name: &str) -> Vec<String> {
    let (open, close) = opening(s, name);
    out(s, &open);
    out(s, "1");
    out(s, close)
}

#[test]
fn a_workspace_holds_at_most_32_functions() {
    let mut c = in_mode(Mode::C);
    for n in 1..=32 {
        assert!(define(&mut c, &format!("F{n}")).is_empty(), "F{n}");
    }
    assert_eq!(
        define(&mut c, "F33").first().map(String::as_str),
        Some("DEFN ERROR")
    );
    assert!(!c.ws.is_function("F33"));
    out(&mut c, ")EDIT F1");
    assert!(out(&mut c, "").is_empty(), "changing one is not another");
    let mut a = in_mode(Mode::A);
    for n in 1..=33 {
        assert!(
            define(&mut a, &format!("F{n}")).is_empty(),
            "(A) has no such limit"
        );
    }
}

#[test]
fn a_function_has_at_most_255_lines() {
    let lines = |mode, n: usize| {
        let mut s = in_mode(mode);
        let (open, close) = opening(&s, "G");
        out(&mut s, &open);
        for _ in 0..n {
            out(&mut s, "1");
        }
        (out(&mut s, close).first().cloned(), s.ws.is_function("G"))
    };
    assert_eq!(lines(Mode::C, 255), (None, true));
    assert_eq!(lines(Mode::C, 256), (Some("DEFN ERROR".into()), false));
    assert_eq!(lines(Mode::A, 256), (None, true));
}
