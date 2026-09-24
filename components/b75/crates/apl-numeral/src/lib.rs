//! A number as the digits it is written with: rounded half to even
//! from its shortest decimal form, and laid out in decimal form to a
//! number of places or in scaled form to a number of digits. What (B)
//! '75's dyadic format puts in each field; see `mode-b.md`, *Format as
//! built*.

mod digits;
mod forms;

pub use forms::layout;
