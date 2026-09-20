//! What a browser keystroke means.

use apl_board::{Act, act};
use apl_keyboard::Move;

#[test]
fn a_printable_key_is_typed_through_the_keymap() {
    assert_eq!(act(":", false, false), Act::Type(':'));
    assert_eq!(act("a", false, false), Act::Type('a'));
}

#[test]
fn alt_holds_a_key_back_from_the_keymap() {
    assert_eq!(act(":", false, true), Act::Literal(':'));
}

#[test]
fn ctrl_bracket_is_the_overstrike_key() {
    assert_eq!(act("]", true, false), Act::Overstrike);
    assert_eq!(
        act("]", false, false),
        Act::Type(']'),
        "without Ctrl it is an ordinary key, and the keymap decides"
    );
}

#[test]
fn the_editing_keys_are_named_not_printable() {
    assert_eq!(act("Enter", false, false), Act::Submit);
    assert_eq!(act("Backspace", false, false), Act::Backspace);
    assert_eq!(act("Delete", false, false), Act::Delete);
    assert_eq!(act("ArrowLeft", false, false), Act::Carriage(Move::Left));
    assert_eq!(act("End", false, false), Act::Carriage(Move::End));
}

#[test]
fn a_modifier_or_a_function_key_on_its_own_does_nothing() {
    for key in ["Shift", "Control", "F5", "Escape", "Unidentified"] {
        assert_eq!(act(key, false, false), Act::Ignore, "the {key} key");
    }
}

#[test]
fn a_browser_chord_we_do_not_claim_is_left_to_the_browser() {
    // Ctrl-C copies, Ctrl-V pastes, Ctrl-R reloads. Only the two the
    // CLI claims are taken, and the rest fall through untouched.
    assert_eq!(act("v", true, false), Act::Ignore);
    assert_eq!(act("r", true, false), Act::Ignore);
    assert_eq!(act("c", true, false), Act::Clear);
}

#[test]
fn the_page_is_told_where_the_carriage_is_not_how_far_along_it_is() {
    // A count of characters would not do: a glyph may be two code
    // points, so the page is handed the text before the carriage and
    // measures it itself.
    use apl_board::state;
    use apl_keyboard::Keyboard;

    let mut keyboard = Keyboard::default();
    keyboard.type_key('a');
    keyboard.overstrike();
    keyboard.type_key('F');
    keyboard.type_key('1');
    keyboard.move_cursor(Move::Left);
    let drawn: serde_json::Value =
        serde_json::from_str(&state(&keyboard, false, false, true)).unwrap();
    assert_eq!(drawn["text"], "A\u{332}1");
    assert_eq!(drawn["before"], "A\u{332}", "one cell, two code points");
    assert_eq!(drawn["acted"], true);
}
