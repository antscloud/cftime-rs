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

/// PyCFDuration is a wrapper around Rust CFDuration
/// All the methods depends on the Calendar definitions found in
/// [udunits package](https://github.com/nco/nco/blob/master/data/udunits.dat)
///
/// This duration can be added to a PyCFDatetime
/// The result of the substraction between a PyCFDatetime and a PyCFDatetime gives a PyCFDuration
#[pyclass]
pub struct PyCFDuration {
    pub duration: CFDuration,
}

#[pymethods]
impl PyCFCalendar {
    #[staticmethod]
    pub fn from_str(s: String) -> PyResult<Self> {
        let calendar = Calendar::from_str(s.as_str())
            .map_err(|e| PyValueError::new_err(format!("Could not parse calendar: {}", e)))?;
        Ok(Self { calendar })
    }
}

#[pymethods]
impl PyCFDuration {
    /// Makes a new `PyCFDuration` with given number of seconds, nanoseconds and specific calendar.
    #[new]
    pub fn new(seconds: i64, nanoseconds: i64, calendar: PyCFCalendar) -> Self {
        Self {
            duration: CFDuration::new(seconds, nanoseconds, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of years and specific calendar.
    #[staticmethod]
    pub fn from_years(years: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_years(years, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of months and specific calendar.
    #[staticmethod]
    pub fn from_months(months: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_months(months, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of weeks and specific calendar.
    #[staticmethod]
    pub fn from_weeks(weeks: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_weeks(weeks, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of days and specific calendar.
    #[staticmethod]
    pub fn from_days(days: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_days(days, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of hours and specific calendar.
    #[staticmethod]
    pub fn from_hours(hours: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_hours(hours, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of minutes and specific calendar.
    #[staticmethod]
    pub fn from_minutes(minutes: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_minutes(minutes, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of seconds and specific calendar.
    #[staticmethod]
    pub fn from_seconds(seconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_seconds(seconds, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of milliseconds and specific calendar.
    #[staticmethod]
    pub fn from_milliseconds(milliseconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_milliseconds(milliseconds, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of microseconds and specific calendar.
    #[staticmethod]
    pub fn from_microseconds(microseconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_microseconds(microseconds, calendar.calendar),
        }
    }
    /// Makes a new `PyCFDuration` with given number of nanoseconds and specific calendar.
    #[staticmethod]
    pub fn from_nanoseconds(nanoseconds: i64, calendar: PyCFCalendar) -> PyCFDuration {
        Self {
            duration: CFDuration::from_nanoseconds(nanoseconds, calendar.calendar),
        }
    }
    /// Returns the total number of years in the duration.
    pub fn num_years(&self) -> f64 {
        self.duration.num_years()
    }
    /// Returns the total number of months in the duration.
    pub fn num_months(&self) -> f64 {
        self.duration.num_months()
    }
    /// Returns the total number of weeks in the duration.
    pub fn num_weeks(&self) -> f64 {
        self.duration.num_weeks()
    }
    /// Returns the total number of days in the duration.
    pub fn num_days(&self) -> f64 {
        self.duration.num_days()
    }
    /// Returns the total number of hours in the duration.
    pub fn num_hours(&self) -> f64 {
        self.duration.num_hours()
    }
    /// Returns the total number of minutes in the duration.
    pub fn num_minutes(&self) -> f64 {
        self.duration.num_minutes()
    }
    /// Returns the total number of seconds in the duration.
    pub fn num_seconds(&self) -> f64 {
        self.duration.num_seconds()
    }
    /// Returns the total number of milliseconds in the duration.
    pub fn num_milliseconds(&self) -> f64 {
        self.duration.num_milliseconds()
    }
    /// Returns the total number of microseconds in the duration.
    pub fn num_microseconds(&self) -> f64 {
        self.duration.num_microseconds()
    }
    /// Returns the total number of nanoseconds in the duration.
    pub fn num_nanoseconds(&self) -> f64 {
        self.duration.num_nanoseconds()
    }
    /// Returns an ISO 8601 formatted string.
    pub fn __repr__(&self) -> String {
        format!("{}", self.duration)
    }
    /// Returns an ISO 8601 formatted string.
    pub fn __str__(&self) -> String {
        self.duration.to_string()
    }

    pub fn __sub__(&self, other: &PyCFDuration) -> PyCFDuration {
        PyCFDuration {
            duration: &self.duration - &other.duration,
        }
    }

    pub fn __add__(&self, other: &PyCFDuration) -> PyCFDuration {
        PyCFDuration {
            duration: &self.duration + &other.duration,
        }
    }

    pub fn __neg__(&self) -> PyCFDuration {
        let duration = -&self.duration;
        PyCFDuration { duration: duration }
    }
}

/// PyCFDatetime is a wrapper around Rust CFDatetime
/// It represents a date in a specific calendar
/// All the methods depends on the Calendar definitions found in
/// [udunits package](https://github.com/nco/nco/blob/master/data/udunits.dat)
#[pyclass]
#[derive(Clone)]
pub struct PyCFDatetime {
    pub dt: Arc<CFDatetime>,
}

#[pymethods]
impl PyCFDatetime {
    /// Makes a new `PyCFDatetime` with given year, month, day, hour, minute, second and specific calendar
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
    /// Returns the year, month and day of the date.
    pub fn ymd(&self) -> PyResult<(i64, u8, u8)> {
        let (year, month, day, _, _, _) = self
            .ymd_hms()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok((year, month, day))
    }
    /// Returns the hour, minute and second of the date.
    pub fn hms(&self) -> PyResult<(u8, u8, u8)> {
        let (_, _, _, hour, min, sec) = self
            .ymd_hms()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok((hour, min, sec))
    }
    /// Returns the year, month, day, hour, minute, second of the date.
    pub fn ymd_hms(&self) -> PyResult<(i64, u8, u8, u8, u8, u8)> {
        self.dt
            .ymd_hms()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Makes a new `PyCFDatetime` with given year, month, day, hour, minute, second and specific calendar
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
    /// Makes a new `PyCFDatetime` with given hour, minute, second and specific calendar.
    /// The year, month, day are set to 1970-01-01
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
    /// Makes a new `PyCFDatetime` with given year, month, day and specific calendar.
    /// The hour, minute, second are set to 0
    #[staticmethod]
    pub fn from_ymd(year: i64, month: u8, day: u8, calendar: PyCFCalendar) -> PyResult<Self> {
        let dt = CFDatetime::from_ymd_hms(year, month, day, 0, 0, 0.0, calendar.calendar)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { dt: dt.into() })
    }
    /// Makes a new `PyCFDatetime` with given timestamp, nanoseconds and specific calendar.
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
    /// Returns the hours of the date.
    pub fn hours(&self) -> PyResult<u8> {
        let (hour, _, _) = self.hms()?;
        Ok(hour)
    }
    /// Returns the minutes of the date.
    pub fn minutes(&self) -> PyResult<u8> {
        let (_, min, _) = self.hms()?;
        Ok(min)
    }
    /// Returns the seconds of the date.
    pub fn seconds(&self) -> PyResult<u8> {
        let (_, _, sec) = self.hms()?;
        Ok(sec)
    }
    /// Returns the nanoseconds of the date.
    pub fn nanoseconds(&self) -> u32 {
        self.dt.nanoseconds()
    }
    fn __repr__(&self) -> String {
        format!("PyCFDatetime({})", self.dt)
    }
    fn __str__(&self) -> String {
        self.dt.to_string()
    }
    fn __sub__(&self, other: &PyCFDatetime) -> PyCFDuration {
        let duration = &*self.dt - &*other.dt;
        PyCFDuration { duration: duration }
    }
    fn __add__(&self, other: &PyCFDuration) -> PyResult<PyCFDatetime> {
        let dt = (&*self.dt + &other.duration).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyCFDatetime { dt: dt.into() })
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
    Int32,
    Int64,
    Float32,
    Float64,
    Unknown,
}

const INT_32_TYPES: &[&str] = &["i32"];
const INT_64_TYPES: &[&str] = &["i64", "i", "integer", "int"];
const FLOAT_32_TYPES: &[&str] = &["f32"];
const FLOAT_64_TYPES: &[&str] = &["f64", "f", "float"];

impl FromStr for DType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            s if INT_32_TYPES.iter().any(|&x| x == s) => Ok(DType::Int32),
            s if INT_64_TYPES.iter().any(|&x| x == s) => Ok(DType::Int64),
            s if FLOAT_32_TYPES.iter().any(|&x| x == s) => Ok(DType::Float32),
            s if FLOAT_64_TYPES.iter().any(|&x| x == s) => Ok(DType::Float64),
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
        DType::Int32 => {
            let numbers: Vec<i32> = dts
                .encode_cf(units.as_str(), calendar)
                .map_err(|e| PyValueError::new_err(format!("Could not encode datetimes: {}", e)))?;
            Ok(numbers.into_py(py))
        }
        DType::Int64 => {
            let numbers: Vec<i64> = dts
                .encode_cf(units.as_str(), calendar)
                .map_err(|e| PyValueError::new_err(format!("Could not encode datetimes: {}", e)))?;
            Ok(numbers.into_py(py))
        }
        DType::Float32 => {
            let numbers: Vec<f32> = dts
                .encode_cf(units.as_str(), calendar)
                .map_err(|e| PyValueError::new_err(format!("Could not encode datetimes: {}", e)))?;
            Ok(numbers.into_py(py))
        }
        DType::Float64 => {
            let numbers: Vec<f64> = dts
                .encode_cf(units.as_str(), calendar)
                .map_err(|e| PyValueError::new_err(format!("Could not encode datetimes: {}", e)))?;
            Ok(numbers.into_py(py))
        }
        DType::Unknown => Err(PyValueError::new_err(format!(
            "Invalid dtype `{}`. For i32 use {}. For i64 use {}. For f32 use {}. For f64 use {}.",
            dtype,
            INT_32_TYPES.join(", "),
            INT_64_TYPES.join(", "),
            FLOAT_32_TYPES.join(", "),
            FLOAT_64_TYPES.join(", ")
        ))),
    }
}

/// cftime_rs is a python module that is implemented in Rust.
#[pymodule]
fn cftime_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(num2date, m)?)?;
    m.add_function(wrap_pyfunction!(date2num, m)?)?;
    m.add_class::<PyCFCalendar>()?;
    m.add_class::<PyCFDuration>()?;
    m.add_class::<PyCFDatetime>()?;

    Ok(())
}
