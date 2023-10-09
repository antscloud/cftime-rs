//! Crate that implement the logic for the different datetimes
//! Here is the list of available datetimes :
//! - All Leap
//! - No Leap
//! - 360 Day
//! - Julian
//! - Standard
//! - Proleptic Gregorian
//!
//! The definition of these datetime can be found in the [CF Conventions](https://cfconventions.org/Data/cf-conventions/cf-conventions-1.10/cf-conventions.html#time-coordinate)
pub mod all_leap;
pub mod day_360;
pub mod julian;
pub mod no_leap;
pub mod proleptic_gregorian;
pub mod standard;
pub mod traits;
