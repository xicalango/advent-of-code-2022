use std::ops::{Add, Sub};

pub trait Decrement {
    type Output;

    fn dec(self) -> Self::Output;
}

pub trait Increment {
    type Output;

    fn inc(self) -> Self::Output;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl<T: From<u8>> Zero for T {
    fn zero() -> Self {
        (0 as u8).into()
    }
}

impl<T: From<u8>> One for T {
    fn one() -> Self {
        (1 as u8).into()
    }
}

impl<T: One + Add<T, Output=T>> Increment for T{
    type Output = T;

    fn inc(self) -> Self::Output {
        self + T::one()
    }
}

impl<T: One + Sub<T, Output=T>> Decrement for T{
    type Output = T;

    fn dec(self) -> Self::Output {
        self - T::one()
    }
}
