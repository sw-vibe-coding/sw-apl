//! The system functions, which (B) '75 has and (A) '70 does not. The
//! IBM 5110 APL Reference Manual, Chapter 5; the APL/CMS User's
//! Manual for the APLSV rules the 5110 manual leaves out; and
//! docs/mode-b.md.
//!
//! Defining a function from characters, and getting them back, meet
//! the del editor and locked functions: a locked function's
//! characters are not to be had, and a function `⎕FX` makes is one
//! the editor could have made.

use apl_session::{Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
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

/// `(B)` with the manual's INTG defined in the editor.
fn with_intg() -> Session {
    let mut b = in_mode(Mode::B);
    for line in [
        "∇R←INTG A",
        "R←A⍴0",
        "I←1",
        "START:R[I]←A",
        "I←I+1",
        "→(I≤A)/START",
        "∇",
    ] {
        out(&mut b, line);
    }
    b
}

#[test]
fn canonical_representation_is_the_function_flush_left() {
    let mut b = with_intg();
    assert!(out(&mut b, "M←⎕CR 'INTG'").is_empty());
    assert_eq!(out(&mut b, "⍴M"), vec!["6 12"]);
    assert_eq!(
        out(&mut b, "M"),
        vec![
            "R←INTG A    ",
            "R←A⍴0       ",
            "I←1         ",
            "START:R[I]←A",
            "I←I+1       ",
            "→(I≤A)/START",
        ]
    );
}

#[test]
fn canonical_representation_of_anything_else_is_empty() {
    let mut b = with_intg();
    out(&mut b, "V←5");
    for line in ["⍴⎕CR 'V'", "⍴⎕CR 'NONE'", "⍴⎕CR ''", "⍴⎕CR '1X'"] {
        assert_eq!(out(&mut b, line), vec!["0 0"], "{line}");
    }
    assert_eq!(error(&mut b, "⎕CR 5"), "DOMAIN ERROR");
    assert_eq!(error(&mut b, "⎕CR 2 4⍴'INTG'"), "RANK ERROR");
}

#[test]
fn a_locked_function_s_characters_are_not_to_be_had() {
    let mut b = in_mode(Mode::B);
    for line in ["∇R←SECRET", "R←42", "⍫"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "SECRET"), vec!["42"], "it runs");
    assert_eq!(out(&mut b, "⍴⎕CR 'SECRET'"), vec!["0 0"]);
    // Nor by fixing a new definition over it, which the editor could
    // not have done either: the header row is at fault.
    assert_eq!(out(&mut b, "⎕FX 2 8⍴'R←SECRETR←0     '"), vec!["0"]);
    assert_eq!(out(&mut b, "SECRET"), vec!["42"], "unchanged");
}

#[test]
fn fix_defines_the_function_and_gives_its_name() {
    let mut b = with_intg();
    out(&mut b, "M←⎕CR 'INTG'");
    out(&mut b, "M[4;12]←'I'");
    assert_eq!(out(&mut b, "⎕FX M"), vec!["INTG"]);
    assert_eq!(out(&mut b, "INTG 5"), vec!["1 2 3 4 5"]);
    assert_eq!(
        out(&mut b, "∇INTG[⎕]∇"),
        vec![
            "      ∇R←INTG A",
            "[1]   R←A⍴0",
            "[2]   I←1",
            "[3]   START:R[I]←I",
            "[4]   I←I+1",
            "[5]   →(I≤A)/START",
            "      ∇",
        ],
        "the editor reopens it"
    );
    assert_eq!(out(&mut b, "(⎕FX 2 7⍴'Z←A F BZ←A+B  '),' 4'"), vec!["F 4"]);
    assert_eq!(out(&mut b, "3 F 4"), vec!["7"]);
}

#[test]
fn a_fixed_function_round_trips() {
    let mut b = with_intg();
    out(&mut b, "M←⎕CR 'INTG'");
    out(&mut b, "⎕EX 'INTG'");
    assert!(out(&mut b, "X←⎕FX M").is_empty());
    assert_eq!(out(&mut b, "∧/,M=⎕CR 'INTG'"), vec!["1"]);
}

