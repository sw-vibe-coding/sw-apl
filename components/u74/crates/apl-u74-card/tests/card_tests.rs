//! A card: 80 columns, a statement in 1-72, a sequence number in
//! 73-80, and only what an IBM 029 punches (docs/mode-u.md).

use apl_u74_card::{Card, deck, punchable};

fn card(text: &str) -> Card {
    Card::new(1, text)
}

#[test]
fn the_statement_is_columns_1_to_72() {
    let seq = format!("{:<72}{}", "A $AL 5", "00000010");
    assert_eq!(card(&seq).statement(), "A $AL 5");
    assert_eq!(card("B $AL 1   ").statement(), "B $AL 1");
    assert_eq!(card(&seq).columns, seq, "the listing shows all 80");
}

#[test]
fn what_lies_past_column_80_is_not_on_the_card() {
    let long = "X".repeat(90);
    assert_eq!(card(&long).columns.chars().count(), 80);
}

#[test]
fn a_control_card_has_an_at_sign_in_column_1() {
    assert_eq!(card("@RUN   ALBANY,76104").control(), Some("RUN"));
    assert_eq!(card("@APL,I").control(), Some("APL"));
    assert_eq!(card("@FIN").control(), Some("FIN"));
    assert_eq!(card(" @FIN").control(), None, "column 2 is APL's");
    assert_eq!(card("X $AL 1").control(), None);
}

#[test]
fn an_029_punches_upper_case_digits_and_its_specials() {
    for c in "AZ09 &-/.<(+|!$*);¬,%_>?:#@'=\"".chars() {
        assert!(punchable(c), "{c}");
    }
    for c in "az[]\\^~⍴¢{}".chars() {
        assert!(!punchable(c), "{c}");
    }
    assert_eq!(card("A $AL 1").unpunchable(), None);
    assert_eq!(card("A [1]").unpunchable(), Some(3), "column 3");
    let seq = format!("{:<72}{}", "A", "0000001a");
    assert_eq!(card(&seq).unpunchable(), Some(80));
}

#[test]
fn a_deck_is_a_card_a_line_numbered_from_1() {
    let cards = deck("@RUN\r\nA $AL 1\n\n@FIN\n");
    let numbers: Vec<usize> = cards.iter().map(|c| c.number).collect();
    assert_eq!(numbers, [1, 2, 3, 4], "a blank card is a card");
    assert_eq!(cards[1].statement(), "A $AL 1");
    assert_eq!(cards[3].control(), Some("FIN"));
    assert!(deck("").is_empty());
}
