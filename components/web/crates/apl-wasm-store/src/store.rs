//! The session's store, from what the page sent.

use apl_session::{Added, Memory, Source};
use wasm_bindgen::{JsCast, JsValue};

use crate::local::{kept, shelf};
use crate::shipped::lib1;

/// The libraries a browser session starts with: library 0 from
/// `stored`, the JSON the page kept of an earlier visit; library 1 as
/// shipped; and `extra`, the libraries the page fetched -- an array of
/// objects with a number (as a string), a name, the URL they came from, and `work`,
/// their workspaces by key. Each added library is read-only.
#[must_use]
pub fn store(stored: &str, extra: Option<&JsValue>) -> Added {
    let work = js_sys::JSON::parse(stored)
        .map(|w| shelf(&w))
        .unwrap_or_default();
    let held = Memory {
        work,
        lib1: lib1(),
        kept: Some(kept),
    };
    let mut added = Added::new(Box::new(held));
    let libraries = extra.and_then(|e| e.dyn_ref::<js_sys::Array>().cloned());
    for library in libraries.iter().flat_map(js_sys::Array::iter) {
        let get = |k: &str| js_sys::Reflect::get(&library, &JsValue::from_str(k)).ok();
        let text = |k: &str| get(k).and_then(|v| v.as_string()).unwrap_or_default();
        let Ok(number) = text("number").parse::<usize>() else {
            continue;
        };
        let work = shelf(&get("work").unwrap_or(JsValue::NULL));
        added = added.with(number, &text("name"), &text("place"), Source::Kept(work));
    }
    added
}
