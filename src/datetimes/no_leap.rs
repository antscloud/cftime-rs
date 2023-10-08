use crate::calendars::Calendar;
use crate::datetimes::traits::{CalendarDatetime, IsLeap};
use crate::timezone::Tz;
use crate::utils::{get_timestamp_from_hms, get_timestamp_from_ymd, get_ymd_hms_from_timestamp};

use super::traits::CalendarDatetimeCreator;

pub struct NoLeapDatetime {
    pub timestamp: i64,
    pub nanoseconds: u32,
    pub tz: Tz,
    pub calendar: Calendar,
}

impl NoLeapDatetime {
    pub fn new(timestamp: i64, nanoseconds: u32, tz: Tz) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz,
            calendar: Calendar::NoLeap,
        }
    }
}
impl IsLeap for NoLeapDatetime {
    fn is_leap(_year: i64) -> bool {
        false
    }
}

impl CalendarDatetime for NoLeapDatetime {
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
        Ok(get_ymd_hms_from_timestamp::<NoLeapDatetime>(self.timestamp))
    }
}
impl CalendarDatetimeCreator for NoLeapDatetime {
    fn from_timestamp(timestamp: i64, nanoseconds: u32) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::NoLeap,
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
        timestamp += get_timestamp_from_ymd::<NoLeapDatetime>(year, month, day)?;
        Ok(Self {
            timestamp,
            nanoseconds,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::NoLeap,
        })
    }
}
