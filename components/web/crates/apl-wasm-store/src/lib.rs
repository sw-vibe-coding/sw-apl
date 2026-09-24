//! The libraries a browser session keeps. A browser has no
//! filesystem, and a worker cannot reach local storage or wait for
//! the page to reach it, so every library is held in memory here:
//! library 0 as the page kept it, told to the page whenever it
//! changes; library 1 baked into the bundle, which is how a static
//! host ships workspaces at all; and any library the page fetched
//! from a URL before the session started -- library 2, EXTENDED, by
//! convention -- read-only.
//!
//! What a browser keeps is that browser's. A workspace saved in one
//! is not in another, on the same machine or elsewhere.

#[cfg(target_arch = "wasm32")]
mod local;
#[cfg(target_arch = "wasm32")]
mod shipped;
#[cfg(target_arch = "wasm32")]
mod store;

#[cfg(target_arch = "wasm32")]
pub use store::store;
