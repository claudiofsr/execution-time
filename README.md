# Execution Time

This Rust crate provides a simple and convenient way to measure and display the execution time of code blocks.
It allows you to start a timer and then print or retrieve the elapsed time in a human-readable format, including days, hours, minutes, and seconds.

`ExecutionTime` provides more user-friendly output than the default `std::time::Instant` alone.

## Features

*   **Easy to Use:** Simple API for starting the timer and getting the elapsed time.
*   **Human-Readable Output:** Formats the elapsed time into a string with days, hours, minutes, and seconds.
*   **Flexible:**  Provides methods to get the elapsed time as a formatted `String`, a `Duration`, or a custom `Time` struct.
*   **Accurate:** Uses `std::time::Instant` for precise time measurement.

## API Reference

### `ExecutionTime` Struct

```
struct ExecutionTime {
    start_time: Instant,
}
```

* `ExecutionTime::start()`: Creates and returns a new instance of `ExecutionTime`, starting the timer.
* `ExecutionTime::get_duration()`: Returns a std::time::Duration representing the elapsed time.
* `ExecutionTime::get_time()`: Returns a Time struct (defined in this crate) representing the elapsed time, broken down into days, hours, minutes, and seconds. 
This is useful for custom formatting.
* `ExecutionTime::get_elapsed_time()`: Returns a `String` containing the formatted elapsed time (e.g., "0.045123 second (45.123ms)").
* `ExecutionTime::print_elapsed_time()`: Prints the formatted elapsed time to the console.

### `Time` Struct

```
struct Time {
    days: u64,
    hours: u8,
    minutes: u8,
    seconds: f64,
}
```

The `Time` struct (returned by `ExecutionTime::get_time()`) has the following fields:

* `days: u64` - The number of days.
* `hours: u8` - The number of hours (0-23).
* `minutes: u8` - The number of minutes (0-59).
* `seconds: f64` - The number of seconds (including fractional seconds).

It also has the method:

* `format_time()`: Formats the Time struct into a human-readable string.

## Usage

1.  **Add the dependency** to your `Cargo.toml` file:

    ```toml
    [dependencies]
    execution-time = "0.3" # Or the latest version
    ```

2. **Import and Use** the library in your `main.rs` file (or any other Rust file):

   ```rust
   use execution_time::ExecutionTime;
   use std::thread;
   use std::time::Duration;

   fn main() {
       // Start the timer
       let timer = ExecutionTime::start();

       // Simulate some work being done. 
       // Replace this with the code you want to measure.
       thread::sleep(Duration::from_millis(123));

       // Option 1: Print the elapsed time directly
       timer.print_elapsed_time();

       // Option 2: Get the elapsed time as a formatted string and print it
       println!("Elapsed time: {}", timer.get_elapsed_time());

       // Option 3: Get the raw Duration
       println!("Duration: {:?}", timer.get_duration());

       // Option 4: Get the Time struct for custom formatting
       println!("Time: {:?}", timer.get_time());
   }
    ```

4. **Example Output**

    The output of `timer.print_elapsed_time()` (and similar methods) might look like this:

    ```
    // The actual output will vary depending on execution time.
    Elapsed time: 0.000123456 second (123.456 ms)
    ```

4. **Examples Tests**

    To run the examples and see the output for different elapsed times:

    1. Clone the repository and run the tests:
    ```
    git clone https://github.com/claudiofsr/execution-time.git
    cd execution-time
    cargo test -- --show-output
    ```

    2. The output:

    ```
    ---- tests::main stdout ----
    duration: 37ns
    time: Time { days: 0, hours: 0, minutes: 0, seconds: 3.7e-8 }
    time: Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 3.7e-8,
    }

    ---- tests::elapsed_time_more_than_nanosecond stdout ----
    duration: 57ns
    time: Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 5.7e-8,
    }
    formatted_output: 0.000000057 second (57ns)

    ---- tests::elapsed_time_more_than_microsecond stdout ----
    duration: 80.057µs
    time: Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 8.0057e-5,
    }
    formatted_output: 0.000080057 second (80.057µs)

    ---- tests::elapsed_time_more_than_millisecond stdout ----
    duration: 15.2ms
    time: Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0.0152,
    }
    formatted_output: 0.015200 second (15.2ms)

    ---- tests::elapsed_time_more_than_second stdout ----
    duration: 5.080012045s
    time: Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 5.080012045,
    }
    formatted_output: 5.080 seconds (5.080012045s)

    ---- tests::elapsed_time_more_than_minute stdout ----
    duration: 65.000012345s
    time: Time {
        days: 0,
        hours: 0,
        minutes: 1,
        seconds: 5.000012345,
    }
    formatted_output: 1 minute, 5.000 seconds (65.000012345s)

    ---- tests::elapsed_time_more_than_hour stdout ----
    duration: 3700.05689173s
    time: Time {
        days: 0,
        hours: 1,
        minutes: 1,
        seconds: 40.05689173,
    }
    formatted_output: 1 hour, 1 minute, 40.057 seconds (3700.05689173s)

    ---- tests::elapsed_time_more_than_day stdout ----
    duration: 93928.03s
    time: Time {
        days: 1,
        hours: 2,
        minutes: 5,
        seconds: 28.03,
    }
    formatted_output: 1 day, 2 hours, 5 minutes, 28.030 seconds (93928.03s)
    ```
