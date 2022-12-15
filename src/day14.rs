use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;

type Position = u32;

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
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

impl From<(Position, Position)> for Vec2 {
    fn from(tuple: (Position, Position)) -> Self {
        let (x,y) = tuple;
        Vec2(x, y)
    }
}

impl From<Vec2> for (Position, Position) {
    fn from(vec: Vec2) -> Self {
        let Vec2(x, y) = vec;
        (x, y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line(Vec2, Vec2);

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Box(Vec2, Vec2);

impl Box {

    pub fn new() -> Box {
        Box::default()
    }

    pub fn new_with_dimension(top_left: Vec2, bottom_right: Vec2) -> Box {
        Box(top_left, bottom_right)
    }

    pub fn new_with_size(top_left: Vec2, size: Vec2) -> Box {
        let Vec2(x, y) = top_left;
        let Vec2(sx, sy) = size;

        Box(Vec2(x, y), Vec2(x + sx, y + sy))
    }

    pub fn size(&self) -> Vec2 {
        let Box(Vec2(sx, sy), Vec2(ex, ey)) = self;
        Vec2(ex - sx, ey - sy)
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

    fn get_top_left(&self) -> Vec2 {
        self.get_bounds().get_top_left()
    }

    fn get_top_right(&self) -> Vec2 {
        self.get_bounds().get_top_right()
    }

    fn get_bottom_left(&self) -> Vec2 {
        self.get_bounds().get_bottom_left()
    }

    fn get_bottom_right(&self) -> Vec2 {
        self.get_bounds().get_bottom_right()
    }
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

    fn get_top_left(&self) -> Vec2 {
        self.0.clone()
    }

    fn get_top_right(&self) -> Vec2 {
        let Box(Vec2(_, sy), Vec2(ex, _)) = self;

        Vec2(*ex, *sy)
    }

    fn get_bottom_left(&self) -> Vec2 {
        let Box(Vec2(sx, _), Vec2(_, ey)) = self;

        Vec2(*sx, *ey)
    }

    fn get_bottom_right(&self) -> Vec2 {
        self.1.clone()
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

impl Display for WorldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldElement::Nothing => write!(f, "."),
            WorldElement::Wall => write!(f, "#"),
            WorldElement::NewSand => write!(f, "+"),
            WorldElement::FixedSand => write!(f, "o"),
        }
    }
}

#[derive(Debug)]
pub struct World {
    size: Vec2,
    buffer: Vec<WorldElement>,
}

impl World {

    pub fn new(size: Vec2) -> World {
        let Vec2(width, height) = &size;
        let buffer =vec![WorldElement::Nothing; *width as usize * *height as usize];

        World {
            size,
            buffer,
        }
    }

    pub fn insert_lines(&mut self, line_defs: &Vec<LineRow>) {
        for line_def in line_defs {
            let LineRow(lines) = line_def;
            for line in lines {
                self.insert_wall(line);
            }
        }
    }

    fn insert_wall(&mut self, wall: &Line) {
        let Line(Vec2(sx, sy), Vec2(ex, ey)) = wall;

        let asx = min(sx, ex);
        let aex = max(sx, ex);
        let asy = min(sy, ey);
        let aey = max(sy, ey);

        for x in *asx..=*aex {
            for y in *asy..=*aey {
                let wall_pos = Vec2(x, y);
                self.insert_wall_at(&wall_pos);
            }
        }
    }

    fn insert_wall_at(&mut self, pos: &Vec2) {
        let buffer_pos = self.vec2_to_buffer_pos(pos);
        self.buffer[buffer_pos] = WorldElement::Wall;
    }

    fn vec2_to_buffer_pos(&self, pos: &Vec2) -> usize {
        let Vec2(x, y) = pos;
        let Vec2(width, height) = &self.size;

        assert!(x < width);
        assert!(y < height);

        (*y * width + *x) as usize
    }

    pub fn is_in_bounds(&self, pos: &Vec2) -> bool {
        let Vec2(x, y) = pos;
        let Vec2(width, height) = &self.size;

        x < width && y < height
    }

    pub fn view_port(&self) -> ViewPort {
        ViewPort {
            world: self,
            view_port: Box::new_with_dimension(Vec2(494, 0), Vec2(503, 9)),
        }
    }

    pub fn get_element_at(&self, pos: &Vec2) -> &WorldElement {
        let buffer_pos = self.vec2_to_buffer_pos(&pos);
        &self.buffer[buffer_pos]
    }

    pub fn try_get_element_at(&self, pos: &Vec2) -> Option<&WorldElement> {
        if !self.is_in_bounds(&pos) {
            return None;
        }

        Some(self.get_element_at(pos))
    }

    pub fn drop_sand(&mut self, insert_pos: &Vec2) -> bool {
        todo!()
    }

}

#[derive(Debug)]
pub struct ViewPort<'a> {
    world: &'a World,
    view_port: Box,
}

impl<'a> Display for ViewPort<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Box(Vec2(sx, sy), Vec2(ex, ey)) = &self.view_port;

        for y in *sy..=*ey {
            for x in *sx..=*ex {
                let cur = Vec2(x, y);
                let element = self.world.try_get_element_at(&cur).unwrap_or(&WorldElement::Nothing);
                write!(f, "{}", element)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
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

        let mut world = World::new(Vec2(504, 10));
        world.insert_lines(&rows);
        let view_port = world.view_port();
        println!("{}", view_port);
    }

}
