//! Utils crate where common behaviour for computing dates are shared

use crate::{
    calendars::Calendar,
    constants,
    datetime::CFDatetime,
    datetimes::traits::IsLeap,
    duration::CFDuration,
    parser::{parse_cf_time, Unit},
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
        let year_to_look_at = current_year - (current_year > constants::UNIX_DEFAULT_YEAR) as i64;
        let seconds_in_year: i64 = if T::is_leap(year_to_look_at) {
            constants::SECONDS_PER_YEAR_LEAP
        } else {
            constants::SECONDS_PER_YEAR_NON_LEAP
        };

        if current_year > constants::UNIX_DEFAULT_YEAR {
            timestamp += seconds_in_year;
            current_year -= 1;
        } else {
            timestamp -= seconds_in_year;
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
    let mut current_year = constants::UNIX_DEFAULT_YEAR;

    // Determine the direction (past or future)
    let direction = if timestamp >= 0 { 1 } else { -1 };

    loop {
        let year_to_look_at = if current_year > constants::UNIX_DEFAULT_YEAR {
            current_year
        } else {
            current_year - 1
        };
        let seconds_in_year: i64 = if T::is_leap(year_to_look_at) {
            constants::SECONDS_PER_YEAR_LEAP
        } else {
            constants::SECONDS_PER_YEAR_NON_LEAP
        };

        let new_remaining = remaining_timestamp - direction * seconds_in_year;

        // After UNIX epoch we can stop
        if direction == 1 && (new_remaining < 0) {
            break;
        }
        // Before UNIX epoch we substract one year if needed
        // This ensure remaining_timestamp is positive or equals 0
        else if direction == -1 && (new_remaining >= 0) {
            remaining_timestamp = new_remaining;
            current_year += direction;
            break;
        }
        remaining_timestamp = new_remaining;
        current_year += direction;
    }

    // Calculate months
    // remaining_timestamp is positive or equals 0
    let mut month: i64 = 0;
    loop {
        let days_in_month: i64 = if T::is_leap(current_year) {
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
    (current_year, month as u8 + 1, day + 1, hour, min, sec)
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
    // Optimization : Adds 1 for negative years, 0 for non-negative years
    // We extract the sign bit from the year i64 variable
    let f_year = ((year >> 63) & 1) + year;
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
    // Optimization : Adds 1 for negative years, 0 for non-negative years
    // We extract the sign bit from the year i64 variable
    (((year >> 63) & 1) + year) % 4 == 0
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
    if !(0.0..60.0).contains(&sec) {
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

pub fn get_datetime_and_unit_from_units(
    units: &str,
    calendar: Calendar,
) -> Result<(CFDatetime, Unit), crate::errors::Error> {
    let parsed_cf_time = parse_cf_time(units)?;
    let (year, month, day) = parsed_cf_time.datetime.ymd;
    let (hour, minute, second) = match parsed_cf_time.datetime.hms {
        Some(hms) => (hms.0, hms.1, hms.2),
        None => (0, 0, 0.0),
    };
    let cf_datetime = CFDatetime::from_ymd_hms(year, month, day, hour, minute, second, calendar)?;
    let unit = parsed_cf_time.unit;
    Ok((cf_datetime, unit))
}
/// Normalize the given number of nanoseconds into seconds and remaining nanoseconds.
///
/// # Arguments
///
/// * `nanoseconds` - The number of nanoseconds to normalize.
///
/// # Returns
///
/// A tuple containing the remaining seconds and remaining nanoseconds.
///
/// # Examples
///
/// ```
/// use cftime_rs::utils::normalize_nanoseconds;
/// let nanoseconds = 1_500_000_000;
/// let (seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
/// assert_eq!(seconds, 1);
/// assert_eq!(remaining_nanoseconds, 500_000_000);
/// ```
///
/// ```
/// use cftime_rs::utils::normalize_nanoseconds;
/// let nanoseconds = -2_500_000_000;
/// let (seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
/// assert_eq!(seconds, -3);
/// assert_eq!(remaining_nanoseconds, 500_000_000);
/// ```
pub fn normalize_nanoseconds(nanoseconds: i64) -> (i64, u32) {
    // Calculate the number of remaining seconds
    let mut remaining_seconds = nanoseconds / 1e9 as i64;

    // Calculate the number of remaining nanoseconds
    let remaining_nanoseconds: i64 = if remaining_seconds < 0 {
        // If the remaining seconds is negative, subtract 1 and calculate the remaining nanoseconds accordingly
        remaining_seconds -= 1;
        (nanoseconds + (remaining_seconds.abs() * 1_000_000_000)) % 1_000_000_000
    } else {
        // If the remaining seconds is positive or zero, calculate the remaining nanoseconds directly
        nanoseconds % 1e9 as i64
    };
    (remaining_seconds, remaining_nanoseconds as u32)
}

/// Converts a unit of time to its corresponding encoded value.
///
/// # Arguments
///
/// * `unit` - The unit of time to encode.
/// * `duration` - The duration to encode.
///
/// # Returns
///
/// The encoded value of the unit of time.
pub fn unit_to_encode(unit: &Unit, duration: CFDuration) -> f64 {
    match unit {
        Unit::Year => duration.num_years(),     // Convert to years
        Unit::Month => duration.num_months(),   // Convert to months
        Unit::Day => duration.num_days(),       // Convert to days
        Unit::Hour => duration.num_hours(),     // Convert to hours
        Unit::Minute => duration.num_minutes(), // Convert to minutes
        Unit::Second => duration.num_seconds(), // Convert to seconds
        Unit::Millisecond => duration.num_milliseconds(), // Convert to milliseconds
        Unit::Microsecond => duration.num_microseconds(), // Convert to microseconds
        Unit::Nanosecond => duration.num_nanoseconds(), // Convert to nanoseconds
    }
}
