use std::fmt::Debug;
use std::num::ParseIntError;

pub mod vec2;
pub mod ranges;
pub mod num;
pub mod vec3;
pub mod bv;
pub mod ringbuf;

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

pub trait Surroundings {
    fn get_surroundings(&self) -> Vec<Self> where Self: Sized;
}
