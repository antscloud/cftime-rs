use cftime_rs::{calendars::Calendar, constants};

pub trait CFTimeEncoder {
    fn cf_time_encode(unit: &str, calendar: Calendar);
}
pub trait CFTimeDecoder {
    fn cf_time_decode(self, unit: &str, calendar: Option<Calendar>);
}

fn main() {
    println!("Hello, world!");
}
