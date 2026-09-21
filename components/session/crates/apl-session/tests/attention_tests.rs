//! ATTN against real APL: a loop of lines, one long statement, and
//! two sessions of which only one is asked.
//!
//! Each test runs a session on its own thread and asks it to stop
//! from another, which is how every host does it -- a signal handler
//! at the CLI, the listening thread at the service, the page in a
//! browser. A run that cannot be stopped hangs the test, so each has
//! a deadline and fails rather than hangs.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use apl_attn::{Flag, attend};
use apl_session::Session;

/// Long enough that a statement cannot finish first, short enough
/// that a test that fails does so quickly.
const PATIENCE: Duration = Duration::from_secs(10);

/// A session with room for a large array, on a thread of its own,
/// with its own flag installed. It runs `setup` then `line`, and
/// hands back what `line` replied.
fn run(setup: &'static [&'static str], line: &'static str, flag: Flag) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        attend(Box::new(flag));
        let mut s = Session::default();
        s.ws.quota = 2_000_000_000;
        for step in setup {
            let _ = s.respond(step);
        }
        let _ = tx.send(s.respond(line).lines.join("\n"));
    });
    rx
}

/// Ask `flag` after a moment, so the run is well under way.
fn ask_soon(flag: &Flag) {
    let far = flag.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(150));
        far.ask();
    });
}

/// A loop of lines, the APL\360 shape: `→1` back to the top forever.
/// This is the case the interrupt always covered, between lines.
const LOOP: &[&str] = &["∇SPIN", "X←1", "→1", "∇"];

#[test]
fn attention_stops_a_loop_of_lines() {
    let flag = Flag::default();
    let reply = run(LOOP, "SPIN", flag.clone());
    ask_soon(&flag);
    let said = reply
        .recv_timeout(PATIENCE)
        .expect("the loop was not stopped");
    assert!(said.contains("INTERRUPT"), "{said}");
}

#[test]
fn attention_stops_one_long_statement() {
    // One statement, no lines to stop between: this stops only because
    // the primitives poll. Before they did, it ran to the end.
    let flag = Flag::default();
    let reply = run(&[], "+/(⍳300000000)×⍳300000000", flag.clone());
    ask_soon(&flag);
    let said = reply
        .recv_timeout(PATIENCE)
        .expect("the statement was not stopped");
    assert!(said.contains("INTERRUPT"), "{said}");
}

#[test]
fn attention_stops_one_session_and_not_another() {
    // Two sessions, as a service holds them, both looping. Only one is
    // asked. The other must still be running when the first has
    // stopped -- which is why the flag stopped being one global.
    let (asked, spared) = (Flag::default(), Flag::default());
    let first = run(LOOP, "SPIN", asked.clone());
    let second = run(LOOP, "SPIN", spared.clone());
    ask_soon(&asked);
    let said = first
        .recv_timeout(PATIENCE)
        .expect("the one asked was not stopped");
    assert!(said.contains("INTERRUPT"), "{said}");
    assert!(
        second.recv_timeout(Duration::from_millis(500)).is_err(),
        "the one not asked stopped too"
    );
    spared.ask();
    let said = second
        .recv_timeout(PATIENCE)
        .expect("the second was not stopped when asked");
    assert!(said.contains("INTERRUPT"), "{said}");
}

#[test]
fn a_session_goes_on_working_after_an_attention() {
    // Stopped is not broken: the next line runs as it would have.
    let flag = Flag::default();
    let (tx, rx) = mpsc::channel();
    let far = flag.clone();
    thread::spawn(move || {
        attend(Box::new(far));
        let mut s = Session::default();
        for step in LOOP {
            let _ = s.respond(step);
        }
        let stopped = s.respond("SPIN").lines.join("\n");
        let after = s.respond("2+2").lines.join("\n");
        let _ = tx.send((stopped, after));
    });
    ask_soon(&flag);
    let (stopped, after) = rx.recv_timeout(PATIENCE).expect("never stopped");
    assert!(stopped.contains("INTERRUPT"), "{stopped}");
    assert_eq!(after.trim(), "4");
}
