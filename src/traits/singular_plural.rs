/// Unit types with their singular/plural pairs
#[derive(Debug)]
pub enum Unit {
    Second,
    Minute,
    Hour,
    Day,
}

/// Trait for defining singular and plural forms of words.
pub trait SingularPlural {
    /// Gets the singular form of the word.
    fn singular(&self) -> &str;

    /// Gets the plural form of the word.
    fn plural(&self) -> &str;
}

impl SingularPlural for Unit {
    fn singular(&self) -> &str {
        match self {
            Unit::Second => "second",
            Unit::Minute => "minute",
            Unit::Hour => "hour",
            Unit::Day => "day",
        }
    }

    fn plural(&self) -> &str {
        match self {
            Unit::Second => "seconds",
            Unit::Minute => "minutes",
            Unit::Hour => "hours",
            Unit::Day => "days",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singular_forms() {
        // Test that singular method returns the correct singular form for all units
        assert_eq!(Unit::Second.singular(), "second");
        assert_eq!(Unit::Minute.singular(), "minute");
        assert_eq!(Unit::Hour.singular(), "hour");
        assert_eq!(Unit::Day.singular(), "day");
    }

    #[test]
    fn test_plural_forms() {
        // Test that plural method returns the correct plural form for all units
        assert_eq!(Unit::Second.plural(), "seconds");
        assert_eq!(Unit::Minute.plural(), "minutes");
        assert_eq!(Unit::Hour.plural(), "hours");
        assert_eq!(Unit::Day.plural(), "days");
    }

    #[test]
    fn test_singular_plural_trait() {
        // Test that a type implementing SingularPlural returns singular and plural
        let unit = Unit::Hour;
        assert_eq!(unit.singular(), "hour");
        assert_eq!(unit.plural(), "hours");
    }
}
