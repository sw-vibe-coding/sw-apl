//! The processor and its pairs of variables.

use apl_d83_shares::Processor;
use apl_store::Store;
use apl_value::{Array, Number};

use crate::command::{Open, command};
use crate::transfer::transfer;

/// How many pairs AP210 takes at once.
const PAIRS: usize = 4;

/// AP210: the pairs shared with it.
#[derive(Debug, Default)]
pub struct Ap210 {
    pairs: Vec<Pair>,
}

/// One pair, by what follows the C and the D: which of the two are
/// shared, what the data variable was last set to, and the file open.
#[derive(Debug, Default)]
pub(crate) struct Pair {
    pub(crate) name: String,
    pub(crate) control: bool,
    pub(crate) data: bool,
    pub(crate) given: Option<Array>,
    pub(crate) open: Option<Open>,
}

impl Processor for Ap210 {
    /// Processor 210 takes a C or D name whose partner is free, while
    /// it has fewer than four pairs or the pair is begun.
    fn accepts(&mut self, p: i64, name: &str) -> bool {
        let Some((control, rest)) = split(name).filter(|_| p == 210) else {
            return false;
        };
        let at = self.pairs.iter().position(|q| q.name == rest);
        let at = match at {
            Some(i) => i,
            None if self.pairs.len() < PAIRS => {
                self.pairs.push(Pair {
                    name: rest.to_string(),
                    ..Pair::default()
                });
                self.pairs.len() - 1
            }
            None => return false,
        };
        let slot = if control {
            &mut self.pairs[at].control
        } else {
            &mut self.pairs[at].data
        };
        !std::mem::replace(slot, true)
    }

    /// A value in a data variable is kept for the next write; one in a
    /// control variable is a command or a subcommand, answered.
    fn set(&mut self, name: &str, value: &Array, store: &mut dyn Store) -> Vec<(String, Array)> {
        let Some((control, rest)) = split(name) else {
            return Vec::new();
        };
        let Some(pair) = self.pairs.iter_mut().find(|q| q.name == rest) else {
            return Vec::new();
        };
        if !control {
            pair.given = Some(value.clone());
            return Vec::new();
        }
        let (code, read) = match pair.open {
            Some(_) => transfer(pair, value, store),
            None => (Some(command(pair, value, store)), None),
        };
        let code = code.map(|c| (name.to_string(), Array::scalar(Number::Int(c))));
        let read = read.filter(|_| pair.data).map(|r| (format!("D{rest}"), r));
        code.into_iter().chain(read).collect()
    }

    fn retract(&mut self, name: &str) {
        let Some((control, rest)) = split(name) else {
            return;
        };
        if let Some(pair) = self.pairs.iter_mut().find(|q| q.name == rest) {
            if control {
                pair.control = false;
            } else {
                (pair.data, pair.given) = (false, None);
            }
        }
        self.pairs.retain(|q| q.control || q.data);
    }
}

/// Whether `name` is a control variable's, and what follows its C or D.
fn split(name: &str) -> Option<(bool, &str)> {
    match name.chars().next()? {
        'C' => Some((true, &name[1..])),
        'D' => Some((false, &name[1..])),
        _ => None,
    }
}
