use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};
use std::str::FromStr;
use crate::utils::{Error, Surroundings};
use crate::utils::num::{Decrement, Increment};
use crate::utils::vec2::Vec2;

pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Debug> Debug for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Vec3(x, y, z) = self;
        write!(f, "Vec3({:?}, {:?}, {:?})", x, y, z)
    }
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3(x, y, z)
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Vec3(T::default(), T::default(), T::default())
    }
}

impl<T: Default> From<Vec2<T>> for Vec3<T> {
    fn from(vec2: Vec2<T>) -> Self {
        let Vec2(x, y) = vec2;
        Vec3(x, y, T::default())
    }
}

impl<T: Clone> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        let Vec3(x, y, z) = self;
        Vec3(x.clone(), y.clone(), z.clone())
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        let Vec3(sx, sy, sz) = self;
        let Vec3(ox, oy, oz) = other;
        return sx == ox && sy == oy && sz == oz;
    }
}

impl<T: Eq> Eq for Vec3<T> {}

impl<T: PartialOrd> PartialOrd for Vec3<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Vec3(sx, sy, sz) = self;
        let Vec3(ox, oy, oz) = other;
        match sx.partial_cmp(ox) {
            None => None,
            Some(Ordering::Equal) => match sy.partial_cmp(oy) {
                None => None,
                Some(Ordering::Equal) => sz.partial_cmp(oz),
                o => o,
            },
            o => o,
        }
    }
}

impl<T: Ord> Ord for Vec3<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let Vec3(sx, sy, sz) = self;
        let Vec3(ox, oy, oz) = other;
        match sx.cmp(ox) {
            Ordering::Equal => match sy.cmp(oy) {
                Ordering::Equal => sz.cmp(oz),
                o => o,
            },
            o => o,
        }
    }
}

pub trait Vector3<T> {
    fn get_x(&self) -> &T;
    fn get_y(&self) -> &T;
    fn get_z(&self) -> &T;

    fn set_x(&mut self, x: T);
    fn set_y(&mut self, y: T);
    fn set_z(&mut self, z: T);
}

impl<T: Copy> Vector3<T> for Vec3<T> {
    fn get_x(&self) -> &T {
        let Vec3(x, _, _) = self;
        x
    }

    fn get_y(&self) -> &T {
        let Vec3(_, y, _) = self;
        y
    }

    fn get_z(&self) -> &T {
        let Vec3(_, _, z) = self;
        z
    }

    fn set_x(&mut self, x: T) {
        let Vec3(_, y, z) = self;
        *self = Vec3(x, *y, *z)
    }

    fn set_y(&mut self, y: T) {
        let Vec3(x, _, z) = self;
        *self = Vec3(*x, y, *z)
    }

    fn set_z(&mut self, z: T) {
        let Vec3(x, y, _) = self;
        *self = Vec3(*x, *y, z)
    }
}

impl<T: FromStr> FromStr for Vec3<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, ",").collect();
        let xp = parts[0].parse().map_err(|_| Error(format!("cannot parse x {}", parts[0])))?;
        let yp = parts[1].parse().map_err(|_| Error(format!("cannot parse y {}", parts[1])))?;
        let zp = parts[2].parse().map_err(|_| Error(format!("cannot parse y {}", parts[2])))?;
        Ok(Vec3(xp, yp, zp))
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(tuple: (T, T, T)) -> Self {
        let (x,y, z) = tuple;
        Vec3(x, y, z)
    }
}

impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(vec: Vec3<T>) -> Self {
        let Vec3(x, y, z) = vec;
        (x, y, z)
    }
}

impl<T: Add<Output=T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let Vec3(lx, ly, lz) = self;
        let Vec3(rx, ry, rz) = rhs;

        Vec3(lx + rx, ly + ry, lz + rz)
    }
}

impl<T: Sub<Output=T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let Vec3(lx, ly, lz) = self;
        let Vec3(rx, ry, rz) = rhs;

        Vec3(lx - rx, ly - ry, lz - rz)
    }
}

impl<T: Hash> Hash for Vec3<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Vec3(x, y, z) = self;
        x.hash(state);
        y.hash(state);
        z.hash(state);
    }
}

impl<T: Copy + Increment<Output=T> + Decrement<Output=T>> Surroundings for Vec3<T> {
    fn get_surroundings(&self) -> Vec<Self> where Self: Sized {
        let Vec3(x, y, z) = self;
        let mut surroundings = Vec::new();

        surroundings.push(Vec3(x.dec(), *y, *z));
        surroundings.push(Vec3(x.inc(), *y, *z));

        surroundings.push(Vec3(*x, y.dec(), *z));
        surroundings.push(Vec3(*x, y.inc(), *z));

        surroundings.push(Vec3(*x, *y, z.dec()));
        surroundings.push(Vec3(*x, *y, z.inc()));

        surroundings
    }
}