use crate::{SingularPlural, Unit};

/// Trait for formatting floating-point values with their units
pub trait FormatFloatValue {
    /// Formats a value with its unit (singular or plural).
    ///
    /// ### Arguments
    ///
    /// * `decimal` - The number of decimal places to display.
    /// * `unit` - Unit types with their singular/plural pairs.
    ///
    /// ### Returns
    ///
    /// A formatted string.
    fn format_float_unit(&self, decimal: usize, unit: Unit) -> String;
}

impl FormatFloatValue for f64 {
    fn format_float_unit(&self, decimal: usize, unit: Unit) -> String {
        let unit = if *self >= 2.0 {
            unit.plural()
        } else {
            unit.singular()
        };
        format!("{self:.decimal$} {unit}")
    }
}

/// Trait for formatting integer values with their units
pub trait FormatIntegerValue {
    /// Formats a value with its unit (singular or plural).
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit types with their singular/plural pairs.
    ///
    /// # Returns
    ///
    /// A formatted string.
    fn format_unit(&self, unit: Unit) -> String;
}

impl<T> FormatIntegerValue for T
where
    T: std::fmt::Display + PartialOrd + From<u8>,
{
    fn format_unit(&self, unit: Unit) -> String {
        // T::from(2u8) or 2.into()
        let unit = if *self >= T::from(2u8) {
            unit.plural()
        } else {
            unit.singular()
        };
        format!("{self} {unit}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_formatting() {
        assert_eq!(0u64.format_unit(Unit::Day), "0 day");
        assert_eq!(1u64.format_unit(Unit::Day), "1 day");
        assert_eq!(2u64.format_unit(Unit::Day), "2 days");
    }

    #[test]
    fn test_floating_point_formatting() {
        assert_eq!(2.0f64.format_float_unit(0, Unit::Hour), "2 hours");
        assert_eq!(0.567f64.format_float_unit(2, Unit::Second), "0.57 second");
        assert_eq!(1.567f64.format_float_unit(2, Unit::Second), "1.57 second");
        assert_eq!(2.567f64.format_float_unit(2, Unit::Second), "2.57 seconds");
    }
}
