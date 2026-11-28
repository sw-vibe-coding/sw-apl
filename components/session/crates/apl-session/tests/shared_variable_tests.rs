//! (D) '83's shared variables, as IBM PC APL's manual (May 1983,
//! chapter 9) gives them: `⎕SVO`, `⎕SVR`, `⎕SVC` and `⎕SVQ`, and a
//! shared variable set and answered by its partner. The partner here
//! is a test processor; AP205 is the one (D) loads.

use apl_d83_shares::{Processor, Shares};
use apl_session::{Mode, Session, Store};
use apl_value::{Array, Number};

/// Takes names beginning with E, and answers a set with the count of
/// what was set.
#[derive(Debug)]
struct Counter;

impl Processor for Counter {
    fn accepts(&mut self, p: i64, name: &str) -> bool {
        p == 9 && name.starts_with('E')
    }
    fn set(&mut self, name: &str, value: &Array, _: &mut dyn Store) -> Vec<(String, Array)> {
        let n = i64::try_from(value.shape.iter().product::<usize>()).unwrap_or(0);
        vec![(name.to_string(), Array::scalar(Number::Int(n)))]
    }
    fn retract(&mut self, _name: &str) {}
}

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s
}

fn with_counter() -> Session {
    let mut s = in_mode(Mode::D);
    s.ws.shares = Shares::with(vec![Box::new(Counter)]);
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn an_offer_is_acknowledged_and_shared_once_the_partner_takes_it() {
    let mut d = with_counter();
    assert_eq!(out(&mut d, "9 ⎕SVO 'E'"), ["1"]);
    assert_eq!(out(&mut d, "⎕SVO 'E'"), ["2"]);
    assert_eq!(out(&mut d, "9 ⎕SVO 2 1⍴'EF'"), ["2 1"], "E asked; F new");
    assert_eq!(out(&mut d, "⎕SVO 2 1⍴'EF'"), ["2 1"], "nobody takes F");
    assert_eq!(out(&mut d, "⎕SVO 'G'"), ["0"]);
    assert_eq!(
        out(&mut d, "9 ⎕SVO 'E1 SURR'"),
        ["1"],
        "a surrogate has no effect"
    );
}

#[test]
fn a_processor_not_loaded_never_matches() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "232 ⎕SVO 2 1⍴'CD'"), ["1 1"]);
    assert_eq!(out(&mut d, "⎕SVO 2 1⍴'CD'"), ["1 1"]);
    assert_eq!(out(&mut d, "⎕SVR 2 1⍴'CD'"), ["1 1"]);
    assert_eq!(out(&mut d, "⎕SVO 'C'"), ["0"]);
}

#[test]
fn a_shared_variable_holds_what_its_partner_answered() {
    let mut d = with_counter();
    out(&mut d, "9 ⎕SVO 'E'");
    out(&mut d, "E←'HELLO'");
    assert_eq!(out(&mut d, "E"), ["5"], "the partner's answer");
    out(&mut d, "F←'HELLO'");
    assert_eq!(out(&mut d, "F"), ["HELLO"], "an ordinary variable");
    assert_eq!(out(&mut d, "⎕SVR 'E'"), ["2"]);
    out(&mut d, "E←'HELLO'");
    assert_eq!(out(&mut d, "E"), ["HELLO"], "retracted, it is ordinary");
}

#[test]
fn a_function_or_label_name_is_not_offered() {
    let mut d = with_counter();
    out(&mut d, "∇EF");
    out(&mut d, "EL:1");
    out(&mut d, "∇");
    assert_eq!(out(&mut d, "9 ⎕SVO 'EF'"), ["0"]);
    assert_eq!(out(&mut d, "⎕SVO 'EF'"), ["0"]);
}

#[test]
fn access_control_is_set_once_offered() {
    let mut d = with_counter();
    assert_eq!(out(&mut d, "⎕SVC 'E'"), ["0 0 0 0"]);
    out(&mut d, "9 ⎕SVO 'E'");
    assert_eq!(out(&mut d, "1 ⎕SVC 'E'"), ["1 1 1 1"]);
    assert_eq!(out(&mut d, "0 1 1 0 ⎕SVC 'E'"), ["0 1 1 0"]);
    assert_eq!(out(&mut d, "⎕SVC 'E'"), ["0 1 1 0"]);
    assert_eq!(out(&mut d, "⍴⎕SVC 2 1⍴'EF'"), ["2 4"]);
    assert!(out(&mut d, "2 ⎕SVC 'E'")[0].starts_with("DOMAIN ERROR"));
    assert!(out(&mut d, "1 0 ⎕SVC 'E'")[0].starts_with("LENGTH ERROR"));
}

#[test]
fn nothing_is_offered_to_this_user() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "⍴⎕SVQ ⍳0"), ["0"]);
    assert_eq!(out(&mut d, "⍴⎕SVQ 210"), ["0 0"]);
}

#[test]
fn bad_arguments_are_errors() {
    let mut d = in_mode(Mode::D);
    for (line, error) in [
        ("0 ⎕SVO 'X'", "DOMAIN"),
        ("1.5 ⎕SVO 'X'", "DOMAIN"),
        ("205 ⎕SVO 3", "DOMAIN"),
        ("205 ⎕SVO '3X'", "DOMAIN"),
        ("205 ⎕SVO 'A B C'", "DOMAIN"),
        ("205 210 ⎕SVO 3 1⍴'ABC'", "LENGTH"),
        ("1 ⎕SVR 'X'", "SYNTAX"),
    ] {
        let shown = out(&mut d, line);
        assert!(shown[0].starts_with(error), "{line}: {shown:?}");
    }
}

#[test]
fn erasing_or_clearing_retracts() {
    let mut d = with_counter();
    out(&mut d, "9 ⎕SVO 3 2⍴'E1E2E3'");
    out(&mut d, ")ERASE E1");
    assert_eq!(out(&mut d, "⎕SVO 'E1'"), ["0"], ")ERASE");
    out(&mut d, "⎕EX 'E2'");
    assert_eq!(out(&mut d, "⎕SVO 'E2'"), ["0"], "⎕EX");
    out(&mut d, ")CLEAR");
    assert_eq!(out(&mut d, "⎕SVO 'E3'"), ["0"], ")CLEAR");
}

#[test]
fn a_local_is_retracted_when_its_function_ends_and_does_not_reach_a_global_share() {
    let mut d = with_counter();
    out(&mut d, "∇R←INNER;E");
    out(&mut d, "[1] E←1 2 3");
    out(&mut d, "[2] R←E");
    out(&mut d, "∇");
    out(&mut d, "∇OWN2;E");
    out(&mut d, "[1] 9 ⎕SVO 'E'");
    out(&mut d, "∇");
    out(&mut d, "9 ⎕SVO 'E'");
    assert_eq!(out(&mut d, "INNER"), ["1 2 3"], "a local is another E");
    out(&mut d, "E←'AB'");
    assert_eq!(out(&mut d, "E"), ["2"], "the global is still shared");
    out(&mut d, "⎕SVR 'E'");
    out(&mut d, "OWN2");
    assert_eq!(
        out(&mut d, "⎕SVO 'E'"),
        ["0"],
        "the local's share ended with it"
    );
}

#[test]
fn b_has_no_shared_variables() {
    let mut b = in_mode(Mode::B);
    assert!(out(&mut b, "⎕SVO 'X'")[0].starts_with("SYNTAX ERROR"));
}