#[test]
fn a_matrix_the_editor_could_not_make_fixes_nothing() {
    let mut b = with_intg();
    out(&mut b, "V←1");
    // The row at fault, counted as the function's line numbers are:
    // the header is 0.
    let cases = [
        ("⎕FX 1 3⍴'1+2'", "0"),
        ("⎕FX 1 1⍴'V'", "0"),
        ("⎕FX 2 4⍴'INTG[1] '", "1"),
        ("⎕FX 3 4⍴'G   1+2 ''AB'", "2"),
        ("⎕FX 2 1⍴'G∇'", "1"),
    ];
    for (line, row) in cases {
        assert_eq!(out(&mut b, line), vec![row], "{line}");
    }
    assert_eq!(out(&mut b, "⎕NC 'G'"), vec!["0"], "nothing defined");
    assert_eq!(out(&mut b, "V"), vec!["1"], "nothing replaced");
    assert_eq!(error(&mut b, "⎕FX 'R←G'"), "RANK ERROR");
    assert_eq!(error(&mut b, "⎕FX 2 2⍴1"), "DOMAIN ERROR");
}

#[test]
fn fix_does_not_replace_a_function_that_is_running() {
    let mut b = in_mode(Mode::B);
    for line in ["∇R←SELF", "R←⎕FX 2 4⍴'SELFR←1 '", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "SELF"), vec!["0"]);
    // Nor a name that is local, until local function names are built.
    for line in ["∇R←OUTER;H", "R←⎕FX 1 1⍴'H'", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "OUTER"), vec!["0"]);
    assert_eq!(out(&mut b, "⎕NC 'H'"), vec!["0"]);
}

#[test]
fn expunge_erases_and_says_whether_the_name_is_free() {
    let mut b = with_intg();
    out(&mut b, "V←1");
    assert_eq!(out(&mut b, "⎕EX 'V'"), vec!["1"]);
    assert_eq!(error(&mut b, "V"), "VALUE ERROR");
    assert_eq!(out(&mut b, "⎕EX 3 4⍴'INTGNONE1X  '"), vec!["1 1 0"]);
    assert_eq!(out(&mut b, "⎕NC 'INTG'"), vec!["0"]);
    assert_eq!(error(&mut b, "⎕EX 1 1 1⍴'V'"), "RANK ERROR");
    assert_eq!(error(&mut b, "⎕EX 1"), "DOMAIN ERROR");
}

#[test]
fn expunge_leaves_a_running_function_and_a_label() {
    let mut b = in_mode(Mode::B);
    for line in ["∇R←RUN", "L:R←(⎕EX 'RUN'),⎕EX 'L'", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "RUN"), vec!["0 0"]);
    assert_eq!(out(&mut b, "⎕NC 'RUN'"), vec!["3"]);
}

#[test]
fn expunge_takes_the_active_referent() {
    let mut b = in_mode(Mode::B);
    out(&mut b, "X←'GLOBAL'");
    for line in ["∇R←F;X", "X←1", "R←⎕EX 'X'", "R←R,⎕NC 'X'", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "F"), vec!["1 0"]);
    assert_eq!(out(&mut b, "X"), vec!["GLOBAL"], "the global is back");
}

#[test]
fn name_classification() {
    let mut b = with_intg();
    out(&mut b, "V←1");
    assert_eq!(out(&mut b, "⎕NC 'INTG'"), vec!["3"]);
    assert_eq!(out(&mut b, "⎕NC 'V'"), vec!["2"]);
    assert_eq!(out(&mut b, "⎕NC 'FREE'"), vec!["0"]);
    assert_eq!(out(&mut b, "⎕NC '2X'"), vec!["4"]);
    assert_eq!(out(&mut b, "⎕NC '⎕IO'"), vec!["4"]);
    assert_eq!(out(&mut b, "⍴⍴⎕NC 'V'"), vec!["0"], "a scalar");
    assert_eq!(out(&mut b, "⎕NC 4 4⍴'INTGV   FREEA B '"), vec!["3 2 0 4"]);
    for line in ["∇R←LAB", "L:R←⎕NC 2 1⍴'LR'", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "LAB"), vec!["1 0"], "R has no value yet");
    assert_eq!(error(&mut b, "⎕NC 5"), "DOMAIN ERROR");
}

