//! Years across the strip.

/// The last year the axis marks; past it are the owner's own projects.
pub const NOW: i32 = 2026;

/// From `first` to `last`, `per_year` pixels to a year, with `margin`
/// pixels before the first and after the last.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    pub first: i32,
    pub last: i32,
    pub per_year: f64,
    pub margin: f64,
}

impl Scale {
    /// Where `year` is, from the strip's left edge.
    #[must_use]
    pub fn x(&self, year: f64) -> f64 {
        self.margin + (year - f64::from(self.first)) * self.per_year
    }

    /// The whole strip's width.
    #[must_use]
    pub fn width(&self) -> f64 {
        self.x(f64::from(self.last)) + self.margin
    }

    /// How far to scroll for `year` to open the view, 16 pixels in from
    /// its left edge -- or for a box whose left edge is `first`, if that
    /// reaches further left, to show whole.
    #[must_use]
    pub fn opening(&self, year: f64, first: Option<f64>) -> f64 {
        let x = self.x(year);
        (first.map_or(x, |left| left.min(x)) - 16.0).max(0.0)
    }

    /// Pixels to a year for `years` years to fill a view `wide`
    /// across, kept between a crowded strip and a sparse one.
    #[must_use]
    pub fn fitting(wide: f64, years: f64) -> f64 {
        (wide / years).clamp(140.0, 260.0)
    }
}
