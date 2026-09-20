//! The page, from the worker's side of the shared channel.
//!
//! This is where the browser session blocks. `recv` parks the worker
//! thread in `Atomics.wait` until the page stores a line and wakes
//! it; the page's own thread is never blocked, which is what makes
//! this legal at all -- `Atomics.wait` is forbidden on a browser's
//! main thread and free on a worker's.
//!
//! Only reading has to block, so frames go back the ordinary way, by
//! `postMessage`.

use std::io;

use apl_wire::{Frame, Link, text};
use js_sys::{Atomics, Int32Array, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DedicatedWorkerGlobalScope;

use crate::channel::{BODY, CLOSED, LENGTH, READY, STATE, WAITING, take};

/// The channel, and the page at the other end of it.
#[derive(Debug)]
pub struct Shared {
    header: Int32Array,
    body: Uint8Array,
    page: DedicatedWorkerGlobalScope,
}

/// What a browser says when it refuses, as an error the service
/// understands.
fn refused(error: &JsValue) -> io::Error {
    let said = error.as_string().unwrap_or_else(|| format!("{error:?}"));
    io::Error::other(said)
}

impl Shared {
    /// The worker's end of a `SharedArrayBuffer` the page made.
    #[must_use]
    pub fn new(buffer: &JsValue) -> Shared {
        Shared {
            header: Int32Array::new_with_byte_offset_and_length(buffer, 0, 2),
            body: Uint8Array::new_with_byte_offset(buffer, BODY as u32),
            page: js_sys::global().unchecked_into(),
        }
    }
}

impl Link for Shared {
    fn send(&mut self, frame: &Frame) -> io::Result<()> {
        let message = JsValue::from_str(&text(frame)?);
        self.page.post_message(&message).map_err(|e| refused(&e))
    }

    /// Sleep until the page has put a line in the channel. The wait
    /// is in a loop because `Atomics.wait` may return without one --
    /// and because the page may instead say it has gone, which is a
    /// read with no line to be had, and so an INTERRUPT.
    fn recv(&mut self) -> io::Result<Option<String>> {
        loop {
            match Atomics::load(&self.header, STATE).map_err(|e| refused(&e))? {
                CLOSED => return Ok(None),
                READY => break,
                _ => {
                    Atomics::wait(&self.header, STATE, WAITING).map_err(|e| refused(&e))?;
                }
            }
        }
        let len = Atomics::load(&self.header, LENGTH).map_err(|e| refused(&e))?;
        let len = usize::try_from(len).unwrap_or(0);
        let mut bytes = vec![0u8; len];
        self.body.subarray(0, len as u32).copy_to(&mut bytes);
        Atomics::store(&self.header, STATE, WAITING).map_err(|e| refused(&e))?;
        Ok(Some(take(&bytes, len)))
    }
}
