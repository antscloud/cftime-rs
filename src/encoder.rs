//! Module that implements the encode_cf method for`CFDatetime` and `Vec<CFDatetime>`.

use crate::{
    calendars::Calendar,
    datetime::CFDatetime,
    utils::{get_datetime_and_unit_from_units, unit_to_encode},
};

/// This trait represents a CFEncoder.
/// A CFEncoder is responsible for encoding [CFDatetime] into a specific format.
pub trait CFEncoder<T> {
    /// Encodes the data into a specific format.
    ///
    /// # Arguments
    ///
    /// * `units` - The units of the data.
    /// * `calendar` - The calendar to use.
    ///
    /// # Returns
    ///
    /// The encoded data as a Result<T, crate::errors::Error>.
    fn encode_cf(&self, units: &str, calendar: Calendar) -> Result<T, crate::errors::Error>;
}

macro_rules! impl_cf_encoder {
    ($type:ty) => {
        impl CFEncoder<$type> for CFDatetime {
            fn encode_cf(
                &self,
                units: &str,
                calendar: Calendar,
            ) -> Result<$type, crate::errors::Error> {
                let (cf_datetime, unit) = get_datetime_and_unit_from_units(units, calendar)?;
                let duration = (self - cf_datetime)?;
                let result = unit_to_encode(&unit, duration);
                Ok(result as $type)
            }
        }
    };
}

impl_cf_encoder!(i64);
impl_cf_encoder!(i32);
impl_cf_encoder!(f32);
impl_cf_encoder!(f64);

macro_rules! impl_vec_cf_encoder {
    ($type:ty) => {
        impl CFEncoder<Vec<$type>> for Vec<CFDatetime> {
            fn encode_cf(
                &self,
                units: &str,
                calendar: Calendar,
            ) -> Result<Vec<$type>, crate::errors::Error> {
                let (cf_datetime, unit) = get_datetime_and_unit_from_units(units, calendar)?;
                let mut result: Vec<$type> = Vec::with_capacity(self.len());
                for datetime in self {
                    let duration = (datetime - &cf_datetime)?;
                    result.push(unit_to_encode(&unit, duration) as $type);
                }
                Ok(result)
            }
        }
    };
}

impl_vec_cf_encoder!(i64);
impl_vec_cf_encoder!(i32);
impl_vec_cf_encoder!(f32);
impl_vec_cf_encoder!(f64);

macro_rules! impl_vec_ref_cf_encoder {
    ($type:ty) => {
        impl CFEncoder<Vec<$type>> for Vec<&CFDatetime> {
            fn encode_cf(
                &self,
                units: &str,
                calendar: Calendar,
            ) -> Result<Vec<$type>, crate::errors::Error> {
                let (cf_datetime, unit) = get_datetime_and_unit_from_units(units, calendar)?;
                let mut result: Vec<$type> = Vec::with_capacity(self.len());
                for datetime in self {
                    let duration = (*datetime - &cf_datetime)?;
                    result.push(unit_to_encode(&unit, duration) as $type);
                }
                Ok(result)
            }
        }
    };
}

impl_vec_ref_cf_encoder!(i64);
impl_vec_ref_cf_encoder!(i32);
impl_vec_ref_cf_encoder!(f32);
impl_vec_ref_cf_encoder!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_cf() {
        let datetime = CFDatetime::from_ymd(2000, 1, 1, Calendar::Standard).unwrap();
        let result: i64 = datetime
            .encode_cf("seconds since 2000-01-01 00:00:00", Calendar::Standard)
            .unwrap();
        assert_eq!(result, 0);
        let datetime = CFDatetime::from_ymd(2023, 1, 1, Calendar::Standard).unwrap();
        let result: i64 = datetime
            .encode_cf("seconds since 1970-01-01 00:00:00", Calendar::Standard)
            .unwrap();
        assert_eq!(result, 1672531200);
    }
    #[test]
    fn test_vec_encode_cf() {
        let datetimes = vec![
            CFDatetime::from_ymd(2000, 1, 1, Calendar::Standard).unwrap(),
            CFDatetime::from_ymd(2000, 1, 2, Calendar::Standard).unwrap(),
            CFDatetime::from_ymd(2000, 1, 3, Calendar::Standard).unwrap(),
        ];
        let result: Vec<i64> = datetimes
            .encode_cf("seconds since 2000-01-01 00:00:00", Calendar::Standard)
            .unwrap();
        assert_eq!(result, vec![0, 86400, 172800]);
    }
}
