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

pub struct CFDatetime {
    inner: Box<dyn CalendarDatetime + Send + Sync>,
}

/// Immplementation of the CF convention specifications :
/// - [CF Conventions](https://cfconventions.org/Data/cf-conventions/cf-conventions-1.10/cf-conventions.html#time-coordinate)
impl CFDatetime {
    pub fn calendar(&self) -> Calendar {
        self.inner.calendar()
    }

    pub fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    pub fn ymd(&self) -> Result<(i64, u8, u8), crate::errors::Error> {
        let (year, month, day, _, _, _) = self.ymd_hms()?;
        Ok((year, month, day))
    }

    pub fn hms(&self) -> Result<(u8, u8, u8), crate::errors::Error> {
        let (_, _, _, hour, min, sec) = self.ymd_hms()?;
        Ok((hour, min, sec))
    }

    pub fn ymd_hms(&self) -> Result<(i64, u8, u8, u8, u8, u8), crate::errors::Error> {
        self.inner.ymd_hms()
    }

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
            Calendar::Standard | Calendar::Gregorian => Ok(Self {
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
            Calendar::NoLeap | Calendar::Day365 => Ok(Self {
                inner: Box::new(NoLeapDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
            Calendar::AllLeap | Calendar::Day366 => Ok(Self {
                inner: Box::new(AllLeapDatetime::from_ymd_hms(
                    year, month, day, hour, minute, second,
                )?),
            }),
        }
    }
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

    pub fn from_ymd(
        year: i64,
        month: u8,
        day: u8,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        Self::from_ymd_hms(year, month, day, 0, 0, 0.0, calendar)
    }
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
            Calendar::Standard | Calendar::Gregorian => Ok(Self {
                inner: Box::new(StandardDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::Day360 => Ok(Self {
                inner: Box::new(Day360Datetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::Julian => Ok(Self {
                inner: Box::new(JulianDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::NoLeap | Calendar::Day365 => Ok(Self {
                inner: Box::new(NoLeapDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
            Calendar::AllLeap | Calendar::Day366 => Ok(Self {
                inner: Box::new(AllLeapDatetime::from_timestamp(timestamp, nanoseconds)),
            }),
        }
    }
    pub fn hours(&self) -> Result<u8, crate::errors::Error> {
        let (hour, _, _) = self.hms()?;
        Ok(hour)
    }
    pub fn minutes(&self) -> Result<u8, crate::errors::Error> {
        let (_, min, _) = self.hms()?;
        Ok(min)
    }
    pub fn seconds(&self) -> Result<u8, crate::errors::Error> {
        let (_, _, sec) = self.hms()?;
        Ok(sec)
    }
    pub fn nanoseconds(&self) -> u32 {
        self.inner.nanoseconds()
    }
}

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

impl std::ops::Add<CFDuration> for CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn add(self, rhs: CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 + rhs.nanoseconds as i64;
        let (_remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = self.timestamp() + rhs.seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}

impl std::ops::Add<&CFDuration> for CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn add(self, rhs: &CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 + rhs.nanoseconds as i64;
        let (_remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = self.timestamp() + rhs.seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}

impl std::ops::Add<CFDuration> for &CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn add(self, rhs: CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 + rhs.nanoseconds as i64;
        let (_remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = self.timestamp() + rhs.seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}

impl std::ops::Add<&CFDuration> for &CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn add(self, rhs: &CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 + rhs.nanoseconds as i64;
        let (_remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = self.timestamp() + rhs.seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}

impl std::ops::Sub<CFDuration> for CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn sub(self, rhs: CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds as i64;
        let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = (self.timestamp() - rhs.seconds) + remaining_seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}
impl std::ops::Sub<CFDuration> for &CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn sub(self, rhs: CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds as i64;
        let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = (self.timestamp() - rhs.seconds) + remaining_seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}
impl std::ops::Sub<&CFDuration> for CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn sub(self, rhs: &CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds as i64;
        let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = (self.timestamp() - rhs.seconds) + remaining_seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}

impl std::ops::Sub<&CFDuration> for &CFDatetime {
    type Output = Result<CFDatetime, crate::errors::Error>;
    fn sub(self, rhs: &CFDuration) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds as i64;
        let (remaining_seconds, remaining_nanoseconds) = normalize_nanoseconds(nanoseconds);
        let new_timestamp = (self.timestamp() - rhs.seconds) + remaining_seconds;
        CFDatetime::from_timestamp(new_timestamp, remaining_nanoseconds, self.calendar())
    }
}

impl std::ops::Sub<CFDatetime> for CFDatetime {
    type Output = CFDuration;
    fn sub(self, rhs: CFDatetime) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds() as i64;
        let new_timestamp = self.timestamp() - rhs.timestamp();
        CFDuration::new(new_timestamp, nanoseconds, self.calendar())
    }
}
impl std::ops::Sub<&CFDatetime> for &CFDatetime {
    type Output = CFDuration;
    fn sub(self, rhs: &CFDatetime) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds() as i64;
        let new_timestamp = self.timestamp() - rhs.timestamp();
        CFDuration::new(new_timestamp, nanoseconds, self.calendar())
    }
}
impl std::ops::Sub<CFDatetime> for &CFDatetime {
    type Output = CFDuration;
    fn sub(self, rhs: CFDatetime) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds() as i64;
        let new_timestamp = self.timestamp() - rhs.timestamp();
        CFDuration::new(new_timestamp, nanoseconds, self.calendar())
    }
}
impl std::ops::Sub<&CFDatetime> for CFDatetime {
    type Output = CFDuration;
    fn sub(self, rhs: &CFDatetime) -> Self::Output {
        let nanoseconds = self.nanoseconds() as i64 - rhs.nanoseconds() as i64;
        let new_timestamp = self.timestamp() - rhs.timestamp();
        CFDuration::new(new_timestamp, nanoseconds, self.calendar())
    }
}

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
            calendars::Calendar::Day365,
            calendars::Calendar::Day366,
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
            calendars::Calendar::Day365,
            calendars::Calendar::Day366,
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
            calendars::Calendar::Day365,
            calendars::Calendar::Day366,
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
