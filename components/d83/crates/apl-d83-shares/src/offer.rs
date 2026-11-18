//! Offering a name, asking how far it is shared, and retracting it.

use crate::shares::{Share, Shares};

impl Shares {
    /// `p ⎕SVO name`, offered `depth` calls down. A new offer reports
    /// 1, offered and not yet matched, as it did on the PC whether or
    /// not the processor was there; one the processor takes is 2 when
    /// next asked. A name already offered is only asked about.
    pub fn offer(&mut self, p: i64, name: &str, depth: usize) -> u8 {
        if self.offers.contains_key(name) {
            return self.coupling(name);
        }
        let partner = self.processors.iter_mut().position(|x| x.accepts(p, name));
        let share = Share {
            partner,
            depth,
            ours: [false; 4],
        };
        self.offers.insert(name.to_string(), share);
        1
    }

    /// `⎕SVO name`: 0 not offered, 1 offered, 2 shared.
    #[must_use]
    pub fn coupling(&self, name: &str) -> u8 {
        self.offers
            .get(name)
            .map_or(0, |s| 1 + u8::from(s.partner.is_some()))
    }

    /// `⎕SVR name`: end the sharing, and say how far it had got. The
    /// variable keeps the value it last had, as an ordinary one.
    pub fn retract(&mut self, name: &str) -> u8 {
        let coupling = self.coupling(name);
        let partner = self.offers.remove(name).and_then(|s| s.partner);
        if let Some(p) = partner.and_then(|i| self.processors.get_mut(i)) {
            p.retract(name);
        }
        coupling
    }

    /// Retract every name, as loading or clearing a workspace does.
    pub fn retract_all(&mut self) {
        let names: Vec<String> = self.offers.keys().cloned().collect();
        for name in names {
            self.retract(&name);
        }
    }
}
