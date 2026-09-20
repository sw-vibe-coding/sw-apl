//! Starting the session on the worker thread.

use std::path::PathBuf;

use apl_serve::serve;
use apl_session::QUOTA;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::link::Shared;

/// Hold a session on the shared channel until it ends.
///
/// This never returns while the session lasts, which is the point:
/// it is the worker's thread, and a statement that reads is allowed
/// to stop it. The page must therefore call this on a worker and
/// never on its own thread.
///
/// There is no filesystem under a browser, so `)SAVE` and `)LOAD`
/// report that there is nowhere to write. Giving them somewhere is
/// the next step.
#[wasm_bindgen]
pub fn start(channel: &wasm_bindgen::JsValue) {
    console_error_panic_hook::set_once();
    let link = Shared::new(channel);
    let ended = serve(Box::new(link), (QUOTA, PathBuf::from("/")));
    if let Err(error) = ended {
        web_sys::console::error_1(&format!("sw-apl: the session ended: {error}").into());
    }
}
