//! (D) '83's AP210, the DOS file processor, as the May 1983 APL
//! Reference gives it (pages 3-21 to 3-28): pairs of control and data
//! variables, the commands IR, IW, DL and RN, the read and write
//! subcommands, the four codes, and the return codes. A drive's files
//! are library 0's.

use apl_session::{Memory, Mode, Session};

fn session() -> Session {
    let mut s = Session::default();
    s.ws.mode = Mode::D;
    s.ws.store = Box::new(Memory::default());
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

/// A session with C1 and D1 shared with AP210.
fn shared() -> Session {
    let mut s = session();
    out(&mut s, "Z←210 ⎕SVO 2 2⍴'C1D1'");
    s
}

/// Set C1 and give back the return code.
fn code(s: &mut Session, command: &str) -> String {
    out(s, &format!("C1←{command}"));
    out(s, "C1").concat()
}

/// The manual's own example, page 3-27: a file written, read back out
/// of order, renamed and deleted.
#[test]
fn the_manuals_example() {
    let mut d = session();
    assert_eq!(out(&mut d, "210 ⎕SVO 2 2⍴'C1D1'"), ["1 1"]);
    assert_eq!(out(&mut d, "⎕SVO 2 2⍴'C1D1'"), ["2 2"]);
    assert_eq!(code(&mut d, "'IW,B:FILE.EXT'"), "0");
    out(&mut d, "D1←⍳10");
    assert_eq!(code(&mut d, "1"), "0");
    out(&mut d, "D1←2 3⍴⍳6");
    assert_eq!(code(&mut d, "1"), "0");
    assert_eq!(code(&mut d, "''"), "", "an empty vector closes the file");
    assert_eq!(code(&mut d, "'IR,B:FILE.EXT'"), "0");
    assert_eq!(code(&mut d, "0 1"), "0");
    assert_eq!(out(&mut d, "D1"), ["1 2 3", "4 5 6"]);
    assert_eq!(code(&mut d, "0 0"), "0");
    assert_eq!(out(&mut d, "D1"), ["1 2 3 4 5 6 7 8 9 10"]);
    out(&mut d, "C1←⍳0");
    assert_eq!(code(&mut d, "'RN,B:FILE.EXT,NEWFILE.XXX'"), "0");
    assert_eq!(code(&mut d, "'DL,B:NEWFILE.XXX'"), "0");
    assert_eq!(out(&mut d, "⎕SVR 2 2⍴'C1D1'"), ["2 2"]);
}

#[test]
fn up_to_four_pairs_named_c_and_d_alike() {
    let mut d = session();
    for pair in ["'CD'", "'C1D1'", "'CXJJDXJJ'"] {
        let names = pair.trim_matches('\'');
        let half = names.len() / 2;
        let offer = format!("210 ⎕SVO 2 {half}⍴{pair}");
        assert_eq!(out(&mut d, &offer), ["1 1"]);
        assert_eq!(out(&mut d, &offer[4..]), ["2 2"], "{pair}");
    }
    out(&mut d, "Z←210 ⎕SVO 2 2⍴'C4D4'");
    assert_eq!(out(&mut d, "⎕SVO 2 2⍴'C4D4'"), ["2 2"]);
    out(&mut d, "Z←210 ⎕SVO 2 2⍴'C5D5'");
    assert_eq!(
        out(&mut d, "⎕SVO 2 2⍴'C5D5'"),
        ["1 1"],
        "a fifth pair is not taken"
    );
    out(&mut d, "Z←210 ⎕SVO 'X1'");
    assert_eq!(out(&mut d, "⎕SVO 'X1'"), ["1"], "only C and D names");
}

#[test]
fn reading_goes_on_from_the_last_record_to_the_end() {
    let mut d = shared();
    code(&mut d, "'IW,SEQ'");
    for n in 1..=3 {
        out(&mut d, &format!("D1←{n}"));
        assert_eq!(code(&mut d, "1"), "0");
    }
    code(&mut d, "''");
    code(&mut d, "'IR,SEQ'");
    for n in 1..=3 {
        assert_eq!(code(&mut d, "0"), "0");
        assert_eq!(out(&mut d, "D1"), [n.to_string()]);
    }
    assert_eq!(code(&mut d, "0"), "1", "end of file");
    assert_eq!(out(&mut d, "⍴D1"), ["0"], "and an empty vector");
}

#[test]
fn the_return_codes() {
    let mut d = shared();
    assert_eq!(code(&mut d, "'IR,NOSUCH.DAT'"), "255");
    assert_eq!(code(&mut d, "'XX,FILE'"), "20");
    assert_eq!(code(&mut d, "1"), "20", "a number is no command");
    assert_eq!(code(&mut d, "'DL,NOSUCH.DAT'"), "255");
    code(&mut d, "'IW,F'");
    assert_eq!(code(&mut d, "2"), "21");
    assert_eq!(code(&mut d, "'IR,F'"), "21", "text is no subcommand");
    out(&mut d, "D1←100⍴0.5");
    assert_eq!(code(&mut d, "1"), "24", "800 bytes and a header in 128");
    assert_eq!(code(&mut d, "1 0 900"), "0", "but not in 900");
    code(&mut d, "''");
    code(&mut d, "'IR,F'");
    assert_eq!(code(&mut d, "1"), "21", "no writing a file opened to read");
    code(&mut d, "''");
    out(&mut d, "Z←210 ⎕SVO 'C2'");
    out(&mut d, "C2←'IW,G'");
    out(&mut d, "C2←1");
    assert_eq!(out(&mut d, "C2"), ["23"], "no data variable shared");
    assert_eq!(code(&mut d, "'RN,F,G'"), "255", "a duplicate name");
}

#[test]
fn ascii_chars_and_bool_records_fill_the_record_length() {
    let mut d = shared();
    code(&mut d, "'IW,TEXT.TXT,D'");
    out(&mut d, "D1←'HELLO'");
    assert_eq!(code(&mut d, "1 0 8"), "0");
    out(&mut d, "D1←'⍴'");
    assert_eq!(code(&mut d, "1"), "24", "no APL characters in ASCII");
    out(&mut d, "D1←'TOO LONG FOR IT'");
    assert_eq!(code(&mut d, "1"), "24");
    assert_eq!(code(&mut d, "0 0"), "0");
    assert_eq!(out(&mut d, "'[',D1,']'"), ["[HELLO   ]"]);
    code(&mut d, "''");
    code(&mut d, "'IW,BITS,B'");
    out(&mut d, "D1←1 0 1");
    assert_eq!(code(&mut d, "1 0 1"), "0");
    assert_eq!(code(&mut d, "0 0"), "0");
    assert_eq!(out(&mut d, "D1"), ["1 0 1 0 0 0 0 0"]);
    out(&mut d, "D1←2");
    assert_eq!(code(&mut d, "1"), "24", "only bits");
    code(&mut d, "''");
    code(&mut d, "'IR,BITS,A'");
    assert_eq!(code(&mut d, "0"), "24", "bits are no APL object");
    assert_eq!(out(&mut d, "⍴D1"), ["0"]);
}

#[test]
fn a_files_are_kept_in_library_zero() {
    let mut d = shared();
    code(&mut d, "'IW,A:KEEP.DAT'");
    out(&mut d, "D1←'KEPT'");
    code(&mut d, "1");
    code(&mut d, "''");
    assert!(d.ws.store.list(0).iter().any(|k| k.starts_with("KEEP.DAT")));
    out(&mut d, "⎕SVR 2 2⍴'C1D1'");
    out(&mut d, "Z←210 ⎕SVO 2 2⍴'C1D1'");
    assert_eq!(
        code(&mut d, "'IR,C:KEEP.DAT'"),
        "0",
        "every drive is library 0"
    );
    code(&mut d, "0");
    assert_eq!(out(&mut d, "D1"), ["KEPT"]);
}
