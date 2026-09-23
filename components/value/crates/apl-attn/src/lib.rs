//! Attention: the flag that stops a run.
//!
//! On a 2741 the ATTN key stopped a running statement. Here it is a
//! flag the host sets and the interpreter reads, and it lives at the
//! bottom of the crate graph because the place a run spends its time
//! -- a reduction, a scan, an inner product -- is in the primitives,
//! which know nothing of sessions or terminals.
//!
//! Per session, by thread. Every session already owns a thread: the
//! process at the CLI, one per connection at the service, the worker
//! in a browser. So each installs its own flag on its own thread and
//! a primitive reads "this thread's", which is that session's, with
//! nothing passed down and no signature changed. A service holding
//! sixteen sessions can stop one of them.

mod attention;
mod flag;
mod interrupt;

pub use attention::{Attention, STRIDE, asked, attend, polled};
pub use flag::Flag;
pub use interrupt::catch_interrupt;
