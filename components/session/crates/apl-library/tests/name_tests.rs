//! What a workspace may be called.
//!
//! A workspace name becomes a filename, so it is not only APL's
//! business. Until this was checked, `)SAVE A/B` made a directory
//! and `)SAVE ../../X` wrote outside the library altogether.
//!
//! The rule is the one the lexer already uses for every other name:
//! a letter, delta or delta-underbar, then letters, deltas and
//! digits. It is what APL\360 allowed, and it happens to make every
//! path question go away at once.

use apl_library::valid;

#[test]
fn an_apl_name_is_a_workspace_name() {
    for name in [
        "A",
        "CLASS",
        "A1",
        "X9Y",
        "\u{2206}",
        "\u{2206}X",
        "\u{2359}Y",
    ] {
        assert!(valid(name), "{name}");
    }
    // Lower case is a name everywhere else in sw-apl, so it is one
    // here. APL\360 had no lower case at all; see parity.md.
    assert!(valid("lower"));
    assert!(valid("MiXeD"));
}

#[test]
fn a_name_must_begin_with_a_letter() {
    for bad in ["1", "1DIGIT", "9", "\u{af}A"] {
        assert!(!valid(bad), "{bad}");
    }
}

#[test]
fn nothing_that_could_reach_out_of_the_library_is_a_name() {
    // These are the ones that mattered. Each of them worked before
    // the rule existed, and the last wrote two directories up.
    for bad in ["A/B", ".HIDDEN", "..", ".", "///", "../../ESCAPED", "~"] {
        assert!(!valid(bad), "{bad}");
    }
}

#[test]
fn the_password_forms_are_not_names_either() {
    // APL\360's )SAVE took a lock and )LOAD a key, after a colon.
    // sw-apl has one user and nothing to lock against, so the form
    // is refused rather than taken as part of the name -- which is
    // what it was, giving a workspace called WS:PASS.
    assert!(!valid("WS:PASS"));
    assert!(!valid("WS:"));
    assert!(!valid(":PASS"));
}

#[test]
fn an_empty_name_is_not_a_name() {
    assert!(!valid(""));
}

#[test]
fn punctuation_and_glyphs_are_not_names() {
    for bad in ["A-B", "A.B", "A+B", "A_B", "\u{2373}", "A\u{2373}", "*"] {
        assert!(!valid(bad), "{bad}");
    }
}
