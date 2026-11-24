//! (D) '83's transfer form, as the May 1983 APL Reference gives it:
//! `⎕TF` (page 8-11) and the `)IN` and `)OUT` commands (pages 12-8
//! and 12-13), which carry objects in and out of `.AIO` files.

use apl_session::{Memory, Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.ws.store = Box::new(Memory::default());
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn define(s: &mut Session, lines: &[&str]) {
    for line in lines {
        out(s, line);
    }
}

const PLUS: &[&str] = &["∇Z←L PLUS R", "Z←L+R", "∇"];

const PRIMES: &[&str] = &[
    "∇V←PRIMES N;⎕IO;M",
    "⎕IO←1",
    "M←⍳N",
    "V←(1=0+.=(1+M)∘.|M)/M",
    "∇",
];

/// The manual's own examples, page 8-13.
#[test]
fn transfer_form_of_the_manuals_objects() {
    let mut d = in_mode(Mode::D);
    out(&mut d, "THIS←2 3⍴⍳6");
    assert_eq!(out(&mut d, "⎕TF 'THIS'"), ["NTHIS 2 2 3 1 2 3 4 5 6"]);
    out(&mut d, "THAT←3 4⍴'ABCDEFGHIJKL'");
    assert_eq!(out(&mut d, "⎕TF 'THAT'"), ["CTHAT 2 3 4 ABCDEFGHIJKL"]);
    assert_eq!(out(&mut d, "⍴⎕TF 'THAT'"), ["24"]);
    define(&mut d, PLUS);
    assert_eq!(
        out(&mut d, "⎕TF 'PLUS'"),
        ["FPLUS 2 2 10 Z←L PLUS RZ←L+R     "],
        "the second row padded"
    );
    assert_eq!(out(&mut d, "⍴⎕TF 'PLUS'"), ["33"]);
    define(&mut d, PRIMES);
    assert_eq!(out(&mut d, "⍴⎕TF 'PRIMES'"), ["99"]);
}

#[test]
fn transfer_form_of_scalars_negatives_and_fifteen_digits() {
    let mut d = in_mode(Mode::D);
    out(&mut d, "X←5");
    assert_eq!(out(&mut d, "⎕TF 'X'"), ["NX 0 5"]);
    out(&mut d, "V←¯1.5 2");
    assert_eq!(out(&mut d, "⎕TF 'V'"), ["NV 1 2 ¯1.5 2"]);
    out(&mut d, "PI←○1");
    assert_eq!(out(&mut d, "⎕TF 'PI'"), ["NPI 0 3.14159265358979"]);
    out(&mut d, "E←''");
    assert_eq!(out(&mut d, "⍴⎕TF 'E'"), ["7"], "CE 1 0 and its blank");
}

#[test]
fn no_transfer_form_for_what_is_not_there_or_locked() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "⍴⎕TF 'NOTHING'"), ["0"]);
    define(&mut d, &["∇SECRET", "1", "⍫"]);
    assert_eq!(out(&mut d, "⍴⎕TF 'SECRET'"), ["0"]);
}

#[test]
fn the_inverse_transfer_form_establishes_a_variable() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "⎕TF 'NNEW 1 3 1 2 ¯3'"), ["NEW"]);
    assert_eq!(out(&mut d, "NEW"), ["1 2 ¯3"]);
    assert_eq!(out(&mut d, "⎕TF 'CM 2 2 2 ABCD'"), ["M"]);
    assert_eq!(out(&mut d, "M"), ["AB", "CD"]);
    assert_eq!(out(&mut d, "⍴⎕TF 'NBAD 1 3 1 2'"), ["0"], "too few numbers");
    assert_eq!(out(&mut d, "⍴⎕TF 'QX 0 1'"), ["0"], "no such type");
    assert_eq!(out(&mut d, "⍴⎕TF 'N1X 0 1'"), ["0"], "not a name");
}

#[test]
fn the_inverse_transfer_form_establishes_a_function() {
    let mut d = in_mode(Mode::D);
    define(&mut d, PLUS);
    out(&mut d, "Z←⎕TF 'PLUS'");
    out(&mut d, ")ERASE PLUS");
    assert_eq!(out(&mut d, "⎕TF Z"), ["PLUS"]);
    assert_eq!(out(&mut d, "2 PLUS 3"), ["5"]);
}

#[test]
fn the_inverse_transfer_form_ignores_name_class_conflicts() {
    let mut d = in_mode(Mode::D);
    out(&mut d, "X←5");
    assert_eq!(out(&mut d, "⎕TF 'FX 2 2 3 Z←XZ←7'"), ["X"]);
    assert_eq!(out(&mut d, "⎕NC 'X'"), ["3"]);
    assert_eq!(out(&mut d, "X"), ["7"]);
    assert_eq!(out(&mut d, "⎕TF 'NX 0 4'"), ["X"]);
    assert_eq!(out(&mut d, "⎕NC 'X'"), ["2"]);
}

