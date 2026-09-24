//! Configuring libraries beyond 0 and 1: a flag, a file, and which
//! wins. The owner's workspaces repository is library 2 by
//! convention; tests/libs/extended stands in for it.

use std::path::PathBuf;

use apl_config::{LibrarySpec, configured, from_file, parse_lib};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn spec(number: usize, name: &str, path: &str) -> LibrarySpec {
    LibrarySpec {
        number,
        name: name.to_string(),
        path: PathBuf::from(path),
    }
}

#[test]
fn a_flag_names_a_number_a_directory_and_a_name() {
    assert_eq!(
        parse_lib("2=../sw-apl-workspaces/ws,EXTENDED"),
        Ok(spec(2, "EXTENDED", "../sw-apl-workspaces/ws"))
    );
    assert_eq!(
        parse_lib("3=/tmp/x"),
        Ok(spec(3, "LIB3", "/tmp/x")),
        "a name is optional"
    );
    assert_eq!(
        parse_lib("2=dir,extended").map(|s| s.name),
        Ok("EXTENDED".to_string())
    );
}

#[test]
fn a_flag_that_is_not_a_library_is_refused() {
    for bad in [
        "2",
        "=dir",
        "x=dir",
        "0=dir",
        "1=dir",
        "2=",
        "2=dir,",
        "2=dir,BAD NAME",
    ] {
        assert!(parse_lib(bad).is_err(), "{bad}");
    }
}

#[test]
fn a_file_lists_libraries_and_a_relative_path_is_from_the_file() {
    let dir = std::env::temp_dir().join(format!("apl-config-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("sw-apl.toml");
    std::fs::write(&file, "[[library]]\nnumber = 2\nname = \"EXTENDED\"\npath = \"ws\"\n\n[[library]]\nnumber = 4\nname = \"MINE\"\npath = \"/abs/lib\"\n").unwrap();
    let got = from_file(&file).unwrap();
    assert_eq!(
        got,
        vec![
            spec(2, "EXTENDED", &dir.join("ws").to_string_lossy()),
            spec(4, "MINE", "/abs/lib")
        ]
    );
    std::fs::write(
        &file,
        "[[library]]\nnumber = 1\nname = \"X\"\npath = \"ws\"\n",
    )
    .unwrap();
    assert!(from_file(&file).is_err(), "library 1 is sw-apl's own");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_flag_wins_over_the_file_for_the_same_number() {
    let dir = std::env::temp_dir().join(format!("apl-config-win-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("given.toml");
    std::fs::write(&file, "[[library]]\nnumber = 2\nname = \"FROMFILE\"\npath = \"/f\"\n[[library]]\nnumber = 3\nname = \"KEPT\"\npath = \"/k\"\n").unwrap();
    let flags = vec![spec(2, "FROMFLAG", "/g")];
    let got = configured(Some(&file), &flags).unwrap();
    assert_eq!(got, vec![spec(2, "FROMFLAG", "/g"), spec(3, "KEPT", "/k")]);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_named_config_file_that_is_not_there_is_an_error() {
    assert!(configured(Some(&root().join("no-such.toml")), &[]).is_err());
}
