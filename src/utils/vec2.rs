use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, BitOr, Mul, Sub};
use std::str::FromStr;
use crate::utils::Error;

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

    fn set_x(&mut self, x: T);
    fn set_y(&mut self, y: T);
}

impl<T: Copy> Vector2<T> for Vec2<T> {
    fn get_x(&self) -> &T {
        let Vec2(x, _) = self;
        x
    }

    fn get_y(&self) -> &T {
        let Vec2(_, y) = self;
        y
    }

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