#[test]
fn system_variables_have_a_transfer_form() {
    let mut d = in_mode(Mode::D);
    assert_eq!(out(&mut d, "⎕TF '⎕IO'"), ["N⎕IO 0 1"]);
    assert_eq!(out(&mut d, "⎕TF 'N⎕IO 0 0'"), ["⎕IO"]);
    assert_eq!(out(&mut d, "⎕IO"), ["0"]);
}

#[test]
fn b_has_no_transfer_form() {
    let mut b = in_mode(Mode::B);
    out(&mut b, "X←5");
    assert!(out(&mut b, "⎕TF 'X'")[0].ends_with("ERROR"));
    assert_eq!(out(&mut b, ")OUT T"), ["INCORRECT COMMAND"]);
    assert_eq!(out(&mut b, ")IN T"), ["INCORRECT COMMAND"]);
}

fn stocked(d: &mut Session) {
    out(d, "THIS←2 3⍴⍳6");
    out(d, "THAT←3 4⍴'ABCDEFGHIJKL'");
    out(d, "⎕IO←0");
    define(d, PLUS);
}

#[test]
fn out_writes_a_transfer_file_and_in_reads_it_back() {
    let mut d = in_mode(Mode::D);
    stocked(&mut d);
    let written = out(&mut d, ")OUT T");
    assert_eq!(written.len(), 1);
    assert!(!written[0].starts_with("SAVED"), "time and date only");
    let file = d.ws.store.read(0, "T.AIO").expect("T.AIO in library 0");
    assert!(
        file.lines().any(|l| l == "NTHIS 2 2 3 1 2 3 4 5 6"),
        "{file}"
    );
    assert!(file.lines().any(|l| l == "N⎕IO 0 0"), "{file}");
    out(&mut d, ")CLEAR");
    let read = out(&mut d, ")IN T");
    assert_eq!(read, [format!("SAVED {}", written[0])]);
    assert_eq!(out(&mut d, "THIS"), ["1 2 3", "4 5 6"]);
    assert_eq!(out(&mut d, "THAT"), ["ABCD", "EFGH", "IJKL"]);
    assert_eq!(out(&mut d, "2 PLUS 3"), ["5"]);
    assert_eq!(out(&mut d, "⎕IO"), ["0"]);
}

#[test]
fn in_and_out_take_the_objects_named() {
    let mut d = in_mode(Mode::D);
    stocked(&mut d);
    out(&mut d, ")OUT T THIS PLUS");
    out(&mut d, ")CLEAR");
    assert_eq!(
        out(&mut d, ")IN T PLUS")[0].split(' ').next(),
        Some("SAVED")
    );
    assert_eq!(out(&mut d, "⎕NC 'THIS'"), ["0"]);
    assert_eq!(out(&mut d, "2 PLUS 3"), ["5"]);
}

#[test]
fn objects_not_found_are_reported() {
    let mut d = in_mode(Mode::D);
    stocked(&mut d);
    assert_eq!(out(&mut d, ")OUT T THIS NOPE"), ["NOPE", "NOT FOUND"]);
    assert!(d.ws.store.read(0, "T.AIO").is_none(), "nothing written");
    out(&mut d, ")OUT T THIS");
    let read = out(&mut d, ")IN T THIS NOPE");
    assert_eq!(&read[1..], ["NOPE", "NOT FOUND"]);
    assert_eq!(out(&mut d, ")IN NOFILE"), ["NOT FOUND"]);
}

#[test]
fn in_waits_for_a_clear_state_indicator() {
    let mut d = in_mode(Mode::D);
    stocked(&mut d);
    out(&mut d, ")OUT T");
    define(&mut d, &["∇BAD", "1÷0", "∇"]);
    out(&mut d, "BAD");
    assert_eq!(out(&mut d, ")IN T"), ["NOT WITH UNCLEAR SI"]);
}

#[test]
fn transfer_files_are_listed_apart_from_workspaces() {
    let mut d = in_mode(Mode::D);
    stocked(&mut d);
    out(&mut d, ")OUT T");
    assert!(out(&mut d, ")LIB").is_empty(), "not a workspace");
    assert_eq!(out(&mut d, ")LIB .AIO"), ["T"]);
    assert_eq!(out(&mut d, ")LIB 0 .AIO"), ["T"]);
    let file = d.ws.store.read(0, "T.AIO").expect("T.AIO");
    let mut a = in_mode(Mode::A);
    a.ws.store.write(0, "T.AIO", Some(&file)).expect("kept");
    assert!(out(&mut a, ")LIB").is_empty(), "nor in (A)");
    assert_eq!(out(&mut a, ")LOAD T"), ["WS NOT FOUND"]);
    assert_eq!(out(&mut d, ")DROP T.AIO").len(), 1);
    assert!(out(&mut d, ")LIB .AIO").is_empty());
}
