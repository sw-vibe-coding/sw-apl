//! What a strike that forms no glyph says about why.
//!
//! A reader typed the lamp as cap struck with jot; it is cap with
//! circle (docs/plan.md, "say the near miss").

use apl_strike::near_miss;
use apl_value::OVERSTRIKE_B;

#[test]
fn cap_with_jot_points_to_the_lamp() {
    assert_eq!(
        near_miss('∩', '∘', &[]),
        "∩ with ∘ forms no glyph. ∩ forms ⍝ with ○."
    );
}

#[test]
fn every_pair_the_base_is_in_is_listed() {
    let hint = near_miss('○', 'x', &[]);
    for (glyph, over) in [("⍟", "*"), ("⌽", "|"), ("⊖", "-"), ("⍉", "\\"), ("⍝", "∩")] {
        assert!(hint.contains(&format!("{glyph} with {over}")), "{hint}");
    }
}

#[test]
fn only_the_modes_pairs_are_offered() {
    assert!(!near_miss('⊥', 'x', &[]).contains('⍎'));
    assert!(near_miss('⊥', 'x', &OVERSTRIKE_B).contains("⍎ with ∘"));
}

#[test]
fn a_letter_is_pointed_at_the_underbar() {
    assert_eq!(
        near_miss('A', '∘', &[]),
        "A with ∘ forms no glyph. A forms A\u{332} with _."
    );
}

#[test]
fn the_underbar_is_pointed_at_a_letter() {
    assert_eq!(
        near_miss('_', '∘', &[]),
        "_ with ∘ forms no glyph. _ forms ⍙ with ∆, and an underscored letter with any letter."
    );
}
