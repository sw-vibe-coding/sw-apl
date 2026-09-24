//! Starting the session on the worker thread. Its libraries are
//! `apl-wasm-store`'s: library 0 from the browser's storage, library
//! 1 baked into the bundle, and any the page fetched.

use apl_serve::serve;
use apl_session::{Host, Mode, QUOTA};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::channel::Attn;
use crate::link::Shared;

/// One field of the message the page sent, if it has one.
///
/// A message from an older page is not one: it is the channel
/// itself, with no fields at all. That reads as `None` here rather
/// than as a failure, which is what lets `start` take either.
fn field(message: &JsValue, name: &str) -> Option<JsValue> {
    let got = js_sys::Reflect::get(message, &JsValue::from_str(name)).ok()?;
    (!got.is_undefined() && !got.is_null()).then_some(got)
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
    // The page's tab names the mode; a page that names none is '70's.
    let mode = field(message, "mode").and_then(|v| v.as_string());
    let host = Host {
        quota: QUOTA,
        store: Box::new(apl_wasm_store::store(
            &stored.unwrap_or_default(),
            field(message, "libraries").as_ref(),
        )),
        mode: mode.as_deref().and_then(Mode::parse).unwrap_or_default(),
    };
    let ended = serve(Box::new(link), host);
    if let Err(error) = ended {
        web_sys::console::error_1(&format!("sw-apl: the session ended: {error}").into());
    }
}
