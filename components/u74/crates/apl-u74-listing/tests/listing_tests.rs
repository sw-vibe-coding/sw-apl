//! The printout's lines (docs/mode-u.md, "The listing").

use apl_u74_card::Card;
use apl_u74_listing::{BANNER, banner, card, character, column, located};

#[test]
fn the_banner_names_maryland_and_the_clear_workspace() {
    assert_eq!(banner(None), [BANNER, "CLEAR WS"]);
    let timed = banner(Some("14.02.07 11/24/76"));
    assert_eq!(timed[0], format!("{BANNER}   14.02.07 11/24/76"));
}

#[test]
fn a_card_is_listed_with_its_number_its_text_under_column_7() {
    let c = Card::new(3, "A $AL 1");
    assert_eq!(card(&c), "0003  A $AL 1");
    assert_eq!(
        &card(&c)[6..],
        c.columns,
        "where the echo's statement starts"
    );
}

#[test]
fn an_error_names_its_card_and_column() {
    assert_eq!(located(6, Some(12)), "CARD 6 COLUMN 12");
    assert_eq!(located(6, None), "CARD 6");
    let c = Card::new(5, "A $XY 1");
    assert_eq!(
        character(&c, 3),
        [
            "CHARACTER ERROR",
            "      A $XY 1",
            "        ^",
            "CARD 5 COLUMN 3"
        ]
    );
}

#[test]
fn the_caret_is_traced_back_to_its_column() {
    let lines: Vec<String> = ["SYNTAX ERROR", "      X←1+", "         ^"]
        .map(String::from)
        .to_vec();
    let columns = [1, 3, 7, 8];
    assert_eq!(column(&lines, "X←1+", &columns), Some(8));
    assert_eq!(column(&lines, "Y←2", &columns), None, "another statement");
}
