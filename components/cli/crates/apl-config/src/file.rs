//! The configuration file, where it is looked for, and the flags
//! laid over it.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use apl_clock::TimeSource;

use crate::spec::{LibrarySpec, check, parse_clock};

/// The file's shape: a list of libraries.
#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    library: Vec<Entry>,
    /// `none` or `hardware`, as `--clock` takes it.
    clock: Option<String>,
}

/// One `[[library]]` table.
#[derive(Deserialize)]
struct Entry {
    number: usize,
    name: String,
    path: PathBuf,
}

/// The libraries a file lists. A relative path is from the file's own
/// directory, so a file can travel with what it names.
///
/// # Errors
/// A file that cannot be read or parsed, or a library it may not name.
pub fn from_file(file: &Path) -> Result<Vec<LibrarySpec>, String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let config: Config = toml::from_str(&text).map_err(|e| format!("{}: {e}", file.display()))?;
    let base = file.parent().unwrap_or(Path::new("."));
    config
        .library
        .into_iter()
        .map(|e| {
            let name = e.name.to_ascii_uppercase();
            check(e.number, &name)?;
            Ok(LibrarySpec {
                number: e.number,
                name,
                path: base.join(e.path),
            })
        })
        .collect()
}

/// The libraries to add: the file's -- `given`, or the first of
/// `./sw-apl.toml` and the user's `sw-apl/config.toml` that is there --
/// with `flags` laid over them, a flag winning for its number.
///
/// # Errors
/// A file named with `--config` that is not there, and anything
/// `from_file` refuses.
pub fn configured(given: Option<&Path>, flags: &[LibrarySpec]) -> Result<Vec<LibrarySpec>, String> {
    let file = match given {
        Some(f) if !f.is_file() => return Err(format!("{}: no such file", f.display())),
        Some(f) => Some(f.to_path_buf()),
        None => found(),
    };
    let mut all = file.map(|f| from_file(&f)).transpose()?.unwrap_or_default();
    all.retain(|l| !flags.iter().any(|f| f.number == l.number));
    all.extend(flags.iter().cloned());
    all.sort_by_key(|l| l.number);
    Ok(all)
}

/// The clock a session is given: `flag` when `--clock` was given, else
/// the configuration file's `clock` key -- `given`, or the first file
/// there is -- else none.
///
/// # Errors
/// A file that cannot be read or parsed, or a clock it may not name.
pub fn clock(given: Option<&Path>, flag: Option<TimeSource>) -> Result<TimeSource, String> {
    if let Some(flag) = flag {
        return Ok(flag);
    }
    let Some(file) = given.map(Path::to_path_buf).or_else(found) else {
        return Ok(TimeSource::None);
    };
    let text = std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
    let config: Config = toml::from_str(&text).map_err(|e| format!("{}: {e}", file.display()))?;
    let named = config.clock.as_deref().map(parse_clock).transpose();
    Ok(named
        .map_err(|e| format!("{}: {e}", file.display()))?
        .unwrap_or_default())
}

/// The first configuration file there is, where none was named.
fn found() -> Option<PathBuf> {
    let here = PathBuf::from("sw-apl.toml");
    let home = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")));
    let user = home.map(|h| h.join("sw-apl/config.toml"));
    [Some(here), user]
        .into_iter()
        .flatten()
        .find(|p| p.is_file())
}
