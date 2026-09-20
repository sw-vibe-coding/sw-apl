//! A system command typed while a definition is open.
//!
//! The manual: "A system command entered during function definition
//! will not be accepted as a statement in the definition. Some
//! commands, such as )COPY, will be rejected with the message NOT
//! WITH OPEN DEFINITION (see Table 2.1); most will be executed
//! immediately."
//!
//! So there are two rules, and the first is the larger: a `)` line
//! in definition mode is never a body line. sw-apl used to make one,
//! which turned a mistyped command into a statement that failed
//! later, when the function was run.

use apl_session::{Files, Session};

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

/// Open FOO and write one line into it.
fn defining(name: &str) -> Session {
    let mut s = Session::default();
    out(&mut s, &format!("\u{2207}{name}"));
    out(&mut s, "A\u{2190}1");
    s
}

#[test]
fn a_command_is_never_a_body_line() {
    let mut s = defining("FOO");
    out(&mut s, ")FNS");
    out(&mut s, ")SAVE");
    out(&mut s, "B\u{2190}2");
    out(&mut s, "\u{2207}");
    // Two statements, not four: the commands did not go in.
    let shown = out(&mut s, "\u{2207}FOO[\u{2395}]\u{2207}");
    assert_eq!(shown.len(), 4, "{shown:?}");
    assert!(shown[1].contains("A\u{2190}1"), "{shown:?}");
    assert!(shown[2].contains("B\u{2190}2"), "{shown:?}");
}

#[test]
fn the_four_that_store_or_copy_are_refused() {
    let mut s = defining("FOO");
    for command in [")SAVE", ")SAVE NAME", ")COPY X", ")PCOPY X", ")CONTINUE"] {
        assert_eq!(
            out(&mut s, command),
            vec!["NOT WITH OPEN DEFINITION"],
            "{command}"
        );
    }
    // Shortened names are the same commands, so they are refused too.
    assert_eq!(out(&mut s, ")CONT"), vec!["NOT WITH OPEN DEFINITION"]);
    assert_eq!(out(&mut s, ")PCOP X"), vec!["NOT WITH OPEN DEFINITION"]);
}

#[test]
fn the_definition_survives_being_refused() {
    let mut s = defining("FOO");
    out(&mut s, ")SAVE");
    // Still open, still on the next line, and the line written after
    // the refusal is the second statement.
    assert_eq!(s.prompt(), "[2]   ");
    out(&mut s, "B\u{2190}2");
    out(&mut s, "\u{2207}");
    assert!(s.ws.is_function("FOO"));
    assert_eq!(out(&mut s, "FOO"), Vec::<String>::new());
    assert_eq!(out(&mut s, "B"), vec!["2"]);
}

#[test]
fn every_other_command_runs_at_once() {
    let mut s = Session::default();
    out(&mut s, "V\u{2190}7");
    let mut s = {
        out(&mut s, "\u{2207}FOO");
        s
    };
    // The listings answer, from inside definition mode.
    assert_eq!(out(&mut s, ")VARS"), vec!["V"]);
    assert_eq!(out(&mut s, ")SI"), Vec::<String>::new());
    // And a setting takes effect, replying as it always does.
    assert_eq!(out(&mut s, ")ORIGIN 0"), vec!["WAS 1"]);
    out(&mut s, "R\u{2190}\u{2373}3");
    out(&mut s, "\u{2207}");
    assert_eq!(out(&mut s, "FOO"), Vec::<String>::new());
    assert_eq!(out(&mut s, "R"), vec!["0 1 2"], "origin 0, as set");
}

#[test]
fn loading_is_not_refused_and_the_definition_goes_with_the_workspace() {
    // )LOAD has no report 6 in the manual, and the reason is worth
    // keeping: a load replaces the whole workspace, so there is
    // nothing half-written left behind to be inconsistent with.
    let dir = std::env::temp_dir().join(format!("sw-apl-opendef-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut s = Session::default();
    s.ws.store = Box::new(Files(dir.clone()));
    for line in [")WSID STORED", "KEPT\u{2190}5", ")SAVE", ")CLEAR"] {
        out(&mut s, line);
    }
    out(&mut s, "\u{2207}FOO");
    out(&mut s, "A\u{2190}1");
    let loaded = out(&mut s, ")LOAD STORED");
    assert!(loaded[0].starts_with("SAVED "), "{loaded:?}");
    assert_eq!(out(&mut s, "KEPT"), vec!["5"]);
    // The open definition went with the workspace it belonged to,
    // rather than swallowing the lines of the one being loaded.
    assert_eq!(s.prompt(), "      ", "not in definition mode");
    assert!(!s.ws.is_function("FOO"));
    std::fs::remove_dir_all(&dir).ok();
}
