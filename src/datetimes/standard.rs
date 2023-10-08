use crate::calendars::Calendar;
use crate::datetimes::traits::{CalendarDatetime, IsLeap};
use crate::timezone::Tz;
use crate::utils::{
    get_timestamp_from_hms, get_timestamp_from_ymd, get_ymd_hms_from_timestamp, is_leap_gregorian,
    is_leap_julian,
};

use super::traits::CalendarDatetimeCreator;
pub struct StandardDatetime {
    pub timestamp: i64,
    pub nanoseconds: u32,
    pub tz: Tz,
    pub calendar: Calendar,
}

impl StandardDatetime {
    pub fn new(timestamp: i64, nanoseconds: u32, tz: Tz) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz,
            calendar: Calendar::Standard,
        }
    }
}
impl IsLeap for StandardDatetime {
    fn is_leap(year: i64) -> bool {
        if year < 1582 {
            is_leap_julian(year)
        } else {
            is_leap_gregorian(year)
        }
    }
}

impl CalendarDatetime for StandardDatetime {
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
        let gregorian_begin = get_timestamp_from_ymd::<StandardDatetime>(1582, 10, 15)?;
        let mut timestamp = self.timestamp;
        if self.timestamp < gregorian_begin {
            let seconds_in_10_days = 10 * 24 * 60 * 60;
            timestamp -= seconds_in_10_days
        }
        Ok(get_ymd_hms_from_timestamp::<StandardDatetime>(timestamp))
    }
}

impl CalendarDatetimeCreator for StandardDatetime {
    fn from_timestamp(timestamp: i64, _nanoseconds: u32) -> Self {
        Self {
            timestamp,
            nanoseconds: 0,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::Standard,
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
        if year == 1582
            && month == 10
            && ((day == 4 && (hour > 0 || minute > 0 || second > 0.0)) || (5..15).contains(&day))
        {
            return Err(crate::errors::Error::InvalidDate(
                "Date between 1582-10-04 and 1582-10-15 are not defined in the standard calendar"
                    .to_string(),
            ));
        }
        if year < 1582 || (year == 1582 && month < 10) || (year == 1582 && month == 10 && day < 15)
        {
            // Add 10 days from julian / gregorian break
            timestamp += 10 * 24 * 60 * 60
        }

        timestamp += get_timestamp_from_ymd::<StandardDatetime>(year, month, day)?;
        Ok(Self {
            timestamp,
            nanoseconds,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::Standard,
        })
    }
}
