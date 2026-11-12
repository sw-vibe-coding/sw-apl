//! APL80's characters to glyphs and back (docs/mode-c.md, The keys).

use apl_c80_keys::{glyphs, name, shown};
use apl_value::ErrorKind;

/// `line` in glyphs, with `F` and `G` the only functions.
fn typed(line: &str) -> String {
    glyphs(line, &|n| n == "F" || n == "G").expect(line).text
}

#[test]
fn substitutes_and_shifted_letters() {
    assert_eq!(typed("2%4"), "2÷4");
    assert_eq!(typed("3 4p5"), "3 4⍴5");
    assert_eq!(typed("i5"), "⍳5");
    assert_eq!(typed("x/i5"), "×/⍳5");
    assert_eq!(typed("@2"), "⍟2");
    assert_eq!(typed("A$B&C"), "A≠B∧C");
    assert_eq!(typed("A\".+B"), "A∘.+B");
    assert_eq!(typed("+.xM"), "+.×M");
    assert_eq!(typed("rgA"), "⌽⍉A");
}

#[test]
fn the_minus_signs_move_down_a_notch() {
    assert_eq!(typed("-3"), "¯3");
    assert_eq!(typed("-3_6"), "¯3-6");
    assert_eq!(typed("1E-5"), "1E¯5");
    assert_eq!(typed("_X"), "-X");
}

#[test]
fn names_may_hold_dots_and_minus_signs() {
    assert_eq!(typed("M-N]3"), "M∆N←3");
    assert_eq!(typed("A.B_1"), "A⍙B-1");
    assert_eq!(typed("D.TRIM X"), "D⍙TRIM X");
    assert_eq!(typed("1.5+A.5"), "1.5+A⍙5");
    assert_eq!(name("M-N"), "M∆N");
    assert_eq!(name("D.TRIM"), "D⍙TRIM");
}

#[test]
fn the_arrows_and_their_codes() {
    assert_eq!(typed("X←3"), "X←3");
    assert_eq!(typed("X]3"), "X←3", "93 is the left arrow");
    assert_eq!(typed("^3"), "→3", "94 the right");
    assert_eq!(typed("2↑X"), "2↑X");
    assert_eq!(typed("2[X"), "2↑X", "91 the up arrow");
    assert_eq!(typed("2\\X"), "2↓X", "92 the down arrow");
}

#[test]
fn a_monadic_arrow_grades() {
    assert_eq!(typed("↑3 1 2"), "⍋3 1 2");
    assert_eq!(typed("↓X"), "⍒X");
    assert_eq!(typed("X(↑X)"), "X[⍋X]");
    assert_eq!(
        typed("F↑X"),
        "F⍋X",
        "after a function there is no left argument"
    );
    assert_eq!(typed("X↑Y"), "X↑Y");
}

#[test]
fn round_brackets_index_a_variable() {
    assert_eq!(typed("A(2)"), "A[2]");
    assert_eq!(typed("A((1+2);2)"), "A[(1+2);2]");
    assert_eq!(typed("F(2)"), "F(2)", "a function's argument");
    assert_eq!(typed("(A+1)"), "(A+1)");
    assert_eq!(typed("A(2)]5"), "A[2]←5");
    assert_eq!(typed("' *'(1+G)"), "' *'[1+G]", "a string is indexed too");
}

#[test]
fn d_chooses_the_axis() {
    assert_eq!(typed("+/1dM"), "+/[1]M");
    assert_eq!(typed("+/KdM"), "+/[K]M");
    assert_eq!(typed("1r1dM"), "1⌽[1]M");
}

#[test]
fn format_status_comments_and_strings() {
    assert_eq!(typed("f6 2"), "⍕6 2");
    assert_eq!(typed("s4"), "⌶4");
    assert_eq!(typed("#'A'"), "#'A'");
    assert_eq!(typed("c A NOTE, p"), "⍝ A NOTE, ⍴");
    assert_eq!(
        typed("'A%p-_ (1)'"),
        "'A÷⍴¯- (1)'",
        "a string is translated letter by letter"
    );
}

#[test]
fn a_character_apl80_has_not_got_is_an_error() {
    let err = glyphs("2+⍴3", &|_| false).expect_err("a glyph");
    assert_eq!(err.kind, ErrorKind::Character('⍴'));
    assert_eq!(err.caret, Some(2));
}

#[test]
fn the_shifted_u_peeks_and_pokes() {
    assert_eq!(typed("u16000"), "⌷16000");
    assert_eq!(typed("65u16000"), "65⌷16000");
    assert_eq!(typed("#u15360+i5"), "#⌷15360+⍳5");
}

#[test]
fn each_glyph_knows_where_it_was_typed() {
    let t = glyphs("+/1dM", &|_| false).expect("typed");
    assert_eq!(t.text, "+/[1]M");
    assert_eq!(t.from, vec![0, 1, 2, 2, 3, 4]);
}

#[test]
fn what_is_shown_goes_back_to_apl80s_characters() {
    assert_eq!(shown("÷⍴¯3 ⍋⍒∆⍙←→↑↓"), "%p-3 ↑↓-.←→↑↓");
    assert_eq!(shown("1 ¯2.5 1E¯5"), "1 -2.5 1E-5");
    assert_eq!(shown("M∆N D⍙TRIM"), "M-N D.TRIM");
    assert_eq!(shown("A-B ⎕: ⍞"), "A_B q: m");
    assert_eq!(shown("(C) '80"), "(C) '80");
}
