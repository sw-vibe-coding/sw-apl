//! Writing and reading what is in a field.

use apl_value::{Array, Data, Number};

use crate::screen::{Done, Screen};

impl Screen {
    /// Commands 2,f and 4,f: the characters given, into field `f` row
    /// by row, blank after them; more than it holds are cut off.
    pub fn write(&mut self, f: i64, given: &Array) -> Done {
        let (Data::Char(text), Some((spots, field))) = (&given.data, self.spots(f, false)) else {
            return (2, None);
        };
        let color = self.color(field[5]);
        let text = text.iter().copied().chain(std::iter::repeat(' '));
        for (s, c) in spots.into_iter().zip(text) {
            (self.cells[s], self.colors[s]) = (c, color);
        }
        self.changed = true;
        (0, None)
    }

    /// Commands 2,f,k and 4,f,k: ones and zeros into field `f`'s
    /// pixels, row by row -- written (k 1), or combined with what is
    /// there by and (2), or (3) or exclusive or (4). 5 on the
    /// monochrome display, which has none.
    pub fn blit(&mut self, f: i64, k: i64, given: &Array) -> Done {
        if self.display == 1 {
            return (5, None);
        }
        let bit = |n: &Number| match Number::from_f64(n.as_f64()) {
            Number::Int(b @ 0..=1) => Some(b == 1),
            _ => None,
        };
        let bits: Option<Vec<bool>> = match &given.data {
            Data::Num(ns) => ns.iter().map(bit).collect(),
            Data::Char(_) => None,
        };
        let (Some(bits), Some((spots, field))) = (bits, self.spots(f, true)) else {
            return (2, None);
        };
        let color = self.color(field[5]);
        for (s, b) in spots.into_iter().zip(bits) {
            self.pixels[s] = combined(k, self.pixels[s], b, color);
        }
        self.changed = true;
        (0, None)
    }

    /// Commands 5,f and 5,f,1: what field `f` holds, its characters or
    /// its pixels as ones and zeros, to give back in the data variable.
    #[must_use]
    pub fn read(&self, f: i64, pixels: bool) -> Done {
        if pixels && self.display == 1 {
            return (5, None);
        }
        let Some((spots, _)) = self.spots(f, pixels) else {
            return (2, None);
        };
        let back = if pixels {
            let lit = |s: &usize| Number::Int(i64::from(self.pixels[*s] > 0));
            Array::vector(spots.iter().map(lit).collect())
        } else {
            let text: Vec<char> = spots.iter().map(|&s| self.cells[s]).collect();
            Array {
                shape: vec![text.len()],
                data: Data::Char(text),
            }
        };
        (0, Some(back))
    }
}

/// A pixel that was `old`, after bit `b` by operation `k`. A pixel
/// newly lit, or written, takes the field's `color`; one lit already
/// keeps its own.
fn combined(k: i64, old: u8, b: bool, color: u8) -> u8 {
    let lit = match k {
        1 => b,
        2 => old > 0 && b,
        3 => old > 0 || b,
        _ => (old > 0) != b,
    };
    match (lit, old) {
        (false, _) => 0,
        (true, 0) => color,
        _ if k == 1 => color,
        _ => old,
    }
}
