use cftime_rs::calendars::Calendar;
use cftime_rs::decoder::*;
use std::str::FromStr;
fn main() {
    let to_decode: Vec<i64> = (0..1_000_000_000).into_iter().collect();
    let units = "seconds since 2000-01-01 00:00:00";
    let calendar = Calendar::from_str("standard").unwrap();
    let datetimes = to_decode.decode_cf(units, calendar).unwrap();
    // for datetime in datetimes {
    //     println!("{}", datetime);
    // }
}
