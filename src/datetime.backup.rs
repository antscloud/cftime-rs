use crate::utils::{get_timestamp_from_ymd, get_ymd_from_timestamp};
use crate::{calendars::Calendar, constants};
/// calendar represents the calendar to use
/// timestamp represents the number of seconds since 1970-01-01 00:00:00 UTC
/// nanoseconds represents the number of nanoseconds to add to the timestamp
pub struct CFDatetime {
    calendar: Calendar,
    timestamp: i64,
    nanoseconds: i64,
}

/// Immplementation of the CF convention specifications :
/// - https://cfconventions.org/Data/cf-conventions/cf-conventions-1.10/cf-conventions.html#time-coordinate
impl CFDatetime {
    /// Creates a new CFDatetime instance.
    ///
    /// # Arguments
    ///
    /// * `calendar` - The calendar to use.
    /// * `timestamp` - The timestamp value.
    /// * `nanoseconds` - The nanoseconds value.
    ///
    /// # Example
    ///
    /// ```
    /// let calendar = Calendar::Standard;
    /// let datetime = CFDatetime::new(calendar, 1623153600, 0);
    /// ```
    pub fn new(calendar: Calendar, timestamp: i64, nanoseconds: i64) -> Self {
        Self {
            calendar,
            timestamp,
            nanoseconds,
        }
    }

