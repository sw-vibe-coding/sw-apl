//! The dyadic grades of IBM PC APL, `L⍋R` and `L⍒R`: the items of a
//! character array put in order by a collating sequence (APL
//! Reference, May 1983, pages 7-46 to 7-48).

mod collate;

pub use collate::grade;
