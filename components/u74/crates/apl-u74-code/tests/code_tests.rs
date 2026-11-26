//! The holes an 029 punches for each character (docs/mode-u.md).

use apl_u74_card::{PUNCHED, punchable};
use apl_u74_code::{Holes, ROWS, holes};

fn rows(c: char) -> Vec<&'static str> {
    holes(c)
        .expect("punchable")
        .rows()
        .map(|r| ROWS[r])
        .collect()
}

#[test]
fn the_rows_run_from_twelve_down_to_nine() {
    assert_eq!(
        ROWS,
        ["12", "11", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
    );
}

#[test]
fn a_digit_is_one_hole_in_its_row() {
    assert_eq!(rows('0'), ["0"]);
    assert_eq!(rows('7'), ["7"]);
}

#[test]
fn a_letter_is_a_zone_and_a_digit() {
    assert_eq!(rows('A'), ["12", "1"]);
    assert_eq!(rows('R'), ["11", "9"]);
    assert_eq!(rows('S'), ["0", "2"]);
    assert_eq!(rows('Z'), ["0", "9"]);
}

#[test]
fn the_specials_are_the_029s() {
    assert_eq!(rows('$'), ["11", "3", "8"]);
    assert_eq!(rows('¬'), ["11", "7", "8"]);
    assert_eq!(rows('@'), ["4", "8"]);
    assert_eq!(rows('&'), ["12"]);
    assert_eq!(rows('/'), ["0", "1"]);
    assert_eq!(rows('"'), ["7", "8"]);
}

#[test]
fn blank_punches_nothing() {
    assert_eq!(holes(' '), Some(Holes(0)));
}

#[test]
fn every_punchable_character_has_its_own_code() {
    let all: Vec<char> = ('A'..='Z')
        .chain('0'..='9')
        .chain(PUNCHED.chars())
        .collect();
    let mut codes: Vec<u16> = all.iter().map(|&c| holes(c).expect("punched").0).collect();
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes.len(), all.len());
}

#[test]
fn what_an_029_cannot_punch_has_no_code() {
    for c in ['[', ']', '\\', '^', 'a', '⍴', '~'] {
        assert!(!punchable(c));
        assert_eq!(holes(c), None, "{c}");
    }
}
