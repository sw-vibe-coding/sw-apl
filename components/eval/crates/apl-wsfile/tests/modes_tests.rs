//! What the writer says about modes and settings: the `⍝!MODES` line,
//! worked out from what the workspace uses, and the settings as
//! directives both modes read.

use std::rc::Rc;

use apl_ast::Defn;
use apl_modes::{Mode, Modes, modes};
use apl_workspace::Saved;
use apl_wsfile::{plain, write};

const WHEN: &str = "1.00.00 01/01/70";

fn with(body: &str, locked: bool) -> Saved {
    let mut saved = Saved::default();
    let f = Defn {
        name: "F".to_string(),
        result: Some("R".to_string()),
        body: vec![body.to_string()],
        locked,
        ..Defn::default()
    };
    saved.funcs.insert(f.name.clone(), Rc::new(f));
    saved
}

#[test]
fn the_modes_line_is_second() {
    let text = write(&with("R←1", false), WHEN);
    assert_eq!(text.lines().nth(1), Some("⍝!MODES (A)(B)"), "{text}");
}

#[test]
fn the_settings_are_directives_not_commands() {
    let mut saved = Saved::default();
    saved.env.io = 0;
    saved.print.digits = 3;
    saved.print.width = 80;
    let text = write(&saved, WHEN);
    for line in ["⍝!ORIGIN 0", "⍝!DIGITS 3", "⍝!WIDTH 80"] {
        assert!(text.lines().any(|l| l == line), "{line} in {text}");
    }
    assert!(!text.contains(")ORIGIN"), "no '70-only command: {text}");
    assert!(!text.contains(")DIGITS") && !text.contains(")WIDTH"));
    assert_eq!(modes(&text), Modes::ALL, "settings alone run anywhere");
}

#[test]
fn an_i_beam_makes_a_workspace_68_only() {
    let text = write(&with("R←⌶20", false), WHEN);
    assert_eq!(modes(&text), Modes::only(Mode::A));
}

#[test]
fn a_group_makes_a_workspace_68_only() {
    let mut saved = with("R←1", false);
    saved.groups.insert("G".to_string(), vec!["F".to_string()]);
    assert_eq!(modes(&write(&saved, WHEN)), Modes::only(Mode::A));
}

#[test]
fn execute_format_or_a_quad_name_make_a_workspace_75_only() {
    for body in ["R←⍎'1+1'", "R←⍕1 2", "R←⎕IO", "R←⎕FX 'F'"] {
        let text = write(&with(body, false), WHEN);
        assert_eq!(modes(&text), Modes::only(Mode::B), "{body}");
    }
}

#[test]
fn quad_and_quote_quad_run_in_both() {
    for body in ["R←⎕", "⎕←1", "R←⍞"] {
        assert_eq!(
            modes(&write(&with(body, false), WHEN)),
            Modes::ALL,
            "{body}"
        );
    }
}

#[test]
fn a_glyph_in_a_string_or_a_comment_is_not_a_use() {
    for body in ["R←'⌶⍎⍕'", "R←1 ⍝ ⌶ AND ⍎", "R←'IT''S ⍎'"] {
        assert_eq!(
            modes(&write(&with(body, false), WHEN)),
            Modes::ALL,
            "{body}"
        );
    }
}

#[test]
fn an_obscured_workspace_names_its_modes_in_the_clear() {
    let text = write(&with("R←⌶20", true), WHEN);
    assert_eq!(modes(&text), Modes::only(Mode::A), "{text}");
    let open = write(&with("R←⌶20", false), WHEN);
    assert_eq!(
        plain(&text),
        open.replace("\n∇\n", "\n⍫\n"),
        "revealed as written"
    );
}
