//! The system variables, which (B) '75 has and (A) '70 does not. The
//! IBM 5110 APL Reference Manual, Chapter 5, and docs/mode-b.md.
//!
//! Those that stand for a setting read and set the same state the '70
//! commands and directives do: there is one index origin, one
//! precision, one width and one random link, whichever mode set them.

use apl_session::{Files, Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn error(s: &mut Session, line: &str) -> String {
    let reply = s.respond(line);
    assert!(reply.error, "{line}: {:?}", reply.lines);
    reply.lines[0].clone()
}

#[test]
fn a_clear_75_workspace_has_the_5110s_settings() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "⎕IO"), vec!["1"]);
    assert_eq!(out(&mut b, "⎕CT"), vec!["1E¯13"]);
    assert_eq!(out(&mut b, "⎕PP"), vec!["5"]);
    assert_eq!(out(&mut b, "⎕PW"), vec!["64"]);
    assert_eq!(out(&mut b, "⎕RL"), vec!["16807"]);
    assert_eq!(out(&mut b, "÷3"), vec!["0.33333"], "five digits");
    assert_eq!(
        out(&mut b, "1048576"),
        vec!["1048576"],
        "a whole number in full"
    );
    assert_eq!(
        out(&mut b, "12345678901"),
        vec!["1.2346E10"],
        "unless past ten"
    );
    let mut a = in_mode(Mode::A);
    assert_eq!(out(&mut a, "÷3"), vec!["0.3333333333"], "(A) keeps ten");
}

#[test]
fn a_setting_is_the_same_state_the_70_commands_set() {
    let mut b = in_mode(Mode::B);
    assert!(
        out(&mut b, "⎕IO←0").is_empty(),
        "an assignment shows nothing"
    );
    assert_eq!(b.ws.saved.env.io, 0);
    assert_eq!(out(&mut b, "⍳3"), vec!["0 1 2"]);
    out(&mut b, "⎕PP←3");
    assert_eq!(b.ws.saved.print.digits, 3);
    assert_eq!(out(&mut b, "÷3"), vec!["0.333"]);
    out(&mut b, "⎕PW←30");
    assert_eq!(b.ws.saved.print.width, 30);
    out(&mut b, "⎕RL←5");
    assert_eq!(b.ws.saved.env.link, 5);
    // And the other way: what a directive sets, the variable reads.
    out(&mut b, "⍝!ORIGIN 1");
    assert_eq!(out(&mut b, "⎕IO"), vec!["1"]);
}

#[test]
fn the_random_link_moves_as_roll_uses_it() {
    let mut b = in_mode(Mode::B);
    let before = out(&mut b, "⎕RL");
    out(&mut b, "?6");
    assert_ne!(out(&mut b, "⎕RL"), before);
    out(&mut b, "⎕RL←16807");
    let first = out(&mut b, "?100 100 100");
    out(&mut b, "⎕RL←16807");
    assert_eq!(out(&mut b, "?100 100 100"), first, "a link set replays");
}

#[test]
fn an_assignment_gives_its_value() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "1+⎕IO←0"), vec!["1"]);
    assert_eq!(out(&mut b, "⎕IO"), vec!["0"]);
    assert_eq!(out(&mut b, "X←⎕PP"), Vec::<String>::new());
    assert_eq!(out(&mut b, "X"), vec!["5"]);
}

#[test]
fn a_value_a_setting_cannot_take_is_a_domain_error() {
    let mut b = in_mode(Mode::B);
    for line in [
        "⎕IO←2",
        "⎕IO←0.5",
        "⎕IO←'A'",
        "⎕IO←0 1",
        "⎕PP←0",
        "⎕PP←17",
        "⎕PW←29",
        "⎕PW←255",
        "⎕RL←0",
        "⎕RL←¯5",
    ] {
        assert_eq!(error(&mut b, line), "DOMAIN ERROR", "{line}");
    }
    assert_eq!(out(&mut b, "⎕IO,⎕PP,⎕PW"), vec!["1 5 64"], "nothing moved");
    assert_eq!(out(&mut b, "⎕IO←,0"), Vec::<String>::new(), "one element");
    assert_eq!(
        out(&mut b, "⎕IO←1.0"),
        Vec::<String>::new(),
        "a whole number"
    );
}

