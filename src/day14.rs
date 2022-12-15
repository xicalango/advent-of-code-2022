use std::cmp::{max, min};
use std::str::FromStr;
use crate::Error;

type Position = u32;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Vec2(Position, Position);

impl FromStr for Vec2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or(Error(format!("invalid coor: {}", s)))?;
        Ok(Vec2(x.parse()?, y.parse()?))
    }
}

impl Vec2 {

    pub fn product(&self) -> Position {
        let Vec2(x, y) = self;
        x * y
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line(Vec2, Vec2);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Box(Vec2, Vec2);

impl Box {

    pub fn size(&self) -> Vec2 {
        let Box(Vec2(sx, sy), Vec2(ex, ey)) = self;
        Vec2(ex - sx, ey - sy)
    }

    pub fn top_left(&self) -> &Vec2 {
        &self.0
    }

    pub fn bottom_right(&self) -> &Vec2 {
        &self.1
    }

}

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
    fn get_bounds(&self) -> Box;
}

impl BoundingBox for Vec2 {
    fn get_bounds(&self) -> Box {
        Box(self.clone(), self.clone())
    }
}

impl BoundingBox for Line {
    fn get_bounds(&self) -> Box {
        let Line(l1, l2) = self;
        let Vec2(x1, y1) = l1;
        let Vec2(x2, y2) = l2;

        let min_vec = Vec2(min(*x1, *x2), min(*y1, *y2));
        let max_vec = Vec2(max(*x1, *x2), max(*y1, *y2));

        Box(min_vec, max_vec)
    }
}

impl BoundingBox for Box {
    fn get_bounds(&self) -> Box {
        self.clone()
    } 
}

impl<'a, B: 'a + BoundingBox> FromIterator<&'a B> for Box {
    fn from_iter<T: IntoIterator<Item = &'a B>>(iter: T) -> Self {
        let mut min_x = Position::MAX;
        let mut min_y = Position::MAX;
        let mut max_x = Position::MIN;
        let mut max_y = Position::MIN;

        for item in iter {
            let bounding_box = item;
            let Box(Vec2(x1, y1), Vec2(x2, y2)) = bounding_box.get_bounds();
            let local_min_x = min(x1, x2);
            let local_min_y = min(y1, y2);
            let local_max_x = max(x1, x2);
            let local_max_y = max(y1, y2);
        
            min_x = min(min_x, local_min_x);
            min_y = min(min_y, local_min_y);
            max_x = max(max_x, local_max_x);
            max_y = max(max_y, local_max_y);
        }
        
        Box(Vec2(min_x, min_y), Vec2(max_x, max_y))
    }
}

impl BoundingBox for LineRow {
    fn get_bounds(&self) -> Box {
        let LineRow(lines) = self;
        lines.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub enum WorldElement {
    Nothing,
    Wall,
    NewSand,
    FixedSand,
}

#[derive(Debug)]
pub struct World {
    size: Vec2,
    buffer: Vec<WorldElement>,
    offset: Vec2,
}

impl World {
    
    pub fn new_from_lines(line_defs: &Vec<LineRow>) -> World {
        let bounding_box: Box = line_defs.iter().collect();
        let size = bounding_box.size();

        let area = size.product() as usize;

        println!("size: {:?}, area: {:?}", size, area);

        let mut world = World {
            size,
            buffer: vec![WorldElement::Nothing; area],
            offset: bounding_box.top_left().clone(),
        };

        for line_def in line_defs {
            let LineRow(lines) = line_def;
            for line in lines {
                world.insert_wall(line);
            }
        }

        world
    }

    fn insert_wall(&mut self, wall: &Line) {
        let Line(Vec2(sx, sy), Vec2(ex, ey)) = wall;

        for x in *sx..=*ex {
            for y in *sy..=*ey {
                self.insert_wall_at(&Vec2(x, y));
            }
        }
    }

    fn insert_wall_at(&mut self, pos: &Vec2) {
        let buffer_pos = self.vec2_to_buffer_pos(pos);
        self.buffer[buffer_pos] = WorldElement::Wall;
    }

    fn vec2_to_buffer_pos(&self, pos: &Vec2) -> usize {
        let Vec2(x, y) = pos;
        let Vec2(sx, sy) = &self.size;
        assert!(x < sx);
        assert!(y < sy);

        (y * sx + x) as usize
    }



}

#[cfg(test)]
mod test {

    use super::*;

    static EXAMPLE : &'static str = include_str!("../res/day14-paths_example.txt");

    #[test]
    fn test_example() {
        let rows: Result<Vec<LineRow>, Error> = EXAMPLE.lines().map(str::trim_end).map(|l| l.parse::<LineRow>()).collect();
        let rows = rows.unwrap();
        println!("{:#?}", rows);

        let bounding_box: Box = rows.iter().collect();
        println!("{:?}", bounding_box);

        let world = World::new_from_lines(&rows);
        println!("{:?}", world);
    }

}
