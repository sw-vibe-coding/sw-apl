//! What `)SAVE` writes and `)LOAD` reads back, and the one change a
//! call makes to it: a local name hiding a global.

use apl_console::Print;
use apl_prims::Env;
use apl_space::{Funcs, Groups, Vars};
use apl_value::Array;

use crate::activation::{Activation, Referent};

/// Everything `)SAVE` writes and `)LOAD` reads back: the symbol
/// table, the state indicator, and the settings kept beside them.
///
/// What the terminal adds is deliberately not here -- the console,
/// the clock, the moment the session signed on, and whatever the
/// current line has displayed. None of it can be written to a file
/// and read back, and a loaded workspace must not carry someone
/// else's terminal along with it.
/// Clonable so that a command which fills a workspace by running APL
/// -- `)LOAD`, `)COPY` -- can put the old one aside and give it back
/// if the new one does not fit.
#[derive(Debug, Default, Clone)]
pub struct Saved {
    /// Names that hold a value. A name holds a variable or a
    /// function, never both, which `define` and `set` keep true.
    pub vars: Vars,
    /// Names that hold a defined function.
    pub funcs: Funcs,
    /// Names that stand for a list of other names. A group is a
    /// handle for copying or erasing several things at once; its
    /// members need not exist, so this holds names, not referents.
    pub groups: Groups,
    /// The activation stack: running and stopped calls, outermost
    /// first. It is the state indicator.
    pub stack: Vec<Activation>,
    /// Index origin and random link. The link is saved so a loaded
    /// workspace carries on its sequence and a transcript that rolls
    /// still reproduces.
    pub env: Env,
    /// Print precision and width.
    pub print: Print,
    /// The workspace identifier `)WSID` reports. `None` until it is
    /// named, which the session shows as CLEAR WS.
    pub id: Option<String>,
    /// The latent expression, `⎕LX`: a line run when the workspace is
    /// loaded. Only (B) can set it, so a workspace that has one runs
    /// only there. `None` when it is empty.
    pub latent: Option<Array>,
}

impl Saved {
    /// Make `name` hold `referent`, and give back what it held -- a
    /// global a local is about to hide, or, on return, the local's
    /// value or a function fixed under the local name.
    pub fn swap(&mut self, name: &str, referent: Option<Referent>) -> Option<Referent> {
        if name.starts_with('⎕') {
            return Some(self.swap_setting(name, referent));
        }
        let function = self.funcs.remove(name).map(Referent::Function);
        let held = function.or_else(|| self.vars.remove(name).map(Referent::Value));
        match referent {
            Some(Referent::Value(value)) => self.vars.insert(name.to_string(), value).map(drop),
            Some(Referent::Function(f)) => self.funcs.insert(name.to_string(), f).map(drop),
            Some(Referent::Settings(..)) | None => None,
        };
        held
    }

    /// A setting made local. It keeps the value it had -- the 5110
    /// left it undefined until assigned, and reported IMPLICIT ERROR
    /// if it was used first; sw-apl, like APL2, does not -- and on
    /// return it is given back its value from `back`.
    fn swap_setting(&mut self, name: &str, back: Option<Referent>) -> Referent {
        if let Some(Referent::Settings(env, print)) = back {
            match name {
                "⎕IO" => self.env.io = env.io,
                "⎕CT" => self.env.ct = env.ct,
                "⎕RL" => self.env.link = env.link,
                "⎕PP" => self.print.digits = print.digits,
                "⎕PW" => self.print.width = print.width,
                _ => {}
            }
        }
        Referent::Settings(self.env.clone(), self.print)
    }

    /// This workspace as it would be with every call returned: the
    /// globals back in place of the locals that hid them, and the
    /// state indicator empty. It is what a saved file holds, since a
    /// file cannot hold a suspension, and it must not hold a local
    /// in place of the global it hid.
    #[must_use]
    pub fn unwound(&self) -> Saved {
        let mut saved = self.clone();
        while let Some(activation) = saved.stack.pop() {
            for (name, was) in activation.displaced {
                saved.swap(&name, was);
            }
        }
        saved
    }
}
