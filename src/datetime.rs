/// Wrapper for all the different datetime and calendars
use crate::datetimes::all_leap::AllLeapDatetime;
use crate::datetimes::day_360::Day360Datetime;
use crate::datetimes::julian::JulianDatetime;
use crate::datetimes::no_leap::NoLeapDatetime;
use crate::datetimes::proleptic_gregorian::ProlepticGregorianDatetime;
use crate::datetimes::standard::StandardDatetime;
use crate::datetimes::traits::CalendarDatetime;
use crate::datetimes::traits::CalendarDatetimeCreator;
use crate::duration::CFDuration;

use crate::utils::normalize_nanoseconds;
use crate::{calendars::Calendar, constants};

/// Represents a calendar CF datetime.
///
/// Internally it uses the timestamp in seconds representation
/// With this implementation it is faster to add a duration to a CFDatetime
/// However if we ask for a representation of the date calling the format method
/// it will calculate the year, month, day, hour, minute, second and nanoseconds
/// from timestamp which can make print a bit slow
///
/// # Examples
///
/// ## Creating a datetime
///
/// ```rust
/// let cf_datetime = CFDatetime::from_ymd_hms(1970, 1, 1, 0, 0, 0, Calendar::Standard).unwrap();
/// // Computation of the timestamp
/// assert_eq!(cf_datetime.timestamp(), 0);
/// // Idempotence
/// assert_eq!(cf_datetime.ymd_hms().unwrap(), (1970, 1, 1, 0, 0, 0));
/// ```
/// ## Duration between two datetimes
/// ```rust
/// let cf_datetime_1 = CFDatetime::from_ymd_hms(1970, 1, 1, 0, 0, 0, Calendar::Standard).unwrap();
/// let cf_datetime_2 = CFDatetime::from_ymd_hms(1970, 1, 2, 0, 0, 0, Calendar::Standard).unwrap();
/// let duration = cf_datetime_2 - cf_datetime_1;
/// assert_eq!(duration.num_days, 1);
/// ```
pub struct CFDatetime {
    inner: Box<dyn CalendarDatetime + Send + Sync>,
}

