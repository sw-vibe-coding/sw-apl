//! The shared channel's layout, which is the one part of the browser
//! session that can be checked without a browser.

use apl_wasm::channel::{ATTN, BODY, LENGTH, SIZE, STATE, put, take};

#[test]
fn a_typed_line_goes_in_and_comes_back() {
    let mut body = [0u8; 64];
    let len = put(&mut body, "2+2");
    assert_eq!(len, 3);
    assert_eq!(take(&body, len), "2+2");
}

#[test]
fn glyphs_survive_the_channel() {
    let mut body = [0u8; 64];
    let len = put(&mut body, "X←⍳5");
    assert_eq!(take(&body, len), "X←⍳5");
    assert!(len > 4, "the glyphs are more than one byte each");
}

#[test]
fn an_empty_line_is_a_line() {
    let mut body = [0u8; 8];
    assert_eq!(put(&mut body, ""), 0);
    assert_eq!(take(&body, 0), "");
}

#[test]
fn a_line_too_long_for_the_channel_is_cut_at_a_glyph() {
    // Never in the middle of one: the far end decodes what it is
    // given, and half a glyph is not UTF-8.
    let mut body = [0u8; 7];
    let len = put(&mut body, "⍳⍳⍳");
    assert_eq!(len, 6, "two glyphs fit, the third does not");
    assert_eq!(take(&body, len), "⍳⍳");
}

#[test]
fn the_body_starts_after_the_header_and_the_channel_holds_a_line() {
    // The page builds its views on these two numbers, so they are
    // checked where they are read rather than trusted.
    const { assert!(BODY.is_multiple_of(4), "the header is whole Int32 slots") };
    const { assert!(SIZE > BODY + 1024, "a typed line fits with room to spare") };
}

#[test]
fn attention_has_a_slot_of_its_own() {
    // ATTN is written by the page while the worker is inside a run and
    // reading nothing else, so it cannot share a slot with the state the
    // worker sleeps on or the length of a line: a write to either would
    // be read as the other.
    let slots = [STATE, LENGTH, ATTN];
    for (i, a) in slots.iter().enumerate() {
        for b in &slots[i + 1..] {
            assert_ne!(a, b, "two header fields share a slot");
        }
    }
    // And the line's bytes start after every slot, not on top of one.
    let slot_count = usize::try_from(ATTN).expect("a small slot number") + 1;
    assert!(BODY >= slot_count * 4, "the body overlaps the header");
}
