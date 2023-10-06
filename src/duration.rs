use crate::{calendars::Calendar, constants};

#[derive(Debug)]
pub struct CFDuration {
    pub seconds: i64,
    pub nanoseconds: i64,
    pub calendar: Calendar,
}

impl CFDuration {
    pub fn new(seconds: i64, nanoseconds: i64, calendar: Calendar) -> Self {
        let mut new_seconds = seconds;
        if nanoseconds > 1_000_000_000 {
            new_seconds = new_seconds + (nanoseconds / 1_000_000_000) as i64;
        }
        Self {
            seconds: new_seconds,
            nanoseconds: nanoseconds % 1_000_000_000,
            calendar,
        }
    }
}

impl CFDuration {
    /// Makes a new `Duration` with given number of years.
    /// Depends on the Calendar definitions found in https://github.com/nco/nco/blob/master/data/udunits.dat
    pub fn years(years: i64, calendar: Calendar) -> CFDuration {
        let secs_per_year = match calendar {
            Calendar::Gregorian => 365.2425 * constants::SECS_PER_DAY as f64,
            Calendar::ProlepticGregorian | Calendar::Standard => 3.15569259747e7,
            Calendar::NoLeap | Calendar::Day365 => 365.0 * constants::SECS_PER_DAY as f64,
            Calendar::AllLeap | Calendar::Day366 => 366.0 * constants::SECS_PER_DAY as f64,
            Calendar::Julian => 365.25 * constants::SECS_PER_DAY as f64,
            Calendar::Day360 => 360.0 * constants::SECS_PER_DAY as f64,
        };
        let secs = secs_per_year as i64 * years;
        Self::new(secs, 0, calendar)
    }

    pub fn months(months: i64, calendar: Calendar) -> CFDuration {
        let seconds_for_one_year = CFDuration::years(1, calendar).seconds;
        Self::new(seconds_for_one_year / months, 0, calendar)
    }
    pub fn weeks(weeks: i64, calendar: Calendar) -> CFDuration {
        Self::new(weeks * 7 * 24 * 60 * 60, 0, calendar)
    }
    pub fn days(days: i64, calendar: Calendar) -> CFDuration {
        Self::new(days * 24 * 60 * 60, 0, calendar)
    }
    pub fn hours(hours: i64, calendar: Calendar) -> CFDuration {
        Self::new(hours * 60 * 60, 0, calendar)
    }
    pub fn minutes(minutes: i64, calendar: Calendar) -> CFDuration {
        Self::new(minutes * 60, 0, calendar)
    }
    pub fn seconds(seconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(seconds, 0, calendar)
    }
    pub fn milliseconds(milliseconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(0, milliseconds * 1_000_000, calendar)
    }
    pub fn nanoseconds(nanoseconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(0, nanoseconds, calendar)
    }
    pub fn microseconds(microseconds: i64, calendar: Calendar) -> CFDuration {
        Self::new(1, 1000 * microseconds, calendar)
    }
}

impl std::ops::Add for CFDuration {
    type Output = CFDuration;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.seconds + rhs.seconds,
            self.nanoseconds + rhs.nanoseconds,
            self.calendar,
        )
    }
}
impl std::ops::Sub for CFDuration {
    type Output = CFDuration;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.seconds - rhs.seconds,
            self.nanoseconds - rhs.nanoseconds,
            self.calendar,
        )
    }
}
impl std::ops::Neg for CFDuration {
    type Output = CFDuration;
    fn neg(self) -> Self::Output {
        Self::new(-self.seconds, -self.nanoseconds, self.calendar)
    }
}

impl std::ops::Mul<i64> for CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.seconds * rhs, self.nanoseconds * rhs, self.calendar)
    }
}

impl std::ops::Mul<i64> for &CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: i64) -> Self::Output {
        CFDuration::new(self.seconds * rhs, self.nanoseconds * rhs, self.calendar)
    }
}

impl std::ops::Mul<i32> for CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(
            self.seconds * rhs as i64,
            self.nanoseconds * rhs as i64,
            self.calendar,
        )
    }
}

impl std::ops::Mul<i32> for &CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: i32) -> Self::Output {
        CFDuration::new(
            self.seconds * rhs as i64,
            self.nanoseconds * rhs as i64,
            self.calendar,
        )
    }
}

impl std::ops::Mul<f64> for CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: f64) -> Self::Output {
        let seconds_multiplier = (rhs / 1_000_000.0) as i64;
        let nanoseconds_remaining = (rhs % 1_000_000.0) as i64;
        Self::new(
            self.seconds * seconds_multiplier,
            nanoseconds_remaining,
            self.calendar,
        )
    }
}

impl std::ops::Mul<f64> for &CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: f64) -> Self::Output {
        let seconds_multiplier = (rhs / 1_000_000.0) as i64;
        let nanoseconds_remaining = (rhs % 1_000_000.0) as i64;
        CFDuration::new(
            self.seconds * seconds_multiplier,
            nanoseconds_remaining,
            self.calendar,
        )
    }
}

impl std::ops::Mul<f32> for CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: f32) -> Self::Output {
        let seconds_multiplier = (rhs as f64 / 1_000_000.0) as i64;
        let nanoseconds_remaining = (rhs as f64 % 1_000_000.0) as i64;
        Self::new(
            self.seconds * seconds_multiplier,
            nanoseconds_remaining,
            self.calendar,
        )
    }
}

impl std::ops::Mul<f32> for &CFDuration {
    type Output = CFDuration;
    fn mul(self, rhs: f32) -> Self::Output {
        let seconds_multiplier = (rhs as f64 / 1_000_000.0) as i64;
        let nanoseconds_remaining = (rhs as f64 % 1_000_000.0) as i64;
        CFDuration::new(
            self.seconds * seconds_multiplier,
            nanoseconds_remaining,
            self.calendar,
        )
    }
}
