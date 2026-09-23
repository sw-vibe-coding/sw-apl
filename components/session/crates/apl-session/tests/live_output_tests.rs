//! Output that appears as it is written. A console that prints as it
//! goes is handed a function's output while the statement is still
//! running, as a 2741's carriage printed it; one that keeps a
//! transcript -- a test, a batch run -- is not, and sees it all at the
//! end, as before.

use std::cell::RefCell;
use std::rc::Rc;

use apl_session::{Console, Session, Shown};

/// A console that shows as it goes, and remembers each showing.
#[derive(Debug, Default)]
struct Live(Rc<RefCell<Vec<Vec<String>>>>);

impl Console for Live {
    fn read(&mut self, _shown: &Shown, _prompt: &str) -> Option<String> {
        None
    }

    fn show(&mut self, lines: &[String]) -> bool {
        self.0.borrow_mut().push(lines.to_vec());
        true
    }
}

fn session() -> (Session, Rc<RefCell<Vec<Vec<String>>>>) {
    let mut s = Session::default();
    let seen = Rc::new(RefCell::new(Vec::new()));
    s.ws.console = Box::new(Live(Rc::clone(&seen)));
    (s, seen)
}

fn define(s: &mut Session, lines: &[&str]) {
    for line in lines {
        assert!(!s.respond(line).error, "{line}");
    }
}

#[test]
fn a_functions_output_is_shown_as_it_is_written() {
    let (mut s, seen) = session();
    define(&mut s, &["∇R←F", "⎕←1", "2", "R←3", "∇"]);
    let reply = s.respond("F");
    assert_eq!(*seen.borrow(), vec![vec!["1"], vec!["2"]]);
    assert_eq!(reply.lines, vec!["3"], "the result comes at the end");
}

#[test]
fn a_line_left_open_waits_for_what_finishes_it() {
    let (mut s, seen) = session();
    define(&mut s, &["∇F", "⍞←'A'", "⎕←'B'", "∇"]);
    let reply = s.respond("F");
    assert_eq!(*seen.borrow(), vec![vec!["AB"]], "one line, joined");
    assert!(reply.lines.is_empty(), "{:?}", reply.lines);
}

#[test]
fn a_transcript_still_sees_it_all_at_the_end() {
    let mut s = Session::default();
    define(&mut s, &["∇R←F", "⎕←1", "2", "R←3", "∇"]);
    assert_eq!(s.respond("F").lines, vec!["1", "2", "3"]);
}
