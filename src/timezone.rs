#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tz {
    hour: i8,
    minute: u8,
}

impl Tz {
    pub fn new(hour: i8, minute: u8) -> Result<Self, crate::errors::Error> {
        if !(-23..=23).contains(&hour) {
            return Err(crate::errors::Error::InvalidTz(format!(
                "Hour is out of bounds {}:{}",
                hour, minute
            )));
        }
        if minute > 59 {
            return Err(crate::errors::Error::InvalidTz(format!(
                "Minute is out of bounds {}:{}",
                hour, minute
            )));
        }
        Ok(Self { hour, minute })
    }
}
