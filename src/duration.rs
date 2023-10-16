//! Module related to duration
//! These CFDuration can be added to a CFDateTime by using the `+` or `-` operator
//! Internally it uses the timestamp in seconds representation

use crate::{calendars::Calendar, constants, utils::normalize_nanoseconds};

/// A CF duration
#[derive(Debug)]
pub struct CFDuration {
    pub seconds: i64,
    pub nanoseconds: u32,
    pub calendar: Calendar,
}

impl CFDuration {
    pub fn new(seconds: i64, nanoseconds: i64, calendar: Calendar) -> Self {
        let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        Self {
            seconds: seconds + remaining_seconds,
            nanoseconds: (remaining_nanoseconds),
            calendar,
        }
    }
}

impl CFDuration {
    /// Returns the calendar
    pub fn calendar(&self) -> Calendar {
        self.calendar
    }
    /// Makes a new `Duration` with given number of years.
    /// Depends on the Calendar definitions found in  the CF conventions
    /// See also [Calendar]
    pub fn from_years(years: i64, calendar: Calendar) -> CFDuration {
        let secs_per_year = match calendar {
            Calendar::ProlepticGregorian | Calendar::Standard => 3.15569259747e7,
            Calendar::NoLeap => 365.0 * constants::SECS_PER_DAY as f64,
            Calendar::AllLeap => 366.0 * constants::SECS_PER_DAY as f64,
            Calendar::Julian => 365.25 * constants::SECS_PER_DAY as f64,
            Calendar::Day360 => 360.0 * constants::SECS_PER_DAY as f64,
        };
        let secs = secs_per_year as i64 * years;
        Self::new(secs, 0, calendar)
    }
    /// Makes a new `Duration` with given number of months.
    pub fn from_months(months: i64, calendar: Calendar) -> CFDuration {
        let seconds_for_one_year = CFDuration::from_years(1, calendar).seconds;
        Self::new(seconds_for_one_year / 12 * months, 0, calendar)
    }
    /// Makes a new `Duration` with given number of weeks
    pub fn from_weeks(weeks: i64, calendar: Calendar) -> CFDuration {
        Self::new(weeks * 7 * 24 * 60 * 60, 0, calendar)
    }
    /// Makes a new `Duration` with given number of days
    pub fn from_days(days: i64, calendar: Calendar) -> CFDuration {
        Self::new(days * 24 * 60 * 60, 0, calendar)
    }
    /// Makes a new `Duration` with given number of hours
    pub fn from_hours(hours: i64, calendar: Calendar) -> CFDuration {
        Self::new(hours * 60 * 60, 0, calendar)
    }
    /// Makes a new `Duration` with given number of minutes
    pub fn from_minutes(minutes: i64, calendar: Calendar) -> CFDuration {
        Self::new(minutes * 60, 0, calendar)
    }
    /// Makes a new `Duration` with given number of seconds
    pub fn from_seconds(seconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(seconds, 0, calendar)
    }
    /// Makes a new `Duration` with given number of milliseconds
    pub fn from_milliseconds(milliseconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(0, milliseconds * 1_000_000, calendar)
    }
    /// Makes a new `Duration` with given number of microseconds
    pub fn from_microseconds(microseconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(0, 1_000 * microseconds, calendar)
    }
    /// Makes a new `Duration` with given number of nanoseconds
    pub fn from_nanoseconds(nanoseconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(0, nanoseconds, calendar)
    }
    /// Return the total number of years in the duration.
    pub fn num_years(&self) -> f64 {
        match self.calendar {
            Calendar::ProlepticGregorian | Calendar::Standard => {
                self.num_seconds() / 3.15569259747e7
            }
            Calendar::NoLeap => self.num_days() / 365.0,
            Calendar::AllLeap => self.num_days() / 366.0,
            Calendar::Julian => self.num_days() / 365.25,
            Calendar::Day360 => self.num_days() / 360.0,
        }
    }
    /// Return the total number of motnhs in the duration.
    pub fn num_months(&self) -> f64 {
        self.num_years() * 12.
    }
    /// Return the total number of weeks in the duration.
    pub fn num_weeks(&self) -> f64 {
        self.num_days() / 7.
    }
    /// Return the total number of days in the duration.
    pub fn num_days(&self) -> f64 {
        self.num_hours() / 24.
    }
    /// Return the total number of hours in the duration.
    pub fn num_hours(&self) -> f64 {
        self.num_minutes() / 60.
    }
    /// Return the total number of minutes in the duration.
    pub fn num_minutes(&self) -> f64 {
        self.num_seconds() / 60.0
    }
    /// Return the total number of seconds in the duration.
    pub fn num_seconds(&self) -> f64 {
        self.seconds as f64 + self.nanoseconds as f64 / 1e9
    }
    /// Return the total number of milliseconds in the duration.
    pub fn num_milliseconds(&self) -> f64 {
        self.num_seconds() * 1e3
    }
    /// Return the total number of microseconds in the duration.
    pub fn num_microseconds(&self) -> f64 {
        self.num_seconds() * 1e6
    }
    /// Return the total number of nanoseconds in the duration.
    pub fn num_nanoseconds(&self) -> f64 {
        (self.seconds * 1_000_000_000 + self.nanoseconds as i64) as f64
    }
}

