//! Labels at the start of a body line, and the del header.

use apl_scan::{labels, parse_header, without_label};

fn body(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|&l| l.to_string()).collect()
}

#[test]
fn a_label_is_a_name_and_a_colon_at_the_start_of_a_line() {
    assert_eq!(
        labels(&body(&["I\u{2190}1", "N:I", "I\u{2190}I+1", "LOOP:  I"])),
        [("N".to_string(), 2), ("LOOP".to_string(), 4)]
    );
}

#[test]
fn a_colon_anywhere_else_is_not_a_label() {
    assert_eq!(labels(&body(&["'N: NOT A LABEL'"])), []);
    assert_eq!(labels(&body(&["I\u{2190}1 \u{235d} N:"])), []);
    assert_eq!(labels(&body(&["1:2"])), []);
    // An unterminated literal is a lexical error, not a label.
    assert_eq!(labels(&body(&["N:'"])), []);
}

#[test]
fn stripping_a_label_keeps_every_other_character_in_place() {
    assert_eq!(without_label("LOOP:I\u{2190}I+1"), "     I\u{2190}I+1");
    assert_eq!(without_label("  N: 2+2"), "     2+2");
    assert_eq!(without_label("2+2"), "2+2");
}

#[test]
fn a_header_names_the_function_its_arguments_and_its_locals() {
    let defn = parse_header("R\u{2190}A HYP B;T", "").unwrap();
    assert_eq!(defn.name, "HYP");
    assert_eq!(defn.names(), ["R", "A", "B", "T"]);
}
