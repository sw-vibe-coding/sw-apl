//! A flag another thread can set, which is what a native host uses.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::attention::Attention;

/// An attention flag that may be set from any thread and read by the
/// session that installed it. Cloning shares it: the session keeps
/// one and whoever may interrupt it -- a signal handler, the thread
/// listening on a connection -- keeps another.
#[derive(Debug, Clone, Default)]
pub struct Flag(Arc<AtomicBool>);

impl Flag {
    /// Ask the session holding this flag to stop. Safe from a signal
    /// handler: it only stores.
    pub fn ask(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

impl Attention for Flag {
    fn asked(&self) -> bool {
        self.0.swap(false, Ordering::Relaxed)
    }
}
