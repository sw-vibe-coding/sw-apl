//! Library 0, kept in the browser's own storage by the page.

use apl_session::Shelf;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DedicatedWorkerGlobalScope;

/// Tell the page what library 0 holds now, as an object rather than
/// the string a frame is, which is how the page tells the two apart.
///
/// The names and the texts are set as properties rather than written
/// out as JSON here, so that nothing in a workspace has to be
/// escaped by hand. A page that has nowhere to keep them says so on
/// the paper; the session is not told, because the workspace is
/// saved either way and only its survival past the tab is in doubt.
pub fn kept(shelf: &Shelf) {
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

/// A JavaScript object of key and text, as a shelf. Anything else is
/// an empty one.
#[must_use]
pub fn shelf(value: &JsValue) -> Shelf {
    let mut shelf = Shelf::new();
    let Some(object) = value.dyn_ref::<js_sys::Object>() else {
        return shelf;
    };
    for pair in js_sys::Object::entries(object).iter() {
        let pair: js_sys::Array = pair.unchecked_into();
        if let (Some(key), Some(text)) = (pair.get(0).as_string(), pair.get(1).as_string()) {
            shelf.insert(key, text);
        }
    }
    shelf
}
