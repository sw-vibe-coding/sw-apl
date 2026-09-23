//! The del editor on its own: where the prompt stands, how fractional
//! line numbers behave, and what closing produces. Session transcripts
//! cover the same ground end to end; these pin the arithmetic, which
//! is the part that is easy to get subtly wrong.

use apl_ast::Defn;
use apl_editor::Definition;
use apl_value::{ErrorKind, SYNTAX};

fn header(name: &str) -> Defn {
    Defn {
        name: name.to_string(),
        ..Defn::default()
    }
}

fn open(body: &[&str]) -> Definition {
    Definition::start(
        Defn {
            name: "F".to_string(),
            body: body.iter().map(|l| (*l).to_string()).collect(),
            ..Defn::default()
        },
        false,
        "",
    )
}

fn typed(definition: &mut Definition, lines: &[&str]) {
    for line in lines {
        definition.line(line).expect("the editor accepted it");
    }
}

#[test]
fn the_prompt_offers_the_line_after_the_last() {
    assert_eq!(Definition::start(header("F"), false, "").prompt(), "[1]   ");
    assert_eq!(open(&["a", "b"]).prompt(), "[3]   ");
    let mut wide = open(&[]);
    typed(&mut wide, &["[10]"]);
    assert_eq!(
        wide.prompt(),
        "[10]  ",
        "the number fills the same six columns"
    );
}

#[test]
fn the_prompt_steps_at_the_grain_the_number_uses() {
    for (number, text, next) in [
        ("[1]", "R\u{2190}1", "[2]   "),
        ("[1.5]", "R\u{2190}1", "[1.6] "),
        ("[1.55]", "R\u{2190}1", "[1.56]"),
        ("[1.555]", "R\u{2190}1", "[1.556]"),
        // Line 0 is the header, so what is typed there must be one.
        ("[0]", "R\u{2190}F N", "[1]   "),
    ] {
        let mut definition = open(&[]);
        typed(&mut definition, &[number, text]);
        assert_eq!(definition.prompt(), next, "after {number}");
    }
}

#[test]
fn three_decimal_places_is_as_fine_as_a_line_number_goes() {
    let mut definition = open(&[]);
    assert!(definition.line("[1.999]").is_ok());
    assert_eq!(
        definition.line("[1.9999]").unwrap_err().kind,
        ErrorKind::Defn
    );
    for bad in ["[1", "[]", "[X]", "[1.2.3]", "[-1]", "[\u{2206}]"] {
        assert_eq!(
            definition.line(bad).unwrap_err().kind,
            ErrorKind::Defn,
            "{bad}"
        );
    }
}

#[test]
fn closing_renumbers_the_lines_from_one() {
    let mut definition = open(&["first", "last"]);
    typed(&mut definition, &["[1.5] middle", "[1.25] second"]);
    let (defn, renamed) = definition.close(false);
    assert_eq!(defn.body, ["first", "second", "middle", "last"]);
    assert!(!defn.locked);
    assert_eq!(renamed, None, "the name did not change");
}

#[test]
fn a_header_edit_that_renames_reports_the_name_it_replaced() {
    let mut definition = open(&["body"]);
    typed(&mut definition, &["[0] R\u{2190}G N"]);
    let (defn, renamed) = definition.close(false);
    assert_eq!(defn.name, "G");
    assert_eq!(defn.right.as_deref(), Some("N"));
    assert_eq!(defn.body, ["body"], "the body survived the header edit");
    assert_eq!(renamed.as_deref(), Some("F"));
}

#[test]
fn del_tilde_closes_it_locked() {
    let mut definition = open(&["body"]);
    let step = definition.line("\u{236b}").expect("del-tilde closes");
    assert_eq!(step.closed, Some(true));
    assert!(definition.close(true).0.locked);
}

#[test]
fn a_command_and_a_close_may_share_a_line() {
    let mut definition = open(&["one"]);
    let step = definition
        .line("[\u{2395}]\u{2207}")
        .expect("show then close");
    assert_eq!(step.closed, Some(false));
    assert_eq!(
        step.lines,
        ["      \u{2207}F", "[1]   one", "      \u{2207}"]
    );
}

#[test]
fn the_editor_uses_the_glyphs_the_table_defines() {
    let glyph = |name: &str| {
        SYNTAX
            .iter()
            .find(|(_, n, _)| *n == name)
            .unwrap_or_else(|| panic!("{name} is not in the glyph table"))
            .0
    };
    assert_eq!(glyph("del"), '\u{2207}');
    assert_eq!(glyph("del-tilde"), '\u{236b}');
    assert_eq!(glyph("quad"), '\u{2395}');
    assert_eq!(glyph("delta"), '\u{2206}');
    // The characters most often typed in their place are different
    // code points, and would be taken for ordinary body text.
    assert_ne!(glyph("delta"), '\u{0394}', "Greek capital delta");
    assert_ne!(glyph("del-tilde"), '\u{2b6b}');
    assert_ne!(glyph("quad"), '\u{25a1}', "white square");
}
