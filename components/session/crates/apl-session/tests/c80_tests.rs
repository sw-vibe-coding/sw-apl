//! (C) '80, APL80 on the TRS-80: what its manual's "Limitations" leave
//! out of the APL the core runs, each beside (A), which keeps it
//! (docs/mode-c.md). The manual names no error for what is not
//! implemented; (C) answers NONCE ERROR, as (B) does for what the 5110
//! dropped, DOMAIN ERROR for an argument of the wrong kind, SYNTAX ERROR
//! for a statement that specifies twice, and DEFN ERROR past the limits
//! on functions and lines.

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

/// The first line `line` answers in (C), and in (A).
fn both(line: &str) -> (String, String) {
    let first = |mode| {
        out(&mut in_mode(mode), line)
            .first()
            .cloned()
            .unwrap_or_default()
    };
    (first(Mode::C), first(Mode::A))
}

#[test]
fn c_is_named_and_starts_with_apl80s_settings() {
    let mut c = in_mode(Mode::C);
    assert_eq!(out(&mut c, ")DIALECT"), vec!["(C) '80"]);
    assert_eq!(out(&mut c, "÷3"), vec!["0.333333"], "six digits, as )PS");
    assert_eq!(c.ws.saved.print.width, 64, "the TRS-80's 64 columns");
    assert_eq!(
        out(&mut c, "+\\1 2 3 4"),
        vec!["1 3 6 10"],
        "the disk version scans"
    );
}

#[test]
fn domino_is_not_implemented() {
    assert_eq!(
        both("⌹2 2⍴4 0 0 2"),
        ("NONCE ERROR".into(), "0.25   0".into())
    );
    assert_eq!(both("1 2⌹2 2⍴1 0 0 1").0, "NONCE ERROR");
}

#[test]
fn lamination_is_not_implemented() {
    assert_eq!(both("1 2,[0.5]3 4"), ("NONCE ERROR".into(), "1 2".into()));
    assert_eq!(both("1 2,[1]3 4").0, "1 2 3 4", "catenation stays");
}

#[test]
fn the_diagonal_transpose_is_not_implemented() {
    assert_eq!(both("1 1⍉2 2⍴⍳4"), ("NONCE ERROR".into(), "1 4".into()));
    assert_eq!(
        both("2 1⍉2 2⍴⍳4").0,
        "1 3",
        "a transpose that names each axis once stays"
    );
}

#[test]
fn the_hyperbolic_functions_are_not_implemented() {
    assert_eq!(both("5○0"), ("NONCE ERROR".into(), "0".into()));
    assert_eq!(both("¯7○0").0, "NONCE ERROR");
    assert_eq!(both("5 1∘.○0").0, "NONCE ERROR", "nor through a product");
    assert_eq!(both("1○0").0, "0", "the circular functions stay");
    assert_eq!(
        both("○/1 0").0,
        "0",
        "and a reduction that does not reach one"
    );
}

#[test]
fn factorial_and_binomial_take_whole_numbers() {
    assert_eq!(both("!2.5").0, "DOMAIN ERROR");
    assert_eq!(both("2.5!5").0, "DOMAIN ERROR");
    assert_eq!(both("!/2.5 3").0, "DOMAIN ERROR", "nor through a reduction");
    assert_eq!(both("!5").0, "120");
    assert_eq!(both("2!5").0, "10");
}

#[test]
fn encode_and_decode_take_vectors() {
    assert_eq!(both("10 10⊤2 2⍴⍳4").0, "NONCE ERROR");
    assert_eq!(both("(2 2⍴10)⊥1 2").0, "NONCE ERROR");
    assert_eq!(both("10⊥1 2 3").0, "123", "a scalar and a vector stay");
    assert_eq!(both("10 10 10⊤123").0, "1 2 3");
}

#[test]
fn a_statement_specifies_once_and_not_as_a_value() {
    assert_eq!(both("X←Y←9"), ("SYNTAX ERROR".into(), String::new()));
    assert_eq!(
        both("÷X←4").0,
        "SYNTAX ERROR",
        "a specification used as a value"
    );
    assert_eq!(both("(X←3)+1").0, "SYNTAX ERROR");
    let mut c = in_mode(Mode::C);
    assert!(out(&mut c, "X←3").is_empty());
    assert!(out(&mut c, "X[2]←5").is_empty() || c.ws.saved.vars.contains_key("X"));
    assert_eq!(out(&mut c, "X"), vec!["3"]);
}

/// Define `name` with the del, one line long.
fn define(s: &mut Session, name: &str) -> Vec<String> {
    out(s, &format!("∇{name}"));
    out(s, "1");
    out(s, "∇")
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
    assert!(
        define(&mut c, "F1").is_empty(),
        "redefining one is not another"
    );
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
        out(&mut s, "∇G");
        for _ in 0..n {
            out(&mut s, "1");
        }
        (out(&mut s, "∇").first().cloned(), s.ws.is_function("G"))
    };
    assert_eq!(lines(Mode::C, 255), (None, true));
    assert_eq!(lines(Mode::C, 256), (Some("DEFN ERROR".into()), false));
    assert_eq!(lines(Mode::A, 256), (None, true));
}
