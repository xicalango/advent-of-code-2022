use std::ops::RangeInclusive;

pub trait RangeExt {

    fn intersects(&self, other: &Self) -> bool where Self: Sized {
        self.intersect(other).is_some()
    }

    fn contains_fully(&self, other: &Self) -> bool;

    fn intersect(&self, other: &Self) -> Option<Self> where Self: Sized;

    fn join(&self, other: &Self) -> (Self, Option<Self>) where Self: Sized;

    fn join_mut(&mut self, other: Self) -> Option<Self> where Self: Sized {
        let (joined, reminder) = self.join(&other);
        *self = joined;
        reminder
    }

}

impl<T> RangeExt for RangeInclusive<T>
where T: Ord + Copy
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
}
