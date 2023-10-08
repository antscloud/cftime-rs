use crate::{calendars::Calendar, timezone::Tz};

pub trait IsLeap {
    fn is_leap(year: i64) -> bool;
}

pub trait CalendarDatetime {
    fn ymd_hms(&self) -> Result<(i64, u8, u8, u8, u8, u8), crate::errors::Error>;
    fn timestamp(&self) -> i64;
    fn nanoseconds(&self) -> u32;
    fn timezone(&self) -> Tz;
    fn calendar(&self) -> Calendar;
}
pub trait CalendarDatetimeCreator
where
    Self: Sized,
{
    fn from_ymd_hms(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f32,
    ) -> Result<Self, crate::errors::Error>;
    fn from_timestamp(timestamp: i64, nanoseconds: u32) -> Self;
}
