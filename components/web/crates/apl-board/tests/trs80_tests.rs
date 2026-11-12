//! The keys in (C) '80: a key types its own character, as a TRS-80's
//! does, an unshifted letter its capital and a shifted one the
//! lower-case code APL80 reads as a symbol (docs/mode-c.md).

use apl_board::{Act, act, trs80};

#[test]
fn a_letter_types_its_capital_and_a_shifted_one_its_code() {
    assert_eq!(trs80(act("a", false, false)), Act::Literal('A'));
    assert_eq!(
        trs80(act("P", false, false)),
        Act::Literal('p'),
        "shifted P, rho"
    );
    for c in ['+', '%', '-', '_', '(', ')', '1'] {
        assert_eq!(trs80(act(&c.to_string(), false, false)), Act::Literal(c));
    }
}

#[test]
fn the_editing_keys_are_as_they_are() {
    assert_eq!(
        trs80(act("Enter", false, false)),
        act("Enter", false, false)
    );
    assert_eq!(
        trs80(act("Backspace", false, false)),
        act("Backspace", false, false)
    );
}
