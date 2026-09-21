//! Starting the session on the worker thread, and the libraries it
//! saves into.
//!
//! A browser has no filesystem, and a worker cannot reach local
//! storage or wait for the page to reach it. So the libraries are
//! held in memory here and the page is told whenever library 0
//! changes: it keeps what it is told, and hands it back the next
//! time the tab is opened. Library 1 is baked into the bundle, which
//! is how a static host ships workspaces at all.
//!
//! What a browser keeps is that browser's. A workspace saved in one
//! is not in another, on the same machine or elsewhere; nothing
//! leaves the tab.

use apl_serve::serve;
use apl_session::{Memory, QUOTA, Shelf, Store};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DedicatedWorkerGlobalScope;

use crate::channel::Attn;
use crate::link::Shared;

/// Library 1: the workspaces sw-apl ships, read out of the
/// repository when the bundle is built.
const SHIPPED: [(&str, &str); 3] = [
    ("EDIT", include_str!("../../../../../ws/lib1/EDIT.apl.ws")),
    ("LIFE", include_str!("../../../../../ws/lib1/LIFE.apl.ws")),
    ("RACE", include_str!("../../../../../ws/lib1/RACE.apl.ws")),
];

/// Tell the page what library 0 holds now, as an object rather than
/// the string a frame is, which is how the page tells the two apart.
///
/// The names and the texts are set as properties rather than written
/// out as JSON here, so that nothing in a workspace has to be
/// escaped by hand. A page that has nowhere to keep them says so on
/// the paper; the session is not told, because the workspace is
/// saved either way and only its survival past the tab is in doubt.
fn kept(shelf: &Shelf) {
    let work = js_sys::Object::new();
    for (name, text) in shelf {
        let (name, text) = (JsValue::from_str(name), JsValue::from_str(text));
        let _ = js_sys::Reflect::set(&work, &name, &text);
    }
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&message, &JsValue::from_str("work"), &work);
    let page: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
    let _ = page.post_message(&message);
}

/// One field of the message the page sent, if it has one.
///
/// A message from an older page is not one: it is the channel
/// itself, with no fields at all. That reads as `None` here rather
/// than as a failure, which is what lets `start` take either.
fn field(message: &JsValue, name: &str) -> Option<JsValue> {
    let got = js_sys::Reflect::get(message, &JsValue::from_str(name)).ok()?;
    (!got.is_undefined() && !got.is_null()).then_some(got)
}

/// The libraries this session starts with: what sw-apl ships, and
/// what an earlier visit left behind. `stored` is the JSON the page
/// kept; anything else is an empty library 0.
fn libraries(stored: &str) -> Memory {
    let mut held = Memory {
        kept: Some(kept),
        ..Memory::default()
    };
    for (name, text) in SHIPPED {
        held.lib1.insert(name.to_string(), text.to_string());
    }
    let Ok(work) = js_sys::JSON::parse(stored) else {
        return held;
    };
    let Ok(work) = work.dyn_into::<js_sys::Object>() else {
        return held;
    };
    for pair in js_sys::Object::entries(&work).iter() {
        let pair: js_sys::Array = pair.unchecked_into();
        if let (Some(name), Some(text)) = (pair.get(0).as_string(), pair.get(1).as_string()) {
            held.work.insert(name, text);
        }
    }
    held
}

/// Hold a session on the shared channel until it ends.
///
/// This never returns while the session lasts, which is the point:
/// it is the worker's thread, and a statement that reads is allowed
/// to stop it. The page must therefore call this on a worker and
/// never on its own thread.
///
/// One argument, and it is the whole message the page posted. It was
/// two for one build, and a browser holding the worker from before
/// that build called it with one -- so the second arrived as
/// `undefined`, the session never started, and the page showed a
/// prompt that ignored typing. A message is one value and travels as
/// one; taking it apart here rather than at the call means the
/// worker has nothing to get wrong, and an older worker's call is
/// still a call this understands.
#[wasm_bindgen]
pub fn start(message: &JsValue) {
    console_error_panic_hook::set_once();
    let channel = field(message, "channel").unwrap_or_else(|| message.clone());
    let stored = field(message, "stored").and_then(|v| v.as_string());
    let link = Shared::new(&channel);
    // This session's attention is the channel's ATTN slot, which the
    // page can write while this thread is busy running. Installed on
    // this thread, which is the session's for as long as it lasts.
    apl_attn::attend(Box::new(Attn::new(&channel)));
    let store: Box<dyn Store> = Box::new(libraries(&stored.unwrap_or_default()));
    let ended = serve(Box::new(link), (QUOTA, store));
    if let Err(error) = ended {
        web_sys::console::error_1(&format!("sw-apl: the session ended: {error}").into());
    }
}
