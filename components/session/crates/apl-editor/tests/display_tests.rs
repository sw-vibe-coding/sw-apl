//! The display commands as APL\360 gives them (SH20-0906-0 p. 3.18,
//! "Display"), and the 5110 manual's table agrees: `[⎕]` the whole
//! function, then the next statement; `[⎕N]` from N on, then the last
//! one shown for replacement; `[N⎕]` statement N alone, then N for
//! replacement.

use apl_ast::Defn;
use apl_editor::Definition;
use apl_value::ErrorKind;

fn open() -> Definition {
    let defn = Defn {
        name: "F".to_string(),
        result: Some("R".to_string()),
        body: ["A", "B", "C"].iter().map(|l| format!("R←{l}")).collect(),
        ..Defn::default()
    };
    Definition::start(defn, false, "")
}

fn shows(definition: &mut Definition, line: &str) -> Vec<String> {
    definition.line(line).expect("the editor accepted it").lines
}

#[test]
fn quad_alone_shows_the_whole_function_and_leaves_the_prompt() {
    let mut f = open();
    assert_eq!(
        shows(&mut f, "[⎕]"),
        [
            "      ∇R←F",
            "[1]   R←A",
            "[2]   R←B",
            "[3]   R←C",
            "      ∇"
        ]
    );
    assert_eq!(f.prompt(), "[4]   ");
}

#[test]
fn n_quad_shows_statement_n_alone_and_offers_it() {
    let mut f = open();
    assert_eq!(shows(&mut f, "[2⎕]"), ["[2]   R←B"]);
    assert_eq!(f.prompt(), "[2]   ", "awaits replacement of statement 2");
    shows(&mut f, "R←BB");
    assert_eq!(f.close(false).0.body, ["R←A", "R←BB", "R←C"]);
}

#[test]
fn quad_n_shows_from_n_on_and_offers_the_last() {
    let mut f = open();
    assert_eq!(shows(&mut f, "[⎕2]"), ["[2]   R←B", "[3]   R←C"]);
    assert_eq!(f.prompt(), "[3]   ", "awaits replacement of the last shown");
}

#[test]
fn the_header_is_statement_0() {
    let mut f = open();
    assert_eq!(shows(&mut f, "[0⎕]"), ["[0]   R←F"]);
    assert_eq!(f.prompt(), "[0]   ");
}

#[test]
fn a_display_of_nothing_is_a_defn_error() {
    for line in ["[5⎕]", "[⎕5]"] {
        let mut f = open();
        assert_eq!(f.line(line).unwrap_err().kind, ErrorKind::Defn, "{line}");
        assert_eq!(f.prompt(), "[4]   ", "{line} left the prompt alone");
    }
}
