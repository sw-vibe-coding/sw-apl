//! What quad input does with a reply that is not an expression. The
//! manuals agree, APL\360's and the 5100's and 5110's: an invalid
//! entry gets its error report and the request is made again; a
//! system command is executed and the request is made again, except
//! one that replaces the active workspace. Quote-quad takes either as
//! characters.

use std::collections::VecDeque;

use apl_session::{Console, Mode, Session, Shown};

/// A console answering reads from lines typed in advance, and keeping
/// everything it was shown, prompts and answers included.
#[derive(Debug, Default)]
struct Typist {
    typed: VecDeque<String>,
    shown: Vec<String>,
}

impl Console for Typist {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        self.shown.extend(shown.lines.iter().cloned());
        if !prompt.is_empty() {
            self.shown.push(prompt.to_string());
        }
        let typed = self.typed.pop_front()?;
        self.shown.push(format!("      {typed}"));
        Some(typed)
    }

    fn take(&mut self) -> Vec<String> {
        std::mem::take(&mut self.shown)
    }
}

/// A session in `mode` holding the report's two functions: `T` shows
/// a line and calls `F`, and `F` reads its result with quad.
fn session(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    for line in ["∇R←F", "R←⎕", "∇", "∇T", "'X'", "X←F", "∇"] {
        assert!(!s.respond(line).error, "{line}");
    }
    s
}

/// Run `T`, answering the reads with `typed`.
fn run(s: &mut Session, typed: &[&str]) -> apl_session::Reply {
    let typist = Typist {
        typed: typed.iter().map(ToString::to_string).collect(),
        ..Typist::default()
    };
    s.ws.console = Box::new(typist);
    s.respond("T")
}

fn value(s: &mut Session, name: &str) -> Vec<String> {
    s.respond(name).lines
}

#[test]
fn an_inquiry_is_answered_and_the_request_made_again() {
    for mode in [Mode::A, Mode::B] {
        let mut s = session(mode);
        let reply = run(&mut s, &[")SI", "5"]);
        assert!(!reply.error, "{mode:?}: {:?}", reply.lines);
        let expected = ["X", "⎕:", "      )SI", "F[1]", "T[2]", "⎕:", "      5"];
        assert_eq!(reply.lines, expected, "{mode:?}");
        assert_eq!(value(&mut s, "X"), ["5"], "{mode:?}");
        assert_eq!(
            value(&mut s, ")SI"),
            [] as [&str; 0],
            "{mode:?}: nothing waits"
        );
    }
}

#[test]
fn every_inquiry_runs_at_the_prompt() {
    let mut s = session(Mode::B);
    let reply = run(&mut s, &[")FNS", ")DIALECT", ")LIBS", ")WSID", "5"]);
    assert!(!reply.error, "{:?}", reply.lines);
    for line in ["F T", "(B) '75", "0 USER", "1 CORE", "CLEAR WS"] {
        assert!(
            reply.lines.iter().any(|l| l == line),
            "{line}: {:?}",
            reply.lines
        );
    }
    assert_eq!(reply.lines.iter().filter(|l| *l == "⎕:").count(), 5);
}

#[test]
fn an_error_in_the_reply_is_reported_and_the_request_made_again() {
    for mode in [Mode::A, Mode::B] {
        let mut s = session(mode);
        let reply = run(&mut s, &["1 2 3+4 5", "7"]);
        assert!(!reply.error, "{mode:?}: {:?}", reply.lines);
        let expected = [
            "X",
            "⎕:",
            "      1 2 3+4 5",
            "LENGTH ERROR",
            "      1 2 3+4 5",
            "           ^",
            "⎕:",
            "      7",
        ];
        assert_eq!(reply.lines, expected, "{mode:?}");
        assert_eq!(value(&mut s, "X"), ["7"], "{mode:?}");
        assert_eq!(
            value(&mut s, ")SI"),
            [] as [&str; 0],
            "{mode:?}: nothing suspended"
        );
    }
}

#[test]
fn a_function_failing_in_the_reply_leaves_nothing_on_the_state_indicator() {
    let mut s = session(Mode::A);
    for line in ["∇R←G", "R←÷0", "∇"] {
        assert!(!s.respond(line).error, "{line}");
    }
    let reply = run(&mut s, &["G", "7"]);
    assert!(!reply.error, "{:?}", reply.lines);
    assert!(
        reply.lines.contains(&"DOMAIN ERROR".to_string()),
        "{:?}",
        reply.lines
    );
    assert!(
        reply.lines.contains(&"G[1]  R←÷0".to_string()),
        "{:?}",
        reply.lines
    );
    assert_eq!(value(&mut s, "X"), ["7"]);
    assert_eq!(value(&mut s, ")SI"), [] as [&str; 0]);
}

#[test]
fn clear_abandons_the_request() {
    let mut s = session(Mode::B);
    let reply = run(&mut s, &[")CLEAR"]);
    assert!(!reply.error, "{:?}", reply.lines);
    assert_eq!(reply.lines, ["X", "⎕:", "      )CLEAR", "CLEAR WS"]);
    assert_eq!(value(&mut s, ")FNS"), [] as [&str; 0], "a clear workspace");
    assert_eq!(value(&mut s, ")SI"), [] as [&str; 0]);
}

#[test]
fn off_abandons_the_request_and_ends_the_session() {
    let mut s = session(Mode::A);
    let reply = run(&mut s, &[")OFF"]);
    assert!(reply.off, "{:?}", reply.lines);
}

#[test]
fn a_command_in_reply_to_quote_quad_is_characters() {
    let mut s = session(Mode::A);
    for line in ["∇R←Q", "R←⍞", "∇"] {
        assert!(!s.respond(line).error, "{line}");
    }
    s.ws.console = Box::new(Typist {
        typed: [")SI".to_string()].into(),
        ..Typist::default()
    });
    assert!(!s.respond("Y←Q").error);
    assert_eq!(value(&mut s, "Y"), [")SI"]);
}

#[test]
fn a_branch_still_escapes_the_request() {
    let mut s = session(Mode::A);
    let reply = run(&mut s, &["→"]);
    assert!(reply.error, "escaping is an interrupt: {:?}", reply.lines);
}
