//! Libraries as directories of files, which is where the CLI and the
//! service keep them.
//!
//! This is the behaviour sw-apl has always had, moved behind the
//! trait and not otherwise altered: library 0 is `work/` under the
//! library root and library 1 is `ws/lib1/`, a workspace is a file
//! named for it, and library 0 is made when something is first
//! written to it rather than having to exist first.

use std::fs;
use std::path::PathBuf;

use crate::store::{SUFFIX, Store};

/// The directory the libraries sit under: what `--library` names.
#[derive(Debug, Clone)]
pub struct Files(pub PathBuf);

impl Files {
    /// The directory a library number names. `None` when there is no
    /// such library, which is a different thing from an empty one.
    fn at(&self, library: usize) -> Option<PathBuf> {
        match library {
            0 => Some(self.0.join("work")),
            1 => Some(self.0.join("ws/lib1")),
            _ => None,
        }
    }
}

impl Store for Files {
    fn read(&self, library: usize, name: &str) -> Option<String> {
        fs::read_to_string(self.at(library)?.join(format!("{name}{SUFFIX}"))).ok()
    }

    fn write(&mut self, library: usize, name: &str, text: Option<&str>) -> Result<(), String> {
        let dir = self.at(library).ok_or("there is no such library")?;
        let path = dir.join(format!("{name}{SUFFIX}"));
        let Some(text) = text else {
            return fs::remove_file(path).map_err(|e| e.to_string());
        };
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        fs::write(path, text).map_err(|e| e.to_string())
    }

    fn list(&self, library: usize) -> Vec<String> {
        let Some(dir) = self.at(library) else {
            return Vec::new();
        };
        let Ok(entries) = fs::read_dir(dir) else {
            return Vec::new();
        };
        let stem =
            |e: fs::DirEntry| Some(e.file_name().to_str()?.strip_suffix(SUFFIX)?.to_string());
        let mut found: Vec<String> = entries.filter_map(Result::ok).filter_map(stem).collect();
        found.sort();
        found
    }
}
