use std::cmp::{max, min};
use std::str::FromStr;
use crate::Error;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Vec2(u32, u32);

impl FromStr for Vec2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or(Error(format!("invalid coor: {}", s)))?;
        Ok(Vec2(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
pub struct Line(Vec2, Vec2);

#[derive(Debug)]
pub struct LineRow(Vec<Line>);

impl FromStr for LineRow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::new();
        let mut split = s.split(" -> ");

        let mut cur = split.next();

        while let Some(cur_coor) = cur {
            let next = split.next();
            if let Some(next_coor) = next {
                let cur_coor: Vec2 = cur_coor.parse()?;
                let next_coor: Vec2 = next_coor.parse()?;
                result.push(Line(cur_coor, next_coor));
            }
            cur = next;
        }

        Ok(LineRow(result))
    }
}

pub trait BoundingBox {
    fn get_bounds(&self) -> (Vec2, Vec2);
}

impl BoundingBox for Vec2 {
    fn get_bounds(&self) -> (Vec2, Vec2) {
        (self.clone(), self.clone())
    }
}

impl BoundingBox for Line {
    fn get_bounds(&self) -> (Vec2, Vec2) {
        let Line(l1, l2) = self;
        let Vec2(x1, y1) = l1;
        let Vec2(x2, y2) = l2;

        let min_vec = Vec2(min(*x1, *x2), min(*y1, *y2));
        let max_vec = Vec2(max(*x1, *x2), max(*y1, *y2));

        (min_vec, max_vec)
    }
}

impl BoundingBox for LineRow {
    fn get_bounds(&self) -> (Vec2, Vec2) {
        let LineRow(lines) = self;

        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = u32::MIN;
        let mut max_y = u32::MIN;

        for line in lines {
            let Line(Vec2(x1, y1), Vec2(x2, y2)) = line;
            let local_min_x = min(x1, x2);
            let local_min_y = min(y1, y2);
            let local_max_x = max(x1, x2);
            let local_max_y = max(y1, y2);
            min_x = min(min_x, *local_min_x);
            min_y = min(min_y, *local_min_y);
            max_x = max(max_x, *local_max_x);
            max_y = max(max_y, *local_max_y);
        }

        (Vec2(min_x, min_y), Vec2(max_x, max_y))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static EXAMPLE : &'static str = include_str!("../res/day14-paths_example.txt");

    #[test]
    fn test_example() {
        for line in EXAMPLE.lines().map(str::trim_end) {
            let row: LineRow = line.parse().unwrap();
            println!("{:?}, {:?}", row, row.get_bounds());
        }
    }

}
