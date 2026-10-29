//! `[N⎕M]` line editing in the del editor, (A) '70 only: the APL\360
//! manual's mechanism (SH20-0906-0 p. 3.18), driven line by line as a
//! session drives it. The column arithmetic is pinned in `apl-patch`.

use apl_ast::Defn;
use apl_editor::Definition;
use apl_value::ErrorKind;

/// A function F open in `glyphs`' mode -- empty is (A) -- with `body`.
fn open_in(glyphs: &'static str, body: &[&str]) -> Definition {
    let defn = Defn {
        name: "F".to_string(),
        body: body.iter().map(|l| (*l).to_string()).collect(),
        ..Defn::default()
    };
    Definition::start(defn, false, glyphs)
}

/// Type `line`, and hand back what it displayed.
fn shows(definition: &mut Definition, line: &str) -> Vec<String> {
    definition.line(line).expect("the editor accepted it").lines
}

fn body(definition: Definition) -> Vec<String> {
    definition.close(false).0.body
}

#[test]
fn the_manuals_example_from_end_to_end() {
    let mut f = open_in("", &["Z←(33×Q+(R×5)-6"]);
    assert_eq!(shows(&mut f, "[1⎕9]"), ["[1]   Z←(33×Q+(R×5)-6"]);
    assert_eq!(f.prompt(), " ".repeat(9), "the carriage under position 9");
    assert_eq!(shows(&mut f, "/   1 /1"), ["[1]   Z←(3×Q +( ×5)-6"]);
    assert_eq!(f.prompt(), "[1]   Z←(3×Q", "up to the first blank put in");
    assert!(
        shows(&mut f, ")  T").is_empty(),
        "a line begun with ) is a fill, not a command"
    );
    assert_eq!(f.prompt(), "[2]   ", "as if line 1 had been typed whole");
    assert_eq!(body(f), ["Z←(3×Q)+(T×5)-6"]);
}

#[test]
fn a_blank_edit_leaves_the_statement_as_it_was() {
    let mut f = open_in("", &["X←1", "Y←2"]);
    for line in ["[1⎕0]", "", ""] {
        shows(&mut f, line);
    }
    assert_eq!(body(f), ["X←1", "Y←2"]);
}

#[test]
fn changing_the_number_moves_the_statement_and_leaves_n() {
    // Delete the 1 of [1] and open a blank before the ], then type 3.
    let mut f = open_in("", &["X←1", "Y←2"]);
    shows(&mut f, "[1⎕1]");
    assert_eq!(shows(&mut f, "/1"), ["[ ]   X←1"]);
    assert_eq!(f.prompt(), "[");
    shows(&mut f, "3");
    assert_eq!(body(f), ["X←1", "Y←2", "X←1"]);
}

#[test]
fn the_header_is_statement_0() {
    let mut f = open_in("", &["X←1"]);
    assert_eq!(shows(&mut f, "[0⎕6]"), ["[0]   F"]);
    assert_eq!(shows(&mut f, "2"), ["[0]     F"]);
    shows(&mut f, "R←");
    let (defn, _) = f.close(false);
    assert_eq!(defn.result.as_deref(), Some("R"), "{defn:?}");
}

#[test]
fn an_improper_request_is_a_defn_error() {
    for line in ["[5⎕3]", "[1⎕X]", "[1⎕¯1]", "[⎕3]"] {
        let mut f = open_in("", &["X←1"]);
        let err = f.line(line).expect_err(line);
        assert_eq!(err.kind, ErrorKind::Defn, "{line}");
        assert_eq!(f.prompt(), "[2]   ", "{line} left the prompt alone");
    }
}

#[test]
fn an_illegitimate_overstrike_in_the_fill_ends_the_edit() {
    let mut f = open_in("", &["X←1"]);
    shows(&mut f, "[1⎕6]");
    assert_eq!(shows(&mut f, "1"), ["[1]    X←1"]);
    let err = f.line(" 2").expect_err("2 over X");
    assert!(matches!(err.kind, ErrorKind::Character(_)), "{err:?}");
    assert_eq!(f.prompt(), "[2]   ", "the edit is over");
    assert_eq!(body(f), ["X←1"]);
}

#[test]
fn b_has_no_slash_line() {
    // The 5110 edits [n⎕m] in place instead (mode-b.md); not yet.
    let mut f = open_in("⍎⍕", &["X←1"]);
    assert_eq!(f.line("[1⎕3]").unwrap_err().kind, ErrorKind::Defn);
}
