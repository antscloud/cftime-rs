use crate::calendars::Calendar;
use crate::datetimes::traits::{CalendarDatetime, IsLeap};
use crate::timezone::Tz;
use crate::utils::{
    get_timestamp_from_hms, get_timestamp_from_ymd, get_ymd_hms_from_timestamp, is_leap_gregorian,
};

use super::traits::CalendarDatetimeCreator;
pub struct ProlepticGregorianDatetime {
    pub timestamp: i64,
    pub nanoseconds: u32,
    pub tz: Tz,
    pub calendar: Calendar,
}

impl ProlepticGregorianDatetime {
    pub fn new(timestamp: i64, nanoseconds: u32, tz: Tz) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz,
            calendar: Calendar::ProlepticGregorian,
        }
    }
}
impl IsLeap for ProlepticGregorianDatetime {
    fn is_leap(year: i64) -> bool {
        is_leap_gregorian(year)
    }
}

impl CalendarDatetime for ProlepticGregorianDatetime {
    fn timestamp(&self) -> i64 {
        self.timestamp
    }
    fn nanoseconds(&self) -> u32 {
        self.nanoseconds
    }
    fn calendar(&self) -> Calendar {
        self.calendar
    }
    fn timezone(&self) -> Tz {
        self.tz
    }
    fn ymd_hms(&self) -> Result<(i64, u8, u8, u8, u8, u8), crate::errors::Error> {
        Ok(get_ymd_hms_from_timestamp::<ProlepticGregorianDatetime>(
            self.timestamp,
        ))
    }
}
impl CalendarDatetimeCreator for ProlepticGregorianDatetime {
    fn from_timestamp(timestamp: i64, nanoseconds: u32) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::ProlepticGregorian,
        }
    }
    fn from_ymd_hms(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f32,
    ) -> Result<Self, crate::errors::Error> {
        let (mut timestamp, nanoseconds) = get_timestamp_from_hms(hour, minute, second)?;
        timestamp += get_timestamp_from_ymd::<ProlepticGregorianDatetime>(year, month, day)?;
        Ok(Self {
            timestamp,
            nanoseconds,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::ProlepticGregorian,
        })
    }
}
