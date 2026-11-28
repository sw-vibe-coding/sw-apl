//! AP210's records, as the May 1983 APL Reference gives them (pages
//! 3-23, 4-9 to 4-11): code A an APL object with its header, B bits,
//! C characters, D ASCII; what fits a record length, and what reads
//! back.

use apl_d83_records::{Code, bytes, read, write};
use apl_value::{Array, Data, Number};

fn ints(v: &[i64]) -> Array {
    Array::vector(v.iter().map(|&n| Number::Int(n)).collect())
}

fn text(s: &str) -> Array {
    let c: Vec<char> = s.chars().collect();
    Array::new(vec![c.len()], Data::Char(c)).unwrap()
}

#[test]
fn the_codes_are_letters() {
    let all = ["A", "b", "C", "d"].map(|l| Code::of(l).map(Code::letter));
    assert_eq!(all, [Some('A'), Some('B'), Some('C'), Some('D')]);
    assert_eq!(Code::of("E"), None);
}

#[test]
fn an_objects_bytes_are_its_header_its_shape_and_its_elements_rounded_even() {
    assert_eq!(bytes(&ints(&[1, 2, 3])), 8 + 2 + 6, "integers, two bytes");
    assert_eq!(
        bytes(&ints(&[1, 0, 1])),
        8 + 2 + 2,
        "bits, a byte, rounded even"
    );
    assert_eq!(
        bytes(&ints(&[40_000])),
        8 + 2 + 8,
        "too big for a word: real"
    );
    assert_eq!(bytes(&Array::scalar(Number::Float(0.5))), 8 + 8);
    assert_eq!(
        bytes(&text("ABC")),
        8 + 2 + 4,
        "characters, a byte, rounded even"
    );
}

#[test]
fn code_a_takes_an_object_that_fits() {
    let v = ints(&(1..=10).collect::<Vec<_>>());
    let r = write(Code::of("A").unwrap(), &v, 128).unwrap();
    assert_eq!(read(Code::of("A").unwrap(), &r, 128), Some(v.clone()));
    assert!(
        write(Code::of("A").unwrap(), &v, 20).is_none(),
        "30 bytes in 20"
    );
}

#[test]
fn codes_c_and_d_fill_the_record_with_blanks() {
    let d = Code::of("D").unwrap();
    let r = write(d, &text("HI"), 4).unwrap();
    assert_eq!(read(d, &r, 4), Some(text("HI  ")));
    assert!(
        write(d, &text("⍴"), 4).is_none(),
        "no APL characters in ASCII"
    );
    assert!(
        write(d, &text("HELLO"), 4).is_none(),
        "longer than the record"
    );
    assert!(write(d, &ints(&[1]), 4).is_none(), "only characters");
    let c = Code::of("C").unwrap();
    assert_eq!(
        read(c, &write(c, &text("⍴"), 2).unwrap(), 2),
        Some(text("⍴ "))
    );
}

#[test]
fn code_b_fills_the_record_with_zero_bits() {
    let b = Code::of("B").unwrap();
    let r = write(b, &ints(&[1, 1]), 1).unwrap();
    assert_eq!(read(b, &r, 1), Some(ints(&[1, 1, 0, 0, 0, 0, 0, 0])));
    assert!(write(b, &ints(&[2]), 1).is_none(), "only bits");
    assert!(write(b, &ints(&[1; 9]), 1).is_none(), "nine bits in a byte");
}

#[test]
fn a_record_read_as_another_code_is_a_data_type_error() {
    let r = write(Code::of("B").unwrap(), &ints(&[1]), 1).unwrap();
    assert_eq!(read(Code::of("A").unwrap(), &r, 1), None);
    let t = write(Code::of("C").unwrap(), &text("X"), 1).unwrap();
    assert_eq!(
        read(Code::of("D").unwrap(), &t, 1),
        Some(text("X")),
        "C and D are both text"
    );
}
