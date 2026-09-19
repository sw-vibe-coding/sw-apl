//! Forming a glyph by striking one character over another.

use apl_strike::{Strike, strike};

/// A struck glyph as `strike` gives it back: text, because an
/// underscored letter is a letter and a combining low line.
fn glyph(s: &str) -> String {
    s.to_string()
}

#[test]
fn the_pairs_the_manual_states() {
    // Each of these is quoted in data/glyphs.toml beside its entry.
    assert_eq!(
        strike('\u{2395}', '\u{f7}'),
        Some(glyph("\u{2339}")),
        "quad, divide"
    );
    assert_eq!(
        strike('\u{2395}', '\''),
        Some(glyph("\u{235e}")),
        "quad, quote"
    );
    assert_eq!(
        strike('\u{2207}', '~'),
        Some(glyph("\u{236b}")),
        "del, tilde"
    );
    assert_eq!(strike('/', '-'), Some(glyph("\u{233f}")), "slash, minus");
    assert_eq!(
        strike('\u{25cb}', '-'),
        Some(glyph("\u{2296}")),
        "circle, minus"
    );
    assert_eq!(
        strike('\u{2206}', '|'),
        Some(glyph("\u{234b}")),
        "delta, stile"
    );
    assert_eq!(
        strike('\u{2229}', '\u{25cb}'),
        Some(glyph("\u{235d}")),
        "cap, circle"
    );
    assert_eq!(
        strike('\u{22a5}', '\u{22a4}'),
        Some(glyph("\u{2336}")),
        "the I-beam"
    );
}

#[test]
fn either_order_forms_the_same_glyph() {
    // On paper there is no difference: backspace only positions the
    // carriage, so both impressions land on one spot.
    for (base, over) in [('\u{2395}', '\''), ('\u{25cb}', '*'), ('\u{2207}', '~')] {
        assert_eq!(strike(base, over), strike(over, base), "{base}{over}");
    }
}

#[test]
fn a_pair_that_forms_nothing_is_none() {
    // "Illegitimate overstrike" is the manual's cause for a
    // CHARACTER error, so the caller reports one; this only says
    // there is no glyph.
    assert_eq!(strike('A', 'B'), None);
    assert_eq!(strike('\u{2395}', '\u{2395}'), None, "over itself");
    assert_eq!(strike('+', '-'), None);
}

#[test]
fn a_component_need_not_be_a_glyph_in_its_own_right() {
    // The 2741's keyboard carried ∩ so that ⍝ could be struck, and
    // APL\360 has no intersection function: typing ∩ alone is a
    // CHARACTER ERROR, and striking it over ○ is a comment.
    assert_eq!(strike('\u{2229}', '\u{25cb}'), Some(glyph("\u{235d}")));
    // The underbar is likewise a component and not a glyph.
    assert_eq!(strike('\u{2206}', '_'), Some(glyph("\u{2359}")));
}

#[test]
fn no_two_overstrikes_use_the_same_two_characters() {
    // Accepting either order is only unambiguous while this holds,
    // so it is checked rather than assumed.
    let mut pairs: Vec<(char, char)> = Vec::new();
    for (_, base, over) in apl_value::OVERSTRIKE {
        let (a, b) = if base <= over {
            (base, over)
        } else {
            (over, base)
        };
        assert!(!pairs.contains(&(a, b)), "{a} and {b} form two glyphs");
        pairs.push((a, b));
    }
    assert_eq!(pairs.len(), apl_value::OVERSTRIKE.len());
}

#[test]
fn a_struck_glyph_is_never_also_a_component() {
    // If it were, a strike could be built from a strike, and the
    // machine would have to decide how deep to look.
    for (glyph, _, _) in apl_value::OVERSTRIKE {
        for (_, base, over) in apl_value::OVERSTRIKE {
            assert_ne!(glyph, base, "{glyph} is struck and also a base");
            assert_ne!(glyph, over, "{glyph} is struck and also an overstrike");
        }
    }
}

#[test]
fn the_machine_holds_one_character_until_the_key_is_pressed() {
    let mut s = Strike::default();
    assert_eq!(
        s.typed('2'),
        Some(glyph("2")),
        "an ordinary character passes"
    );
    assert_eq!(
        s.typed('\u{2395}'),
        Some(glyph("\u{2395}")),
        "so does a glyph"
    );
}

#[test]
fn the_key_holds_the_last_character_back_and_the_next_strikes_it() {
    let mut s = Strike::default();
    assert_eq!(s.typed('\u{2395}'), Some(glyph("\u{2395}")));
    assert_eq!(s.back(), Some('\u{2395}'), "the character to take back");
    assert_eq!(
        s.typed('\''),
        Some(glyph("\u{235e}")),
        "struck into quote-quad"
    );
    // And the machine is empty again.
    assert_eq!(s.typed('A'), Some(glyph("A")));
}

