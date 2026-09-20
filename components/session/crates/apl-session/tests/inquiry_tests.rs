//! The inquiry commands as a user meets them, and what a group does
//! when a workspace is saved, loaded and copied from. Each test that
//! saves has a library root of its own.

use apl_session::{Files, Session};

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn in_own_dir(name: &str) -> (Session, Guard) {
    let dir = std::env::temp_dir().join(format!("sw-apl-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut session = Session::default();
    session.ws.store = Box::new(Files(dir.clone()));
    (session, Guard(dir))
}

struct Guard(std::path::PathBuf);

impl Drop for Guard {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}

#[test]
fn the_listings_answer_what_the_workspace_holds() {
    let mut s = Session::default();
    out(&mut s, "ZED\u{2190}1");
    out(&mut s, "ALPHA\u{2190}2");
    for line in ["\u{2207}R\u{2190}HYP B", "R\u{2190}B\u{d7}2", "\u{2207}"] {
        out(&mut s, line);
    }
    assert_eq!(out(&mut s, ")VARS"), vec!["ALPHA ZED"]);
    assert_eq!(out(&mut s, ")FNS"), vec!["HYP"]);
    assert_eq!(out(&mut s, ")GRPS"), Vec::<String>::new());
    assert_eq!(out(&mut s, ")VARS B"), vec!["ZED"], "from a letter");
}

#[test]
fn vars_under_a_suspension_shows_the_globals() {
    let mut s = Session::default();
    out(&mut s, "GLOBAL\u{2190}1");
    out(&mut s, "SHADOWED\u{2190}2");
    for line in [
        "\u{2207}F;LOCAL",
        "SHADOWED\u{2190}9",
        "LOCAL\u{2190}9",
        "NOSUCH",
        "\u{2207}",
    ] {
        out(&mut s, line);
    }
    assert_eq!(out(&mut s, "F")[0], "VALUE ERROR", "F suspends on line 3");
    assert_eq!(out(&mut s, ")SI")[0], "F[3]*", "and is on the stack");
    assert_eq!(out(&mut s, ")VARS"), vec!["GLOBAL SHADOWED"]);
    assert_eq!(out(&mut s, "SHADOWED"), vec!["9"], "the local is in scope");
}

#[test]
fn a_group_survives_a_save_and_a_load() {
    let (mut s, _dir) = in_own_dir("groups");
    out(&mut s, ")WSID GROUPED");
    out(&mut s, "A\u{2190}1");
    out(&mut s, "B\u{2190}2");
    out(&mut s, ")GROUP G A B MISSING");
    out(&mut s, ")SAVE");
    out(&mut s, ")CLEAR");
    assert_eq!(out(&mut s, ")GRPS"), Vec::<String>::new());
    out(&mut s, ")LOAD GROUPED");
    assert_eq!(out(&mut s, ")GRPS"), vec!["G"]);
    assert_eq!(out(&mut s, ")GRP G"), vec!["A B MISSING"]);
}

#[test]
fn copying_a_group_brings_what_it_gathers() {
    let (mut s, _dir) = in_own_dir("copy-group");
    out(&mut s, ")WSID DONOR");
    for line in ["A\u{2190}1", "B\u{2190}2", "SPARE\u{2190}3", ")GROUP G A B"] {
        out(&mut s, line);
    }
    out(&mut s, ")SAVE");
    out(&mut s, ")CLEAR");
    out(&mut s, ")COPY DONOR G");
    assert_eq!(out(&mut s, ")VARS"), vec!["A B"], "the members came too");
    assert_eq!(out(&mut s, ")GRP G"), vec!["A B"], "and the group itself");
    assert_eq!(out(&mut s, "SPARE")[0], "VALUE ERROR", "but nothing else");
}

#[test]
fn erasing_a_group_clears_the_names_it_gathers() {
    let mut s = Session::default();
    for line in ["A\u{2190}1", "B\u{2190}2", "KEEP\u{2190}3", ")GROUP G A B"] {
        out(&mut s, line);
    }
    assert_eq!(out(&mut s, ")ERASE G"), Vec::<String>::new());
    assert_eq!(out(&mut s, ")VARS"), vec!["KEEP"]);
    assert_eq!(out(&mut s, ")GRPS"), Vec::<String>::new());
}

#[test]
fn a_suspended_function_cannot_be_erased() {
    let mut s = Session::default();
    for line in ["\u{2207}F", "NOSUCH", "\u{2207}"] {
        out(&mut s, line);
    }
    out(&mut s, "F");
    assert_eq!(out(&mut s, ")ERASE F"), vec!["NOT ERASED: F"]);
    assert!(out(&mut s, ")FNS").contains(&"F".to_string()));
    // Clearing the state indicator lets it go.
    out(&mut s, "\u{2192}");
    assert_eq!(out(&mut s, ")ERASE F"), Vec::<String>::new());
    assert_eq!(out(&mut s, ")FNS"), Vec::<String>::new());
}

/// `IS n, USED m` as the pair of numbers it reports.
fn symbols(s: &mut Session) -> (usize, usize) {
    let reply = out(s, ")SYMBOLS").remove(0);
    let (is, used) = reply.split_once(", USED ").expect("IS n, USED m");
    let is = is
        .strip_prefix("IS ")
        .expect("IS n")
        .parse()
        .expect("a number");
    (is, used.parse().expect("a number"))
}

#[test]
fn symbols_counts_the_names_and_falls_as_the_workspace_fills() {
    let mut s = Session::default();
    assert_eq!(symbols(&mut s).1, 0, "a clear workspace holds no names");
    out(&mut s, "A\u{2190}\u{2373}100");
    for line in ["\u{2207}F", "\u{2207}"] {
        out(&mut s, line);
    }
    out(&mut s, ")GROUP G A");
    let (is, used) = symbols(&mut s);
    assert_eq!(used, 3, "a variable, a function and a group");
    // The size is what the space left would hold, so it falls with
    // the space rather than standing still.
    out(&mut s, "B\u{2190}\u{2373}1000");
    let (after, used) = symbols(&mut s);
    assert_eq!(used, 4);
    assert!(after < is, "{is} then {after}");
}
