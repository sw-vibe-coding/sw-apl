//! (D) '83's AP205, the full-screen processor, as the May 1983 manual
//! gives it (chapter 3): the variables it takes, its commands and
//! return codes, the fields, and characters and pixels written and
//! read back; and library 1's GRAPHICS, which draws through it.

use std::path::PathBuf;

use apl_session::{Files, Mode, Session};

fn shared() -> Session {
    let mut s = Session::default();
    s.ws.mode = Mode::D;
    s.respond(")CLEAR");
    s.respond("Z←205 ⎕SVO 2 1⍴'CD'");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

/// Set the control variable and give back the return code.
fn code(s: &mut Session, command: &str) -> String {
    out(s, &format!("C←{command}"));
    out(s, "C").concat()
}

#[test]
fn ap205_takes_one_control_and_one_data_variable() {
    let mut d = shared();
    assert_eq!(out(&mut d, "⎕SVO 2 1⍴'CD'"), ["2 2"]);
    assert_eq!(out(&mut d, "205 ⎕SVO 2 2⍴'C2D2'"), ["1 1"]);
    assert_eq!(
        out(&mut d, "⎕SVO 2 2⍴'C2D2'"),
        ["1 1"],
        "one pair at a time"
    );
    assert_eq!(out(&mut d, "205 ⎕SVO 'X'"), ["1"]);
    assert_eq!(out(&mut d, "⎕SVO 'X'"), ["1"], "only C and D names");
    out(&mut d, "⎕SVR 2 1⍴'CD'");
    assert_eq!(out(&mut d, "205 ⎕SVO 'C3'"), ["1"]);
    assert_eq!(out(&mut d, "⎕SVO 'C3'"), ["2"], "free again once retracted");
}

#[test]
fn the_manuals_fields_are_defined_and_read_back() {
    let mut d = shared();
    out(&mut d, "D←3 6⍴2 2 10 10 2 7 12 20 1 1 0 135 20 1 1 40 2 15");
    assert_eq!(code(&mut d, "1"), "0");
    assert_eq!(code(&mut d, "9 2"), "0");
    assert_eq!(out(&mut d, "D"), ["12 20 1 1 0 135"]);
    out(&mut d, "D←'FIELD 1'");
    assert_eq!(code(&mut d, "2 1"), "0");
    assert_eq!(code(&mut d, "5 1"), "0");
    assert_eq!(out(&mut d, "10↑D"), ["FIELD 1   "]);
    assert_eq!(out(&mut d, "⍴D"), ["100"], "ten by ten characters");
}

#[test]
fn pixels_are_written_combined_and_read_on_the_color_display() {
    let mut d = shared();
    assert_eq!(code(&mut d, "0 4"), "0");
    out(&mut d, "D←1 6⍴1 1 1 1 2 1");
    code(&mut d, "1");
    out(&mut d, "D←64⍴1 0");
    assert_eq!(code(&mut d, "2 1 1"), "0");
    out(&mut d, "D←64⍴1");
    assert_eq!(code(&mut d, "2 1 4"), "0", "exclusive or");
    code(&mut d, "5 1 1");
    assert_eq!(out(&mut d, "8↑D"), ["0 1 0 1 0 1 0 1"]);
    assert_eq!(code(&mut d, "0 1"), "0");
    assert_eq!(
        code(&mut d, "2 1 1"),
        "5",
        "the monochrome display has no pixels"
    );
}

#[test]
fn bad_commands_and_data_have_their_codes() {
    let mut d = shared();
    assert_eq!(code(&mut d, "7"), "1");
    assert_eq!(code(&mut d, "'X'"), "1");
    assert_eq!(code(&mut d, "1"), "2", "nothing given yet");
    out(&mut d, "D←1 6⍴20 1 10 10 2 7");
    assert_eq!(code(&mut d, "1"), "2", "past the bottom of the screen");
    out(&mut d, "D←1 6⍴1 1 1 2000 2 7");
    assert_eq!(code(&mut d, "1"), "0", "one line may run on");
    out(&mut d, "⎕SVR 'D'");
    assert_eq!(code(&mut d, "2 1"), "3", "no data variable");
}

#[test]
fn graphics_draws_the_cube_in_three_colors() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let mut d = shared();
    d.ws.store = Box::new(Files(root));
    out(&mut d, ")LOAD 1 GRAPHICS");
    out(&mut d, "SHOW");
    assert_eq!(out(&mut d, "CTL"), ["0"]);
    for field in 1..=3 {
        out(&mut d, &format!("CTL←5 {field} 1"));
        let lit: usize = out(&mut d, "+/DAT").concat().parse().expect("a count");
        assert!(lit > 100, "field {field} has its edges: {lit}");
    }
}