#[test]
fn the_comparison_tolerance_can_be_set() {
    let mut b = in_mode(Mode::B);
    let near = "1+1E¯11";
    // At the clear workspace's 1E¯13 the two differ.
    assert_eq!(out(&mut b, &format!("1={near}")), vec!["0"]);
    assert!(out(&mut b, "⎕CT←1E¯10").is_empty());
    assert_eq!(out(&mut b, "⎕CT"), vec!["1E¯10"]);
    for (line, want) in [
        (format!("1={near}"), "1"),
        (format!("1≠{near}"), "0"),
        (format!("1<{near}"), "0"),
        ("⌊3-1E¯11".to_string(), "3"),
        ("⌈3+1E¯11".to_string(), "3"),
        ("0=1|3-1E¯11".to_string(), "1"),
        (format!("({near})∊1 2 3"), "1"),
        (format!("1 2 3⍳{near}"), "1"),
        (format!("=/1,{near}"), "1"),
        (format!("1∘.={near}"), "1"),
        (format!("1 +.= {near}"), "1"),
    ] {
        assert_eq!(out(&mut b, &line), vec![want], "{line}");
    }
    // Nothing tolerated: numbers the default would call equal.
    out(&mut b, "⎕CT←0");
    assert_eq!(out(&mut b, "1=1+1E¯15"), vec!["0"]);
}

#[test]
fn the_comparison_tolerance_is_between_zero_and_one() {
    let mut b = in_mode(Mode::B);
    for line in ["⎕CT←¯1E¯13", "⎕CT←1", "⎕CT←'A'", "⎕CT←1 2"] {
        assert!(b.respond(line).error, "{line}");
    }
    assert_eq!(out(&mut b, "⎕CT"), vec!["1E¯13"], "unchanged");
}

#[test]
fn a_comparison_tolerance_is_saved_and_makes_the_workspace_75s() {
    let dir = Dir::new("tolerance");
    let mut b = in_mode(Mode::B);
    b.ws.store = Box::new(Files(dir.0.clone()));
    out(&mut b, "⎕CT←1E¯10");
    out(&mut b, ")SAVE TOL");
    out(&mut b, ")CLEAR");
    assert_eq!(out(&mut b, "⎕CT"), vec!["1E¯13"], "a clear resets it");
    out(&mut b, ")LOAD TOL");
    assert_eq!(out(&mut b, "⎕CT"), vec!["1E¯10"]);
    let mut a = in_mode(Mode::A);
    a.ws.store = Box::new(Files(dir.0.clone()));
    assert_eq!(out(&mut a, ")LOAD TOL"), vec!["WS NOT FOUND"]);
}

#[test]
fn the_line_counter_reads_the_state_indicator() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "⍴⎕LC"), vec!["0"], "empty outside a function");
    for line in ["∇R←F", "R←⎕LC", "∇", "∇R←G", "R←0", "R←F", "∇"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "F"), vec!["1"]);
    assert_eq!(out(&mut b, "G"), vec!["1 2"], "innermost first");
}

#[test]
fn the_read_only_ones_ignore_an_assignment() {
    let mut b = in_mode(Mode::B);
    let space = out(&mut b, "⎕WA");
    for line in ["⎕LC←5", "⎕WA←5", "⎕AV←'A'", "⎕TT←5", "⎕UL←5"] {
        assert!(out(&mut b, line).is_empty(), "{line}");
    }
    assert_eq!(out(&mut b, "⍴⎕LC"), vec!["0"]);
    assert_eq!(out(&mut b, "⎕WA"), space);
    assert_eq!(out(&mut b, "⍴⎕AV"), vec!["256"]);
    assert_eq!(out(&mut b, "⎕TT,⎕UL"), vec!["0 1"]);
}

#[test]
fn the_space_available_falls_as_the_workspace_fills() {
    let mut b = in_mode(Mode::B);
    let before: i64 = out(&mut b, "⎕WA")[0].parse().expect("a number");
    out(&mut b, "X←⍳1000");
    let after: i64 = out(&mut b, "⎕WA")[0].parse().expect("a number");
    assert!(after < before, "{after} < {before}");
}

#[test]
fn the_atomic_vector_is_the_5110s() {
    let mut b = in_mode(Mode::B);
    // Appendix B: the letters from 87, the digits from 141, the
    // brackets at 15 and 16, blank at 153, quad at 66.
    assert_eq!(
        out(&mut b, "⎕AV⍳'AZ09[] ⎕'"),
        vec!["87 112 141 150 15 16 153 66"]
    );
    assert_eq!(out(&mut b, "⎕AV[87+⍳5]"), vec!["BCDEF"]);
    assert_eq!(out(&mut b, "⎕AV⍳'⍎⍕'"), vec!["79 78"]);
    assert_eq!(out(&mut b, "+/⎕AV∊⎕AV"), vec!["256"]);
    // Each element is a character of its own.
    assert_eq!(out(&mut b, "+/(⎕AV⍳⎕AV)=⍳256"), vec!["256"]);
}

