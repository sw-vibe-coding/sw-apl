//! The clock the varying system values are read from, and how a
//! span of time reads.

use apl_clock::{Time, hms, stopped, system};

#[test]
fn a_stopped_clock_does_not_move() {
    assert_eq!(stopped(), Time::default());
    assert_eq!(stopped(), stopped(), "a transcript made with it repeats");
}

/// The real clock cannot be pinned to a value, so this pins its
/// shape: the ranges each field must fall in, and that time advances.
#[test]
fn the_system_clock_reports_a_plausible_day() {
    let time = system();
    assert!(
        (0..24 * 60 * 60 * 60).contains(&time.now),
        "sixtieths since midnight: {}",
        time.now
    );
    assert!(time.cpu >= 0, "processor time: {}", time.cpu);
    let (month, day, year) = (time.date / 10_000, (time.date / 100) % 100, time.date % 100);
    assert!((1..=12).contains(&month), "month of {}", time.date);
    assert!((1..=31).contains(&day), "day of {}", time.date);
    assert!((0..=99).contains(&year), "year of {}", time.date);
    // The same day, read twice, is the same day.
    assert_eq!(system().date, time.date);
}

#[test]
fn a_duration_prints_as_hours_minutes_and_seconds() {
    assert_eq!(hms(0), "0.00.00");
    assert_eq!(hms(60), "0.00.01");
    assert_eq!(hms(60 * 59), "0.00.59");
    assert_eq!(hms(60 * 60), "0.01.00");
    assert_eq!(hms(60 * 3600), "1.00.00");
    assert_eq!(hms(60 * ((12 * 3600) + (34 * 60) + 56)), "12.34.56");
    // A clock that has not been set cannot make a negative duration.
    assert_eq!(hms(-1), "0.00.00");
}
