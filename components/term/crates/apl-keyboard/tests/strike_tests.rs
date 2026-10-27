//! A strike can always be finished or left.
//!
//! Found on the browser board (docs/plan.md, "a strike that cannot be
//! finished"): the overstrike key held an underscored letter, which
//! nothing strikes, and a pair that formed no glyph left the strike
//! waiting -- so every key rang and only Erase got out.

use apl_keyboard::Keyboard;

/// A keyboard holding `text`, typed as Unicode.
fn holding(text: &str) -> Keyboard {
    let mut k = Keyboard::default();
    k.paste(text);
    k
}

#[test]
fn the_overstrike_key_refuses_a_glyph_nothing_strikes() {
    for text in ["A\u{332}", "1", " ", "⍟"] {
        let mut k = holding(text);
        assert!(!k.overstrike(), "{text:?} is the base of no overstrike");
        assert!(!k.pending, "{text:?} is held");
    }
}

#[test]
fn the_overstrike_key_holds_a_glyph_something_strikes() {
    for text in ["A", "○", "_", "⎕", "∆"] {
        let mut k = holding(text);
        assert!(k.overstrike(), "{text:?} strikes");
        assert!(k.pending, "{text:?} is not held");
    }
}

#[test]
fn a_modes_own_bases_strike_only_in_that_mode() {
    // ∘ is struck only with ⊥ or ⊤, and only in (B).
    let mut a = holding("∘");
    assert!(!a.overstrike(), "(A) has no pair with jot");
    let mut b = holding("∘");
    b.also = &apl_value::OVERSTRIKE_B;
    assert!(b.overstrike(), "(B) strikes jot");
}

#[test]
fn the_overstrike_key_again_cancels_the_strike() {
    let mut k = holding("A");
    k.overstrike();
    assert!(k.overstrike(), "cancelling is not a refusal");
    assert!(!k.pending);
    k.type_key('b');
    assert_eq!(k.text(), "AB", "B goes beside A, not over it");
}

#[test]
fn a_pair_that_forms_no_glyph_drops_the_key_and_ends_the_strike() {
    let mut k = holding("A");
    k.overstrike();
    assert!(!k.type_key('b'), "A and B form nothing: the bell");
    assert_eq!(k.text(), "A", "the key is dropped, the base kept");
    assert!(!k.pending);
    assert!(k.type_key('b'));
    assert_eq!(k.text(), "AB", "typing goes on as normal");
}

#[test]
fn a_miss_is_remembered_once_as_a_hint() {
    let mut k = holding("∩");
    k.overstrike();
    assert!(!k.paste("∘"), "cap with jot forms nothing");
    assert_eq!(
        k.missed().as_deref(),
        Some("∩ with ∘ forms no glyph. ∩ forms ⍝ with ○.")
    );
    assert_eq!(k.missed(), None, "a hint is said once");
}

#[test]
fn a_strike_that_forms_a_glyph_leaves_no_hint() {
    let mut k = holding("∩");
    k.overstrike();
    assert!(k.paste("○"));
    assert_eq!(k.missed(), None);
}
