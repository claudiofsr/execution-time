/// Trait for rounding floating-point numbers to a specified number of decimal places.
pub trait RoundFloat {
    /// Rounds the floating-point number to the given number of decimal places.
    ///
    /// ### Arguments
    ///
    /// * `decimal` - The number of decimal places to round to.
    ///
    /// ### Returns
    ///
    /// The rounded floating-point number.
    fn round_float(self, decimal: i32) -> Self
    where
        // This trait is object safe and Sized is required to be object safe
        Self: std::marker::Sized;
}

impl RoundFloat for f64 {
    fn round_float(self, decimal: i32) -> f64 {
        if decimal <= 0 || self == 0.0 {
            self.round()
        } else {
            let multiplier: f64 = 10.0_f64.powi(decimal);
            (self * multiplier).round() / multiplier
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_positive_decimal_places() {
        // Test rounding to 2 decimal places
        let num: f64 = 5.22501;
        assert_eq!(num.round_float(2), 5.23);

        // Test rounding to 3 decimal places
        let num = 12.34567;
        assert_eq!(num.round_float(3), 12.346);

        // Test rounding to 1 decimal place
        let num = 7.89;
        assert_eq!(num.round_float(1), 7.9);

        // Test rounding to a greater number of decimal places than available.
        let num = 1.2;
        assert_eq!(num.round_float(4), 1.2);

        //Test with a negative number
        let num = -2.789;
        assert_eq!(num.round_float(2), -2.79)
    }

    #[test]
    fn test_round_zero_decimal_places() {
        // Test rounding to 0 decimal places (should round to nearest whole number)
        let num = 3.6;
        assert_eq!(num.round_float(0), 4.0);

        // Test rounding to 0 decimal places (should round to nearest whole number)
        let num = 3.4;
        assert_eq!(num.round_float(0), 3.0);
    }

    #[test]
    fn test_round_negative_decimal_places() {
        // Test rounding with negative decimal places which should be equivalent to round()
        let num = 123.456;
        assert_eq!(num.round_float(-1), 123.0);

        // Test rounding with negative decimal places with a negative number
        let num = -123.456;
        assert_eq!(num.round_float(-2), -123.0);

        // Test rounding with negative decimal places
        let num = 123.56;
        assert_eq!(num.round_float(-1), 124.0);
    }

    #[test]
    fn test_round_with_zero() {
        // Test with zero number, should return 0
        let num = 0.0;
        assert_eq!(num.round_float(2), 0.0);
        assert_eq!(num.round_float(0), 0.0);
        assert_eq!(num.round_float(-2), 0.0);
    }
}
