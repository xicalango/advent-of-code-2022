use std::fmt::Debug;
use std::num::ParseIntError;

pub mod vec2;
pub mod ranges;
pub mod num;
pub mod vec3;
pub mod bv;
pub mod ringbuf;
pub mod minmax;
pub mod bfs;

#[derive(Debug)]
pub struct Error(pub String);

impl Error {
    pub fn new(message: &impl ToString) -> Error {
        Error(message.to_string())
    }

    pub fn cannot_parse(original: &(impl ToString + ?Sized)) -> Error {
        Error::new(&format!("cannot parse {}", original.to_string()))
    }
}

impl From<ParseIntError> for Error {
    fn from(pie: ParseIntError) -> Self {
        Error(pie.to_string())
    }
}

pub trait Surroundings<const N: usize> {
    fn get_surroundings(&self) -> [Self; N] where Self: Sized;
}
