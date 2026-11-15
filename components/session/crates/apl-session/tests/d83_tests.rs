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
fn d_sees_no_workspace_written_for_another_mode() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let mut d = in_mode(Mode::D);
    d.ws.store = Box::new(Files(root));
    assert_eq!(out(&mut d, ")LIB 1"), Vec::<String>::new());
    assert_eq!(out(&mut d, ")LOAD 1 LIFE")[0], "WS NOT FOUND");
}
