//! The processor, its two variables, and the screen it keeps.

use apl_d83_screen::Screen;
use apl_d83_shares::Processor;
use apl_store::Store;
use apl_value::{Array, Number};

/// AP205: the variables shared with it, what the data variable was
/// last set to, and the screen.
#[derive(Debug, Default)]
pub struct Ap205 {
    pub(crate) control: Option<String>,
    pub(crate) data: Option<String>,
    pub(crate) given: Option<Array>,
    pub(crate) screen: Screen,
}

impl Processor for Ap205 {
    /// Processor 205 takes one control variable and one data variable.
    fn accepts(&mut self, p: i64, name: &str) -> bool {
        let slot = match name.chars().next() {
            Some('C') => &mut self.control,
            Some('D') => &mut self.data,
            _ => return false,
        };
        let taken = p == 205 && slot.is_none();
        if taken {
            *slot = Some(name.to_string());
        }
        taken
    }

    /// A value in the data variable is kept for the next command; one
    /// in the control variable is a command, answered with its return
    /// code and, for a read, the data.
    fn set(&mut self, name: &str, value: &Array, _: &mut dyn Store) -> Vec<(String, Array)> {
        if self.data.as_deref() == Some(name) {
            self.given = Some(value.clone());
            return Vec::new();
        }
        let (code, back) = self.run(value);
        let mut reply = vec![(name.to_string(), Array::scalar(Number::Int(code)))];
        reply.extend(self.data.clone().zip(back));
        reply
    }

    fn retract(&mut self, name: &str) {
        if self.control.as_deref() == Some(name) {
            self.control = None;
        }
        if self.data.as_deref() == Some(name) {
            (self.data, self.given) = (None, None);
        }
    }

    fn picture(&mut self) -> Option<String> {
        self.screen.picture()
    }
}
