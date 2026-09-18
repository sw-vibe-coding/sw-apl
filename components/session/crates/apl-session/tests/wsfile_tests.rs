//! The workspace file. Writing it is apl-wsfile's job; reading it is
//! not parsing but running it, so the round trip is tested here,
//! where there is a session to run it in.

use apl_session::Session;
use apl_wsfile::write;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    let reply = s.respond(line);
    assert!(!reply.off, "unexpected )OFF from {line}");
    reply.lines
}

/// A session holding a workspace worth saving: values of several
/// ranks and types, functions with locals and labels, settings away
/// from their defaults, a random link that has moved, and a name.
///
/// SUM branches on the condition itself rather than on `⍳` of it,
/// because this workspace is in origin 0 and `⍳1` is `0` there: the
/// familiar `→LABEL×⍳COND` is an origin-1 idiom.
fn furnished() -> Session {
    let mut s = Session::default();
    for line in [
        ")WSID CLASS",
        ")ORIGIN 0",
        ")DIGITS 3",
        ")WIDTH 80",
        "A\u{2190}5",
        "V\u{2190}1 2 3",
        "ONE\u{2190}1\u{2374}7",
        "M\u{2190}2 3\u{2374}\u{2373}6",
        "T\u{2190}'IT''S'",
        "E\u{2190}\u{2373}0",
        "F\u{2190}\u{af}1.5",
        "R\u{2190}?100",
    ] {
        out(&mut s, line);
    }
    for line in [
        "\u{2207}R\u{2190}SUM N;I",
        "R\u{2190}0",
        "I\u{2190}0",
        "LOOP:I\u{2190}I+1",
        "R\u{2190}R+I",
        "\u{2192}LOOP\u{d7}I<N",
        "\u{2207}",
        "\u{2207}R\u{2190}A HYP B",
        "R\u{2190}((A*2)+B*2)*0.5",
        "\u{2207}",
    ] {
        out(&mut s, line);
    }
    s
}

#[test]
fn a_workspace_written_out_and_run_back_in_is_the_same_workspace() {
    let mut before = furnished();
    let text = write(&before.ws.saved, "20.00.00 09/17/26");
    // Run it into a session that knows nothing.
    let mut after = Session::default();
    for line in text.lines() {
        out(&mut after, line);
    }
    let (a, b) = (&before.ws.saved, &after.ws.saved);
    assert_eq!(a.id, b.id, "the name");
    assert_eq!(a.env.io, b.env.io, "index origin");
    assert_eq!(a.print, b.print, "print settings");
    let mut names: Vec<&String> = a.vars.keys().collect();
    names.sort();
    for name in names {
        assert_eq!(a.vars[name], b.vars[name], "variable {name}");
    }
    assert_eq!(a.funcs.len(), b.funcs.len(), "how many functions");
    for (name, defn) in &a.funcs {
        assert_eq!(**defn, **b.funcs.get(name).expect(name), "function {name}");
    }
    // And it still computes, which is the point of it.
    assert_eq!(out(&mut after, "SUM 10"), vec!["55"]);
    assert_eq!(out(&mut after, "3 HYP 4"), vec!["5"]);
    assert_eq!(out(&mut before, "SUM 10"), out(&mut after, "SUM 10"));
}

#[test]
fn the_same_workspace_writes_the_same_bytes_twice() {
    let s = furnished();
    let once = write(&s.ws.saved, "20.00.00 09/17/26");
    for _ in 0..8 {
        assert_eq!(write(&s.ws.saved, "20.00.00 09/17/26"), once);
    }
}

#[test]
fn a_locked_function_is_carried_by_the_file() {
    let mut s = Session::default();
    for line in ["\u{2207}R\u{2190}SECRET", "R\u{2190}42", "\u{236b}"] {
        out(&mut s, line);
    }
    let text = write(&s.ws.saved, "20.00.00 09/17/26");
    assert!(text.contains("\u{236b}"), "closed locked: {text}");
    let mut after = Session::default();
    for line in text.lines() {
        out(&mut after, line);
    }
    assert_eq!(out(&mut after, "SECRET"), vec!["42"]);
    assert_eq!(out(&mut after, "\u{2207}SECRET")[0], "DEFN ERROR");
}

/// The file's shape, not only that it round trips: a fixture in
/// `tests/scripts/` is what a reader sees, and a change to the
/// format has to be a deliberate edit of it.
#[test]
fn the_file_looks_like_the_fixture() {
    let text = write(&furnished().ws.saved, "20.00.00 09/17/26");
    let fixture = include_str!("../../../../../tests/scripts/saved-workspace.apl.ws");
    if text != fixture {
        std::fs::write("/tmp/sw-apl-workspace-written.txt", &text).ok();
        panic!(
            "the workspace file changed shape.\nWritten to \
             /tmp/sw-apl-workspace-written.txt; if the change is \
             meant, copy it over tests/scripts/saved-workspace.apl.ws\n\
             --- written ---\n{text}--- fixture ---\n{fixture}"
        );
    }
}
