#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Calendar {
    // alias of Standard
    Gregorian,
    Standard,
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

impl Default for Calendar {
    fn default() -> Calendar {
        Calendar::ProlepticGregorian
    }
}
impl std::fmt::Display for Calendar {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
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

impl Calendar {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "gregorian" => Calendar::Gregorian,
            "standard" => Calendar::Standard,
            "proleptic_gregorian" => Calendar::ProlepticGregorian,
            "no_leap" => Calendar::NoLeap,
            "Day365" => Calendar::Day365,
            "All Leap" => Calendar::AllLeap,
            "Julian" => Calendar::Julian,
            "360 Day" => Calendar::Day360,
            _ => Calendar::Standard,
        }
    }
}
