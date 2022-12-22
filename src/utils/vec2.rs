use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, BitOr, Mul, RangeInclusive, Sub};
use std::str::FromStr;
use crate::utils::{Error, Surroundings};
use crate::utils::num::{Decrement, Increment};
use crate::utils::vec3::Vec3;

pub struct Vec2<T>(pub T, pub T);

impl<T: Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Vec2(x, y) = self;
        write!(f, "Vec2({:?}, {:?})", x, y)
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2(x, y)
    }

    pub fn map<K, F>(&self, mapper: F) -> Vec2<K>
    where F: Fn(&T) -> K
    {
        let Vec2(x, y) = self;
        Vec2::new(mapper(x), mapper(y))
    }

    pub fn transform<K, F>(self, mapper: F) -> Vec2<K>
        where F: Fn(T) -> K
    {
        let Vec2(x, y) = self;
        Vec2::new(mapper(x), mapper(y))
    }

    pub fn extend(self, z: T) -> Vec3<T> {
        let Vec2(x, y) = self;
        Vec3::new(x, y, z)
    }
}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Vec2(T::default(), T::default())
    }
}

impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        let Vec2(x, y) = self;
        Vec2(x.clone(), y.clone())
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        let Vec2(sx, sy) = self;
        let Vec2(ox, oy) = other;
        return sx == ox && sy == oy;
    }
}

impl<T: Eq> Eq for Vec2<T> {}

impl<T: PartialOrd> PartialOrd for Vec2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Vec2(sx, sy) = self;
        let Vec2(ox, oy) = other;
        match sx.partial_cmp(ox) {
            None => None,
            Some(Ordering::Equal) => sy.partial_cmp(oy),
            o => o
        }
    }
}

impl<T: Ord> Ord for Vec2<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let Vec2(sx, sy) = self;
        let Vec2(ox, oy) = other;
        match sx.cmp(ox) {
            Ordering::Equal => sy.cmp(oy),
            o => o
        }
    }
}

pub trait Vector2<T> {
    fn get_x(&self) -> &T;
    fn get_y(&self) -> &T;
}

pub trait Vector2Mut<T> {
    fn set_x(&mut self, x: T);
    fn set_y(&mut self, y: T);
}

impl<T> Vector2<T> for Vec2<T> {
    fn get_x(&self) -> &T {
        let Vec2(x, _) = self;
        x
    }

    fn get_y(&self) -> &T {
        let Vec2(_, y) = self;
        y
    }
}

impl<T: Copy> Vector2Mut<T> for Vec2<T> {
    fn set_x(&mut self, x: T) {
        let Vec2(_, y) = self;
        *self = Vec2(x, *y)
    }

    fn set_y(&mut self, y: T) {
        let Vec2(x, _) = self;
        *self = Vec2(*x, y)
    }
}

impl<T: FromStr> FromStr for Vec2<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or(Error(format!("cannot parse vec2: {}", s)))?;
        let xp = x.parse().map_err(|_| Error(format!("cannot parse x {}", x)))?;
        let yp = y.parse().map_err(|_| Error(format!("cannot parse y {}", y)))?;
        Ok(Vec2(xp, yp))
    }
}

impl<T: Mul<Output = T> + Copy> Vec2<T> {
    pub fn product(&self) -> T {
        let Vec2(x, y) = self;
        *x * *y
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        let (x,y) = tuple;
        Vec2(x, y)
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(vec: Vec2<T>) -> Self {
        let Vec2(x, y) = vec;
        (x, y)
    }
}

impl<T: Add<Output=T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let Vec2(lx, ly) = self;
        let Vec2(rx, ry) = rhs;

        Vec2(lx + rx, ly + ry)
    }
}

impl<T: Sub<Output=T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let Vec2(lx, ly) = self;
        let Vec2(rx, ry) = rhs;

        Vec2(lx - rx, ly - ry)
    }
}

impl<T: Sub<Output=T> + Add<Output=T> + Ord> Vec2<T> {

    pub fn manhattan_dist(self, rhs: Vec2<T>) -> T {
        let Vec2(lx, ly) = self;
        let Vec2(rx, ry) = rhs;

        let dist_x = if lx > rx {
            lx - rx
        } else {
            rx - lx
        };

        let dist_y = if ly > ry {
            ly - ry
        } else {
            ry - ly
        };

        dist_x + dist_y
    }
}

impl<T: Sub<Output=T> + Add<Output=T> + Ord> BitOr for Vec2<T> {
    type Output = T;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.manhattan_dist(rhs)
    }
}

impl<T: Hash> Hash for Vec2<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Vec2(x, y) = self;
        x.hash(state);
        y.hash(state);
    }
}

impl<T: Copy + Increment<Output=T> + Decrement<Output=T> + ?Sized> Surroundings<4> for Vec2<T> {
    fn get_surroundings(&self) -> [Self; 4] where Self: Sized {
        let Vec2(x, y) = self;
        [
            Vec2(x.dec(), *y),
            Vec2(x.inc(), *y),
            Vec2(*x, y.dec()),
            Vec2(*x, y.inc()),
        ]
    }
}

pub struct BoundingBox<T> {
    top_left: Vec2<T>,
    bottom_right: Vec2<T>,
}

impl<'a, E: PartialOrd> FromIterator<&'a Vec2<E>> for BoundingBox<&'a E> {
    fn from_iter<T: IntoIterator<Item=&'a Vec2<E>>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let first_item = iter.next();
        let first_item = first_item.expect("can only compute bounding box for non empty iterators");

        let mut min_x = first_item.get_x();
        let mut max_x = min_x;

        let mut min_y = first_item.get_y();
        let mut max_y = min_y;

        for item in iter {
            let x = item.get_x();
            let y = item.get_y();

            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        BoundingBox {
            top_left: Vec2(min_x, min_y),
            bottom_right: Vec2(max_x, max_y),
        }
    }
}

impl<T> BoundingBox<T> {

    pub fn top_left(&self) -> &Vec2<T> {
        &self.top_left
    }

    pub fn bottom_right(&self) -> &Vec2<T> {
        &self.bottom_right
    }

    pub fn top_right(&self) -> Vec2<&T> {
        Vec2(self.bottom_right.get_x(), self.top_left.get_y())
    }

    pub fn bottom_left(&self) -> Vec2<&T> {
        Vec2(self.top_left.get_x(), self.bottom_right.get_y())
    }

    pub fn map<K, F>(self, mapper: F) -> BoundingBox<K>
        where F: Fn(T) -> K {

        let Vec2(min_x, min_y) = self.top_left;
        let Vec2(max_x, max_y) = self.bottom_right;

        BoundingBox {
            top_left: Vec2(mapper(min_x), mapper(min_y)),
            bottom_right: Vec2(mapper(max_x), mapper(max_y))
        }
    }
}

impl<T: Debug> Debug for BoundingBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoundingBox({:?}, {:?})", self.top_left, self.bottom_right)
    }
}

impl<T: Copy> BoundingBox<T> {

    pub fn y_range(&self) -> RangeInclusive<T> {
        *self.top_left.get_y()..=*self.bottom_right.get_y()
    }

    pub fn x_range(&self) -> RangeInclusive<T> {
        *self.top_left.get_x()..=*self.bottom_right.get_x()
    }

}