#[test]
fn name_list_by_class_and_initial_letter() {
    let mut b = with_intg();
    out(&mut b, "ALPHA←1");
    out(&mut b, "BETA←2");
    out(&mut b, "ABC←3");
    assert_eq!(out(&mut b, "⎕NL 2"), vec!["ABC  ", "ALPHA", "BETA "]);
    assert_eq!(out(&mut b, "⎕NL 3"), vec!["INTG"]);
    assert_eq!(out(&mut b, "⍴⎕NL 3 2"), vec!["4 5"]);
    assert_eq!(out(&mut b, "'BI' ⎕NL 2 3"), vec!["BETA", "INTG"]);
    assert_eq!(out(&mut b, "⍴⎕NL 1"), vec!["0 0"]);
    assert_eq!(out(&mut b, "⎕EX 'B' ⎕NL 2"), vec!["1"]);
    assert_eq!(out(&mut b, "⎕NL 2"), vec!["ABC  ", "ALPHA"]);
    assert_eq!(error(&mut b, "⎕NL 4"), "DOMAIN ERROR");
    assert_eq!(error(&mut b, "⎕NL 'A'"), "DOMAIN ERROR");
    assert_eq!(error(&mut b, "1 ⎕NL 2"), "DOMAIN ERROR");
    assert_eq!(error(&mut b, "⎕NL 2 1⍴2"), "RANK ERROR");
    for line in ["∇R←LAB", "HERE:R←⎕NL 1", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "LAB"), vec!["HERE"]);
}

#[test]
fn console_control_answers_and_does_nothing() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "⎕CC '-'"), vec!["1"]);
    assert_eq!(out(&mut b, "⎕CC 'Q'"), vec!["0"]);
    assert_eq!(out(&mut b, "2 ⎕CC 2 2 2 2"), vec!["1"]);
    assert_eq!(out(&mut b, "4 ⎕CC ¯10 5"), vec!["1"]);
    assert_eq!(out(&mut b, "4 ⎕CC 17"), vec!["0"]);
    assert_eq!(out(&mut b, "5 ⎕CC 6"), vec!["1"]);
    assert_eq!(out(&mut b, "6 ⎕CC 0"), vec!["0"]);
    assert_eq!(error(&mut b, "⎕CC 1"), "DOMAIN ERROR");
    assert_eq!(error(&mut b, "1 ⎕CC 'A'"), "DOMAIN ERROR");
}

#[test]
fn delay_is_the_5110_s_fixed_value() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "⎕DL"), vec!["0"]);
    assert!(out(&mut b, "⎕DL←5").is_empty());
    assert_eq!(out(&mut b, "⎕DL"), vec!["0"], "ignored, as ⎕TT is");
}

#[test]
fn the_wrong_valence_or_no_argument_is_a_syntax_error() {
    let mut b = in_mode(Mode::B);
    assert_eq!(error(&mut b, "⎕CR"), "SYNTAX ERROR");
    assert_eq!(error(&mut b, "'A' ⎕CR 'B'"), "SYNTAX ERROR");
    assert_eq!(error(&mut b, "'A' ⎕NC 'B'"), "SYNTAX ERROR");
    assert_eq!(error(&mut b, "⎕XY 1"), "SYNTAX ERROR");
}

#[test]
fn the_70_mode_has_no_system_functions() {
    let mut a = in_mode(Mode::A);
    for line in ["⎕CR 'F'", "⎕NC 'F'", "⎕FX 1 1⍴'F'", "⎕DL"] {
        assert_eq!(error(&mut a, line), "SYNTAX ERROR", "{line}");
    }
}
