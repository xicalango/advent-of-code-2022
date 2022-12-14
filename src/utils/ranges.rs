use std::ops::{RangeInclusive, Sub};
use crate::utils::num::{Decrement, Increment};

pub trait RangeExt {

    fn intersects(&self, other: &Self) -> bool where Self: Sized {
        self.intersect(other).is_some()
    }

    fn contains_fully(&self, other: &Self) -> bool;

    fn intersect(&self, other: &Self) -> Option<Self> where Self: Sized;

    fn subtract(&self, other: &Self) -> Option<Self> where Self: Sized;

    fn join(&self, other: &Self) -> (Self, Option<Self>) where Self: Sized;

    fn join_mut(&mut self, other: Self) -> Option<Self> where Self: Sized {
        let (joined, reminder) = self.join(&other);
        *self = joined;
        reminder
    }

    fn split_off(&self, other: &Self) -> [Option<Self>; 3] where Self: Sized;
}

impl<T> RangeExt for RangeInclusive<T>
where T: Ord + Copy + Decrement<Output = T> + Increment<Output = T>
{
    fn contains_fully(&self, other: &Self) -> bool {
        let intersection = self.intersect(other);
        if intersection.is_none() {
            return false;
        }
        let intersection = intersection.unwrap();

        return &intersection == self || &intersection == other;
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let max_start = std::cmp::max(self.start(), other.start());
        let min_end = std::cmp::min(self.end(), other.end());

        if min_end >= max_start {
            Some(*max_start..=*min_end)
        } else {
            None
        }
    }

    fn subtract(&self, rhs: &Self) -> Option<Self> where Self: Sized {
        match self.intersect(rhs) {
            None => Some(self.clone()),
            Some(intersection) => {
                if self == &intersection {
                    None
                } else if self.start() == intersection.start() {
                    Some(*intersection.end()..=*self.end())
                } else {
                    Some(*self.start()..=*intersection.start())
                }
            }
        }
    }

    // 123 456 789
    //     456
    // 123 456 789

    // 123 456
    //     456 789
    // 123 456 789

    // 123
    //     456
    // 123 Some(456)

    // 123
    //         789
    // 123     Some(789)

    fn join(&self, other: &Self) -> (Self, Option<Self>) {
        if self.intersects(other) {
            let min_start = std::cmp::min(self.start(), other.start());
            let max_end = std::cmp::max(self.end(), other.end());
            return (*min_start..=*max_end, None);
        } else {
            (self.clone(), Some(other.clone()))
        }
    }

    fn split_off(&self, other: &Self) -> [Option<Self>; 3] where Self: Sized {
        let mut result = [None, None, None];

        let intersection = self.intersect(other);

        if intersection.is_none() {
            result[0] = Some(self.clone());
            return result;
        }

        let intersection = intersection.unwrap();

        if intersection.start() != self.start() {
            result[0] = Some(*self.start()..=intersection.start().dec());
        }

        if intersection.end() != self.end() {
            result[2] = Some(intersection.end().inc()..=*self.end());
        }

        result[1] = Some(intersection);

        result
    }
}

pub trait RangeLength {
    type Output;
    fn len(&self) -> Self::Output;
}

impl<T> RangeLength for RangeInclusive<T>
where T: Sub<T, Output=T> + Increment<Output=T> + Copy {
    type Output = T;

    fn len(&self) -> Self::Output {
        self.end().inc() - *self.start()
    }
}
