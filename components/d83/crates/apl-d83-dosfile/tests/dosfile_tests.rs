//! AP210's files: a DOS file spec as the manual gives it (page 3-22),
//! `[d:]filename[.ext]`, kept in library 0 whatever the drive, a
//! record to a line.

use apl_d83_dosfile::{key, load, save};
use apl_d83_records::{Code, Record};
use apl_store::Memory;
use apl_value::{Array, Data, Number};

#[test]
fn a_file_spec_is_a_key_in_library_zero() {
    assert_eq!(key("B:FILE.EXT").as_deref(), Some("FILE.EXT.DOS"));
    assert_eq!(
        key("file").as_deref(),
        Some("FILE.DOS"),
        "upper case, as DOS has it"
    );
    assert_eq!(key("C:LONGERTHAN8.X"), None, "eight letters at most");
    assert_eq!(key("A.LONG"), None, "an extension of three at most");
    assert_eq!(key(""), None);
    assert_eq!(key("A B"), None);
}

#[test]
fn records_are_kept_and_read_back_whole() {
    let mut store = Memory::default();
    let text: Vec<char> = "TWO\nLINES \\ ONE".chars().collect();
    let records = vec![
        Some(Record {
            code: Code::Apl,
            value: Array::new(
                vec![2, 2],
                Data::Num(vec![
                    Number::Int(1),
                    Number::Float(0.5),
                    Number::Int(-3),
                    Number::Float(1e300),
                ]),
            )
            .unwrap(),
        }),
        None,
        Some(Record {
            code: Code::Chars,
            value: Array::new(vec![text.len()], Data::Char(text)).unwrap(),
        }),
        Some(Record {
            code: Code::Apl,
            value: Array::scalar(Number::Int(7)),
        }),
    ];
    save(&mut store, "F.DOS", &records).unwrap();
    assert_eq!(load(&store, "F.DOS"), Some(records));
    assert_eq!(load(&store, "G.DOS"), None, "no such file");
    save(&mut store, "E.DOS", &[]).unwrap();
    assert_eq!(
        load(&store, "E.DOS"),
        Some(Vec::new()),
        "an empty file is a file"
    );
}
