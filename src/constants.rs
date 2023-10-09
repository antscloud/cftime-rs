const fn cumsum_cal(input: &[u8; 12]) -> [u32; 13] {
    let mut out: [u32; 13] = [0; 13];
    let mut i = 1;
    while i < 13 {
        out[i] = out[i - 1] + input[i - 1] as u32;
        i += 1;
    }
    out
}

// DAYS CALENDARS
pub const DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_PER_MONTH_LEAP: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_PER_MONTH_360: [u8; 12] = [30; 12];

// VARIOUS PRE-COMPUTED CONSTANTS
pub const DAYS_PER_YEAR_LEAP: i64 = 366;
pub const DAYS_PER_YEAR_NON_LEAP: i64 = 365;
pub const SECONDS_PER_YEAR_LEAP: i64 = DAYS_PER_YEAR_LEAP * SECS_PER_DAY as i64;
pub const SECONDS_PER_YEAR_NON_LEAP: i64 = DAYS_PER_YEAR_NON_LEAP * SECS_PER_DAY as i64;

// CUMSUM DAYS CALENDARS
pub const CUM_DAYS_PER_MONTH: [u32; 13] = cumsum_cal(&DAYS_PER_MONTH);
pub const CUM_DAYS_PER_MONTH_LEAP: [u32; 13] = cumsum_cal(&DAYS_PER_MONTH_LEAP);
pub const CUM_DAYS_PER_MONTH_360: [u32; 13] = cumsum_cal(&DAYS_PER_MONTH_360);

// UNIX TIMESTAMP
pub const UNIX_DEFAULT_YEAR: i64 = 1970;
pub const UNIX_DEFAULT_MONTH: u8 = 1;
pub const UNIX_DEFAULT_DAY: u8 = 1;

// GENERALITIES
pub const SECS_PER_HOUR: u32 = 3600;
pub const SECS_PER_MINUTE: u32 = 60;
pub const SECS_PER_DAY: u32 = 24 * SECS_PER_HOUR;

pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const MAX_NS: i64 = 1_000_000_000;
