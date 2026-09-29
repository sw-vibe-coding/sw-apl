//! Convert Unix time fields without assuming the platform's integer width.

pub(crate) fn ticks(seconds: impl Into<i64>, micros: impl Into<i64>) -> i64 {
    seconds.into() * 60 + micros.into() * 60 / 1_000_000
}

#[cfg(test)]
mod tests {
    use super::ticks;

    #[test]
    fn both_unix_widths_preserve_seconds_and_fractional_ticks() {
        assert_eq!(ticks(2_i32, 500_000_i32), 150);
        assert_eq!(ticks(2_i64, 500_000_i64), 150);
        assert_eq!(ticks(i32::MAX, 999_999_i32), i64::from(i32::MAX) * 60 + 59);
    }
}
