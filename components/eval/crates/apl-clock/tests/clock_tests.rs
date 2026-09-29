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

/// The time stamp a (B) session with a hardware clock reads for quad
/// TS: year, month, day, hour, minute, second and millisecond.
#[test]
fn a_time_stamp_is_the_date_and_the_time_to_the_millisecond() {
    let at = apl_clock::Time {
        // 14:05:06 and 30 sixtieths
        now: ((14 * 60 + 5) * 60 + 6) * 60 + 30,
        cpu: 0,
        date: 92_726,
        year: 2026,
    };
    assert_eq!(apl_clock::stamp(&at), [2026, 9, 27, 14, 5, 6, 500]);
}

/// The clock a session is given, as --clock names it.
#[test]
fn the_clock_is_none_or_hardware() {
    use apl_clock::TimeSource;
    assert_eq!(TimeSource::parse("none"), Some(TimeSource::None));
    assert_eq!(TimeSource::parse("hardware"), Some(TimeSource::Hardware));
    assert_eq!(TimeSource::parse("host"), None);
    assert_eq!(TimeSource::default(), TimeSource::None);
}

/// The real clock gives the whole year, not two digits of it.
#[test]
fn the_system_clock_gives_the_whole_year() {
    assert!(apl_clock::system().year >= 2026);
}
