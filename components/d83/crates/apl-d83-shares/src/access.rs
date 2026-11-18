//! Access control, a set passed to the partner, and where a name was
//! offered.

use apl_value::Array;

use crate::shares::{Processor, Shares};

impl Shares {
    /// A session with these processors loaded, as the PC's were named
    /// on the DOS command line that started APL.
    #[must_use]
    pub fn with(processors: Vec<Box<dyn Processor>>) -> Self {
        Shares {
            processors,
            ..Shares::default()
        }
    }

    /// `⎕SVC name`, or `setting ⎕SVC name` when `setting` is given:
    /// the access control vector in force. The processors set none of
    /// their own, so it is the APL side's. A name not offered has
    /// none, and takes none.
    pub fn control(&mut self, name: &str, setting: Option<[bool; 4]>) -> [bool; 4] {
        let Some(share) = self.offers.get_mut(name) else {
            return [false; 4];
        };
        if let Some(setting) = setting {
            share.ours = setting;
        }
        share.ours
    }

    /// The APL side set `name` to `value`: when it is shared, the
    /// partner's answer, each a name and its new value. What the
    /// partner shows now, if it changed, is left in `picture`.
    pub fn assigned(&mut self, name: &str, value: &Array) -> Vec<(String, Array)> {
        let partner = self.offers.get(name).and_then(|s| s.partner);
        let Some(p) = partner.and_then(|i| self.processors.get_mut(i)) else {
            return Vec::new();
        };
        let answer = p.set(name, value);
        self.picture = p.picture().or(self.picture.take());
        answer
    }

    /// How many calls down `name` was offered, while it is shared: a
    /// local of a function deeper than that is another variable.
    #[must_use]
    pub fn depth(&self, name: &str) -> Option<usize> {
        self.offers.get(name).map(|s| s.depth)
    }
}
