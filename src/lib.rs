//! # cftime-rs
//!
//! Cf time implementation in rust based on the [CF Conventions](https://cfconventions.org/Data/cf-conventions/cf-conventions-1.10/cf-conventions.html#time-coordinate)
//!
//! ## Quickstart
//!
//! ### Decoding
//!
//! Decoding needs units, and calendar and can work with `i32`, `i64`, `f32`, ``f64`` and their corresponding vector type `Vec<i32>`, `Vec<i64>`, `Vec<f32>` and `Vec<f64>`. From these type it return either a `CFDatetime` object or a `Vec<CFDatetime>`.
//!
//! ```rust
//! use cftime_rs::calendars::Calendar;
//! use cftime_rs::decoder::*;
//! use std::str::FromStr;
//!
//! let to_decode = vec![0, 1, 2, 3, 4, 5];
//! let units = "days since 2000-01-01 00:00:00";
//! let calendar = Calendar::from_str("standard").unwrap();
//! let datetimes = to_decode.decode_cf(units, calendar).unwrap();
//! for (i, datetime) in datetimes.iter().enumerate() {
//!    println!("{}", datetime);
//! }
//! ```
//! will print :
//! ```shell
//! 2000-01-01 00:00:00.000
//! 2000-01-02 00:00:00.000
//! 2000-01-03 00:00:00.000
//! 2000-01-04 00:00:00.000
//! 2000-01-05 00:00:00.000
//! 2000-01-06 00:00:00.000
//! ```
//!
//! ### Encoding
//!
//! Encoding needs units and calendar and can convert a `CFDatetime` object into an `i32`, `i64`, `f32` or  `f64` or a `Vec<CFDatetime>` into `Vec<i32>`, `Vec<i64>`, `Vec<f32>` or `Vec<f64>`.
//!
//! ```rust
//! use cftime_rs::calendars::Calendar;
//! use cftime_rs::datetime::CFDatetime;
//! use cftime_rs::encoder::*;
//! use cftime_rs::errors::Error;
//! use std::str::FromStr;
//! let calendar = Calendar::from_str("standard").unwrap();
//! // Create vector of datetimes and convert Vec<Result<CFDatetime, Error>>
//! // into Result<Vec<CFDatetime>, Error>
//! let to_encode: Result<Vec<CFDatetime>, Error> = vec![
//!    CFDatetime::from_ymd(2000, 1, 1, calendar),
//!    CFDatetime::from_ymd(2000, 1, 2, calendar),
//!    CFDatetime::from_ymd(2000, 1, 3, calendar),
//!    CFDatetime::from_ymd(2000, 1, 4, calendar),
//!    CFDatetime::from_ymd(2000, 1, 5, calendar),
//!    CFDatetime::from_ymd(2000, 1, 6, calendar),
//! ]
//! .into_iter()
//! .collect();
//! // Define the units
//! let units = "days since 2000-01-01 00:00:00";
//! // The type annotation for result allow us to cast to type we want
//! // here we use Vec<i64>
//! let results: Vec<i64> = to_encode.unwrap().encode_cf(units, calendar).unwrap();
//! for result in results {
//!    println!("{}", result);
//! }
//! ```
//! will print :
//! ```shell
//! 0
//! 1
//! 2
//! 3
//! 4
//! 5
//! ```
//!
//! ## Known issues
//!
//! While this date calculation library can handle a wide range of dates, from approximately -291,672,107,014 BC to 291,672,107,014 AD, there are some performance considerations you should be aware of.
//! As you move further away from the reference date of 1970-01-01 00:00:00, the time of calculation increases. This is because the library needs to account for leap years in various calendars.
//!
//! Here is an example of the computation of 1_000_000_000_000_000 seconds using the units "seconds since 2000-01-01 00:00:00" on my personal computer in release mode :
//!
//! | Calendar          | Computation Time |
//! |-------------------|------------------|
//! | Standard Calendar | 44.470405ms      |
//! | Leap Day Calendar | 8.052179ms       |
//! | 360-Day Calendar  | 12.834µs         |
//!

pub mod calendars;
pub mod constants;
pub mod datetime;
pub mod datetimes;
pub mod decoder;
pub mod duration;
pub mod encoder;
pub mod errors;
pub mod parser;
pub mod py_bindings;
pub mod timezone;
pub mod utils;
