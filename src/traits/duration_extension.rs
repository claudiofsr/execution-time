use crate::{RoundFloat, Time};
use std::time::Duration;

// Constants for seconds in a day, an hour, and a minute to improve readability and performance.
const SECONDS_IN_DAY: f64 = 86400.0;
const SECONDS_IN_HOUR: f64 = 3600.0;
const SECONDS_IN_MINUTE: f64 = 60.0;

/// Trait to extend the `Duration` type with a method to convert it to a `Time` struct.
pub trait DurationExtension {
    /// Converts a `Duration` into a `Time` struct.
    fn get_time(&self) -> Time;
}

impl DurationExtension for Duration {
    fn get_time(&self) -> Time {
        let all_seconds: f64 = self.as_secs_f64();

        let remaining_day = all_seconds % SECONDS_IN_DAY;
        let remaining_hour = remaining_day % SECONDS_IN_HOUR;

        let days = (all_seconds / SECONDS_IN_DAY).floor() as u64;
        let hours = (remaining_day / SECONDS_IN_HOUR).floor() as u8;
        let minutes = (remaining_hour / SECONDS_IN_MINUTE).floor() as u8;
        let seconds = (remaining_hour % SECONDS_IN_MINUTE).round_float(9);

        Time {
            days,
            hours,
            minutes,
            seconds,
        }
    }
}
