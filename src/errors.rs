use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Out of bounds for calendar {0} : {1}")]
    OutOfBoundsCalendar(String, String),
    #[error("Invalid date : {0}")]
    InvalidDate(String),
    #[error("Invalid time : {0}")]
    InvalidTime(String),
    #[error("Invalid tz : {0}")]
    InvalidTz(String),
    #[error("Invalid unit : {0}")]
    UnitParserError(String),
    #[error("Different calendars found : {0} and {1}.")]
    DifferentCalendars(String, String),
    // Parseint error from std
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    // Parsefloat error from std
    #[error("{0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
