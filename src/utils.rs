use crate::{
    calendars::Calendar,
    constants,
    datetimes::traits::{CalendarDatetime, IsLeap},
};
use std::time::Duration;

/// Calculates the timestamp from the given year, month, and day.
///
/// # Arguments
///
/// * `year` - The year.
/// * `month` - The month.
/// * `day` - The day.
///
/// # Returns
///
/// The calculated timestamp.
///
/// # Errors
///
/// Returns an error if there was an issue calculating the timestamp.
pub fn get_timestamp_from_ymd<T: IsLeap>(
    year: i64,
    month: u8,
    day: u8,
) -> Result<i64, crate::errors::Error> {
    let mut timestamp: i64 = 0;

    // Calculate years

    let mut current_year: i64 = year;
    loop {
        if current_year == constants::UNIX_DEFAULT_YEAR {
            break;
        }
        // We have to look at the preceding year. For example if year == 1972
        // we have to look from 1971 to 1972
        let year_to_look_at = if current_year > constants::UNIX_DEFAULT_YEAR {
            current_year - 1
        } else {
            current_year
        };
        let days_in_year: i64 = if T::is_leap(year_to_look_at) {
            constants::DAYS_PER_MONTH_LEAP
                .iter()
                .map(|&x| x as i64)
                .sum()
        } else {
            constants::DAYS_PER_MONTH.iter().map(|&x| x as i64).sum()
        };

        if current_year > constants::UNIX_DEFAULT_YEAR {
            timestamp += days_in_year * constants::SECS_PER_DAY as i64;
            current_year -= 1;
        } else {
            timestamp -= days_in_year * constants::SECS_PER_DAY as i64;
            current_year += 1;
        }
    }

    // Calculate months
    let mut current_month = 0;
    loop {
        if current_month + 1 == month {
            break;
        }
        if T::is_leap(year) {
            timestamp += constants::DAYS_PER_MONTH_LEAP[(current_month) as usize] as i64
                * constants::SECS_PER_DAY as i64;
        } else {
            timestamp += constants::DAYS_PER_MONTH[(current_month) as usize] as i64
                * constants::SECS_PER_DAY as i64;
        }
        current_month += 1;
    }

    // Calculate days
    timestamp += (day as i64 - 1) * constants::SECS_PER_DAY as i64;

    Ok(timestamp)
}

/// Converts a timestamp into hours, minutes, and seconds.
///
/// # Arguments
///
/// * `timestamp` - The timestamp to convert.
///
/// # Returns
///
/// A tuple containing the hours, minutes, and seconds.
pub fn get_hms_from_timestamp(timestamp: i64) -> (u8, u8, u8) {
    let _mod_sec = constants::SECS_PER_DAY as i64;
    let seconds = (timestamp % constants::SECS_PER_DAY as i64 + constants::SECS_PER_DAY as i64)
        % constants::SECS_PER_DAY as i64;
    let sec = (seconds % 60) as u8;
    let min = ((seconds / 60) % 60) as u8;
    let hour = ((seconds / 3600) % 24) as u8;
    (hour, min, sec)
}
/// Converts a timestamp to the year, month, day, hour, minute, and second components.
///
/// # Arguments
///
/// * `timestamp` - The timestamp to convert
///
/// # Generic Parameters
///
/// * `T` - A type that implements the `IsLeap` trait, used to determine if a year is a leap year
///
/// # Returns
///
/// A tuple containing the year, month, day, hour, minute, and second components of the timestamp.
pub fn get_ymd_hms_from_timestamp<T: IsLeap>(timestamp: i64) -> (i64, u8, u8, u8, u8, u8) {
    let mut remaining_timestamp = timestamp;
    let mut year = constants::UNIX_DEFAULT_YEAR;

    // Determine the direction (past or future)
    let direction = if timestamp >= 0 { 1 } else { -1 };

    loop {
        let year_to_look_at = if year > constants::UNIX_DEFAULT_YEAR {
            year
        } else {
            year - 1
        };
        let days_in_year: i64 = if T::is_leap(year_to_look_at) {
            constants::DAYS_PER_MONTH_LEAP
                .iter()
                .map(|&x| x as i64)
                .sum()
        } else {
            constants::DAYS_PER_MONTH.iter().map(|&x| x as i64).sum()
        };

        let seconds_in_year = days_in_year * constants::SECS_PER_DAY as i64;

        let new_remaining = remaining_timestamp - direction * seconds_in_year;

        // After UNIX epoch we can stop
        if direction == 1 && (new_remaining < 0) {
            break;
        }
        // Before UNIX epoch we substract one year if needed
        // This ensure remaining_timestamp is positive or equals 0
        else if direction == -1 && (new_remaining >= 0) {
            remaining_timestamp = new_remaining;
            year += direction;
            break;
        }
        remaining_timestamp = new_remaining;
        year += direction;
    }

    // Calculate months
    // remaining_timestamp is positive or equals 0
    let mut month: i64 = 0;
    loop {
        let days_in_month: i64 = if T::is_leap(year) {
            constants::DAYS_PER_MONTH_LEAP[month as usize] as i64
        } else {
            constants::DAYS_PER_MONTH[month as usize] as i64
        };
        let seconds_in_month = days_in_month * constants::SECS_PER_DAY as i64;

        if remaining_timestamp < seconds_in_month {
            break;
        }
        remaining_timestamp -= seconds_in_month;
        month += 1;
    }

    // Calculate days
    let day = (remaining_timestamp / (constants::SECS_PER_DAY as i64)) as u8;

    let (hour, min, sec) = get_hms_from_timestamp(remaining_timestamp);
    (year, month as u8 + 1, day + 1, hour, min, sec)
}

