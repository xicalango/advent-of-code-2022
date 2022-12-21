
pub trait MinMax<T> {
    fn min_max(self) -> Option<(T, T)>;
}

impl<'a, T: 'a + PartialOrd, I: Iterator<Item=&'a T>> MinMax<&'a T> for I {
    fn min_max(mut self) -> Option<(&'a T, &'a T)> {
        let start = self.next();
        if start.is_none() {
            return None;
        }

        let mut min = start.unwrap();
        let mut max = start.unwrap();

        for i in self {
            if i < min {
                min = i;
            }
            if i > max {
                max = i;
            }
        }

        Some((min, max))
    }
}
