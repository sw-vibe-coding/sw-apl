//! Turning output into transcript lines. The join after `⍞←` is the
//! delicate part: it must not consume a line it does not join.

use apl_console::{Output, Print, Shown, render, render_all};

/// The lines a run of outputs renders to, ignoring the open flag.
fn lines(outs: &[Output], print: Print) -> Vec<String> {
    render_all(outs, print).lines
}
use apl_value::{Array, Data, Number};

fn num(i: i64) -> Output {
    Output::Value(Array::scalar(Number::Int(i)))
}

fn text(s: &str) -> Array {
    let chars: Vec<char> = s.chars().collect();
    Array::new(vec![chars.len()], Data::Char(chars)).unwrap()
}

#[test]
fn a_value_renders_as_its_own_line() {
    let print = Print::default();
    assert_eq!(render(&num(4), print), ["4"]);
    assert_eq!(render(&Output::Nothing, print), Vec::<String>::new());
    assert_eq!(
        render(&Output::Branch(Some(2)), print),
        Vec::<String>::new()
    );
    assert_eq!(render(&Output::Bare("X".into()), print), ["X"]);
}

#[test]
fn a_run_of_values_keeps_every_one_of_them() {
    let print = Print::default();
    assert_eq!(lines(&[num(4)], print), ["4"]);
    assert_eq!(lines(&[num(1), num(2), num(3)], print), ["1", "2", "3"]);
    assert_eq!(
        lines(&[Output::Nothing, num(7), Output::Nothing], print),
        ["7"]
    );
    assert!(lines(&[], print).is_empty());
}

#[test]
fn quote_quad_output_leaves_the_line_open_for_what_follows() {
    let print = Print::default();
    assert_eq!(
        lines(&[Output::Bare("NAME: ".into()), num(42)], print),
        ["NAME: 42"]
    );
    // Only the line straight after it joins; the rest stand alone.
    assert_eq!(
        lines(&[Output::Bare("A".into()), num(1), num(2)], print),
        ["A1", "2"]
    );
    // Two in a row run together.
    assert_eq!(
        lines(
            &[Output::Bare("A".into()), Output::Bare("B".into()), num(3)],
            print
        ),
        ["AB3"]
    );
    // One with nothing after it still shows, and says it is open so
    // an answer typed at a read lands on the same line.
    assert_eq!(
        render_all(&[Output::Bare("A".into())], print),
        Shown {
            lines: vec!["A".to_string()],
            open: true
        }
    );
    // A value after it closes the line again.
    assert!(!render_all(&[Output::Bare("A".into()), num(1)], print).open);
}

#[test]
fn an_open_line_with_nothing_to_join_is_not_eaten() {
    let print = Print::default();
    // Output::Nothing renders to no lines at all, so the value after
    // it must still join rather than be swallowed on its behalf.
    assert_eq!(
        lines(&[Output::Bare("A".into()), Output::Nothing, num(1)], print),
        ["A1"]
    );
}

#[test]
fn mixed_output_prints_its_parts_side_by_side() {
    let print = Print::default();
    let mixed = Output::Mixed(vec![text("X IS "), Array::scalar(Number::Int(5))]);
    assert_eq!(render(&mixed, print), ["X IS 5"]);
}

#[test]
fn the_print_settings_reach_the_formatting() {
    let narrow = Print {
        digits: 3,
        width: 120,
        whole: 0,
    };
    assert_eq!(
        render(
            &Output::Value(Array::scalar(Number::Float(1.0 / 3.0))),
            narrow
        ),
        ["0.333"]
    );
}
