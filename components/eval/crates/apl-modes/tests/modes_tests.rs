//! Modes: the line a workspace names them on, and the libraries each
//! mode sees through it.

use apl_modes::{Mode, Modes, modes, render, retag};

/// A workspace file with the given modes line, or none.
fn ws(line: Option<&str>, body: &str) -> String {
    let mut text = "⍝ sw-apl workspace. Re-executable APL: loading it runs it.\n".to_string();
    if let Some(line) = line {
        text = text + "⍝!MODES " + line + "\n";
    }
    text + body + "\n"
}

#[test]
fn a_mode_is_named_by_its_year_or_its_letter() {
    assert_eq!(Mode::parse("70"), Some(Mode::A));
    assert_eq!(Mode::parse("75"), Some(Mode::B));
    assert_eq!(Mode::parse("A"), Some(Mode::A));
    assert_eq!(Mode::parse("b"), Some(Mode::B));
    assert_eq!(Mode::parse("72"), None, "'72 was (B)'s old name");
    assert_eq!(Mode::parse("68"), None, "and '68 (A)'s");
    assert_eq!(Mode::default(), Mode::A);
}

#[test]
fn the_line_is_rendered_and_read_back() {
    assert_eq!(render(Modes::ALL), "(A)(B)");
    assert_eq!(render(Modes::only(Mode::B)), "(B)");
    assert_eq!(modes(&ws(Some("(A)(B)"), "X←1")), Modes::ALL);
    assert_eq!(modes(&ws(Some("(B)"), "X←1")), Modes::only(Mode::B));
    assert_eq!(modes(&ws(Some("(A)"), "X←1")), Modes::only(Mode::A));
}

#[test]
fn a_workspace_with_no_line_is_a_68_one() {
    assert_eq!(modes(&ws(None, "X←1")), Modes::only(Mode::A));
}

#[test]
fn a_line_further_down_is_not_the_modes_line() {
    // A variable or a function comment that happens to read like the
    // directive is content, not the workspace's modes.
    let text = ws(None, &"X←1\n".repeat(10)) + "⍝!MODES (B)\n";
    assert_eq!(modes(&text), Modes::only(Mode::A));
}

#[test]
fn retagging_replaces_the_line_or_adds_it_second() {
    let tagged = retag(&ws(Some("(A)(B)"), "X←1"), Modes::only(Mode::A));
    assert_eq!(tagged, ws(Some("(A)"), "X←1"));
    let added = retag(&ws(None, "X←1"), Modes::ALL);
    assert_eq!(added, ws(Some("(A)(B)"), "X←1"));
}

#[test]
fn sets_meet_and_subtract() {
    let (a, b) = (Modes::only(Mode::A), Modes::only(Mode::B));
    assert!(Modes::ALL.has(Mode::A) && Modes::ALL.has(Mode::B));
    assert!(!a.has(Mode::B));
    assert!(a.meets(Modes::ALL) && !a.meets(b));
    assert_eq!(Modes::ALL.minus(a), b);
    assert_eq!(a.minus(a), Modes::NONE);
}

#[test]
fn each_mode_says_what_it_adds() {
    assert_eq!(Mode::A.glyphs(), "");
    assert_eq!(Mode::B.glyphs(), "⍎⍕⎕", "quad begins a system name");
    assert!(Mode::A.overstrikes().is_empty());
    let b: Vec<char> = Mode::B.overstrikes().iter().map(|(g, _, _)| *g).collect();
    assert_eq!(b, ['⍎', '⍕']);
}
