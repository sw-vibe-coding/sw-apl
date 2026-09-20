//! One store, two implementations, one set of expectations.
//!
//! The filesystem is what the CLI and the service use and a browser
//! has not got; memory is what a browser uses and what these tests
//! can run without a disc. Everything above the trait works on text,
//! so whatever the two agree on is what the interpreter sees, and
//! that is exactly what is written here: each test is run against
//! both.

use std::path::PathBuf;

use apl_store::{Files, Memory, Store};

/// A directory of this test's own, so two tests cannot tread on each
/// other's libraries.
fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("apl-store-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// The two stores under test: a temporary directory, and memory.
fn both(tag: &str) -> Vec<Box<dyn Store>> {
    vec![Box::new(Files(scratch(tag))), Box::new(Memory::default())]
}

#[test]
fn a_store_reads_back_what_it_was_written() {
    for mut store in both("read-back") {
        assert_eq!(store.write(0, "SIEVE", Some("⍝ ws\n")), Ok(()));
        assert_eq!(store.read(0, "SIEVE"), Some("⍝ ws\n".to_string()));
    }
}

#[test]
fn a_name_that_was_never_written_is_not_there() {
    for store in both("absent") {
        assert_eq!(store.read(0, "SIEVE"), None);
    }
}

#[test]
fn writing_a_name_again_replaces_what_was_there() {
    for mut store in both("replace") {
        let _ = store.write(0, "SIEVE", Some("first"));
        let _ = store.write(0, "SIEVE", Some("second"));
        assert_eq!(store.read(0, "SIEVE"), Some("second".to_string()));
    }
}

#[test]
fn writing_nothing_forgets_the_workspace() {
    for mut store in both("forget") {
        let _ = store.write(0, "SIEVE", Some("⍝ ws\n"));
        assert_eq!(store.write(0, "SIEVE", None), Ok(()));
        assert_eq!(store.read(0, "SIEVE"), None);
    }
}

#[test]
fn forgetting_one_that_is_not_there_is_a_failure() {
    for mut store in both("forget-absent") {
        assert!(store.write(0, "SIEVE", None).is_err());
    }
}

#[test]
fn a_library_lists_its_names_in_order() {
    for mut store in both("list") {
        for name in ["RACE", "EDIT", "LIFE"] {
            let _ = store.write(0, name, Some("⍝ ws\n"));
        }
        assert_eq!(store.list(0), ["EDIT", "LIFE", "RACE"]);
    }
}

#[test]
fn a_library_never_written_to_lists_nothing() {
    for store in both("list-empty") {
        assert!(store.list(0).is_empty());
    }
}

#[test]
fn there_is_no_library_past_the_two() {
    for mut store in both("no-such") {
        assert_eq!(store.read(2, "SIEVE"), None);
        assert!(store.list(2).is_empty());
        assert!(store.write(2, "SIEVE", Some("⍝ ws\n")).is_err());
    }
}

#[test]
fn library_one_is_separate_from_library_zero() {
    for mut store in both("separate") {
        let _ = store.write(0, "SIEVE", Some("⍝ ws\n"));
        assert!(store.list(1).is_empty());
        assert_eq!(store.read(1, "SIEVE"), None);
    }
}

#[test]
fn memory_refuses_to_write_the_shipped_library() {
    let mut store = Memory::default();
    assert!(store.write(1, "LIFE", Some("⍝ ws\n")).is_err());
}

#[test]
fn memory_reads_the_shipped_library_it_was_given() {
    let mut store = Memory::default();
    store
        .lib1
        .insert("LIFE".to_string(), "⍝ life\n".to_string());
    assert_eq!(store.read(1, "LIFE"), Some("⍝ life\n".to_string()));
    assert_eq!(store.list(1), ["LIFE"]);
}

#[test]
fn memory_tells_the_host_whenever_library_zero_changes() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static KEPT: AtomicUsize = AtomicUsize::new(0);
    let mut store = Memory {
        kept: Some(|shelf| {
            KEPT.fetch_add(shelf.len(), Ordering::SeqCst);
        }),
        ..Memory::default()
    };
    let _ = store.write(0, "SIEVE", Some("⍝ ws\n"));
    let _ = store.write(0, "OTHER", Some("⍝ ws\n"));
    let _ = store.write(0, "SIEVE", None);
    // One, then two, then one again: the hook sees the shelf as it
    // stands after each change, and a change that failed is not one.
    let _ = store.write(0, "GONE", None);
    assert_eq!(KEPT.load(Ordering::SeqCst), 4);
}

#[test]
fn files_keeps_a_workspace_where_a_reader_can_find_it() {
    let dir = scratch("on-disc");
    let mut store = Files(dir.clone());
    let _ = store.write(0, "SIEVE", Some("⍝ ws\n"));
    let path = dir.join("work").join("SIEVE.apl.ws");
    assert_eq!(
        std::fs::read_to_string(path).ok(),
        Some("⍝ ws\n".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn files_reads_the_shipped_library_from_ws_lib1() {
    let dir = scratch("lib1");
    std::fs::create_dir_all(dir.join("ws/lib1")).expect("the library");
    std::fs::write(dir.join("ws/lib1/LIFE.apl.ws"), "⍝ life\n").expect("the workspace");
    let store = Files(dir.clone());
    assert_eq!(store.read(1, "LIFE"), Some("⍝ life\n".to_string()));
    assert_eq!(store.list(1), ["LIFE"]);
    let _ = std::fs::remove_dir_all(&dir);
}
