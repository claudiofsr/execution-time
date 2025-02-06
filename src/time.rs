use crate::{FormatFloatValue, FormatIntegerValue, Unit};

// Set a small margin of error for floating-point comparisons.
const EPSILON: f64 = 1e-10;

/// Represents a time duration split into days, hours, minutes, and seconds.
///
/// This struct holds the components of a time duration for formatting and display purposes.
#[derive(Debug, Default, PartialEq)]
pub struct Time {
    pub days: u64,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: f64,
}

impl Time {
    /// Formats the time duration into a human-readable string.
    ///
    /// This method combines the time components (days, hours, minutes, seconds) into a
    /// single, formatted string.  
    ///
    /// It includes only non-zero components, except for seconds, which are always included.
    ///
    /// ### Returns
    ///
    /// A formatted time string.
    pub fn format_time(&self) -> String {
        let mut parts = Vec::new();

        // Add days to the output if they are greater than 0.
        if self.days > 0 {
            parts.push(self.days.format_unit(Unit::Day));
        }

        // Add hours to the output if they are greater than 0, or if days have already been added.
        if self.hours > 0 || !parts.is_empty() {
            parts.push(self.hours.format_unit(Unit::Hour));
        }

        // Add minutes to the output if they are greater than 0, or if hours or days have already been added.
        if self.minutes > 0 || !parts.is_empty() {
            parts.push(self.minutes.format_unit(Unit::Minute));
        }

        // Determine the number of decimal places to use for seconds.
        let decimal: usize = self.calculate_decimal();

        // Always add seconds to the output.
        parts.push(self.seconds.format_float_unit(decimal, Unit::Second));

        parts.join(", ")
    }

    /// Calculates the appropriate number of decimal places for displaying seconds.
    ///
    /// This function determines the number of decimal places to show for the seconds
    /// value based on its magnitude. It aims to provide a balance between precision
    /// and readability.
    fn calculate_decimal(&self) -> usize {
        let sec = self.seconds;

        if sec < EPSILON {
            // Handles the case where 'sec' is approximately zero. Show one decimal place.
            1
        } else if sec >= 1.0 {
            // If seconds is greater than or equal to 1, show three decimal places.
            3
        } else if sec >= 0.001 {
            // If seconds is greater than or equal to 0.001, show six decimal places.
            6
        } else {
            // Otherwise, show nine decimal places for higher precision.
            9
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration_extension::DurationExtension;
    use std::time::Duration;

    #[test]
    fn times_new() {
        let duration = Duration::from_secs(86400 + 3600 + 60 + 1); // 1 day, 1 hour, 1 minute, 1 second
        let time = duration.get_time();
        assert_eq!(time.days, 1);
        assert_eq!(time.hours, 1);
        assert_eq!(time.minutes, 1);
        assert_eq!(time.seconds, 1.0);

        let duration = Duration::from_secs_f64(3661.5); // 1 hour, 1 minute, 1.5 seconds
        let time = duration.get_time();
        assert_eq!(time.days, 0);
        assert_eq!(time.hours, 1);
        assert_eq!(time.minutes, 1);
        assert_eq!(time.seconds, 1.5);

        let duration = Duration::from_secs(0);
        let time = duration.get_time();
        assert_eq!(time.days, 0);
        assert_eq!(time.hours, 0);
        assert_eq!(time.minutes, 0);
        assert_eq!(time.seconds, 0.0);

        let duration = Duration::from_secs_f64(2.5 * 86400.0);
        let time = duration.get_time();
        assert_eq!(time.days, 2);
        assert_eq!(time.hours, 12);
        assert_eq!(time.minutes, 0);
        assert_eq!(time.seconds, 0.0);
    }

    #[test]
    fn times_format() {
        let time = Time {
            days: 1,
            hours: 2,
            minutes: 3,
            seconds: 4.567002,
        };
        assert_eq!(
            time.format_time(),
            "1 day, 2 hours, 3 minutes, 4.567 seconds"
        );

        let time = Time {
            days: 0,
            hours: 2,
            minutes: 3,
            seconds: 4.567,
        };
        assert_eq!(time.format_time(), "2 hours, 3 minutes, 4.567 seconds");

        let time = Time {
            days: 0,
            hours: 0,
            minutes: 3,
            seconds: 4.567111,
        };
        assert_eq!(time.format_time(), "3 minutes, 4.567 seconds");

        let time = Time {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 4.567000444,
        };
        assert_eq!(time.format_time(), "4.567 seconds");

        let time = Time {
            days: 1,
            hours: 0,
            minutes: 0,
            seconds: 0.0,
        };

        assert_eq!(time.format_time(), "1 day, 0 hour, 0 minute, 0.0 second");

        let time = Time {
            days: 1,
            hours: 2,
            minutes: 0,
            seconds: 0.0,
        };
        assert_eq!(time.format_time(), "1 day, 2 hours, 0 minute, 0.0 second");
    }

    #[test]
    fn times_default() {
        let time = Time {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0.0,
        };
        let time_default = Time::default();

        assert_eq!(time, time_default);
        assert_eq!(time.format_time(), "0.0 second");
    }
}
