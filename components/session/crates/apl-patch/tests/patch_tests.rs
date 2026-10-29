//! Line editing, `[N⎕M]`, as the APL\360-OS and APL\360-DOS User's
//! Manual (SH20-0906-0) gives it on p. 3.18, with its example on p.
//! A.5: marks typed under a displayed statement, the statement shown
//! again with them applied, and the gaps filled.

use apl_patch::{Patch, Taken, fill, mark};
use apl_value::ErrorKind;

/// The manual's example: line 1 of ABC as `[1⎕9]` displays it.
const SHOWN: &str = "[1]   Z←(33×Q+(R×5)-6";

#[test]
fn the_manuals_example_marks() {
    // Carriage under position 9, the first 3: `/` deletes it, the 1s
    // open a blank before + and before ×, the second / deletes R.
    let (edited, gap) = mark(SHOWN, 9, "/   1 /1");
    assert_eq!(edited, "[1]   Z←(3×Q +( ×5)-6");
    assert_eq!(
        gap,
        Some(12),
        "the carriage stops at the first blank put in"
    );
}

#[test]
fn the_manuals_example_fills() {
    // A blank keeps what is on the line; a character fills a blank.
    let line = fill("[1]   Z←(3×Q +( ×5)-6", 12, ")  T", &[]).unwrap();
    assert_eq!(line, "[1]   Z←(3×Q)+(T×5)-6");
}

#[test]
fn a_digit_opens_that_many_blanks_and_a_letter_five_times_its_place() {
    assert_eq!(mark("AB", 1, "3").0, "A   B");
    assert_eq!(mark("AB", 1, "0").0, "AB", "0 opens none");
    assert_eq!(mark("AB", 1, "A").0, format!("A{}B", " ".repeat(5)));
    assert_eq!(mark("AB", 1, "C").0, format!("A{}B", " ".repeat(15)));
}

#[test]
fn any_other_mark_is_ignored_and_so_is_one_past_the_end() {
    assert_eq!(mark("ABC", 0, "*-⍴ "), ("ABC".to_string(), None));
    assert_eq!(mark("ABC", 2, " //"), ("ABC".to_string(), None));
}

#[test]
fn a_blank_marks_line_leaves_the_statement_as_it_was() {
    assert_eq!(mark(SHOWN, 9, ""), (SHOWN.to_string(), None));
}

#[test]
fn an_underscored_letter_is_one_position() {
    // A̲ is a letter and a combining low line: one column on paper.
    assert_eq!(mark("A\u{332}BC", 1, "/").0, "A\u{332}C");
}

#[test]
fn a_fill_runs_on_past_the_end_of_the_line() {
    assert_eq!(fill("X←1", 3, "+2", &[]).unwrap(), "X←1+2");
}

#[test]
fn a_character_over_a_character_strikes() {
    assert_eq!(fill("X←○1", 1, " *", &[]).unwrap(), "X←⍟1");
    assert_eq!(
        fill("X←1", 2, "1", &[]).unwrap(),
        "X←1",
        "the same again is the same"
    );
    let err = fill("X←1", 2, "2", &[]).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::Character(_)), "{err:?}");
}

#[test]
fn the_patch_asks_for_marks_then_for_the_fill() {
    let patch = Patch::open(SHOWN, 9);
    assert_eq!(
        patch.prompt(),
        " ".repeat(9),
        "the carriage under position 9"
    );
    let Ok(Taken::Again(patch, edited)) = patch.take("/   1 /1", &[]) else {
        panic!("marks are followed by the fill");
    };
    assert_eq!(edited, "[1]   Z←(3×Q +( ×5)-6");
    assert_eq!(
        patch.prompt(),
        "[1]   Z←(3×Q",
        "the line up to its first gap"
    );
    let Ok(Taken::Line(line)) = patch.take(")  T", &[]) else {
        panic!("the fill finishes the line");
    };
    assert_eq!(line, "[1]   Z←(3×Q)+(T×5)-6");
}

#[test]
fn with_no_gap_the_fill_starts_at_the_end() {
    let patch = Patch::open("[2]   X←1", 0);
    let Ok(Taken::Again(patch, _)) = patch.take("", &[]) else {
        panic!("marks are followed by the fill");
    };
    assert_eq!(patch.prompt(), "[2]   X←1");
}
