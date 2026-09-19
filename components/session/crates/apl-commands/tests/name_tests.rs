//! Only the first four characters of a command name are significant.
//!
//! The manual, in the summary before its table of commands: "Where
//! the first word of a command form is more than four characters
//! long, only the first four are significant. The others are
//! included only for mnemonic reasons, and may be dropped or
//! replaced, as desired. For example, )CLEAR, )CLEA, )CLEAVER,
//! etc., are all equivalent."

use apl_commands::{ABBREVIATED, canonical};

#[test]
fn a_long_name_may_be_cut_to_four() {
    assert_eq!(canonical("CLEA"), "CLEAR");
    assert_eq!(canonical("CONT"), "CONTINUE");
    assert_eq!(canonical("ORIG"), "ORIGIN");
    assert_eq!(canonical("DIGI"), "DIGITS");
    assert_eq!(canonical("SYMB"), "SYMBOLS");
}

#[test]
fn what_follows_the_fourth_character_is_ignored_entirely() {
    // Not typo tolerance: the manual says the rest "may be dropped
    // or replaced, as desired", so CLEAVER is CLEAR and not a near
    // miss that happens to be forgiven.
    assert_eq!(canonical("CLEAVER"), "CLEAR");
    assert_eq!(canonical("CLEARANCE"), "CLEAR");
    assert_eq!(canonical("ORIGAMI"), "ORIGIN");
    assert_eq!(canonical("CLEAR"), "CLEAR", "and the full name still is");
}

#[test]
fn a_name_of_four_or_fewer_must_be_exact() {
    // The rule is about truncating a longer name, not about
    // prefixes: )VAR is three characters and )VARS is the command.
    for exact in ["COPY", "DROP", "FNS", "GRP", "GRPS", "LIB", "SI"] {
        assert_eq!(canonical(exact), exact);
    }
    for wrong in ["VAR", "S", "OF", "COP", "LOA"] {
        assert_eq!(canonical(wrong), wrong, "{wrong} is not a command");
    }
    // A short command's name is not a prefix to be extended either.
    assert_eq!(canonical("SIX"), "SIX");
    assert_eq!(canonical("FNSX"), "FNSX");
}

#[test]
fn a_name_that_is_no_command_comes_back_as_it_was() {
    for nonsense in ["", "X", "NOSUCH", "LOADED"] {
        assert_eq!(canonical(nonsense), nonsense);
    }
}

#[test]
fn a_name_of_glyphs_does_not_cut_a_character_in_half() {
    // The name is whatever followed the parenthesis, so it can be
    // any text at all. Four characters, not four bytes.
    let glyphs = "\u{2373}\u{2374}\u{2375}\u{2376}\u{2377}";
    assert_eq!(canonical(glyphs), glyphs);
    assert_eq!(canonical("\u{2373}"), "\u{2373}");
}

/// The rule only works while no two commands agree in their first
/// four characters, and while no long command's first four spell a
/// short command outright. Both hold today; this is what notices if
/// a command added later breaks either.
#[test]
fn no_two_commands_collide_at_four_characters() {
    let mut seen: Vec<String> = Vec::new();
    for name in ABBREVIATED {
        let first: String = name.chars().take(4).collect();
        assert!(!seen.contains(&first), "{name} collides at {first}");
        seen.push(first.clone());
        assert_eq!(canonical(&first), name, "{first} must reach {name}");
    }
}