#[test]
fn the_key_with_nothing_before_it_does_nothing() {
    let mut s = Strike::default();
    assert_eq!(s.back(), None, "nothing to strike over");
    assert_eq!(
        s.typed('A'),
        Some(glyph("A")),
        "and the next character stands"
    );
}

#[test]
fn a_strike_that_forms_nothing_gives_both_characters_back() {
    // The line is the user's; a refused strike must not eat what
    // they typed. The caller reports the CHARACTER ERROR.
    let mut s = Strike::default();
    s.typed('A');
    assert_eq!(s.back(), Some('A'));
    assert_eq!(s.typed('B'), None, "no glyph");
    assert_eq!(s.refused(), Some(('A', 'B')), "and here is what they typed");
}

#[test]
fn a_line_is_composed_before_it_is_read() {
    // Composing a whole line is how a file, or a socket, carries an
    // overstrike: the keystroke machine is for typing, this is for
    // whatever arrives already typed.
    use apl_strike::{BACK, BACKSPACE, compose};
    let plain = "2+2";
    assert_eq!(
        compose(plain).unwrap(),
        plain,
        "untouched when there is none"
    );
    let typed = format!("A\u{2190}\u{25cb}{BACK}*3");
    assert_eq!(compose(&typed).unwrap(), "A\u{2190}\u{235f}3");
    // A 2741 sent 0x08, and so does a file written elsewhere.
    let sent = format!("\u{2395}{BACKSPACE}'");
    assert_eq!(compose(&sent).unwrap(), "\u{235e}");
}

#[test]
fn a_line_with_an_illegitimate_overstrike_says_which_two() {
    use apl_strike::{BACK, compose};
    let bad = format!("A{BACK}B");
    assert_eq!(compose(&bad), Err(('A', 'B')));
}

#[test]
fn a_strike_at_the_start_of_a_line_strikes_nothing() {
    use apl_strike::{BACK, compose};
    // The carriage is at the left margin; there is nothing to
    // strike over, so the character stands on its own.
    assert_eq!(compose(&format!("{BACK}A")).unwrap(), "A");
}

/// The key is named in one place so that no help text can promise a
/// key that is not bound. These pin the pair; the CLI help and
/// `docs/glyph-entry.md` quote the label, and the reader binds the
/// character.
#[test]
fn the_key_and_its_label_agree() {
    use apl_strike::{BACK, BACK_LABEL};
    assert_eq!(BACK, '\u{1d}', "Ctrl-] is 0x1D");
    assert_eq!(BACK_LABEL, "Ctrl-]");
    // Not Ctrl-H: that is 0x08, which a terminal cannot tell from
    // backspace, and rustyline binds it besides.
    assert_ne!(BACK, '\u{8}');
    assert_ne!(BACK, apl_strike::BACKSPACE);
}

#[test]
fn any_letter_struck_with_the_underbar_is_an_underscored_letter() {
    // A 2741 struck `_` over any letter, and APL\360 used all
    // twenty-six. It is a rule, not twenty-six table rows.
    for letter in apl_value::UNDERSCORED.chars() {
        let want = Some(format!("{letter}{}", apl_value::UNDERSCORE));
        assert_eq!(strike(letter, '_'), want, "{letter} struck with underbar");
        assert_eq!(strike('_', letter), want, "and in the other order");
    }
}

#[test]
fn only_a_letter_takes_the_underbar() {
    // The rule is "a letter and an underbar"; a digit or a glyph
    // struck with one is still an illegitimate overstrike.
    assert_eq!(strike('1', '_'), None, "a digit is not a letter");
    assert_eq!(strike('+', '_'), None, "nor is a primitive");
    assert_eq!(strike('a', '_'), None, "APL\\360 had no lowercase");
    assert_eq!(strike('_', '_'), None, "nor the underbar itself");
}

#[test]
fn the_underscored_letters_take_no_pair_the_table_already_uses() {
    // Either order is unambiguous only while no two pairs share
    // their characters, and the rule adds twenty-six pairs. The one
    // table entry using `_` strikes it over ∆, which is no letter.
    for (glyph, base, over) in apl_value::OVERSTRIKE {
        let letter = |c: char| apl_value::UNDERSCORED.contains(c);
        assert!(
            !((letter(base) && over == '_') || (letter(over) && base == '_')),
            "{glyph} collides with the underscored alphabet"
        );
    }
}

#[test]
fn an_underscored_letter_is_struck_on_a_line_like_any_other() {
    use apl_strike::{BACK, compose};
    let typed = format!("X{BACK}_\u{2190}2");
    assert_eq!(compose(&typed).unwrap(), "X\u{332}\u{2190}2");
}

#[test]
fn striking_over_an_underscored_letter_forms_nothing() {
    // Three impressions on one position is not a glyph anywhere in
    // APL\360; the carriage takes back the low line, and the manual's
    // answer to what follows is an illegitimate overstrike.
    use apl_strike::{BACK, compose};
    let typed = format!("A{BACK}_{BACK}B");
    assert_eq!(compose(&typed), Err(('\u{332}', 'B')));
}
