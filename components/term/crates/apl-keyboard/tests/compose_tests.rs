//! The keyboard: what each key sends, how a glyph is struck, and
//! what a cell is.

use apl_keyboard::{Keyboard, Move};

#[test]
fn arithmetic_keys_and_minus_overstrikes_use_the_typeball_layout() {
    let mut k = Keyboard::default();
    for c in "2-3=4_5+6".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), "2+3×4-5÷6");
    for (base, expected) in [('O', "⊖"), ('/', "⌿"), ('\\', "⍀")] {
        k.clear();
        k.type_key(base);
        k.overstrike();
        assert!(k.type_key('_'));
        assert_eq!(k.text(), expected);
    }
}

#[test]
fn home_row_punctuation_supports_commands_grouping_and_indexing() {
    let mut k = Keyboard::default();
    for c in "\"wsid".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), ")WSID");
    k.clear();
    for c in ":a;1'\"".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), "(A[1])");
    k.clear();
    for c in "90()K".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), "90∨∧'");
}

#[test]
fn physical_keys_compose_quote_quad_before_submission() {
    let mut k = Keyboard::default();
    k.type_key('l');
    k.type_key('k');
    assert_eq!(k.text(), "LK");
    k.clear();
    k.type_key('L');
    assert_eq!(k.text(), "⎕");
    k.overstrike();
    k.type_key('K');
    assert_eq!(k.text(), "⍞");
}

#[test]
fn underscore_is_one_editable_cell_but_two_wire_codepoints() {
    let mut k = Keyboard::default();
    k.type_key('a');
    k.overstrike();
    k.type_key('F');
    assert_eq!(k.text(), "A\u{332}");
    k.backspace();
    assert_eq!(k.text(), "");
}

#[test]
fn invalid_strike_preserves_base_and_repeated_back_is_harmless() {
    let mut k = Keyboard::default();
    k.overstrike();
    k.type_key('L');
    k.overstrike();
    k.overstrike();
    assert!(!k.type_key('a'));
    assert_eq!(k.text(), "⎕");
    k.type_key('K');
    assert_eq!(k.text(), "⍞");
}

#[test]
fn paste_is_unicode_and_cursor_edits_cells() {
    let mut k = Keyboard::default();
    k.paste("A\u{332}←2");
    k.move_cursor(Move::Left);
    k.backspace();
    k.type_key('{');
    assert_eq!(k.text(), "A\u{332}←2");
}

/// Every key of the IBM 2741 APL keyboard, transcribed by eye from
/// `images/redistributed/apl-keyboard/APL-keybd2.svg`, which is the
/// only place the layout exists -- the article about the 2741 carries
/// no key table, and the picture is all outline paths, so no program
/// can read it for us.
///
/// The transcription is therefore the thing to check against the
/// picture when either changes, and this test is what stops the map
/// drifting away from it quietly.
const TYPEBALL: [(char, &str); 49] = [
    // Number row, shifted. Unshifted these are the digits.
    ('!', "¨"),
    ('@', "¯"),
    ('#', "<"),
    ('$', "≤"),
    ('%', "="),
    ('^', "≥"),
    ('&', ">"),
    ('*', "≠"),
    ('(', "∨"),
    (')', "∧"),
    // The two keys right of zero carry arithmetic, not punctuation.
    ('-', "+"),
    ('_', "-"),
    ('=', "×"),
    ('+', "÷"),
    // Top letter row, shifted, then the one key right of P.
    ('Q', "?"),
    ('W', "⍵"),
    ('E', "∊"),
    ('R', "⍴"),
    ('T', "~"),
    ('Y', "↑"),
    ('U', "↓"),
    ('I', "⍳"),
    ('O', "○"),
    ('P', "*"),
    ('[', "→"),
    ('{', "←"),
    // Home row, shifted, then the two keys right of L.
    ('A', "⍺"),
    ('S', "⌈"),
    ('D', "⌊"),
    ('F', "_"),
    ('G', "∇"),
    ('H', "∆"),
    ('J', "∘"),
    ('K', "'"),
    ('L', "⎕"),
    (';', "["),
    (':', "("),
    ('\'', "]"),
    ('"', ")"),
    // Bottom row, shifted, then comma, full stop and solidus.
    ('Z', "⊂"),
    ('X', "⊃"),
    ('C', "∩"),
    ('V', "∪"),
    ('B', "⊥"),
    ('N', "⊤"),
    ('M', "|"),
    ('<', ";"),
    ('>', ":"),
    ('?', "\\"),
];

#[test]
fn every_key_sends_what_the_2741_typeball_carried() {
    for (key, expected) in TYPEBALL {
        let mut keyboard = Keyboard::default();
        keyboard.type_key(key);
        assert_eq!(keyboard.text(), expected, "the {key:?} key");
    }
}

#[test]
fn an_unshifted_letter_is_the_capital_the_typeball_printed() {
    // A 2741's APL element had one alphabet, in italic capitals, and
    // it was typed with the machine in lower case.
    let mut keyboard = Keyboard::default();
    for c in "abcxyz".chars() {
        keyboard.type_key(c);
    }
    assert_eq!(keyboard.text(), "ABCXYZ");
}

#[test]
fn the_brackets_are_on_the_home_row_and_the_arrows_are_not() {
    // The 2741 has one key right of P, carrying both arrows, and two
    // right of L carrying the brackets and parentheses. A US keyboard
    // has two and two, so the bracket keys of one are not the bracket
    // keys of the other.
    let mut keyboard = Keyboard::default();
    for c in ";1'{".chars() {
        keyboard.type_key(c);
    }
    assert_eq!(keyboard.text(), "[1]←");
}

/// A character that arrives by paste while a strike is pending strikes,
/// exactly as a typed key would. The on-screen board sends every glyph
/// this way, and so does an expander such as Espanso; before this, a
/// strike begun with the overstrike key was silently dropped and the
/// glyph put beside its base instead of over it.
#[test]
fn a_character_pasted_while_a_strike_is_pending_strikes() {
    let (mut typed, mut pasted) = (Keyboard::default(), Keyboard::default());
    typed.type_key('a');
    typed.overstrike();
    typed.type_key('F');
    pasted.paste("A");
    pasted.overstrike();
    assert!(pasted.paste("_"), "a pair that forms a glyph does not ring");
    assert_eq!(pasted.text(), typed.text(), "paste and typing disagree");
    assert_eq!(pasted.text(), "A\u{332}");
    assert!(!pasted.pending);
}

#[test]
fn a_pasted_character_that_forms_no_glyph_rings_and_leaves_the_base() {
    let mut k = Keyboard::default();
    k.paste("A");
    k.overstrike();
    assert!(!k.paste("⍴"), "A and rho form nothing, so the bell rings");
    assert_eq!(k.text(), "A");
}

#[test]
fn more_than_one_character_pasted_while_a_strike_is_pending_goes_in_as_typed() {
    // A paste of a whole expression is not the second key of an
    // overstrike, so the strike is abandoned and the text goes in.
    let mut k = Keyboard::default();
    k.paste("A");
    k.overstrike();
    assert!(k.paste("+/⍳5"));
    assert_eq!(k.text(), "A+/⍳5");
    assert!(!k.pending);
}
