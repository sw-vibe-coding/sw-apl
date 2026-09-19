//! The workspace commands, driven through a session because that is
//! what they act on. Each runs with library 0 pointed at a directory
//! of its own, so nothing is written into the repository and tests
//! cannot tread on each other.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    let reply = s.respond(line);
    reply.lines
}

/// A session whose libraries are a directory of its own, so `)SAVE`
/// writes nothing into the repository and no two tests tread on each
/// other. The library root is session state rather than the process's
/// working directory, precisely so this needs no global change --
/// cargo runs these on parallel threads of one process.
fn in_own_dir(name: &str) -> (Session, Guard) {
    let dir = std::env::temp_dir().join(format!("sw-apl-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).expect("mkdir");
    let mut session = Session::default();
    session.ws.libraries.clone_from(&dir);
    (session, Guard(dir))
}

/// Removes the directory when the test ends, however it ends.
struct Guard(std::path::PathBuf);

impl Drop for Guard {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}

#[test]
fn a_workspace_saved_and_loaded_comes_back_whole() {
    let (mut s, dir) = in_own_dir("save");
    for line in [")WSID CLASS", "A\u{2190}5", ")ORIGIN 0", ")DIGITS 3"] {
        out(&mut s, line);
    }
    for line in ["\u{2207}R\u{2190}HYP B", "R\u{2190}B\u{d7}2", "\u{2207}"] {
        out(&mut s, line);
    }
    // )SAVE replies with the moment and the name, and makes library 0.
    let saved = out(&mut s, ")SAVE");
    assert_eq!(saved.len(), 1);
    assert!(saved[0].ends_with(" CLASS"), "{saved:?}");
    assert!(dir.0.join("work/CLASS.apl.ws").exists());
    assert_eq!(out(&mut s, ")LIB"), vec!["CLASS"]);
    out(&mut s, ")CLEAR");
    // )LOAD says SAVED and nothing else: no hint, nothing echoed.
    let loaded = out(&mut s, ")LOAD CLASS");
    assert_eq!(loaded.len(), 1);
    assert!(loaded[0].starts_with("SAVED "), "{loaded:?}");
    assert_eq!(out(&mut s, ")WSID"), vec!["CLASS"]);
    assert_eq!(out(&mut s, "A"), vec!["5"]);
    assert_eq!(out(&mut s, "HYP 4"), vec!["8"]);
    assert_eq!(out(&mut s, "\u{2373}3"), vec!["0 1 2"], "origin came back");
    assert_eq!(out(&mut s, "1\u{f7}3"), vec!["0.333"], "digits came back");
}

/// The manual gives `)COPY` the same SAVED line `)LOAD` prints:
/// "SAVED, followed by the time of day and the date that the source
/// workspace was last stored."
#[test]
fn copying_says_when_the_source_was_stored() {
    let (mut s, _dir) = in_own_dir("copy-saved");
    for line in [")WSID DONOR", "A\u{2190}5", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    let reply = out(&mut s, ")COPY DONOR");
    assert_eq!(reply.len(), 1, "{reply:?}");
    assert!(reply[0].starts_with("SAVED "), "{reply:?}");
    // Naming what to copy does not change the reply.
    let named = out(&mut s, ")COPY DONOR A");
    assert_eq!(named.len(), 1, "{named:?}");
    assert!(named[0].starts_with("SAVED "), "{named:?}");
}

/// And `)PCOPY` adds "NOT COPIED:, followed by the names of objects
/// not copied". Without it, a protected copy that skipped the name
/// you asked for cannot be told from one that worked.
#[test]
fn a_protected_copy_names_what_it_would_not_overwrite() {
    let (mut s, _dir) = in_own_dir("pcopy-report");
    for line in [")WSID DONOR", "A\u{2190}5", "B\u{2190}6", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    out(&mut s, "A\u{2190}99");
    let reply = out(&mut s, ")PCOPY DONOR");
    assert_eq!(reply.len(), 2, "{reply:?}");
    assert!(reply[0].starts_with("SAVED "), "{reply:?}");
    assert_eq!(reply[1], "NOT COPIED: A");
    assert_eq!(out(&mut s, "A"), vec!["99"], "kept");
    assert_eq!(out(&mut s, "B"), vec!["6"], "taken");
    // Nothing in the way, nothing to report.
    out(&mut s, ")CLEAR");
    assert_eq!(out(&mut s, ")PCOPY DONOR").len(), 1);
}

#[test]
fn several_names_kept_are_reported_together() {
    let (mut s, _dir) = in_own_dir("pcopy-several");
    for line in [
        ")WSID DONOR",
        "A\u{2190}5",
        "B\u{2190}6",
        "C\u{2190}7",
        ")SAVE",
        ")CLEAR",
    ] {
        out(&mut s, line);
    }
    for line in ["A\u{2190}99", "C\u{2190}99"] {
        out(&mut s, line);
    }
    let reply = out(&mut s, ")PCOPY DONOR");
    assert_eq!(reply[1], "NOT COPIED: A C", "in the file's order");
}

/// An ordinary `)COPY` overwrites, so it never keeps anything and
/// the manual gives it no NOT COPIED line.
#[test]
fn an_unprotected_copy_reports_nothing_kept() {
    let (mut s, _dir) = in_own_dir("copy-overwrites");
    for line in [")WSID DONOR", "A\u{2190}5", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    out(&mut s, "A\u{2190}99");
    assert_eq!(out(&mut s, ")COPY DONOR").len(), 1);
    assert_eq!(out(&mut s, "A"), vec!["5"], "overwritten");
}

#[test]
fn copying_takes_the_names_and_leaves_the_settings() {
    let (mut s, _dir) = in_own_dir("copy");
    for line in [
        ")WSID DONOR",
        "A\u{2190}5",
        "B\u{2190}6",
        ")ORIGIN 0",
        ")SAVE",
    ] {
        out(&mut s, line);
    }
    out(&mut s, ")CLEAR");
    out(&mut s, "A\u{2190}99");
    assert_eq!(
        out(&mut s, "\u{2373}3"),
        vec!["1 2 3"],
        "this workspace is origin 1"
    );
    out(&mut s, ")COPY DONOR");
    assert_eq!(out(&mut s, "A"), vec!["5"], "copied over");
    assert_eq!(out(&mut s, "B"), vec!["6"]);
    // The donor was origin 0 and this workspace is not: copying must
    // not change the origin under code already written here.
    assert_eq!(out(&mut s, "\u{2373}3"), vec!["1 2 3"]);
    assert_eq!(out(&mut s, ")WSID"), vec!["CLEAR WS"], "nor the name");
}

#[test]
fn protected_copy_leaves_a_name_that_is_already_here() {
    let (mut s, _dir) = in_own_dir("pcopy");
    for line in [")WSID DONOR", "A\u{2190}5", "B\u{2190}6", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    out(&mut s, "A\u{2190}99");
    out(&mut s, ")PCOPY DONOR");
    assert_eq!(out(&mut s, "A"), vec!["99"], "kept");
    assert_eq!(out(&mut s, "B"), vec!["6"], "taken");
}

#[test]
fn copying_can_name_what_it_wants() {
    let (mut s, _dir) = in_own_dir("copy-one");
    for line in [")WSID DONOR", "A\u{2190}5", "B\u{2190}6", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    out(&mut s, ")COPY DONOR B");
    assert_eq!(out(&mut s, "B"), vec!["6"]);
    assert_eq!(out(&mut s, "A")[0], "VALUE ERROR");
}

#[test]
fn dropping_removes_a_saved_workspace() {
    let (mut s, _dir) = in_own_dir("drop");
    out(&mut s, ")WSID GONE");
    out(&mut s, ")SAVE");
    assert_eq!(out(&mut s, ")LIB"), vec!["GONE"]);
    // The reply is the moment it was dropped and nothing else, as
    // APL\360's )DROP printed the time and the date.
    assert_eq!(out(&mut s, ")DROP GONE"), vec!["0.00.00 00/00/00"]);
    assert_eq!(out(&mut s, ")LIB"), Vec::<String>::new());
    assert_eq!(out(&mut s, ")DROP GONE"), vec!["WS NOT FOUND"]);
}

#[test]
fn a_clear_workspace_has_no_name_to_save_under() {
    let (mut s, _dir) = in_own_dir("noname");
    let reply = out(&mut s, ")SAVE");
    assert_eq!(reply, vec!["NOT SAVED, THIS WS IS CLEAR WS"]);
    // Naming it on the command names the workspace too.
    out(&mut s, ")SAVE FIRST");
    assert_eq!(out(&mut s, ")WSID"), vec!["FIRST"]);
}

#[test]
fn continue_saves_and_signs_off() {
    let (mut s, dir) = in_own_dir("continue");
    out(&mut s, "A\u{2190}1");
    let reply = s.respond(")CONTINUE");
    assert!(reply.off, "the session ended");
    assert!(dir.0.join("work/CONTINUE.apl.ws").exists());
    assert!(reply.lines.iter().any(|l| l.starts_with("CONNECTED ")));
}

/// INCORRECT COMMAND is for a command given an argument it does not
/// take. What it is *not* for -- a name that is simply not there --
/// is `report_tests.rs`.
#[test]
fn a_command_given_what_it_does_not_take_is_incorrect() {
    let (mut s, _dir) = in_own_dir("bad");
    for bad in [")LOAD", ")DROP", ")DROP 1 NAME", ")LIB TWO", ")LIB 1 2"] {
        assert_eq!(out(&mut s, bad), vec!["INCORRECT COMMAND"], "{bad}");
    }
}
