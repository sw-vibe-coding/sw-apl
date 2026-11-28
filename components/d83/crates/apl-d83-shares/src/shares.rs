//! The processors loaded, and the names shared with them.

use std::collections::BTreeMap;
use std::fmt::Debug;

use apl_store::Store;
use apl_value::Array;

/// An auxiliary processor: a partner a shared variable couples to.
/// It sets no access control of its own.
pub trait Processor: Debug {
    /// Whether it is processor `p` and takes `name`, offered to it now.
    fn accepts(&mut self, p: i64, name: &str) -> bool;
    /// The APL side set `name` to `value`: what it sets in reply,
    /// each a name it shares and a value. `store` is the libraries,
    /// for a processor that keeps files (AP210).
    fn set(&mut self, name: &str, value: &Array, store: &mut dyn Store) -> Vec<(String, Array)>;
    /// `name` is no longer shared with it.
    fn retract(&mut self, name: &str);
    /// What it shows, when that has changed since it was last asked:
    /// AP205's screen. None for a processor that shows nothing.
    fn picture(&mut self) -> Option<String> {
        None
    }
}

/// The processors this session has, and every name offered to one.
/// Session state: a workspace saved or loaded does not carry them.
#[derive(Debug, Default)]
pub struct Shares {
    pub(crate) processors: Vec<Box<dyn Processor>>,
    pub(crate) offers: BTreeMap<String, Share>,
    /// What a processor has shown since this was last taken.
    pub picture: Option<String>,
}

/// One name offered: which loaded processor took it, if one did, how
/// many calls down it was offered, and the APL side's access control.
#[derive(Debug, Clone)]
pub(crate) struct Share {
    pub(crate) partner: Option<usize>,
    pub(crate) depth: usize,
    pub(crate) ours: [bool; 4],
}
