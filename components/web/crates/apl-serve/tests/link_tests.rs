//! Driving a session over a link with no socket under it.
//!
//! The service is written against `Link`, so a terminal whose
//! operator has already typed everything is a terminal like any
//! other. What these check is the protocol itself: where the prompt
//! comes from, and that a statement which reads gets the line typed
//! after it -- the blocking read, which is the reason the interpreter
//! runs in a service at all.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;

use apl_serve::serve;
use apl_session::{Files, Host, Mode, QUOTA};
use apl_wire::{Frame, Link};

/// Everything the service sent, in order.
type Paper = Rc<RefCell<Vec<Frame>>>;

/// A terminal that has already been typed at.
#[derive(Debug)]
struct Typist(VecDeque<String>, Paper);

impl Link for Typist {
    fn send(&mut self, frame: &Frame) -> io::Result<()> {
        self.1.borrow_mut().push(Frame {
            lines: frame.lines.clone(),
            prompt: frame.prompt.clone(),
            off: frame.off,
        });
        Ok(())
    }

    fn recv(&mut self) -> io::Result<Option<String>> {
        Ok(self.0.pop_front())
    }
}

/// Hold a session for a terminal that types `lines`, and hand back
/// every frame it was sent.
fn session(lines: &[&str]) -> Vec<Frame> {
    let paper: Paper = Rc::new(RefCell::new(Vec::new()));
    let typed = lines.iter().map(|l| (*l).to_string()).collect();
    let link = Typist(typed, Rc::clone(&paper));
    serve(
        Box::new(link),
        Host {
            quota: QUOTA,
            store: Box::new(Files(PathBuf::from("."))),
            mode: Mode::A,
        },
    )
    .unwrap();
    paper.take()
}

/// Every prompt the terminal was given, in order.
fn prompts(sent: &[Frame]) -> Vec<String> {
    sent.iter().filter_map(|f| f.prompt.clone()).collect()
}

/// Every transcript line, in order.
fn text(sent: &[Frame]) -> Vec<String> {
    sent.iter().flat_map(|f| f.lines.clone()).collect()
}

#[test]
fn a_statement_is_answered_and_the_next_prompt_comes_with_it() {
    let sent = session(&["2+2"]);
    assert_eq!(text(&sent), ["4"]);
    assert_eq!(prompts(&sent), ["      ", "      "]);
}

#[test]
fn quad_reads_the_line_typed_after_the_statement_asked_for_it() {
    let sent = session(&["X←⎕", "2+2", "X"]);
    assert_eq!(
        text(&sent),
        ["⎕:", "4"],
        "the quad prompt is printed, and X is what the next line made"
    );
}

#[test]
fn the_del_editor_prompts_with_the_line_number() {
    let sent = session(&["∇F", "'HI'", "∇", "F"]);
    assert_eq!(
        prompts(&sent),
        ["      ", "[1]   ", "[2]   ", "      ", "      "]
    );
    assert_eq!(text(&sent), ["HI"]);
}

#[test]
fn a_line_left_open_is_itself_the_prompt() {
    let sent = session(&["⍞←'NAME: '"]);
    assert_eq!(
        prompts(&sent),
        ["      ", "NAME: "],
        "the carriage stopped on that line, so the typing goes there"
    );
}

#[test]
fn sign_off_sends_a_last_frame_and_stops() {
    let sent = session(&[")OFF", "2+2"]);
    let last = sent.last().unwrap();
    assert!(last.off);
    assert_eq!(last.prompt, None, "nothing more will be typed");
    assert!(
        !text(&sent).contains(&"4".to_string()),
        "nothing after )OFF runs"
    );
}

#[test]
fn a_terminal_that_goes_away_releases_the_session() {
    let sent = session(&[]);
    assert_eq!(prompts(&sent), ["      "]);
    assert!(
        !sent.iter().any(|f| f.off),
        "a dropped line is not a sign-off"
    );
}
