use apl_2741_prototype::keyboard::{Keyboard, display};

#[test]
fn arithmetic_keys_and_minus_overstrikes_use_the_typeball_layout() {
    let mut k = Keyboard::default();
    for c in "2-3=4_5+6".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), "2+3×4-5÷6");
    for (base, expected) in [('O', "⊖"), ('/', "⌿"), ('\\', "⍀")] {
        k.clear();
        k.type_key(base);
        k.overstrike();
        assert!(k.type_key('_'));
        assert_eq!(k.text(), expected);
    }
}

#[test]
fn home_row_punctuation_supports_commands_grouping_and_indexing() {
    let mut k = Keyboard::default();
    for c in "\"wsid".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), ")WSID");
    k.clear();
    for c in ":a;1'\"".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), "(A[1])");
    k.clear();
    for c in "90()K".chars() {
        k.type_key(c);
    }
    assert_eq!(k.text(), "90∨∧'");
}

#[test]
fn physical_keys_compose_quote_quad_before_submission() {
    let mut k = Keyboard::default();
    k.type_key('l');
    k.type_key('k');
    assert_eq!(k.text(), "LK");
    k.clear();
    k.type_key('L');
    assert_eq!(k.text(), "⎕");
    k.overstrike();
    k.type_key('K');
    assert_eq!(k.text(), "⍞");
}

#[test]
fn underscore_is_one_editable_cell_but_two_wire_codepoints() {
    let mut k = Keyboard::default();
    k.type_key('a');
    k.overstrike();
    k.type_key('F');
    assert_eq!(k.text(), "A\u{332}");
    assert_eq!(display(&k.text()), "Ⓐ");
    k.backspace();
    assert_eq!(k.text(), "");
}

#[test]
fn invalid_strike_preserves_base_and_repeated_back_is_harmless() {
    let mut k = Keyboard::default();
    k.overstrike();
    k.type_key('L');
    k.overstrike();
    k.overstrike();
    assert!(!k.type_key('a'));
    assert_eq!(k.text(), "⎕");
    k.type_key('K');
    assert_eq!(k.text(), "⍞");
}

#[test]
fn paste_is_unicode_and_cursor_edits_cells() {
    let mut k = Keyboard::default();
    k.paste("A\u{332}←2");
    k.move_cursor(apl_2741_prototype::keyboard::Move::Left);
    k.backspace();
    k.type_key('{');
    assert_eq!(k.text(), "A\u{332}←2");
}
