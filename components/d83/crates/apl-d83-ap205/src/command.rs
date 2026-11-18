//! A command set in the control variable.

use apl_d83_screen::Done;
use apl_value::{Array, Data, Number};

use crate::processor::Ap205;

impl Ap205 {
    /// Run the command `value`: 1 for what is not one, 3 for one that
    /// needs the data variable when none is shared, 2 when it has been
    /// given nothing. The monochrome display is the one in use until a
    /// command switches.
    pub(crate) fn run(&mut self, value: &Array) -> Done {
        if self.screen.display == 0 {
            self.screen.switch(1);
        }
        let whole = |n: &Number| match Number::from_f64(n.as_f64()) {
            Number::Int(i) => Some(i),
            Number::Float(_) => None,
        };
        let cmd: Option<Vec<i64>> = match &value.data {
            Data::Num(ns) if value.shape.len() <= 1 => ns.iter().map(whole).collect(),
            _ => None,
        };
        let Some(cmd) = cmd else {
            return (1, None);
        };
        let needs = matches!(cmd.as_slice(), [1 | 2 | 4, ..] | [12]);
        match (needs, &self.data, &self.given) {
            (true, None, _) => (3, None),
            (true, _, None) => (2, None),
            _ => self.dispatch(&cmd),
        }
    }

    /// The commands that change the screen, and those that wait on
    /// the keyboard or move its cursor, by number.
    fn dispatch(&mut self, cmd: &[i64]) -> Done {
        let given = self
            .given
            .clone()
            .unwrap_or_else(|| Array::vector(Vec::new()));
        let screen = &mut self.screen;
        match cmd {
            [0] => screen.switch(screen.display),
            [0, n @ (1 | 4 | 8)] => screen.switch(u8::try_from(*n).unwrap_or(1)),
            [1] => screen.define(None, &given),
            [1, f] => screen.define(Some(*f), &given),
            [2 | 4, f] => screen.write(*f, &given),
            [2 | 4, f, k @ 1..=4] => screen.blit(*f, *k, &given),
            // No keyboard to wait on: Enter, at once, in field 1.
            [3 | 8] => (
                0,
                Some(Array::vector([0, 2, 1, 1, 1].map(Number::Int).to_vec())),
            ),
            [12] => (0, None),
            _ => self.inquire(cmd),
        }
    }

    /// The commands that read, and the rest: 1 for what is none.
    fn inquire(&self, cmd: &[i64]) -> Done {
        let int = |n: &usize| Number::Int(i64::try_from(*n).unwrap_or(0));
        let fields = &self.screen.fields;
        match cmd {
            [5, f] => self.screen.read(*f, false),
            [5, f, 1] => self.screen.read(*f, true),
            [9] => {
                let (shape, all) = (vec![fields.len(), 6], fields.iter().flatten());
                let data = Data::Num(all.map(int).collect());
                (0, Some(Array { shape, data }))
            }
            [9, f] => self.screen.spots(*f, false).map_or((2, None), |(_, row)| {
                (0, Some(Array::vector(row.iter().map(int).collect())))
            }),
            _ => (1, None),
        }
    }
}
