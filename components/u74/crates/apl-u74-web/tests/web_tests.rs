//! What the (U) '74 page asks of the WebAssembly: the demo decks,
//! the holes in each card, and a deck's printout.

use apl_u74_listing::{END_OF_RUN, NO_INPUT};
use apl_u74_web::{DECKS, holes, printout};

#[test]
fn every_demo_deck_runs_to_its_end() {
    assert!(DECKS.len() >= 3);
    for deck in DECKS {
        let out = printout(deck.cards);
        assert!(
            out.contains("UNIVERSITY OF MARYLAND APL"),
            "{}: {out}",
            deck.title
        );
        let last = out.lines().last().unwrap_or_default();
        assert!(
            [END_OF_RUN, NO_INPUT].contains(&last),
            "{}: {last}",
            deck.title
        );
    }
}

#[test]
fn the_first_deck_is_the_domino_deck() {
    assert_eq!(DECKS[0].title, "Domino");
    assert!(printout(DECKS[0].cards).contains("\n1 3\n"));
}

#[test]
fn a_card_has_eighty_columns_of_holes() {
    let card = holes("A1 $");
    assert_eq!(card.len(), 80);
    assert_eq!(&card[..4], &[0b1001, 0b1000, 0, 0b0100_0010_0010]);
    assert!(card[4..].iter().all(|&h| h == 0));
}

#[test]
fn what_an_029_cannot_punch_is_marked() {
    let card = holes("[");
    assert_eq!(card[0], u16::MAX);
}

#[test]
fn past_column_80_is_not_on_the_card() {
    assert_eq!(holes(&"1".repeat(90)).len(), 80);
}
