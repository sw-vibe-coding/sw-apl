//! The trouble reports, against the table in the IBM APL\360 User's
//! Manual (Aug 1968). The distinction these pin is the one sw-apl
//! had wrong: INCORRECT COMMAND is for a command given an argument
//! it does not take, and a name that is simply not there is a
//! different answer.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

/// A session with one stored workspace, DONOR, holding A and B.
fn with_a_donor(name: &str) -> (Session, Guard) {
    let dir = std::env::temp_dir().join(format!("sw-apl-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut s = Session::default();
    s.ws.libraries.clone_from(&dir);
    for line in [")WSID DONOR", "A\u{2190}1", "B\u{2190}2", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    (s, Guard(dir))
}

struct Guard(std::path::PathBuf);

impl Drop for Guard {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}

#[test]
fn a_workspace_that_is_not_there_is_ws_not_found() {
    let (mut s, _g) = with_a_donor("wsnf");
    for bad in [
        ")LOAD NOSUCH",
        ")LOAD 1 NOSUCH",
        ")COPY NOSUCH",
        ")PCOPY NOSUCH",
        ")COPY NOSUCH A",
        ")DROP NOSUCH",
    ] {
        assert_eq!(out(&mut s, bad), vec!["WS NOT FOUND"], "{bad}");
    }
}

#[test]
fn a_name_the_workspace_does_not_hold_is_object_not_found() {
    let (mut s, _g) = with_a_donor("objnf");
    assert_eq!(out(&mut s, ")COPY DONOR NOSUCH"), vec!["OBJECT NOT FOUND"]);
    assert_eq!(out(&mut s, ")PCOPY DONOR NOSUCH"), vec!["OBJECT NOT FOUND"]);
    // One missing name is enough: nothing is copied, so a copy that
    // half worked cannot be mistaken for one that worked.
    assert_eq!(
        out(&mut s, ")COPY DONOR A NOSUCH"),
        vec!["OBJECT NOT FOUND"]
    );
    assert_eq!(out(&mut s, "A")[0], "VALUE ERROR", "and A did not arrive");
    // The names it does hold copy, and the copy says when the
    // source was stored.
    assert!(out(&mut s, ")COPY DONOR A")[0].starts_with("SAVED "));
    assert_eq!(out(&mut s, "A"), vec!["1"]);
}

#[test]
fn a_number_that_names_no_library_is_an_improper_reference() {
    let (mut s, _g) = with_a_donor("lib");
    for bad in [")LIB 9", ")LOAD 9 DONOR", ")COPY 9 DONOR", ")PCOPY 9 DONOR"] {
        assert_eq!(
            out(&mut s, bad),
            vec!["IMPROPER LIBRARY REFERENCE"],
            "{bad}"
        );
    }
    // An empty library is not an improper one: library 1 exists here
    // and holds nothing, and silence is the right answer for that.
    assert_eq!(out(&mut s, ")LIB 1"), Vec::<String>::new());
}

#[test]
fn saving_over_a_workspace_that_is_not_this_one_is_refused() {
    let (mut s, _g) = with_a_donor("save-over");
    out(&mut s, ")WSID MINE");
    out(&mut s, "KEEP\u{2190}9");
    assert_eq!(
        out(&mut s, ")SAVE DONOR"),
        vec!["NOT SAVED, THIS WS IS MINE"],
        "DONOR is stored and is not this workspace"
    );
    // The stored one is untouched: it still has A, and no KEEP.
    out(&mut s, ")LOAD DONOR");
    assert_eq!(out(&mut s, "A"), vec!["1"]);
    assert_eq!(out(&mut s, "KEEP")[0], "VALUE ERROR");
    // Re-storing this workspace under its own name is always allowed.
    assert_eq!(out(&mut s, ")SAVE").len(), 1);
    assert_eq!(out(&mut s, ")SAVE DONOR").len(), 1);
}

#[test]
fn a_name_not_yet_stored_saves_under_it() {
    let (mut s, _g) = with_a_donor("save-new");
    out(&mut s, "X\u{2190}1");
    let reply = out(&mut s, ")SAVE FRESH");
    assert_eq!(reply.len(), 1, "{reply:?}");
    assert!(reply[0].ends_with(" FRESH"), "{reply:?}");
    assert_eq!(out(&mut s, ")WSID"), vec!["FRESH"]);
}

#[test]
fn a_name_that_is_not_a_name_is_refused_before_it_reaches_a_path() {
    let (mut s, _g) = with_a_donor("names");
    // Each of these worked before there was a rule. A/B made a
    // directory; ../../ESCAPED wrote outside the library.
    for bad in [
        ")SAVE WS:PASS",
        ")SAVE A/B",
        ")SAVE ../../ESCAPED",
        ")SAVE .HIDDEN",
        ")SAVE 1DIGIT",
        ")LOAD ../DONOR",
        ")COPY A/B",
        ")DROP ../DONOR",
        ")WSID ../X",
    ] {
        assert_eq!(out(&mut s, bad), vec!["INCORRECT COMMAND"], "{bad}");
    }
    // The workspace was not renamed by the attempt.
    assert_eq!(out(&mut s, ")WSID"), vec!["CLEAR WS"]);
    // And a name that is a name still works.
    out(&mut s, ")WSID GOOD");
    assert_eq!(out(&mut s, ")SAVE").len(), 1);
    assert_eq!(out(&mut s, ")LIB"), vec!["DONOR", "GOOD"]);
}