/// Immplementation of the CF convention specifications :
/// - [CF Conventions](https://cfconventions.org/Data/cf-conventions/cf-conventions-1.10/cf-conventions.html#time-coordinate)
impl CFDatetime {
    /// Returns the calendar
    pub fn calendar(&self) -> Calendar {
        self.inner.calendar()
    }
    /// Returns the timestamp
    pub fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    /// Returns the year, month, and day of the date.
    ///
    /// # Returns
    ///
    /// A Result containing a tuple containing the hour, minute, second as `(i64, u8, u8)`.
    /// or an error of type `crate::errors::Error::InvalidDate` if the date cannot be computed from the timestamp.
    pub fn ymd(&self) -> Result<(i64, u8, u8), crate::errors::Error> {
        let (year, month, day, _, _, _) = self.ymd_hms()?;

        Ok((year, month, day))
    }
    /// Returns the hour, minute, second of the date.
    ///
    /// # Returns
    ///
    /// A Result containing a tuple containing the hour, minute, second as `(i64, u8, u8)`.
    /// or an error of type `crate::errors::Error::InvalidDate` if the date cannot be computed from the timestamp.
    ///
    /// hms needs to first compute the date to see if the date is impossible
    pub fn hms(&self) -> Result<(u8, u8, u8), crate::errors::Error> {
        let (_, _, _, hour, min, sec) = self.ymd_hms()?;
        Ok((hour, min, sec))
    }
    /// Returns the year, month,  day, hour, minute, second of the date.
    ///
    /// # Returns
    ///
    /// A Result containing a tuple containing the year, month,  day, hour, minute, second  as
    /// `(i64, u8, u8, u8, u8, u8)` or an error of type `crate::errors::Error::InvalidDate` if
    /// the date cannot be computed from the timestamp.
    pub fn ymd_hms(&self) -> Result<(i64, u8, u8, u8, u8, u8), crate::errors::Error> {
        self.inner.ymd_hms()
    }
    /// Creates a new CFDatetime from the given year, month, day, hour, minute, second, and calendar.
    ///
    /// # Returns
    ///
    ///  A Result containing a new CFDatetime or an error of type `crate::errors::Error::InvalidDate` if
    /// the date is not valid in the calendar
    pub fn from_ymd_hms(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f32,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        match calendar {
            Calendar::ProlepticGregorian => Ok(Self {
                inner: Box::new(ProlepticGregorianDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
            Calendar::Standard => Ok(Self {
                inner: Box::new(StandardDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
            Calendar::Day360 => Ok(Self {
                inner: Box::new(Day360Datetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
            Calendar::Julian => Ok(Self {
                inner: Box::new(JulianDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
            Calendar::NoLeap => Ok(Self {
                inner: Box::new(NoLeapDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
            Calendar::AllLeap => Ok(Self {
                inner: Box::new(AllLeapDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
        }
    }

    /// Creates a new CFDatetime from the given hour, minute, second, and calendar.
    /// It sets the year, month, day to 1970, 1, 1
    ///
    /// # Returns
    ///
    /// A Result containing a new CFDatetime or an error of type `crate::errors::Error::InvalidDate` if
    /// the date is not valid in the calendar
    pub fn from_hms(
        hour: u8,
        minute: u8,
        second: f32,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        Self::from_ymd_hms(
            constants::UNIX_DEFAULT_YEAR,
            constants::UNIX_DEFAULT_MONTH,
            constants::UNIX_DEFAULT_DAY,
            hour,
            minute,
            second,
            calendar,
        )
    }
    /// Creates a new CFDatetime from the given year, month, day and calendar.
    /// It sets the hour, minute, second to 1970, 1, 1
    ///
    /// # Returns
    ///
    /// A Result containing a new CFDatetime or an error of type `crate::errors::Error::InvalidDate` if
    /// the date is not valid in the calendar
    pub fn from_ymd(
        year: i64,
        month: u8,
        day: u8,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        Self::from_ymd_hms(year, month, day, 0, 0, 0.0, calendar)
    }
    /// Creates a new CFDatetime from a given timestamp and calendar atrting from the epoch
    ///
    /// # Returns
    ///
    /// A Result containing a new CFDatetime or an error of type `crate::errors::Error::InvalidDate` if
    /// the date is not valid in the calendar
    pub fn from_timestamp(
        timestamp: i64,
        nanoseconds: u32,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        match calendar {
            Calendar::ProlepticGregorian => Ok(Self {
                inner: Box::new(ProlepticGregorianDatetime::from_timestamp(
                    timestamp,
                    nanoseconds,
                )),
            }),
            Calendar::Standard => Ok(Self {
                inner: Box::new(StandardDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::Day360 => Ok(Self {
                inner: Box::new(Day360Datetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::Julian => Ok(Self {
                inner: Box::new(JulianDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::NoLeap => Ok(Self {
                inner: Box::new(NoLeapDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::AllLeap => Ok(Self {
                inner: Box::new(AllLeapDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
        }
    }

    /// Returns the hours of the date.
    pub fn hours(&self) -> Result<u8, crate::errors::Error> {
        let (hour, _, _) = self.hms()?;
        Ok(hour)
    }
    /// Returns the minutes of the date.
    pub fn minutes(&self) -> Result<u8, crate::errors::Error> {
        let (_, min, _) = self.hms()?;
        Ok(min)
    }
    /// Returns the seconds of the date.
    pub fn seconds(&self) -> Result<u8, crate::errors::Error> {
        let (_, _, sec) = self.hms()?;
        Ok(sec)
    }
    /// Returns the nanoseconds of the date.
    pub fn nanoseconds(&self) -> u32 {
        self.inner.nanoseconds()
    }
    /// Change the calendar of the CFDatetime.
    ///
    /// It get the year, month, day, hour, minute, second and nanoseconds by calling the [Self::ymd_hms]
    /// method and then call the [Self::from_ymd_hms] method with the new calendar. This can be considered
    /// as safe
    ///
    /// # Returns
    /// A Result containing a new CFDatetime or an error of type `crate::errors::Error::InvalidDate` if
    /// the date is not valid in the calendar
    pub fn change_calendar(&self, calendar: Calendar) -> Result<Self, crate::errors::Error> {
        let (year, month, day, hour, minute, second) = self.ymd_hms()?;
        let ns = self.nanoseconds();
        let f_second = second as f32 + ns as f32 / 1e9;
        Self::from_ymd_hms(year, month, day, hour, minute, f_second, calendar)
    }
    /// Change the calendar of the CFDatetime using the timestamp
    ///
    /// It get the year, month, day, hour, minute, second and nanoseconds by calling the [Self::timestamp]
    /// method and then call the [Self::from_timestamp] method with the new calendar.
    ///
    /// Be aware that there is highly chance that the two dates do not correspond.
    /// However their distances from epoch are the same.
    ///
    /// # Returns
    /// A Result containing a new CFDatetime or an error of type `crate::errors::Error::InvalidDate` if
    /// the date is not valid in the calendar
    pub fn change_calendar_from_timestamp(
        &self,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        let timestamp = self.timestamp();
        let nanoseconds = self.nanoseconds();
        Self::from_timestamp(timestamp, nanoseconds, calendar)
    }
}

/// Display a CFDatetime with the following format : `YYYY-MM-DD HH:MM:SS.SSS`
impl std::fmt::Display for CFDatetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let nanoseconds = self.nanoseconds() as f64 / 1_000_000_000.;
        match self.ymd_hms() {
            Ok((year, month, day, hour, minute, second)) => {
                write!(
                    f,
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                    year, month, day, hour, minute, second, nanoseconds
                )
            }
            Err(err) => {
                write!(f, "{:?}", err)
            }
        }
    }
}

macro_rules! impl_add_duration {
    ($rhs:ty, $for:ty) => {
        impl std::ops::Add<$rhs> for $for {
            type Output = Result<CFDatetime, crate::errors::Error>;
            fn add(self, rhs: $rhs) -> Self::Output {
                if self.calendar() != rhs.calendar() {
                    return Err(crate::errors::Error::DifferentCalendars(
                        self.calendar().to_string(),
                        rhs.calendar().to_string(),
                    ));
                }
                let nanoseconds = self.nanoseconds() as i64 + rhs.nanoseconds as i64;
                let (_remaining_seconds, remaining_nanoseconds) =
                    normalize_nanoseconds(nanoseconds);
                let new_timestamp = self.timestamp() + rhs.seconds;
                CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
            }
        }
    };
}
impl_add_duration!(CFDuration, CFDatetime);
impl_add_duration!(&CFDuration, CFDatetime);
impl_add_duration!(CFDuration, &CFDatetime);
impl_add_duration!(&CFDuration, &CFDatetime);

macro_rules! impl_sub_duration {
    ($rhs:ty, $for:ty) => {
        impl std::ops::Sub<$rhs> for $for {
            type Output = Result<CFDatetime, crate::errors::Error>;
            fn sub(self, rhs: $rhs) -> Self::Output {
                if self.calendar() != rhs.calendar() {
                    return Err(crate::errors::Error::DifferentCalendars(
                        self.calendar().to_string(),
                        rhs.calendar().to_string(),
                    ));
                }
                let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds as i64;
                let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
                let new_timestamp = (self.timestamp() - rhs.seconds) + remaining_seconds;
                CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
            }
        }
    };
}
impl_sub_duration!(CFDuration, CFDatetime);
impl_sub_duration!(&CFDuration, CFDatetime);
impl_sub_duration!(CFDuration, &CFDatetime);
impl_sub_duration!(&CFDuration, &CFDatetime);

macro_rules! impl_sub_datetime {
    ($rhs:ty, $for:ty) => {
        impl std::ops::Sub<$rhs> for $for {
            type Output = Result<CFDuration, crate::errors::Error>;
            fn sub(self, rhs: $rhs) -> Self::Output {
                if self.calendar() != rhs.calendar() {
                    return Err(crate::errors::Error::DifferentCalendars(
                        self.calendar().to_string(),
                        rhs.calendar().to_string(),
                    ));
                }
                let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds() as i64;
                let new_timestamp = self.timestamp() - rhs.timestamp();
                Ok(CFDuration::new(new_timestamp, nanoseconds, self.calendar()))
            }
        }
    };
}

impl_sub_datetime!(CFDatetime, CFDatetime);
impl_sub_datetime!(&CFDatetime, CFDatetime);
impl_sub_datetime!(CFDatetime, &CFDatetime);
impl_sub_datetime!(&CFDatetime, &CFDatetime);

#[cfg(test)]
mod tests {
    use crate::calendars;

    use super::*;
    #[test]
    fn test_timestamp_zero_standard() {
        let (year, month, day) = (1970, 1, 1);
        let d = CFDatetime::from_ymd(year, month, day, Calendar::Standard).unwrap();
        assert_eq!(0, d.timestamp());
    }
    #[test]
    fn test_timestamp_20230101_standard() {
        let (year, month, day) = (2023, 1, 1);
        let d = CFDatetime::from_ymd(year, month, day, Calendar::Standard).unwrap();
        assert_eq!(1672531200, d.timestamp());
    }
    #[test]
    fn test_impossible_date_gregorian() {
        let (year, month, day) = (1582, 10, 8);
        let d = CFDatetime::from_ymd(year, month, day, Calendar::Standard);
        assert!(d.is_err());
    }
    #[test]
    fn test_timestamp_minus_one_all_calendars() {
        let cals = vec![
            calendars::Calendar::Standard,
            calendars::Calendar::ProlepticGregorian,
            calendars::Calendar::Julian,
            calendars::Calendar::NoLeap,
            calendars::Calendar::AllLeap,
        ];
        for cal in cals {
            let d = CFDatetime::from_timestamp(-1, 0, cal);
            assert_eq!((1969, 12, 31, 23, 59, 59), d.unwrap().ymd_hms().unwrap());
        }
    }
    #[test]
    fn test_timestamp_limit_gregorian_julian() {
        // -12219206400 == 1582, 10, 15 in Gregorian calendar
        let lower_limit_gregorian = CFDatetime::from_timestamp(-12219292800, 0, Calendar::Standard);
        let upper_limit_julian = CFDatetime::from_timestamp(-12219292801, 0, Calendar::Standard);
        assert_eq!(
            lower_limit_gregorian.unwrap().ymd_hms().unwrap(),
            (1582, 10, 15, 0, 0, 0)
        );
        assert_eq!(
            upper_limit_julian.unwrap().ymd_hms().unwrap(),
            (1582, 10, 4, 23, 59, 59)
        );
    }
    #[test]
    fn test_idempotence_all_calendars() {
        let dates = vec![
            (1970, 1, 1),
            (1972, 1, 1), // leap year
            (1980, 1, 1),
            (2020, 1, 1),
            (100_000, 1, 1),
            (1980, 6, 15),
            (1969, 1, 1),
            (-1_000_000, 1, 1),
            (-100_000, 1, 1),
            (1960, 1, 1), // leap year
            (1980, 6, 15),
            // Encountered issue for 2001-01-03
            (2001, 1, 3),
        ];
        let cals = vec![
            calendars::Calendar::Day360,
            calendars::Calendar::Standard,
            calendars::Calendar::ProlepticGregorian,
            calendars::Calendar::Julian,
            calendars::Calendar::NoLeap,
            calendars::Calendar::AllLeap,
        ];
        for cal in cals {
            for date in dates.clone() {
                let (year, month, day) = date;
                let datetime: CFDatetime = CFDatetime::from_ymd(year, month, day, cal).unwrap();
                let (expected_year, expected_month, expected_day) = datetime.ymd().unwrap();
                assert_eq!(expected_year, year);
                assert_eq!(expected_month, month);
                assert_eq!(expected_day, day);
            }
        }
    }
    #[test]
    fn test_add_duration() {
        let cals = vec![
            calendars::Calendar::Day360,
            calendars::Calendar::Standard,
            calendars::Calendar::ProlepticGregorian,
            calendars::Calendar::Julian,
            calendars::Calendar::NoLeap,
            calendars::Calendar::AllLeap,
        ];
        for calendar in cals {
            let duration_expected = vec![
                (CFDuration::from_hours(1, calendar), (1970, 1, 1, 1, 0, 0)),
                (CFDuration::from_minutes(1, calendar), (1970, 1, 1, 0, 1, 0)),
                (CFDuration::from_seconds(1, calendar), (1970, 1, 1, 0, 0, 1)),
                (CFDuration::from_days(1, calendar), (1970, 1, 2, 0, 0, 0)),
            ];
            for (duration, expected) in duration_expected {
                let datetime = CFDatetime::from_ymd(1970, 1, 1, calendar).unwrap();
                let new_datetime = datetime + duration;
                assert_eq!(new_datetime.unwrap().ymd_hms().unwrap(), expected);
            }
        }
    }
    #[test]
    fn test_timestamp() {
        let timestamp_expected = vec![
            (0, (1970, 1, 1, 0, 0, 0)),
            (315532800, (1980, 1, 1, 0, 0, 0)),
            (631152000, (1990, 1, 1, 0, 0, 0)),
            (946684800, (2000, 1, 1, 0, 0, 0)),
            (949363200, (2000, 2, 1, 0, 0, 0)),
            (957139200, (2000, 5, 1, 0, 0, 0)),
            (946771200, (2000, 1, 2, 0, 0, 0)),
            // Encountered issue for this one :
            (946857600, (2000, 1, 3, 0, 0, 0)),
        ];
        for (timestamp, expected) in timestamp_expected {
            let datetime = CFDatetime::from_timestamp(timestamp, 0, calendars::Calendar::Standard);
            assert_eq!(datetime.unwrap().ymd_hms().unwrap(), expected);
        }
    }
}
