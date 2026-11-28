//! The commands: a character vector set in the control variable while
//! no file is open.

use apl_d83_dosfile::{key, load, save};
use apl_d83_records::{Code, Record};
use apl_store::Store;
use apl_value::{Array, Data};

use crate::processor::Pair;

/// Return codes (page 3-26).
pub(crate) const COMMAND: i64 = 20;
const NOT_FOUND: i64 = 255;

/// A file open: under which key, whether to write, by which code, its
/// records, the record the next subcommand takes by default, and the
/// record length last used.
#[derive(Debug)]
pub(crate) struct Open {
    pub(crate) key: String,
    pub(crate) write: bool,
    pub(crate) code: Code,
    pub(crate) records: Vec<Option<Record>>,
    pub(crate) next: usize,
    pub(crate) length: usize,
}

/// Carry out the command `value`: IR, IW, DL or RN, the two letters, a
/// comma, and what they work on. Its return code.
pub(crate) fn command(pair: &mut Pair, value: &Array, store: &mut dyn Store) -> i64 {
    let Data::Char(c) = &value.data else {
        return COMMAND;
    };
    let text: String = c.iter().collect();
    let mut parts = text.split(',').map(str::trim);
    let (verb, first, second) = (parts.next().unwrap_or(""), parts.next(), parts.next());
    match (verb.to_ascii_uppercase().as_str(), first, parts.next()) {
        ("IR" | "IW", Some(spec), None) => {
            open(pair, (verb.eq_ignore_ascii_case("IW"), spec, second), store)
        }
        ("DL", Some(spec), None) if second.is_none() => remove(spec, store),
        ("RN", Some(spec), None) => second.map_or(COMMAND, |to| rename(spec, to, store)),
        _ => COMMAND,
    }
}

/// Open `spec` to read, or to read and write when `write`, by `code`
/// (A unless given); one opened to write is made when it is not there.
fn open(
    pair: &mut Pair,
    (write, spec, code): (bool, &str, Option<&str>),
    store: &mut dyn Store,
) -> i64 {
    let code = code.map_or(Some(Code::Apl), Code::of);
    let (Some(key), Some(code)) = (key(spec), code) else {
        return if code.is_none() { COMMAND } else { NOT_FOUND };
    };
    let records = match load(store, &key) {
        Some(r) => r,
        None if write && save(store, &key, &[]).is_ok() => Vec::new(),
        None => return NOT_FOUND,
    };
    let (next, length) = (0, 128);
    pair.open = Some(Open {
        key,
        write,
        code,
        records,
        next,
        length,
    });
    0
}

/// Delete `spec`.
fn remove(spec: &str, store: &mut dyn Store) -> i64 {
    let Some(key) = key(spec).filter(|k| load(store, k).is_some()) else {
        return NOT_FOUND;
    };
    store.write(0, &key, None).map_or(NOT_FOUND, |()| 0)
}

/// Rename `spec` to `to`, whose drive, if any, is ignored: there must
/// be the one and not the other.
fn rename(spec: &str, to: &str, store: &mut dyn Store) -> i64 {
    let to = to.rsplit(':').next().unwrap_or(to);
    let (Some(from), Some(to)) = (key(spec), key(to)) else {
        return NOT_FOUND;
    };
    let (Some(records), None) = (load(store, &from), load(store, &to)) else {
        return NOT_FOUND;
    };
    let moved = save(store, &to, &records).and_then(|()| store.write(0, &from, None));
    moved.map_or(NOT_FOUND, |()| 0)
}
