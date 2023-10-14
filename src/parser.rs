//! Module related to parsing the date units
//! Create a `ParsedDatetime` from units

use crate::{calendars::Calendar, duration::CFDuration};

#[derive(Debug, PartialEq)]
pub enum Unit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

impl Unit {
    pub fn to_duration(&self, calendar: Calendar) -> CFDuration {
        match self {
            Unit::Year => CFDuration::from_years(1, calendar),
            Unit::Month => CFDuration::from_months(1, calendar),
            Unit::Day => CFDuration::from_days(1, calendar),
            Unit::Hour => CFDuration::from_hours(1, calendar),
            Unit::Minute => CFDuration::from_minutes(1, calendar),
            Unit::Second => CFDuration::from_seconds(1, calendar),
            Unit::Millisecond => CFDuration::from_milliseconds(1, calendar),
            Unit::Microsecond => CFDuration::from_microseconds(1, calendar),
            Unit::Nanosecond => CFDuration::from_nanoseconds(1, calendar),
        }
    }
}
#[derive(Debug)]
pub struct ParsedDatetime {
    pub ymd: (i64, u8, u8),
    pub hms: Option<(u8, u8, f32)>,
    pub tz: Option<(i8, u8)>,
    pub nanosecond: Option<i64>,
}
#[derive(Debug)]
pub struct ParsedCFTime {
    pub unit: Unit,
    pub datetime: ParsedDatetime,
}
pub fn parse_cf_time(unit: &str) -> Result<ParsedCFTime, crate::errors::Error> {
    let mut matches: Vec<&str> = unit.split(' ').collect();
    // Remove empty strings
    matches.retain(|&s| !s.trim().is_empty());
    if matches.len() < 3 {
        return Err(crate::errors::Error::UnitParserError(unit.to_string()));
    }

    let duration_unit = match matches[0] {
        "common_years" | "common_year" => Unit::Year,
        "months" | "month" => Unit::Month,
        "days" | "day" | "d" => Unit::Day,
        "hours" | "hour" | "hrs" | "hr" | "h" => Unit::Hour,
        "minutes" | "minute" | "mins" | "min" => Unit::Minute,
        "seconds" | "second" | "secs" | "sec" | "s" => Unit::Second,
        "milliseconds" | "millisecond" | "millisecs" | "millisec" | "msecs" | "msec" | "ms" => {
            Unit::Millisecond
        }
        "microseconds" | "microsecond" | "microsecs" | "microsec" => Unit::Microsecond,
        _ => {
            return Err(crate::errors::Error::UnitParserError(
                format!("Invalid duration unit: {unit}").to_string(),
            ))
        }
    };

    if matches[1] != "since" {
        return Err(crate::errors::Error::UnitParserError(
            format!("Expected 'since' found : '{}'", matches[1]).to_string(),
        ));
    }

    let date: Vec<&str> = matches[2].split('-').collect();
    if date.len() != 3 {
        return Err(crate::errors::Error::UnitParserError(
            format!("Invalid date: {unit}").to_string(),
        ));
    }
    let year = date[0].parse::<i64>()?;
    let month = date[1].parse::<u8>()?;
    let day = date[2].parse::<u8>()?;

    if matches.len() <= 3 {
        return Ok(ParsedCFTime {
            unit: duration_unit,
            datetime: ParsedDatetime {
                ymd: (year, month, day),
                hms: None,
                tz: None,
                nanosecond: None,
            },
        });
    }

    let time: Vec<&str> = matches[3].split(':').collect();
    if time.len() != 3 {
        return Err(crate::errors::Error::UnitParserError(
            format!("Invalid time: {unit}").to_string(),
        ));
    }
    let hour = time[0].parse::<u8>()?;
    let minute = time[1].parse::<u8>()?;
    let second = time[2].parse::<f32>()?;

    if matches.len() <= 4 {
        return Ok(ParsedCFTime {
            unit: duration_unit,
            datetime: ParsedDatetime {
                ymd: (year, month, day),
                hms: Some((hour, minute, second)),
                tz: None,
                nanosecond: None,
            },
        });
    }

    let tz: Vec<&str> = matches[4].split(':').collect();
    if tz.len() > 2 || tz.len() <= 0 {
        return Err(crate::errors::Error::UnitParserError(
            format!("Invalid time zone: {unit}").to_string(),
        ));
    }
    let mut tzhour = 0;
    let mut tzminute = 0;
    if tz.len() == 1 {
        tzhour = tz[0].parse::<i8>()?;
        tzminute = 0;
    } else if tz.len() == 2 {
        tzhour = tz[0].parse::<i8>()?;
        tzminute = tz[1].parse::<u8>()?;
    }
    Ok(ParsedCFTime {
        unit: duration_unit,
        datetime: ParsedDatetime {
            ymd: (year, month, day),
            hms: Some((hour, minute, second)),
            tz: Some((tzhour, tzminute)),
            nanosecond: None,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_duration_units() {
        // Test valid duration units
        let units = vec![
            ("common_years since 2023-01-01", Unit::Year),
            ("months since 2023-01-01", Unit::Month),
            ("day since 2023-01-01", Unit::Day),
            // Add more valid units here
        ];

        for (input, expected_unit) in units {
            let result = parse_cf_time(input).unwrap();
            assert!(result.unit == expected_unit);
            assert_eq!(result.datetime.ymd, (2023, 1, 1));
            assert_eq!(result.datetime.hms, None);
            assert_eq!(result.datetime.tz, None);
            assert_eq!(result.datetime.nanosecond, None);
        }
    }

    #[test]
    fn test_valid_date_time_units() {
        // Test valid date and time units with different combinations
        let units = vec![
            // From CF conventions
            (
                "seconds since 1992-10-8 15:15:42.5 -6:00",
                ParsedCFTime {
                    unit: Unit::Second,
                    datetime: ParsedDatetime {
                        ymd: (1992, 10, 8),
                        hms: Some((15, 15, 42.5)),
                        tz: Some((-6, 0)),
                        nanosecond: None,
                    },
                },
            ),
            // Date, no time, no timezone
            (
                "seconds since 1992-10-08",
                ParsedCFTime {
                    unit: Unit::Second,
                    datetime: ParsedDatetime {
                        ymd: (1992, 10, 8),
                        hms: None,
                        tz: None,
                        nanosecond: None,
                    },
                },
            ),
            (
                "minutes since 2000-01-01",
                ParsedCFTime {
                    unit: Unit::Minute,
                    datetime: ParsedDatetime {
                        ymd: (2000, 1, 1),
                        hms: None,
                        tz: None,
                        nanosecond: None,
                    },
                },
            ),
            (
                "hour since 1985-12-31",
                ParsedCFTime {
                    unit: Unit::Hour,
                    datetime: ParsedDatetime {
                        ymd: (1985, 12, 31),
                        hms: None,
                        tz: None,
                        nanosecond: None,
                    },
                },
            ),
            // Date and time, no timezone
            (
                "seconds since 2022-11-30 10:15:20",
                ParsedCFTime {
                    unit: Unit::Second,
                    datetime: ParsedDatetime {
                        ymd: (2022, 11, 30),
                        hms: Some((10, 15, 20.0)),
                        tz: None,
                        nanosecond: None,
                    },
                },
            ),
            (
                "minutes since 2010-05-15 05:30:00",
                ParsedCFTime {
                    unit: Unit::Minute,
                    datetime: ParsedDatetime {
                        ymd: (2010, 5, 15),
                        hms: Some((5, 30, 0.0)),
                        tz: None,
                        nanosecond: None,
                    },
                },
            ),
            (
                "hour since 1999-03-20 12:00:01",
                ParsedCFTime {
                    unit: Unit::Hour,
                    datetime: ParsedDatetime {
                        ymd: (1999, 3, 20),
                        hms: Some((12, 0, 1.0)),
                        tz: None,
                        nanosecond: None,
                    },
                },
            ),
            // Date, time, and timezone
            (
                "seconds since 2015-07-04 16:45:30 +02:30",
                ParsedCFTime {
                    unit: Unit::Second,
                    datetime: ParsedDatetime {
                        ymd: (2015, 7, 4),
                        hms: Some((16, 45, 30.0)),
                        tz: Some((2, 30)),
                        nanosecond: None,
                    },
                },
            ),
            (
                "minutes since 2023-12-25 08:00:00 -05:00",
                ParsedCFTime {
                    unit: Unit::Minute,
                    datetime: ParsedDatetime {
                        ymd: (2023, 12, 25),
                        hms: Some((8, 0, 0.0)),
                        tz: Some((-5, 0)),
                        nanosecond: None,
                    },
                },
            ),
            (
                "hour since 2018-09-10 00:00:00 -03:30",
                ParsedCFTime {
                    unit: Unit::Hour,
                    datetime: ParsedDatetime {
                        ymd: (2018, 9, 10),
                        hms: Some((0, 0, 0.0)),
                        tz: Some((-3, 30)),
                        nanosecond: None,
                    },
                },
            ),
        ];

        for (input, expected_unit) in units {
            let result = parse_cf_time(input).unwrap();
            assert!(result.unit == expected_unit.unit);
            assert_eq!(result.datetime.ymd, expected_unit.datetime.ymd);
            assert_eq!(result.datetime.hms, expected_unit.datetime.hms);
            assert_eq!(result.datetime.tz, expected_unit.datetime.tz);
            assert_eq!(
                result.datetime.nanosecond,
                expected_unit.datetime.nanosecond
            );
        }
    }
    #[test]
    fn test_not_valid_date_time_units() {
        // Test valid date and time units with different combinations
        let units = vec![
            "seconds since 2019-06-15 -07:00",
            "nanoseconds since 2020-01-01 9876543210", // nanoseconds not permitted
            "invalid_unit since 2023-01-01",           // Invalid unit
            "hou since 2023-01-01",                    // Missing 'rs' in 'hours'
            "minutes 2023-01-01",                      // Missing 'since'
        ];

        for input in units {
            let result = parse_cf_time(input);
            assert!(matches!(
                result.err().unwrap(),
                crate::errors::Error::UnitParserError(_)
            ))
        }
    }
    // Add more tests for different valid date and time scenarios
}
