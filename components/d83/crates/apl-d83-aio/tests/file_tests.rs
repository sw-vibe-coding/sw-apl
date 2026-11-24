//! Transfer files read: when they were written, and their objects.

use apl_d83_aio::contents;
use apl_d83_transfer::Object;

#[test]
fn a_file_read() {
    let text = "⍝!SAVED 1.02.03 11/22/26\nCX 1 3 A\nB\nNY 0 5\n";
    let got = contents(text);
    assert_eq!(got.when, "1.02.03 11/22/26");
    let names: Vec<&str> = got.objects.iter().map(Object::name).collect();
    assert_eq!(names, ["X", "Y"]);
}

#[test]
fn reading_stops_at_what_is_not_a_form() {
    let got = contents("NY 0 5\nrubbish\nNZ 0 6\n");
    assert_eq!(got.when, "");
    assert_eq!(got.objects.len(), 1);
}
