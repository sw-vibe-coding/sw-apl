//! (D) '83's screens, as AP205 keeps them (APL Reference, May 1983,
//! chapter 3). The screen is up to 50 rectangular fields, each six
//! numbers: start row and column (from 1), height and width in
//! characters, type (0 read-write, 2 read-only) and attribute. A field
//! of one line may run on into the lines after it. A field holds
//! characters, and on the color display pixels too, eight by eight a
//! character, row by row: in color 40 the screen is 320 by 200 and a
//! field's lit pixels take the color its attribute's residue of 4
//! names -- 0 invisible, 1 cyan, 2 red, 3 white; in color 80 it is 640
//! by 200, in white. Fields that overlap are drawn in order.

mod draw;
mod fields;
mod screen;

pub use screen::{Done, ROWS, Screen};
