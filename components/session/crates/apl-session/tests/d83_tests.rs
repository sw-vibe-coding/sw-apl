//! (D) '83, IBM PC APL: (B)'s language to begin with, IBM PC APL's
//! clear workspace, and its own workspaces alone
//! (docs/ibm-pc-apl-plan.md).

use std::path::PathBuf;

use apl_session::{Files, Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn d_is_named_and_starts_with_pc_apls_settings() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, ")DIALECT"), ["(D) '83"]);
    assert_eq!(out(&mut d, "÷3"), ["0.3333333333"], "ten digits");
    assert_eq!(out(&mut d, "⎕PW"), ["79"], "79 columns");
    assert_eq!(out(&mut d, "⎕IO"), ["1"]);
}

#[test]
fn d_has_bs_language() {
    for line in ["⍎'1+2'", "⍕2 3⍴⍳6", "+\\⍳5", "⎕LX←'X'", "⌶20"] {
        let (mut b, mut d) = (in_mode(Mode::B), in_mode(Mode::D));
        assert_eq!(out(&mut d, line), out(&mut b, line), "{line}");
    }
}

#[test]
fn d_lists_its_own_copies_and_no_other_modes_workspace() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let mut d = in_mode(Mode::D);
    d.ws.store = Box::new(Files(root));
    let own = ["BIRDS", "EDIT", "LEARN", "LIFE", "RACE", "TTTML"];
    assert_eq!(out(&mut d, ")LIB 1"), own, "the (D) copies");
    assert!(out(&mut d, ")LOAD 1 LIFE")[0].starts_with("SAVED "));
    assert!(d.ws.saved.funcs.contains_key("RUN"), "(D)'s LIFE");
}

#[test]
fn a_save_in_d_runs_in_d_alone() {
    let dir = std::env::temp_dir().join(format!("sw-apl-d83-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("work")).expect("a library");
    let mut d = in_mode(Mode::D);
    d.ws.store = Box::new(Files(dir.clone()));
    out(&mut d, "X←⍳3");
    out(&mut d, ")SAVE MINE");
    let saved = std::fs::read_to_string(dir.join("work/MINE.d.apl.ws")).expect("MINE.d");
    assert!(saved.lines().any(|l| l == "⍝!MODES (D)"), "{saved}");
    let mut b = in_mode(Mode::B);
    b.ws.store = Box::new(Files(dir.clone()));
    assert_eq!(
        out(&mut b, ")LIB"),
        Vec::<String>::new(),
        "(B) does not see it"
    );
    assert_eq!(out(&mut d, ")LIB"), ["MINE"]);
    std::fs::remove_dir_all(dir).ok();
}
