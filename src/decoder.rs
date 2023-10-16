//! Module that implements the decode_cf method for `i32`, `i64`, `f32`, `f64`,  `Vec<i32>`, `Vec<i64>`, `Vec<f32>` and `Vec<f64>`.

use crate::utils::get_datetime_and_unit_from_units;
use crate::{calendars::Calendar, datetime::CFDatetime};

/// Trait for decoding CFDatetime from units and calendar
pub trait CFDecoder {
    /// Decodes the given units and calendar into a CFDatetime.
    ///
    /// # Arguments
    ///
    /// * `units` - The units to decode.
    /// * `calendar` - The calendar to use for decoding.
    ///
    /// # Returns
    ///
    /// A Result containing the decoded CFDatetime if successful, or an Error if decoding fails.
    fn decode_cf(
        &self,
        units: &str,
        calendar: Calendar,
    ) -> Result<CFDatetime, crate::errors::Error>;
}

macro_rules! impl_cf_decoder {
    ($type:ty) => {
        impl CFDecoder for $type {
            fn decode_cf(
                &self,
                units: &str,
                calendar: Calendar,
            ) -> Result<CFDatetime, crate::errors::Error> {
                let (cf_datetime, unit) = get_datetime_and_unit_from_units(units, calendar)?;
                let duration = unit.to_duration(calendar);
                let result = (&cf_datetime + (&duration * *self))?;

                Ok(result)
            }
        }
    };
}

impl_cf_decoder!(i64);
impl_cf_decoder!(i32);
impl_cf_decoder!(f32);
impl_cf_decoder!(f64);

pub trait VecCFDecoder {
    fn decode_cf(
        &self,
        units: &str,
        calendar: Calendar,
    ) -> Result<Vec<CFDatetime>, crate::errors::Error>;
}

macro_rules! impl_vec_cf_decoder {
    ($type:ty) => {
        impl VecCFDecoder for Vec<$type> {
            fn decode_cf(
                &self,
                units: &str,
                calendar: Calendar,
            ) -> Result<Vec<CFDatetime>, crate::errors::Error> {
                let (cf_datetime, unit) = get_datetime_and_unit_from_units(units, calendar)?;
                let duration = unit.to_duration(calendar);
                let mut datetimes = Vec::with_capacity(self.len());
                for value in self {
                    let new_datetime = &cf_datetime + (&duration * *value);
                    datetimes.push(new_datetime?);
                }

                Ok(datetimes)
            }
        }
    };
}

impl_vec_cf_decoder!(i64);
impl_vec_cf_decoder!(i32);
impl_vec_cf_decoder!(f32);
impl_vec_cf_decoder!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_i64_cf_with_hms() {
        let to_decode = 2;
        let units = "days since 2000-01-01 00:00:00";
        let calendar = Calendar::Standard;

        let result = to_decode.decode_cf(units, calendar);

        // Assert
        assert!(result.is_ok());
        let cf_datetime = result.unwrap();
        let (year, month, day, hour, minute, second) = cf_datetime.ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 3, 0, 0, 0)
        );
    }
    #[test]
    fn test_decode_f64_cf_with_hms() {
        let to_decode: f64 = 2.0;
        let units = "hours since 2000-01-01 00:00:00";
        let calendar = Calendar::Standard;

        let result = to_decode.decode_cf(units, calendar);

        // Assert
        assert!(result.is_ok());
        let cf_datetime = result.unwrap();
        let (year, month, day, hour, minute, second) = cf_datetime.ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 1, 2, 0, 0)
        );
    }

    #[test]
    fn test_decode_i32_vec_cf_with_hms() {
        let to_decode = vec![0, 1, 2];
        let units = "days since 2000-01-01 00:00:00";
        let calendar = Calendar::Standard;

        let datetimes_result = to_decode.decode_cf(units, calendar);

        // Assert
        assert!(datetimes_result.is_ok());
        let datetimes = datetimes_result.unwrap();
        for (i, datetime) in datetimes.iter().enumerate() {
            let (year, month, day, hour, minute, second) = datetime.ymd_hms().unwrap();
            assert_eq!(
                (year, month, day, hour, minute, second),
                (2000, 1, (i + 1) as u8, 0, 0, 0)
            );
        }
    }
    #[test]
    fn test_decode_f64_vec_cf_with_hms() {
        let to_decode: Vec<f64> = vec![1.0, 1.25, 1.50, 1.75, 2.0];
        let units = "hours since 2000-01-01 00:00:00";
        let calendar = Calendar::Standard;

        let datetimes_result = to_decode.decode_cf(units, calendar);

        // Assert
        assert!(datetimes_result.is_ok());
        let datetimes = datetimes_result.unwrap();
        let (year, month, day, hour, minute, second) = datetimes[0].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 1, 1, 0, 0)
        );
        let (year, month, day, hour, minute, second) = datetimes[1].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 1, 1, 15, 0)
        );
        let (year, month, day, hour, minute, second) = datetimes[2].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 1, 1, 30, 0)
        );
        let (year, month, day, hour, minute, second) = datetimes[3].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 1, 1, 45, 0)
        );
        let (year, month, day, hour, minute, second) = datetimes[4].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2000, 1, 1, 2, 0, 0)
        );
    }
    #[test]
    fn test_decode_95795_from_days() {
        // This error originates from Python bindings
        // From 95795.0 it gives (2232, 4, 11, 23, 57, 52) instead of (2232, 4, 12, 0, 0, 0)
        // But this worked for int
        let to_decode: Vec<f32> = vec![95795.0];
        let units = "days since 1970-01-01";
        let calendar = Calendar::Standard;

        let datetimes_result = to_decode.decode_cf(units, calendar);
        let datetimes = datetimes_result.unwrap();
        let (year, month, day, hour, minute, second) = datetimes[0].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2232, 4, 12, 0, 0, 0)
        );

        let to_decode: Vec<f64> = vec![95795.0];
        let units = "days since 1970-01-01";
        let calendar = Calendar::Standard;

        let datetimes_result = to_decode.decode_cf(units, calendar);
        let datetimes = datetimes_result.unwrap();
        let (year, month, day, hour, minute, second) = datetimes[0].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2232, 4, 12, 0, 0, 0)
        );
        let to_decode: Vec<i64> = vec![95795];
        let units = "days since 1970-01-01";
        let calendar = Calendar::Standard;

        let datetimes_result = to_decode.decode_cf(units, calendar);
        let datetimes = datetimes_result.unwrap();
        let (year, month, day, hour, minute, second) = datetimes[0].ymd_hms().unwrap();
        assert_eq!(
            (year, month, day, hour, minute, second),
            (2232, 4, 12, 0, 0, 0)
        );
    }
    // Add more test cases for other scenarios as needed
}
