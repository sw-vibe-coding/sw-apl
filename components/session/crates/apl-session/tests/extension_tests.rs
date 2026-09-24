//! The commands sw-apl adds, which no historical system had: they are
//! system commands, never quad names, so no program or saved
//! workspace can come to depend on them.

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

#[test]
fn dialect_names_the_mode() {
    assert_eq!(out(&mut in_mode(Mode::A), ")DIALECT"), vec!["(A) '70"]);
    assert_eq!(out(&mut in_mode(Mode::B), ")DIALECT"), vec!["(B) '75"]);
}

#[test]
fn dialect_is_cut_short_as_any_long_command_is() {
    assert_eq!(out(&mut in_mode(Mode::B), ")DIAL"), vec!["(B) '75"]);
    assert_eq!(out(&mut in_mode(Mode::B), ")DIALOGUE"), vec!["(B) '75"]);
}

#[test]
fn dialect_does_not_switch() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, ")DIALECT 70"), vec!["INCORRECT COMMAND"]);
    assert_eq!(out(&mut b, ")DIALECT"), vec!["(B) '75"], "still (B)");
}
