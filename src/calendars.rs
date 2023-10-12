//! Module defining the calendars and their methods

/// Represents the different types of calendars based on the
/// CF Conventions.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
pub enum Calendar {
    // alias of Standard
    #[default]
    Standard,
    ProlepticGregorian,
    NoLeap,
    AllLeap,
    Julian,
    Day360,
}

/// Convert the calendar to a good formatted string
impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match *self {
            Calendar::Standard => "Standard",
            Calendar::ProlepticGregorian => "Proleptic Gregorian",
            Calendar::NoLeap => "No Leap",
            Calendar::AllLeap => "All Leap",
            Calendar::Julian => "Julian",
            Calendar::Day360 => "360 Day",
        };
        write!(f, "{name}")
    }
}

/// Convert a valid cf unit calendar string to a Calendar
/// If no valid string is provided, Standard is returned
impl std::str::FromStr for Calendar {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "standard" | "gregorian" => Ok(Calendar::Standard),
            "proleptic_gregorian" => Ok(Calendar::ProlepticGregorian),
            "no_leap" | "day365" => Ok(Calendar::NoLeap),
            "all_leap" | "day366" => Ok(Calendar::AllLeap),
            "julian" => Ok(Calendar::Julian),
            "360_day" => Ok(Calendar::Day360),
            _ => Ok(Calendar::Standard),
        }
    }
}
