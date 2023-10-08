use cftime_rs::calendars::Calendar;
use cftime_rs::decoder::*;
use std::str::FromStr;
fn main() {
    let to_decode = vec![0, 1, 2, 3, 4, 5];
    let units = "days since 2000-01-01 00:00:00";
    let calendar = Calendar::from_str("standard").unwrap();
    let datetimes = to_decode.decode_cf(units, calendar).unwrap();
    for datetime in datetimes {
        println!("{}", datetime);
    }
}
