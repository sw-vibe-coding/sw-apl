//! The quota as a user meets it: what `⌶22` reports, when WS FULL is
//! raised, and that a load which will not fit leaves the workspace it
//! found. Each test that saves has a library root of its own, so
//! nothing is written into the repository and cargo's parallel test
//! threads cannot tread on each other.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

/// What `⌶22` says, as a number.
fn free(s: &mut Session) -> usize {
    out(s, "\u{2336}22")[0]
        .trim()
        .parse()
        .expect("a whole number")
}

/// The first line of the answer, which for an error is the error:
/// the statement and the caret follow it.
fn first(s: &mut Session, line: &str) -> String {
    out(s, line).first().cloned().unwrap_or_default()
}

#[test]
fn a_clear_workspace_has_the_whole_quota_free() {
    let mut s = Session::default();
    assert_eq!(free(&mut s), 1_048_576, "the default quota");
    // The figure is the quota, whatever the quota is.
    s.ws.quota = 4096;
    assert_eq!(free(&mut s), 4096);
}

#[test]
fn defining_names_takes_space_and_clearing_gives_it_all_back() {
    let mut s = Session::default();
    let empty = free(&mut s);
    out(&mut s, "A\u{2190}\u{2373}100");
    let after_a = free(&mut s);
    assert!(after_a < empty, "a hundred numbers cost something");
    for line in ["\u{2207}R\u{2190}HYP B", "R\u{2190}B\u{d7}2", "\u{2207}"] {
        out(&mut s, line);
    }
    assert!(free(&mut s) < after_a, "a function costs something too");
    out(&mut s, ")CLEAR");
    assert_eq!(free(&mut s), empty, "a clear workspace is empty again");
}

#[test]
fn a_smaller_value_under_the_same_name_gives_the_difference_back() {
    let mut s = Session::default();
    out(&mut s, "A\u{2190}\u{2373}100");
    let big = free(&mut s);
    out(&mut s, "A\u{2190}1");
    assert!(free(&mut s) > big, "the hundred numbers were given back");
}

#[test]
fn a_value_that_will_not_fit_is_ws_full_and_changes_nothing() {
    let mut s = Session::default();
    s.ws.quota = 512;
    let before = free(&mut s);
    assert_eq!(first(&mut s, "BIG\u{2190}\u{2373}1000"), "WS FULL");
    assert_eq!(first(&mut s, "BIG"), "VALUE ERROR", "nothing was stored");
    assert_eq!(free(&mut s), before, "and no space was taken");
}

#[test]
fn a_function_that_will_not_fit_is_ws_full() {
    let mut s = Session::default();
    s.ws.quota = 32;
    out(&mut s, "\u{2207}R\u{2190}LONG B");
    out(&mut s, "R\u{2190}B\u{d7}2");
    // Closing the definition is where it is stored, so that is where
    // it is refused.
    assert_eq!(first(&mut s, "\u{2207}"), "WS FULL");
    assert_eq!(first(&mut s, "LONG 2"), "SYNTAX ERROR", "not defined");
}

#[test]
fn a_call_whose_arguments_will_not_fit_is_ws_full() {
    let mut s = Session::default();
    for line in ["\u{2207}R\u{2190}F B", "R\u{2190}1", "\u{2207}"] {
        out(&mut s, line);
    }
    // Room for the function, and eight bytes over: not enough for
    // the hundred numbers the call binds to its right argument.
    let held = s.ws.quota - free(&mut s);
    s.ws.quota = held + 8;
    assert_eq!(first(&mut s, "F \u{2373}100"), "WS FULL");
}

#[test]
fn a_load_that_will_not_fit_leaves_the_workspace_it_found() {
    let dir = std::env::temp_dir().join(format!("sw-apl-quota-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut s = Session::default();
    s.ws.libraries.clone_from(&dir);
    for line in [")WSID BIG", "A\u{2190}\u{2373}500", ")SAVE"] {
        out(&mut s, line);
    }
    out(&mut s, ")CLEAR");
    out(&mut s, ")WSID MINE");
    out(&mut s, "KEEP\u{2190}7");
    s.ws.quota = 512;
    // The workspace does not fit, so none of it arrives.
    assert_eq!(first(&mut s, ")LOAD BIG"), "WS FULL");
    assert_eq!(out(&mut s, ")WSID"), vec!["MINE"], "still this workspace");
    assert_eq!(out(&mut s, "KEEP"), vec!["7"], "with what it held");
    assert_eq!(first(&mut s, "A"), "VALUE ERROR", "and nothing of BIG");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_copy_that_will_not_fit_leaves_the_workspace_it_found() {
    let dir = std::env::temp_dir().join(format!("sw-apl-quota-copy-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut s = Session::default();
    s.ws.libraries.clone_from(&dir);
    for line in [")WSID DONOR", "A\u{2190}1", "B\u{2190}\u{2373}500", ")SAVE"] {
        out(&mut s, line);
    }
    out(&mut s, ")CLEAR");
    out(&mut s, "KEEP\u{2190}7");
    s.ws.quota = 512;
    assert_eq!(first(&mut s, ")COPY DONOR"), "WS FULL");
    assert_eq!(out(&mut s, "KEEP"), vec!["7"]);
    assert_eq!(first(&mut s, "A"), "VALUE ERROR", "not even the small one");
    std::fs::remove_dir_all(&dir).ok();
}
