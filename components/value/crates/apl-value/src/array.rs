//! `Array`: a shape plus flat row-major data. Rank 0 is a scalar.

use crate::error::{AplError, ErrorKind};
use crate::number::Number;

/// Element storage: numbers or characters, never nested.
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    /// Numeric elements.
    Num(Vec<Number>),
    /// Character elements.
    Char(Vec<char>),
}

impl Data {
    /// Number of elements.
    #[must_use]
    pub fn count(&self) -> usize {
        match self {
            Data::Num(v) => v.len(),
            Data::Char(v) => v.len(),
        }
    }
}

/// A flat APL array.
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    /// Length of each axis; empty for a scalar.
    pub shape: Vec<usize>,
    /// Row-major elements; length is the product of `shape`.
    pub data: Data,
}

impl Array {
    /// Build an array, checking that `data` fills `shape`.
    ///
    /// # Errors
    /// LENGTH ERROR when the element count does not match the shape.
    pub fn new(shape: Vec<usize>, data: Data) -> Result<Self, AplError> {
        if shape.iter().product::<usize>() != data.count() {
            return Err(AplError::new(ErrorKind::Length));
        }
        Ok(Array { shape, data })
    }

    /// A rank-0 numeric array.
    #[must_use]
    pub fn scalar(n: Number) -> Self {
        Array {
            shape: Vec::new(),
            data: Data::Num(vec![n]),
        }
    }

    /// A rank-1 numeric array.
    #[must_use]
    pub fn vector(v: Vec<Number>) -> Self {
        Array {
            shape: vec![v.len()],
            data: Data::Num(v),
        }
    }
}
