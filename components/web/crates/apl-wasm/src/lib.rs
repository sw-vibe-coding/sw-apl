//! sw-apl in a browser: the session in a Web Worker, reached over a
//! shared channel.
//!
//! GitHub Pages is static, so a published demo has no service to dial
//! and the interpreter has to run in the browser. That is possible
//! here only because the blocking read has somewhere to block: this
//! is `apl-serve` on a worker thread, and `Link::recv` parks that
//! thread in `Atomics.wait` until the page posts a line.
//! `Console::read` stays synchronous, `Session` is untouched, and
//! `⎕`, `⍞` and the del editor read as they do everywhere else.
//!
//! The channel layout is ordinary Rust and is tested natively; the
//! rest of the crate exists only on wasm.

pub mod channel;

#[cfg(target_arch = "wasm32")]
mod link;
#[cfg(target_arch = "wasm32")]
mod run;

#[cfg(target_arch = "wasm32")]
pub use run::start;
