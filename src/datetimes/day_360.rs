use crate::calendars::Calendar;
use crate::constants;
use crate::datetimes::traits::CalendarDatetime;
use crate::timezone::Tz;
use crate::utils::{get_hms_from_timestamp, get_timestamp_from_hms};

use super::traits::CalendarDatetimeCreator;
pub struct Day360Datetime {
    pub timestamp: i64,
    pub nanoseconds: u32,
    pub tz: Tz,
    pub calendar: Calendar,
}

impl Day360Datetime {
    pub fn new(timestamp: i64, nanoseconds: u32, tz: Tz) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz,
            calendar: Calendar::Day360,
        }
    }
}

impl CalendarDatetime for Day360Datetime {
    fn timestamp(&self) -> i64 {
        self.timestamp
    }
    fn nanoseconds(&self) -> u32 {
        self.nanoseconds
    }
    fn timezone(&self) -> Tz {
        self.tz
    }
    fn calendar(&self) -> Calendar {
        self.calendar
    }

    fn ymd_hms(&self) -> Result<(i64, u8, u8, u8, u8, u8), crate::errors::Error> {
        let mut nb_days = self.timestamp / constants::SECS_PER_DAY as i64;
        let remaining_seconds = self.timestamp % constants::SECS_PER_DAY as i64;
        if remaining_seconds < 0 {
            nb_days -= 1
        }
        let (nb_year, nb_month_days) = (nb_days / 360, nb_days % 360);

        let (month, day) = (nb_month_days / 30, nb_month_days % 30);
        let year = constants::UNIX_DEFAULT_YEAR + nb_year;
        let (hour, minute, second) = get_hms_from_timestamp(remaining_seconds);
        Ok((
            year,
            (month + 1) as u8,
            (day + 1) as u8,
            hour,
            minute,
            second,
        ))
    }
}

impl CalendarDatetimeCreator for Day360Datetime {
    fn from_timestamp(timestamp: i64, nanoseconds: u32) -> Self {
        Self {
            timestamp,
            nanoseconds,
            tz: Tz::new(0, 0).unwrap(),
            calendar: Calendar::Day360,
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

        // Calculate years and months
        let mut year = year;
        let month = month as i64 - 1;
        let day = day as i64 - 1;

        loop {
            if year == constants::UNIX_DEFAULT_YEAR {
                break;
            }

            if year > constants::UNIX_DEFAULT_YEAR {
                timestamp += 360 * constants::SECS_PER_DAY as i64;
                year -= 1;
            } else {
                timestamp -= 360 * constants::SECS_PER_DAY as i64;
                year += 1;
            }
        }

        // Calculate days
        timestamp += (month * 30 + day) * constants::SECS_PER_DAY as i64;

        Ok(Self {
            calendar: Calendar::Day360,
            timestamp,
            tz: Tz::new(0, 0).unwrap(),
            nanoseconds,
        })
    }
}
