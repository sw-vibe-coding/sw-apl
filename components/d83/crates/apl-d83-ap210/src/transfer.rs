//! The subcommands: a numeric vector set in the control variable while
//! a file is open, `op [record [length]]`; an empty one closes it.

use apl_d83_dosfile::save;
use apl_d83_records::{read, write};
use apl_store::Store;
use apl_value::{Array, Data, Number};

use crate::processor::Pair;

/// Return codes (page 3-26).
const END: i64 = 1;
const SUBCOMMAND: i64 = 21;
const NO_DATA: i64 = 23;
const DATA_TYPE: i64 = 24;

/// Carry out `value` on the pair's open file: its return code, None
/// when the file was closed, and for a read what goes in the data
/// variable.
pub(crate) fn transfer(
    pair: &mut Pair,
    value: &Array,
    store: &mut dyn Store,
) -> (Option<i64>, Option<Array>) {
    let Some(numbers) = whole(value) else {
        return (Some(SUBCOMMAND), None);
    };
    let Some(open) = pair.open.as_mut() else {
        return (Some(SUBCOMMAND), None);
    };
    let [op, rest @ ..] = numbers.as_slice() else {
        pair.open = None;
        return (None, None);
    };
    let record = rest.first().copied().unwrap_or(open.next);
    open.length = rest.get(1).copied().unwrap_or(open.length);
    open.next = record + 1;
    match (*op, open.write) {
        (0, _) => reading(pair, record),
        (1, true) => (Some(writing(pair, record, store)), None),
        _ => (Some(SUBCOMMAND), None),
    }
}

/// Read record `record`: past the end, end of file and an empty vector.
fn reading(pair: &Pair, record: usize) -> (Option<i64>, Option<Array>) {
    let open = pair.open.as_ref().expect("open");
    let empty = Array::vector(Vec::new());
    let Some(Some(r)) = open.records.get(record) else {
        return (Some(END), Some(empty));
    };
    match read(open.code, r, open.length) {
        Some(v) => (Some(0), Some(v)),
        None => (Some(DATA_TYPE), Some(empty)),
    }
}

/// Write the data variable as record `record`, and keep the file.
fn writing(pair: &mut Pair, record: usize, store: &mut dyn Store) -> i64 {
    let Some(given) = pair.given.as_ref() else {
        return NO_DATA;
    };
    let open = pair.open.as_mut().expect("open");
    let Some(r) = write(open.code, given, open.length) else {
        return DATA_TYPE;
    };
    if open.records.len() <= record {
        open.records.resize(record + 1, None);
    }
    open.records[record] = Some(r);
    save(store, &open.key, &open.records).map_or(END, |()| 0)
}

/// `value` as whole numbers not below zero, or None.
fn whole(value: &Array) -> Option<Vec<usize>> {
    let Data::Num(v) = &value.data else {
        return (value.data.count() == 0).then(Vec::new);
    };
    let count = |n: &Number| match *n {
        Number::Int(i) => usize::try_from(i).ok(),
        Number::Float(_) => None,
    };
    v.iter().map(count).collect()
}
