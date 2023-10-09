//! Module defining the calendars and their methods

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
pub enum Calendar {
    // alias of Standard
    Gregorian,
    Standard,
    #[default]
    ProlepticGregorian,
    Day365,
    // Same as 365 days
    NoLeap,
    Day366,
    // Same as Day366
    AllLeap,
    Julian,
    Day360,
}

impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match *self {
            Calendar::Gregorian => "Gregorian",
            Calendar::Standard => "Standard",
            Calendar::ProlepticGregorian => "Proleptic Gregorian",
            Calendar::NoLeap | Calendar::Day365 => "No Leap",
            Calendar::AllLeap | Calendar::Day366 => "All Leap",
            Calendar::Julian => "Julian",
            Calendar::Day360 => "360 Day",
        };
        write!(f, "{name}")
    }
}

impl std::str::FromStr for Calendar {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "gregorian" => Ok(Calendar::Gregorian),
            "standard" => Ok(Calendar::Standard),
            "proleptic_gregorian" => Ok(Calendar::ProlepticGregorian),
            "no_leap" => Ok(Calendar::NoLeap),
            "day365" => Ok(Calendar::Day365),
            "all_leap" => Ok(Calendar::AllLeap),
            "julian" => Ok(Calendar::Julian),
            "360_day" => Ok(Calendar::Day360),
            _ => Ok(Calendar::Standard),
        }
    }
}
