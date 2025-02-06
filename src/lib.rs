mod time;
mod traits;

pub use self::{time::*, traits::*};
use std::time::{Duration, Instant};

/// Measures the execution time of a code block.
///
/// This struct provides methods to start a timer and print the elapsed time
/// in a user-friendly format.
pub struct ExecutionTime {
    start_time: Instant,
}

impl ExecutionTime {
    /// Starts a new stopwatch.
    ///
    /// This function initializes the timer by recording the current time.
    ///
    /// ### Examples
    ///
    /// ```
    /// use execution_time::ExecutionTime;
    ///
    /// let timer = ExecutionTime::start();
    /// // ... your code here ...
    /// timer.print_elapsed_time();
    /// ```
    pub fn start() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// Gets the elapsed time as a `Duration`.
    ///
    /// `Duration` represents a span of time composed of whole seconds
    /// and a fractional part represented in nanoseconds.
    pub fn get_duration(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Gets the elapsed time as a `Time` struct.
    ///
    /// This method converts the `Duration` into a `Time` struct, which
    /// represents the time in terms of days, hours, minutes, and seconds.
    pub fn get_time(&self) -> Time {
        self.get_duration().get_time()
    }

    /// Calculates the time elapsed since the timer was started and formats it as a string.
    ///
    /// This method calculates the elapsed time, formats it into a readable string,
    /// and includes both the formatted time and the raw `Duration` for debugging.
    pub fn get_elapsed_time(&self) -> String {
        let duration: Duration = self.get_duration();
        let time: Time = duration.get_time();
        format!("{} ({duration:?})", time.format_time())
    }

    /// Prints the time elapsed since the timer was started to the console.
    ///
    /// This method prints the formatted elapsed time to `stdout`.
    pub fn print_elapsed_time(&self) {
        println!("Elapsed time: {}", self.get_elapsed_time());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Error = Box<dyn std::error::Error>;

    #[test]
    fn basic_timing() {
        let timer = ExecutionTime::start();
        std::thread::sleep(Duration::from_nanos(50));
        let elapsed = timer.get_duration();
        assert!(elapsed >= Duration::from_nanos(45)); // Allow some margin
    }

    #[test]
    /// `cargo test -- --show-output main`
    fn main() -> Result<(), Error> {
        let timer = ExecutionTime::start();

        let duration = timer.get_duration();
        println!("duration: {duration:?}");

        let time = duration.get_time();
        println!("time: {time:?}");
        println!("time: {time:#?}");

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_nanosecond`
    fn elapsed_time_more_than_nanosecond() -> Result<(), Error> {
        let duration = Duration::new(0, 57); // 0_000_000_057
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0.000000057,
            }
        );

        assert_eq!(formatted_output, "0.000000057 second (57ns)");

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_microsecond`
    fn elapsed_time_more_than_microsecond() -> Result<(), Error> {
        let duration = Duration::new(0, 80_057); // 0_000_080_057
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0.000080057,
            }
        );

        assert_eq!(formatted_output, "0.000080057 second (80.057µs)");

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_millisecond`
    fn elapsed_time_more_than_millisecond() -> Result<(), Error> {
        let duration = Duration::new(0, 15_200_000); // 0_015_200_000
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0.015200,
            }
        );

        assert_eq!(formatted_output, "0.015200 second (15.2ms)");

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_second`
    fn elapsed_time_more_than_second() -> Result<(), Error> {
        let duration = Duration::new(5, 80_012_045); // 5_080_012_045
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 5.080012045,
            }
        );

        assert_eq!(formatted_output, "5.080 seconds (5.080012045s)");

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_minute`
    fn elapsed_time_more_than_minute() -> Result<(), Error> {
        let duration = Duration::new(65, 12_345); // 65_000_012_345
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 5.000012345,
            }
        );

        assert_eq!(formatted_output, "1 minute, 5.000 seconds (65.000012345s)");

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_hour`
    fn elapsed_time_more_than_hour() -> Result<(), Error> {
        let duration = Duration::new(3700, 56_891_730); // 3700.056891730
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 0,
                hours: 1,
                minutes: 1,
                seconds: 40.05689173,
            }
        );

        assert_eq!(
            formatted_output,
            "1 hour, 1 minute, 40.057 seconds (3700.05689173s)"
        );

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output elapsed_time_more_than_day`
    fn elapsed_time_more_than_day() -> Result<(), Error> {
        let seconds = 86400.0 + 2.0 * 3600.0 + 5.0 * 60.0 + 28.03;
        let duration = Duration::from_secs_f64(seconds); // 93928.0300
        let time = duration.get_time();

        let formatted_output = format!("{} ({duration:?})", time.format_time());

        println!("duration: {duration:?}");
        println!("time: {time:#?}");
        println!("formatted_output: {formatted_output}\n");

        assert_eq!(
            time,
            Time {
                days: 1,
                hours: 2,
                minutes: 5,
                seconds: 28.030000,
            }
        );

        assert_eq!(
            formatted_output,
            "1 day, 2 hours, 5 minutes, 28.030 seconds (93928.03s)"
        );

        Ok(())
    }
}