    pub fn calendar(&self) -> Calendar {
        self.calendar
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn ymd(&self) -> Result<(i64, u8, u8), crate::errors::Error> {
        match self.calendar {
            Calendar::Day365 | Calendar::NoLeap => {
                Ok(get_ymd_from_timestamp(self.timestamp, &self.calendar))
            }
            Calendar::Day366 | Calendar::AllLeap => {
                Ok(get_ymd_from_timestamp(self.timestamp, &self.calendar))
            }
            Calendar::ProlepticGregorian => {
                Ok(get_ymd_from_timestamp(self.timestamp, &self.calendar))
            }
            Calendar::Julian => Ok(get_ymd_from_timestamp(self.timestamp, &self.calendar)),
            Calendar::Gregorian | Calendar::Standard => {
                //  1582-10-15 0:0:0 is the first day of Greogorian calendar
                // 1582-10-5  0:0:0 is the last day of Julian calendar
                let (year, month, day) = get_ymd_from_timestamp(self.timestamp, &self.calendar);

                if year == 1582 && month == 10 {
                    if day >= 5 && day <= 15 {
                        return Err(crate::errors::Error::InvalidDate(
                            "Date are not defined for standard calendar between 1582-10-5 and 1582-10-15"
                                .to_string(),
                        ));
                    }
                }
                Ok((year, month, day))
            }
            Calendar::Day360 => {
                let mut nb_days = self.timestamp / constants::SECS_PER_DAY as i64;
                let remaining_seconds = self.timestamp % constants::SECS_PER_DAY as i64;
                if remaining_seconds < 0 {
                    nb_days -= 1
                }
                let (nb_year, nb_month_days) = (nb_days / 360, nb_days % 360);

                let (month, day) = (nb_month_days / 30, nb_month_days % 30);
                let year = constants::UNIX_DEFAULT_YEAR as i64 + nb_year;

                Ok((year, (month + 1) as u8, (day + 1) as u8))
            }
        }
    }

    pub fn hms(&self) -> (u8, u8, u8) {
        let _mod_sec = constants::SECS_PER_DAY as i64;
        let seconds = (self.timestamp % constants::SECS_PER_DAY as i64
            + constants::SECS_PER_DAY as i64)
            % constants::SECS_PER_DAY as i64;
        let sec = (seconds % 60) as u8;
        let min = ((seconds / 60) % 60) as u8;
        let hour = ((seconds / 3600) % 24) as u8;
        (hour, min, sec)
    }

    pub fn ymd_hms(&self) -> Result<(i64, u8, u8, u8, u8, u8), crate::errors::Error> {
        let (year, month, day) = self.ymd()?;
        let (hour, min, sec) = self.hms();
        Ok((year, month, day, hour, min, sec))
    }

    pub fn from_hms(hour: u8, min: u8, sec: u8) -> Result<Self, crate::errors::Error> {
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
        if sec > 59 {
            return Err(crate::errors::Error::InvalidTime(
                format!("Second {sec} is out of bounds").to_string(),
            ));
        }
        let seconds = (hour as u32 * constants::SECS_PER_HOUR
            + min as u32 * constants::SECS_PER_MINUTE
            + sec as u32)
            % constants::SECS_PER_DAY;

        // Ensure the result is positive
        let seconds_positive = (seconds + constants::SECS_PER_DAY) % constants::SECS_PER_DAY;

        Ok(Self {
            calendar: Calendar::ProlepticGregorian,
            timestamp: seconds_positive as i64,
            nanoseconds: 0,
        })
    }

    pub fn from_ymd(
        year: i64,
        month: u8,
        day: u8,
        calendar: Calendar,
    ) -> Result<Self, crate::errors::Error> {
        if month > 12 || month < 1 {
            return Err(crate::errors::Error::InvalidDate(
                format!("Month {month} is out of bounds").to_string(),
            ));
        }
        if day > 31 || day < 1 {
            return Err(crate::errors::Error::InvalidDate(
                format!("Day {day} is out of bounds").to_string(),
            ));
        }
        match &calendar {
            // Proleptic Gregorian is defined as extended Gregorian calendar
            // So the leap year pattern is the same before and after 1582
            Calendar::Day365 | Calendar::NoLeap => {
                let timestamp = get_timestamp_from_ymd(year, month, day, &calendar)?;
                Ok(Self {
                    calendar,
                    timestamp,
                    nanoseconds: 0,
                })
            }
            Calendar::Day366 | Calendar::AllLeap => {
                let timestamp = get_timestamp_from_ymd(year, month, day, &calendar)?;
                Ok(Self {
                    calendar,
                    timestamp,
                    nanoseconds: 0,
                })
            }
            Calendar::ProlepticGregorian => {
                let timestamp = get_timestamp_from_ymd(year, month, day, &calendar)?;
                Ok(Self {
                    calendar,
                    timestamp,
                    nanoseconds: 0,
                })
            }
            Calendar::Julian => {
                let timestamp = get_timestamp_from_ymd(year, month, day, &calendar)?;
                Ok(Self {
                    calendar,
                    timestamp,
                    nanoseconds: 0,
                })
            }
            Calendar::Gregorian | Calendar::Standard => {
                if year == 1582 {
                    if month <= 10 && day < 5 {
                        let timestamp =
                            get_timestamp_from_ymd(year, month, day, &Calendar::Julian)?;
                        return Ok(Self {
                            calendar,
                            timestamp,
                            nanoseconds: 0,
                        });
                    }
                    if month >= 10 && day >= 15 {
                        let timestamp =
                            get_timestamp_from_ymd(year, month, day, &Calendar::Gregorian)?;
                        return Ok(Self {
                            calendar,
                            timestamp,
                            nanoseconds: 0,
                        });
                    }
                    return Err(crate::errors::Error::InvalidDate(
                        "Date are not defined for standard calendar between 1582-10-5 and 1582-10-15"
                            .to_string(),
                    ));
                } else if year < 1582 {
                    let timestamp = get_timestamp_from_ymd(year, month, day, &Calendar::Julian)?;
                    return Ok(Self {
                        calendar,
                        timestamp,
                        nanoseconds: 0,
                    });
                } else {
                    let timestamp = get_timestamp_from_ymd(year, month, day, &Calendar::Gregorian)?;
                    return Ok(Self {
                        calendar,
                        timestamp,
                        nanoseconds: 0,
                    });
                }
            }
            Calendar::Day360 => {
                let mut timestamp: i64 = 0;

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
                    calendar: calendar,
                    timestamp,
                    nanoseconds: 0,
                })
            }
        }
    }
    pub fn from_timestamp(timestamp: i64, calendar: Calendar) -> Self {
        Self {
            calendar: calendar,
            timestamp,
            nanoseconds: 0,
        }
    }

    pub fn hours(&self) -> u8 {
        let (hour, _, _) = self.hms();
        hour
    }
    pub fn minutes(&self) -> u8 {
        let (_, min, _) = self.hms();
        min
    }
    pub fn seconds(&self) -> u8 {
        let (_, _, sec) = self.hms();
        sec
    }
    pub fn nanoseconds(&self) -> i64 {
        self.nanoseconds
    }
}