#[test]
fn the_compatibility_variables_hold_fixed_values_and_take_new_ones() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, "⎕TS"), vec!["1900 0 0 0 0 0 0"]);
    assert_eq!(out(&mut b, "⎕AI"), vec!["0 0 0 0"]);
    out(&mut b, "⎕TS←1977 12 1 9 30 0 0");
    assert_eq!(out(&mut b, "⎕TS"), vec!["1977 12 1 9 30 0 0"]);
    assert_eq!(error(&mut b, "⎕AI←'X'"), "DOMAIN ERROR");
}

#[test]
fn a_name_the_system_does_not_have_is_a_syntax_error() {
    let mut b = in_mode(Mode::B);
    let reply = b.respond("1+⎕XY");
    assert!(reply.error);
    assert_eq!(
        reply.lines,
        vec!["SYNTAX ERROR", "      1+⎕XY", "        ^"]
    );
    assert_eq!(error(&mut b, "⎕XY←1"), "SYNTAX ERROR");
}

#[test]
fn a_quad_with_a_blank_before_a_name_is_still_quad() {
    let mut b = in_mode(Mode::B);
    out(&mut b, "IO←7");
    assert_eq!(out(&mut b, "⎕←IO"), vec!["7"], "output, then a name");
}

#[test]
fn a_system_variable_is_no_name_in_70() {
    let mut a = in_mode(Mode::A);
    for (line, caret) in [("⎕IO", 6), ("⎕IO←0", 6), ("X←⎕PP", 8)] {
        let reply = a.respond(line);
        assert!(reply.error, "{line}");
        let at = " ".repeat(caret) + "^";
        assert_eq!(
            reply.lines,
            vec!["SYNTAX ERROR".to_string(), format!("      {line}"), at]
        );
    }
    assert_eq!(a.ws.saved.env.io, 1);
}

#[test]
fn the_latent_expression_runs_when_its_workspace_is_loaded() {
    let dir = Dir::new("latent");
    let mut b = in_mode(Mode::B);
    b.ws.store = Box::new(Files(dir.0.clone()));
    assert_eq!(out(&mut b, "⍴⎕LX"), vec!["0"], "empty in a clear workspace");
    for line in ["∇HELLO", "'WELCOME'", "∇", "⎕LX←'HELLO'"] {
        out(&mut b, line);
    }
    assert_eq!(out(&mut b, "⎕LX"), vec!["HELLO"]);
    out(&mut b, ")SAVE GREET");
    out(&mut b, ")CLEAR");
    assert_eq!(out(&mut b, "⍴⎕LX"), vec!["0"], "a clear takes it away");
    let loaded = out(&mut b, ")LOAD GREET");
    assert_eq!(loaded.len(), 2, "{loaded:?}");
    assert!(loaded[0].starts_with("SAVED "), "{loaded:?}");
    assert_eq!(loaded[1], "WELCOME");
    assert_eq!(out(&mut b, "⎕LX"), vec!["HELLO"], "it is kept");
}

#[test]
fn a_workspace_with_a_latent_expression_runs_only_in_75() {
    let dir = Dir::new("latent-modes");
    let mut b = in_mode(Mode::B);
    b.ws.store = Box::new(Files(dir.0.clone()));
    out(&mut b, "⎕LX←'''HI'''");
    out(&mut b, ")SAVE HI");
    let mut a = in_mode(Mode::A);
    a.ws.store = Box::new(Files(dir.0.clone()));
    assert_eq!(out(&mut a, ")LOAD HI"), vec!["WS NOT FOUND"]);
}

#[test]
fn a_latent_expression_is_characters() {
    let mut b = in_mode(Mode::B);
    assert_eq!(error(&mut b, "⎕LX←5"), "DOMAIN ERROR");
    assert_eq!(error(&mut b, "⎕LX←2 2⍴'ABCD'"), "RANK ERROR");
    out(&mut b, "⎕LX←'X'");
    out(&mut b, "⎕LX←''");
    assert_eq!(out(&mut b, "⍴⎕LX"), vec!["0"]);
}

#[test]
fn copying_a_workspace_leaves_its_latent_expression() {
    let dir = Dir::new("latent-copy");
    let mut b = in_mode(Mode::B);
    b.ws.store = Box::new(Files(dir.0.clone()));
    out(&mut b, "⎕LX←'''HI'''");
    out(&mut b, "X←1");
    out(&mut b, ")SAVE HI");
    out(&mut b, ")CLEAR");
    out(&mut b, ")COPY HI");
    assert_eq!(out(&mut b, "X"), vec!["1"]);
    assert_eq!(out(&mut b, "⍴⎕LX"), vec!["0"]);
}

/// A library directory of its own, removed when the test ends.
struct Dir(std::path::PathBuf);

impl Dir {
    fn new(tag: &str) -> Dir {
        let dir = std::env::temp_dir().join(format!("sw-apl-sysvar-{tag}-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("mkdir");
        Dir(dir)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}
