//! The shared channel: where a typed line is put, how big it is, and
//! where ATTN is raised.
//!
//! One `SharedArrayBuffer`, three Int32 slots and then bytes. The first
//! slot is what the worker sleeps on with `Atomics.wait` and the page
//! wakes it by storing into; the second is how many bytes of the body
//! the line filled; the third is attention. Frames come back the
//! ordinary way, by `postMessage`, because only the reading side has
//! to block.
//!
//! Attention is here and not in a message because a worker reads its
//! messages only when its thread is idle, and a run in progress never
//! is. Shared memory is the one thing the page can write that a busy
//! worker can see.

/// The state slot, which the worker sleeps on.
pub const STATE: u32 = 0;

/// Nothing to read: the worker waits here.
pub const WAITING: i32 = 0;

/// A line is in the body.
pub const READY: i32 = 1;

/// The page has gone. The read finds no line, which the reader
/// reports as INTERRUPT, and the session ends -- the same thing a
/// dropped socket does at the service.
pub const CLOSED: i32 = 2;

/// The length slot: how many bytes of the body the line filled.
pub const LENGTH: u32 = 1;

/// The attention slot: the page stores non-zero to stop a run, and
/// the interpreter swaps it back to zero when it notices. Not the
/// state slot, which the worker sleeps on -- a store there wakes a
/// read, and ATTN is not a line.
pub const ATTN: u32 = 2;

/// Where the line's bytes start, past the three Int32 slots.
pub const BODY: usize = 12;

/// How big the channel is. A typed line is one line at a terminal;
/// this is room for a very long one and no more.
pub const SIZE: usize = 64 * 1024;

/// Put a typed line in the body, and say how many bytes it took.
///
/// A line longer than the body is cut at a glyph, never through one:
/// the far end decodes what it is handed, and half a glyph is not
/// UTF-8. A 2741 could not type a line this long anyway.
#[must_use]
pub fn put(body: &mut [u8], line: &str) -> usize {
    let fits = line
        .char_indices()
        .map(|(at, c)| at + c.len_utf8())
        .take_while(|end| *end <= body.len())
        .last()
        .unwrap_or(0);
    body[..fits].copy_from_slice(&line.as_bytes()[..fits]);
    fits
}

/// The line the body holds. Bytes that are not UTF-8 cannot arrive
/// from a terminal, and are replaced rather than refused.
#[must_use]
pub fn take(body: &[u8], len: usize) -> String {
    String::from_utf8_lossy(&body[..len.min(body.len())]).into_owned()
}

/// The attention slot, from the worker's side: what the interpreter
/// polls while a run is going.
///
/// A view onto the same shared buffer the page writes. `asked` swaps
/// the slot back to zero, so one attention stops one run, and a stop
/// the page raised is answered by the run it was meant for.
#[cfg(target_arch = "wasm32")]
pub struct Attn(js_sys::Int32Array);

#[cfg(target_arch = "wasm32")]
impl Attn {
    /// The attention slot of the channel the page made.
    #[must_use]
    pub fn new(buffer: &wasm_bindgen::JsValue) -> Attn {
        Attn(js_sys::Int32Array::new_with_byte_offset_and_length(
            buffer,
            0,
            ATTN + 1,
        ))
    }
}

#[cfg(target_arch = "wasm32")]
impl apl_attn::Attention for Attn {
    fn asked(&self) -> bool {
        js_sys::Atomics::exchange(&self.0, ATTN, 0).is_ok_and(|was| was != 0)
    }
}
