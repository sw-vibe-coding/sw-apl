//! A local name shadows every referent of the name, a function as
//! well as a variable: the APL\360 User's Manual on local names, and
//! the APLSV rule the IBM 5110 manual follows. In both modes.

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

fn define(s: &mut Session, lines: &[&str]) {
    for line in lines {
        out(s, line);
    }
}

#[test]
fn a_local_name_hides_a_global_function_for_the_call() {
    for mode in [Mode::A, Mode::B] {
        let mut s = in_mode(mode);
        define(&mut s, &["∇R←G X", "R←X+1", "∇", "∇F;G", "G←5", "G", "∇"]);
        assert_eq!(out(&mut s, "F"), vec!["5"], "{mode:?}");
        assert_eq!(out(&mut s, "G 1"), vec!["2"], "{mode:?}: G is back");
        assert_eq!(out(&mut s, ")FNS"), vec!["F G"], "{mode:?}");
        assert_eq!(out(&mut s, ")VARS"), Vec::<String>::new(), "{mode:?}");
    }
}

#[test]
fn fns_under_a_suspension_lists_the_global_functions() {
    let mut s = in_mode(Mode::A);
    define(
        &mut s,
        &["∇R←G X", "R←X+1", "∇", "∇F;G", "G←5", "NOSUCH", "∇"],
    );
    let reply = s.respond("F");
    assert_eq!(reply.lines[0], "VALUE ERROR");
    assert_eq!(out(&mut s, ")FNS"), vec!["F G"], "G is a global function");
    assert_eq!(out(&mut s, ")VARS"), Vec::<String>::new(), "G is local");
    assert_eq!(out(&mut s, "G"), vec!["5"], "the local is in scope");
    out(&mut s, "→");
    assert_eq!(out(&mut s, "G 1"), vec!["2"]);
}

#[test]
fn a_function_fixed_under_a_local_name_is_local() {
    let mut b = in_mode(Mode::B);
    define(
        &mut b,
        &["∇R←TWICE X;H", "R←⎕FX 3 5⍴'R←H YR←Y+Y     '", "R←H X", "∇"],
    );
    assert_eq!(out(&mut b, "TWICE 4"), vec!["8"]);
    assert_eq!(out(&mut b, "⎕NC 'H'"), vec!["0"], "gone with the call");
    // A local function hides a global one, and expunging it takes the
    // local only.
    define(&mut b, &["∇R←H Y", "R←0", "∇"]);
    assert_eq!(out(&mut b, "TWICE 4"), vec!["8"]);
    assert_eq!(out(&mut b, "H 4"), vec!["0"], "the global H is back");
    define(
        &mut b,
        &[
            "∇R←GONE;H;X",
            "X←⎕FX 1 1⍴'H'",
            "X←⎕EX 'H'",
            "R←X,⎕NC 'H'",
            "∇",
        ],
    );
    assert_eq!(out(&mut b, "GONE"), vec!["1 0"]);
    assert_eq!(out(&mut b, "⎕NC 'H'"), vec!["3"], "the global H survives");
}

#[test]
fn a_save_under_a_suspension_keeps_the_globals_not_the_locals() {
    let dir = std::env::temp_dir().join(format!("sw-apl-locals-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut s = in_mode(Mode::A);
    s.ws.store = Box::new(apl_session::Files(dir.clone()));
    define(
        &mut s,
        &[
            "V←1",
            "∇R←G X",
            "R←X+1",
            "∇",
            "∇H;G;V",
            "G←5",
            "V←7",
            "NOSUCH",
            "∇",
        ],
    );
    assert_eq!(s.respond("H").lines[0], "VALUE ERROR");
    out(&mut s, ")SAVE T");
    let mut t = in_mode(Mode::A);
    t.ws.store = Box::new(apl_session::Files(dir.clone()));
    out(&mut t, ")LOAD T");
    assert_eq!(out(&mut t, "V"), vec!["1"], "the global V, not the local");
    assert_eq!(out(&mut t, "G 1"), vec!["2"], "the global function G");
    let _ = std::fs::remove_dir_all(&dir);
}
