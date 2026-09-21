//! ATTN on the wire: one line a terminal may send unprompted.
//!
//! A terminal sends a JSON string per typed line, and `nc` sends raw
//! text, so an attention is a JSON object. It cannot be confused with a
//! typed line, and it cannot be sent by accident at `nc`: a line of APL
//! is never that object, because `{` and `"` are not APL.

use apl_wire::{ATTENTION, attention};

#[test]
fn the_attention_line_is_an_attention() {
    assert!(attention(ATTENTION));
    assert!(
        attention(&format!("{ATTENTION}\n")),
        "with the line's own newline"
    );
    assert!(attention("{ \"attn\" : true }"), "however it is spaced");
}

#[test]
fn no_line_of_apl_is_an_attention() {
    for line in [
        "2+2",
        "+/⍳10",
        "'{\"attn\":true}'",
        "∇R←F X",
        ")LOAD 1 LIFE",
        "⍝ {\"attn\":true}",
        "\"2+2\"",
        "\"{\\\"attn\\\":true}\"",
        "",
        "{}",
        "{\"attn\":false}",
        "{\"attn\":1}",
    ] {
        assert!(!attention(line), "{line:?} was taken for ATTN");
    }
}
