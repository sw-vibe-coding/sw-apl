//! The display in use, what is on it, and where a field lies.

use apl_value::Array;
use serde_json::json;

/// Lines on either display.
pub const ROWS: usize = 25;

/// A return code as AP205 gives it, and what a read gives back.
pub type Done = (i64, Option<Array>);

/// The display in use, its fields, and what is on it.
#[derive(Debug, Default)]
pub struct Screen {
    /// 1 monochrome, 4 color 40, 8 color 80; 0 before the first switch.
    pub display: u8,
    /// Characters a line.
    pub cols: usize,
    /// Start row, start column, height, width, type, attribute.
    pub fields: Vec<[usize; 6]>,
    pub(crate) cells: Vec<char>,
    /// Each character's color, as a pixel's.
    pub(crate) colors: Vec<u8>,
    /// The color display's pixels, row by row: 0 dark, or a color.
    pub(crate) pixels: Vec<u8>,
    /// Whether it has changed since the last picture.
    pub changed: bool,
}

impl Screen {
    /// Switch to display `n`, 1, 4 or 8, and clear it. The fields stay.
    pub fn switch(&mut self, n: u8) -> Done {
        self.display = n;
        self.cols = if n == 4 { 40 } else { 80 };
        self.cells = vec![' '; ROWS * self.cols];
        self.colors = vec![0; ROWS * self.cols];
        self.pixels = if n == 1 {
            Vec::new()
        } else {
            vec![0; ROWS * 64 * self.cols]
        };
        self.changed = true;
        (0, None)
    }

    /// Where field `f` (from 1) lies -- the characters it covers, or its
    /// pixels, each row by row -- and its definition.
    #[must_use]
    pub fn spots(&self, f: i64, pixels: bool) -> Option<(Vec<usize>, [usize; 6])> {
        let field = *self.fields.get(usize::try_from(f).ok()?.checked_sub(1)?)?;
        let [row, col, high, wide, ..] = field;
        let (cols, k) = (self.cols, if pixels { 8 } else { 1 });
        let at = move |y: usize, x: usize| {
            let l = (row - 1 + y / k) * cols + col - 1 + x / k;
            let (down, across) = (l / cols, l % cols);
            if pixels {
                (down * 8 + y % 8) * cols * 8 + across * 8 + x % 8
            } else {
                l
            }
        };
        let spots: Vec<usize> = (0..high * k)
            .flat_map(|y| (0..wide * k).map(move |x| at(y, x)))
            .collect();
        let limit = if pixels {
            self.pixels.len()
        } else {
            self.cells.len()
        };
        spots.iter().all(|&s| s < limit).then_some((spots, field))
    }

    /// The color an attribute gives on the display in use.
    pub(crate) fn color(&self, attribute: usize) -> u8 {
        match self.display {
            4 => u8::try_from(attribute % 4).unwrap_or(0),
            _ => 3,
        }
    }

    /// What is on the screen, once it has changed, as JSON: the display,
    /// the pixels as digits, and each line's characters and colors.
    pub fn picture(&mut self) -> Option<String> {
        if !std::mem::take(&mut self.changed) {
            return None;
        }
        let digits = |cs: &[u8]| cs.iter().map(|c| char::from(b'0' + c)).collect::<String>();
        let cols = self.cols.max(1);
        let rows: Vec<String> = self
            .cells
            .chunks(cols)
            .map(|r| r.iter().collect())
            .collect();
        let colors: Vec<String> = self.colors.chunks(cols).map(digits).collect();
        let picture = json!({
            "display": self.display, "width": cols * 8, "height": ROWS * 8,
            "pixels": digits(&self.pixels), "rows": rows, "colors": colors,
        });
        Some(picture.to_string())
    }
}
