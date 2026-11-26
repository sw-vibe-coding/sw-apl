//! The `$` spellings: two letters after a dollar for each APL
//! character an 029 cannot punch (docs/mode-u.md).

use apl_u74_mnemonic::{MNEMONICS, glyph, mnemonic, spell};

#[test]
fn the_notes_spellings_and_ours() {
    assert_eq!(glyph("RH"), Some('⍴'));
    assert_eq!(glyph("AL"), Some('←'));
    assert_eq!(glyph("DM"), Some('⌹'));
    assert_eq!(glyph("NA"), Some('⍲'), "nand, on the right glyph");
    assert_eq!(glyph("NO"), Some('⍱'), "nor");
    assert_eq!(glyph("LB"), Some('['));
    assert_eq!(glyph("BS"), Some('\\'));
    assert_eq!(glyph("DE"), None, "no execute");
    assert_eq!(glyph("FM"), None, "no format");
    assert_eq!(mnemonic('⍳'), Some("IO"));
}

#[test]
fn every_spelling_is_two_letters_for_one_glyph() {
    for (code, g) in MNEMONICS {
        assert_eq!(code.len(), 2, "{code}");
        assert!(code.chars().all(|c| c.is_ascii_uppercase()), "{code}");
        let same = MNEMONICS.iter().filter(|(_, h)| *h == g).count();
        assert_eq!(same, 1, "{g} spelled twice");
    }
}

#[test]
fn a_statement_is_spelled_out_in_glyphs() {
    let s = spell("A $AL 2 2 $RH 2 1 1 3").expect("spelled");
    assert_eq!(s.text, "A ← 2 2 ⍴ 2 1 1 3");
    assert_eq!(s.columns[2], 3, "the arrow came from column 3");
    assert_eq!(s.columns[4], 7, "the 2 after it from column 7");
}

#[test]
fn inside_quotes_too_and_a_doubled_dollar_is_one() {
    let s = spell("'$RH COSTS $$5'").expect("spelled");
    assert_eq!(s.text, "'⍴ COSTS $5'");
}

#[test]
fn the_029s_not_sign_is_tilde() {
    assert_eq!(spell("¬1 0").expect("spelled").text, "~1 0");
}

#[test]
fn an_unknown_spelling_is_reported_at_its_column() {
    assert_eq!(spell("A $XY 1"), Err(3));
    assert_eq!(spell("A $R"), Err(3), "cut short");
    assert_eq!(spell("$"), Err(1));
}
