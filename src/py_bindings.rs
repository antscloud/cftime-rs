use crate::calendars::Calendar;
use crate::datetime::CFDatetime;
use crate::duration::CFDuration;
use crate::encoder::CFEncoder;
use crate::{constants, decoder::*};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct PyCFCalendar {
    pub calendar: Calendar,
}

#[pyclass]
pub struct PyCFDuration {
    pub duration: CFDuration,
}

#[pymethods]
impl PyCFDuration {
    #[new]
    pub fn new(seconds: i64, nanoseconds: i64, calendar: PyCFCalendar) -> Self {
        Self {
            duration: CFDuration::new(seconds, nanoseconds, calendar.calendar),
        }
    }
    /// Makes a new `Duration` with given number of years.
    /// Depends on the Calendar definitions found in [udunits package](https://github.com/nco/nco/blob/master/data/udunits.dat)
    #[staticmethod]
    pub fn from_years(years: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_years(years, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_months(months: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_months(months, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_weeks(weeks: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_weeks(weeks, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_days(days: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_days(days, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_hours(hours: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_hours(hours, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_minutes(minutes: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_minutes(minutes, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_seconds(seconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_seconds(seconds, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_milliseconds(milliseconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_milliseconds(milliseconds, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_microseconds(microseconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_microseconds(microseconds, calendar.calendar),
        }
    }
    #[staticmethod]
    pub fn from_nanoseconds(nanoseconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_nanoseconds(nanoseconds, calendar.calendar),
        }
    }
    pub fn num_years(&self) -> f64 {
        self.duration.num_years()
    }
    pub fn num_months(&self) -> f64 {
        self.duration.num_months()
    }
    pub fn num_weeks(&self) -> f64 {
        self.duration.num_weeks()
    }
    pub fn num_days(&self) -> f64 {
        self.duration.num_days()
    }
    pub fn num_hours(&self) -> f64 {
        self.duration.num_hours()
    }
    pub fn num_minutes(&self) -> f64 {
        self.duration.num_minutes()
    }
    pub fn num_seconds(&self) -> f64 {
        self.duration.num_seconds()
    }
    pub fn num_milliseconds(&self) -> f64 {
        self.duration.num_milliseconds()
    }
    pub fn num_microseconds(&self) -> f64 {
        self.duration.num_microseconds()
    }
    pub fn num_nanoseconds(&self) -> f64 {
        self.duration.num_nanoseconds()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyCFDatetime {
    pub dt: Arc<CFDatetime>,
}

#[pymethods]
impl PyCFDatetime {
    #[new]
    pub fn new(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f32,
        calendar: PyCFCalendar,
    ) -> PyResult<Self> {
        let dt =
            CFDatetime::from_ymd_hms(year, month, day, hour, minute, second, calendar.calendar)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { dt: dt.into() })
    }
    pub fn ymd(&self) -> PyResult<(i64, u8, u8)> {
        let (year, month, day, _, _, _) = self
            .ymd_hms()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok((year, month, day))
    }

    pub fn hms(&self) -> PyResult<(u8, u8, u8)> {
        let (_, _, _, hour, min, sec) = self
            .ymd_hms()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok((hour, min, sec))
    }

    pub fn ymd_hms(&self) -> PyResult<(i64, u8, u8, u8, u8, u8)> {
        self.dt
            .ymd_hms()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
    #[staticmethod]
    pub fn from_ymd_hms(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f32,
        calendar: PyCFCalendar,
    ) -> PyResult<Self> {
        let dt =
            CFDatetime::from_ymd_hms(year, month, day, hour, minute, second, calendar.calendar)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { dt: dt.into() })
    }
    #[staticmethod]
    pub fn from_hms(hour: u8, minute: u8, second: f32, calendar: PyCFCalendar) -> PyResult<Self> {
        let dt = CFDatetime::from_ymd_hms(
            constants::UNIX_DEFAULT_YEAR,
            constants::UNIX_DEFAULT_MONTH,
            constants::UNIX_DEFAULT_DAY,
            hour,
            minute,
            second,
            calendar.calendar,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { dt: dt.into() })
    }
    #[staticmethod]
    pub fn from_ymd(year: i64, month: u8, day: u8, calendar: PyCFCalendar) -> PyResult<Self> {
        let dt = CFDatetime::from_ymd_hms(year, month, day, 0, 0, 0.0, calendar.calendar)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { dt: dt.into() })
    }
    #[staticmethod]
    pub fn from_timestamp(
        timestamp: i64,
        nanoseconds: u32,
        calendar: PyCFCalendar,
    ) -> PyResult<Self> {
        let dt = CFDatetime::from_timestamp(timestamp, nanoseconds, calendar.calendar)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { dt: dt.into() })
    }
    pub fn hours(&self) -> PyResult<u8> {
        let (hour, _, _) = self.hms()?;
        Ok(hour)
    }
    pub fn minutes(&self) -> PyResult<u8> {
        let (_, min, _) = self.hms()?;
        Ok(min)
    }
    pub fn seconds(&self) -> PyResult<u8> {
        let (_, _, sec) = self.hms()?;
        Ok(sec)
    }
    pub fn nanoseconds(&self) -> u32 {
        self.dt.nanoseconds()
    }
    fn __repr__(&self) -> String {
        format!("PyCFDatetime({})", self.dt)
    }
    fn __str__(&self) -> String {
        self.dt.to_string()
    }
}
impl std::fmt::Display for PyCFDatetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.dt.fmt(f)
    }
}

macro_rules! decode_numbers {
    ($numbers:expr, $units:expr, $calendar:expr, $($t:ty),+) => {
        {
            $(
                if let Ok(numbers) = $numbers.extract::<Vec<$t>>() {
                    numbers.decode_cf($units.as_str(), $calendar)
                        .map_err(|e| PyValueError::new_err(format!("Could not decode numbers: {}", e)))?
                } else
            )+
            {
                let supported_types = stringify!($($t),+);
                return Err(PyValueError::new_err(format!(
                "Could not convert array to supported types. \
                `num2date` function needs an array of one the following types: {}",supported_types)))
            }
        }
    };
}

#[pyfunction]
fn num2date(numbers: &PyAny, units: String, calendar: String) -> PyResult<Vec<PyCFDatetime>> {
    let calendar = Calendar::from_str(calendar.as_str())
        .map_err(|e| PyValueError::new_err(format!("Could not parse calendar: {}", e)))?;
    let datetimes = decode_numbers!(numbers, units, calendar, i32, i64, f32, f64);
    Ok(datetimes
        .into_iter()
        .map(|dt| PyCFDatetime { dt: dt.into() })
        .collect())
}

enum DType {
    Int,
    Float,
    Unknown,
}

const INT_TYPES: &[&str] = &["i32", "i64", "i", "integer", "int"];
const FLOAT_TYPES: &[&str] = &["f32", "f64", "f", "float"];

impl FromStr for DType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            s if INT_TYPES.iter().any(|&x| x == s) => Ok(DType::Int),
            s if FLOAT_TYPES.iter().any(|&x| x == s) => Ok(DType::Float),
            _ => Ok(DType::Unknown),
        }
    }
}

#[pyfunction]
fn date2num(
    py: Python,
    datetimes: Vec<PyCFDatetime>,
    units: String,
    calendar: String,
    dtype: String,
) -> PyResult<PyObject> {
    let calendar = Calendar::from_str(calendar.as_str())
        .map_err(|e| PyValueError::new_err(format!("Could not parse calendar: {}", e)))?;
    let dts: Vec<&CFDatetime> = datetimes.iter().map(|pydatetime| &*pydatetime.dt).collect();
    let dtype_enum = DType::from_str(dtype.as_str())
        .map_err(|e| PyValueError::new_err(format!("Could not parse dtype: {}", e)))?;
    match dtype_enum {
        DType::Int => {
            let numbers: Vec<i64> = dts
                .encode_cf(units.as_str(), calendar)
                .map_err(|e| PyValueError::new_err(format!("Could not encode datetimes: {}", e)))?;
            Ok(numbers.into_py(py))
        }
        DType::Float => {
            let numbers: Vec<f64> = dts
                .encode_cf(units.as_str(), calendar)
                .map_err(|e| PyValueError::new_err(format!("Could not encode datetimes: {}", e)))?;
            Ok(numbers.into_py(py))
        }
        DType::Unknown => Err(PyValueError::new_err(format!(
            "Invalid dtype `{}`. Must be one of {} or one of {}",
            dtype,
            INT_TYPES.join(", "),
            FLOAT_TYPES.join(", "),
        ))),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn cftime_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(num2date, m)?)?;
    m.add_function(wrap_pyfunction!(date2num, m)?)?;
    m.add_class::<PyCFCalendar>()?;
    m.add_class::<PyCFDuration>()?;
    m.add_class::<PyCFDatetime>()?;

    Ok(())
}
