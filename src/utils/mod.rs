use std::fmt::Debug;
use std::num::ParseIntError;

pub mod vec2;

#[derive(Debug)]
pub struct Error(pub String);

impl Error {
    pub fn new(message: &impl ToString) -> Error {
        Error(message.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(pie: ParseIntError) -> Self {
        Error(pie.to_string())
    }
}