/// Display a CFDuration with te ISO 8601 format of duration.
///
/// # Example
/// ```
/// CFDuration::from_days(1).__repr__()
/// assert_eq!(CFDuration::from_days(1).__repr__(),  "P0Y0M1DT0H0M0S");
/// ```
///
impl std::fmt::Display for CFDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "P{}Y{}M{}DT{}H{}M{}S",
            self.num_years() as i64,
            self.num_months() as i64 % 12,
            self.num_days() as i64 % 31,
            self.num_hours() as i64 % 24,
            self.num_minutes() as i64 % 60,
            self.num_seconds() as i64 % 60
        )
    }
}

macro_rules! impl_add_for_cf_duration {
    ($self_dur:ty, $rhs_dur:ty) => {
        impl std::ops::Add for $self_dur {
            type Output = Result<CFDuration, crate::errors::Error>;
            fn add(self, rhs: $rhs_dur) -> Self::Output {
                if self.calendar() != rhs.calendar() {
                    return Err(crate::errors::Error::DifferentCalendars(
                        self.calendar().to_string(),
                        rhs.calendar().to_string(),
                    ));
                }
                Ok(CFDuration::new(
                    self.seconds + rhs.seconds,
                    self.nanoseconds as i64 + rhs.nanoseconds as i64,
                    self.calendar,
                ))
            }
        }
    };
}
impl_add_for_cf_duration!(CFDuration, CFDuration);
impl_add_for_cf_duration!(&CFDuration, &CFDuration);

macro_rules! impl_sub_for_cf_duration {
    ($self_dur:ty, $rhs_dur:ty) => {
        impl std::ops::Sub for $self_dur {
            type Output = Result<CFDuration, crate::errors::Error>;
            fn sub(self, rhs: $rhs_dur) -> Self::Output {
                if self.calendar() != rhs.calendar() {
                    return Err(crate::errors::Error::DifferentCalendars(
                        self.calendar().to_string(),
                        rhs.calendar().to_string(),
                    ));
                }
                Ok(CFDuration::new(
                    self.seconds - rhs.seconds,
                    self.nanoseconds as i64 - rhs.nanoseconds as i64,
                    self.calendar,
                ))
            }
        }
    };
}

impl_sub_for_cf_duration!(CFDuration, CFDuration);
impl_sub_for_cf_duration!(&CFDuration, &CFDuration);

impl std::ops::Neg for CFDuration {
    type Output = CFDuration;
    fn neg(self) -> Self::Output {
        Self::new(-self.seconds, -(self.nanoseconds as i64), self.calendar)
    }
}
impl std::ops::Neg for &CFDuration {
    type Output = CFDuration;
    fn neg(self) -> Self::Output {
        CFDuration::new(-self.seconds, -(self.nanoseconds as i64), self.calendar)
    }
}

