//! What the paper shows, which is not always what the wire carries.

use apl_keyboard::Keyboard;
use apl_paper::display;

#[test]
fn an_underscored_capital_is_shown_as_a_circled_one() {
    let mut keyboard = Keyboard::default();
    keyboard.type_key('a');
    keyboard.overstrike();
    keyboard.type_key('F');
    assert_eq!(keyboard.text(), "A\u{332}", "the wire carries the low line");
    assert_eq!(
        display(&keyboard.text()),
        "\u{24b6}",
        "the paper shows a circle"
    );
}

#[test]
fn control_characters_never_reach_the_page() {
    assert_eq!(display("A\u{7}B"), "AB");
    assert_eq!(display("A\nB"), "A\nB", "a line ending is not one of them");
}
