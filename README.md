# CF Time

This repo is an implementation in `rust` of the [cf time](https://cfconventions.org/Data/cf-conventions/cf-conventions-1.10/cf-conventions.html#time-coordinate) convention.

## Installation

```
cargo install cftime-rs
```

## Examples 

### Decoding 

```rust
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
```

will print :

```
2000-01-01 00:00:00.000
2000-01-02 00:00:00.000
2000-01-03 00:00:00.000
2000-01-04 00:00:00.000
2000-01-05 00:00:00.000
2000-01-06 00:00:00.000
```

### Encoding 

```rust
use cftime_rs::calendars::Calendar;
use cftime_rs::datetime::CFDatetime;
use cftime_rs::encoder::*;
use cftime_rs::errors::Error;
use std::str::FromStr;
fn main() {
    let calendar = Calendar::from_str("standard").unwrap();
    // Create vector of datetimes and convert Vec<Result<CFDatetime, Error>>
    // into Result<Vec<CFDatetime>, Error>
    let to_encode: Result<Vec<CFDatetime>, Error> = vec![
        CFDatetime::from_ymd(2000, 1, 1, calendar),
        CFDatetime::from_ymd(2000, 1, 2, calendar),
        CFDatetime::from_ymd(2000, 1, 3, calendar),
        CFDatetime::from_ymd(2000, 1, 4, calendar),
        CFDatetime::from_ymd(2000, 1, 5, calendar),
        CFDatetime::from_ymd(2000, 1, 6, calendar),
    ]
    .into_iter()
    .collect();
    // define the units
    let units = "days since 2000-01-01 00:00:00";
    // The type annotation for result allow us to cast to type we want
    // here we use Vec<i64>
    let results: Vec<i64> = to_encode.unwrap().encode_cf(units, calendar).unwrap();
    for result in results {
        println!("{}", result);
    }
}
```

will print :

```
0
1
2
3
4
5
```

## Known issues
While this date calculation library can handle a wide range of dates, from approximately -291,672,107,014 BC to 291,672,107,014 AD, there are some performance considerations you should be aware of.
As you move further away from the reference date of 1970-01-01 00:00:00, the calculation speed may decrease significantly. This is because the library needs to account for leap years in various calendars.

Here is an example of the computation of the year 751 417 763 (i.e. tranforming timestamp in seconds to date) on my personal computer :

| Calendar          | Computation Time     |
|-------------------|----------------------|
| Standard Calendar | 1.6 seconds          |
| Leap Day Calendar | 172.298 milliseconds |
| 360-Day Calendar  | 1.142 microseconds   |