macro_rules! impl_mul_for_cf_duration_int {
    ($which_dur:ty, $rhs_type:ty) => {
        impl std::ops::Mul<$rhs_type> for $which_dur {
            type Output = CFDuration;
            fn mul(self, rhs: $rhs_type) -> Self::Output {
                CFDuration::new(
                    self.seconds * rhs as i64,
                    self.nanoseconds as i64 * rhs as i64,
                    self.calendar,
                )
            }
        }
    };
}

impl_mul_for_cf_duration_int!(CFDuration, i64);
impl_mul_for_cf_duration_int!(CFDuration, i32);
impl_mul_for_cf_duration_int!(&CFDuration, i64);
impl_mul_for_cf_duration_int!(&CFDuration, i32);

macro_rules! impl_mul_for_cf_duration_float {
    ($which_dur:ty, $rhs_type:ty) => {
        impl std::ops::Mul<$rhs_type> for $which_dur {
            type Output = CFDuration;
            fn mul(self, rhs: $rhs_type) -> Self::Output {
                // Classic (a+b)(d+c)
                // f32 to i64 does not give the same result all the time
                // i.e. 8276688000.0 gives 8276687872
                let _rhs: f64 = rhs.into();
                let mut new_seconds = (self.seconds as f64 * _rhs) as i64;
                let mut new_ns = (self.nanoseconds as f64 * _rhs) as i64;
                let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(new_ns);
                new_seconds += remaining_seconds;
                new_ns += remaining_nanoseconds as i64;
                CFDuration::new(new_seconds, new_ns, self.calendar)
            }
        }
    };
}

impl_mul_for_cf_duration_float!(CFDuration, f64);
impl_mul_for_cf_duration_float!(CFDuration, f32);
impl_mul_for_cf_duration_float!(&CFDuration, f64);
impl_mul_for_cf_duration_float!(&CFDuration, f32);

#[cfg(test)]
mod tests {
    use crate::calendars;

    use super::*;

    #[test]
    fn test_idempotence_duration_all_calendars() {
        let cals = vec![
            calendars::Calendar::Day360,
            calendars::Calendar::Standard,
            calendars::Calendar::ProlepticGregorian,
            calendars::Calendar::Julian,
            calendars::Calendar::NoLeap,
            calendars::Calendar::AllLeap,
        ];
        for cal in cals.clone() {
            println!("{}", cal);
            println!("Week");
            let duration = CFDuration::from_weeks(1, cal);
            let duration_result = duration.num_weeks();
            assert_eq!(duration_result, 1.0);
            println!("Day");
            let duration = CFDuration::from_days(1, cal);
            let duration_result = duration.num_days();
            assert_eq!(duration_result, 1.0);
            println!("Hours");
            let duration = CFDuration::from_hours(1, cal);
            let duration_result = duration.num_hours();
            assert_eq!(duration_result, 1.0);
            println!("Minutes");
            let duration = CFDuration::from_minutes(1, cal);
            let duration_result = duration.num_minutes();
            assert_eq!(duration_result, 1.0);
            println!("Seconds");
            let duration = CFDuration::from_seconds(1, cal);
            let duration_result = duration.num_seconds();
            assert_eq!(duration_result, 1.0);
            println!("Milliseconds");
            let duration = CFDuration::from_milliseconds(1, cal);
            let duration_result = duration.num_milliseconds();
            assert_eq!(duration_result, 1.0);
            println!("Microseconds");
            let duration = CFDuration::from_microseconds(1, cal);
            let duration_result = duration.num_microseconds();
            assert_eq!(duration_result, 1.0);
            println!("Nanoseconds");
            let duration = CFDuration::from_nanoseconds(1, cal);
            let duration_result = duration.num_nanoseconds();
            assert_eq!(duration_result, 1.0);
        }
        // Years and month are not exact so we need to test by omparing with an epsilon
        let epsilon = 1e-6;
        for cal in cals {
            println!("Year");
            let duration = CFDuration::from_years(1, cal);
            let duration_result = duration.num_years();
            assert!((duration_result - 1.0).abs() < epsilon);
            println!("Month");
            let duration = CFDuration::from_months(1, cal);
            let duration_result = duration.num_months();
            assert!((duration_result - 1.0).abs() < epsilon);
        }
    }
}
