use cftime_rs::calendars::Calendar;
use cftime_rs::decoder::*;
use cftime_rs::encoder::CFEncoder;
use std::str::FromStr;
use std::time::Instant;

fn compute_billion_seconds() {
    let calendars = vec![
        Calendar::from_str("standard").unwrap(),
        Calendar::from_str("all_leap").unwrap(),
        Calendar::from_str("360_day").unwrap(),
    ];
    let to_decode: i64 = 1_000_000_000_000_000;
    let units = "seconds since 2000-01-01 00:00:00";
    for calendar in calendars {
        let now = Instant::now();
        let datetime = to_decode.decode_cf(units, calendar).unwrap();
        println!("{}", datetime);
        println!("Execution time: Calendar {calendar} : {:?}", now.elapsed());
    }
}

fn encode_then_decode_one_million() {
    let calendars = vec![
        Calendar::from_str("standard").unwrap(),
        Calendar::from_str("all_leap").unwrap(),
        Calendar::from_str("360_day").unwrap(),
    ];
    let to_decode: Vec<i64> = (0..1_000_000).into_iter().collect();
    let units = "seconds since 2000-01-01 00:00:00";
    for calendar in calendars {
        let now = Instant::now();
        let datetimes = to_decode.decode_cf(units, calendar).unwrap();
        let _: Vec<i64> = datetimes.encode_cf(units, calendar).unwrap();
        println!("Execution time: Calendar {calendar} : {:?}", now.elapsed());
    }
}
fn main() {
    compute_billion_seconds();
    encode_then_decode_one_million();
}
