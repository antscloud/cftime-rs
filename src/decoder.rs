use crate::utils::get_datetime_and_unit_from_units;
use crate::{
    calendars::Calendar, datetime::CFDatetime, duration::CFDuration, parser::parse_cf_time,
};
pub trait CFDecoder {
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
    fn test_decode_vec_cf_with_hms() {
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

    // Add more test cases for other scenarios as needed
}