/// Determines if a given year is a leap year according to the Gregorian calendar.
///
/// # Arguments
///
/// * `year` - The year to be checked.
///
/// # Returns
///
/// Returns `true` if the year is a leap year, `false` otherwise.
pub fn is_leap_gregorian(year: i64) -> bool {
    let mut f_year = year;
    if year < 0 {
        f_year = year + 1;
    }
    (f_year % 400 == 0) || ((f_year % 4 == 0) && (f_year % 100 != 0))
}

/// Determines if a given year is a leap year in the Julian calendar.
///
/// # Arguments
///
/// * `year` - The year to check for leapness.
///
/// # Returns
///
/// * `true` if the year is a leap year, `false` otherwise.
pub fn is_leap_julian(year: i64) -> bool {
    let mut f_year = year;
    if year < 0 {
        f_year = year + 1;
    }
    f_year % 4 == 0
}

fn extract_seconds_and_nanoseconds(seconds: f32) -> (u64, u32) {
    let duration = Duration::from_secs_f32(seconds);
    let secs = duration.as_secs();
    let nanosecs = duration.subsec_nanos();

    (secs, nanosecs)
}

/// Converts the given hour, minute, and second values into a timestamp.
///
/// # Arguments
///
/// * `hour` - The hour value (0-23).
/// * `min` - The minute value (0-59).
/// * `sec` - The second value (0.0-59.999...).
///
/// # Returns
///
/// A tuple containing the total number of seconds and the number of nanoseconds.
///
/// # Errors
///
/// Returns an error if any of the input values are out of bounds.
pub fn get_timestamp_from_hms(
    hour: u8,
    min: u8,
    sec: f32,
) -> Result<(i64, u32), crate::errors::Error> {
    if hour > 23 {
        return Err(crate::errors::Error::InvalidTime(
            format!("Hour {hour} is out of bounds").to_string(),
        ));
    }
    if min > 59 {
        return Err(crate::errors::Error::InvalidTime(
            format!("Minute {min} is out of bounds").to_string(),
        ));
    }
    if sec >= 60.0 || sec < 0.0 {
        return Err(crate::errors::Error::InvalidTime(
            format!("Second {sec} is out of bounds").to_string(),
        ));
    }
    let (round_seconds, nanoseconds) = extract_seconds_and_nanoseconds(sec);
    let total_seconds = (hour as u32 * constants::SECS_PER_HOUR
        + min as u32 * constants::SECS_PER_MINUTE
        + round_seconds as u32)
        % constants::SECS_PER_DAY;

    Ok((total_seconds as i64, nanoseconds))
}
