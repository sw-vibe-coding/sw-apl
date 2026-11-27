//! What the (U) '74 page asks of the WebAssembly: the demo decks,
//! the holes in each card, and a deck's printout.

use apl_u74_listing::{END_OF_RUN, NO_INPUT};
use apl_u74_web::{DECKS, catalog, holes, printout, reference};

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

#[test]
fn the_reference_spells_every_glyph_a_card_cannot_hold() {
    let lines = reference();
    let lines: Vec<&str> = lines.lines().collect();
    assert_eq!(lines.len(), 45);
    assert!(lines.contains(&"$RH ⍴"));
    assert!(lines.contains(&"$NA ⍲"));
    assert!(lines.contains(&"$LB ["));
}

#[test]
fn the_catalog_is_every_deck_a_title_then_its_cards_split_by_form_feeds() {
    let decks: Vec<&str> = catalog().leak().split('\x0c').collect();
    assert_eq!(decks.len(), DECKS.len());
    for (text, deck) in decks.iter().zip(DECKS.iter()) {
        let (title, cards) = text.split_once('\n').unwrap();
        assert_eq!(title, deck.title);
        assert_eq!(cards, deck.cards.trim_end());
    }
}
