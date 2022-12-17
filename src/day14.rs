use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::day14::WorldElement::*;
use crate::Error;
pub use crate::utils::vec2::Vector2;
pub use crate::utils::vec2::Vec2;

pub type Position = u32;

pub type PosVec = Vec2<Position>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line(PosVec, PosVec);

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Box(PosVec, PosVec);

impl Box {

    pub fn new() -> Box {
        Box::default()
    }

    pub fn new_with_dimension(top_left: PosVec, bottom_right: PosVec) -> Box {
        Box(top_left, bottom_right)
    }

    pub fn new_with_size(top_left: PosVec, size: PosVec) -> Box {
        let Vec2(x, y) = top_left;
        let Vec2(sx, sy) = size;

        Box(Vec2(x, y), Vec2(x + sx, y + sy))
    }

    pub fn size(&self) -> PosVec {
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
                let cur_coor: PosVec = cur_coor.parse()?;
                let next_coor: PosVec = next_coor.parse()?;
                result.push(Line(cur_coor, next_coor));
            }
            cur = next;
        }

        Ok(LineRow(result))
    }
}

pub trait BoundingBox {
    fn get_bounds(&self) -> Box;

    fn get_top_left(&self) -> PosVec {
        self.get_bounds().get_top_left()
    }

    fn get_top_right(&self) -> PosVec {
        self.get_bounds().get_top_right()
    }

    fn get_bottom_left(&self) -> PosVec {
        self.get_bounds().get_bottom_left()
    }

    fn get_bottom_right(&self) -> PosVec {
        self.get_bounds().get_bottom_right()
    }
}

impl BoundingBox for PosVec {
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

    fn get_top_left(&self) -> PosVec {
        self.0.clone()
    }

    fn get_top_right(&self) -> PosVec {
        let Box(Vec2(_, sy), Vec2(ex, _)) = self;

        Vec2(*ex, *sy)
    }

    fn get_bottom_left(&self) -> PosVec {
        let Box(Vec2(sx, _), Vec2(_, ey)) = self;

        Vec2(*sx, *ey)
    }

    fn get_bottom_right(&self) -> PosVec {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WorldElement {
    Nothing,
    Wall,
    NewSand,
    FixedSand,
}

impl Display for WorldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Nothing => write!(f, "."),
            Wall => write!(f, "#"),
            NewSand => write!(f, "+"),
            FixedSand => write!(f, "o"),
        }
    }
}

#[derive(Debug)]
pub struct World {
    size: PosVec,
    buffer: Vec<WorldElement>,
    insert_pos: PosVec,
}

impl World {

    pub fn new(size: PosVec, insert_pos: PosVec) -> World {
        let Vec2(width, height) = &size;
        let buffer =vec![Nothing; *width as usize * *height as usize];

        World {
            size,
            buffer,
            insert_pos,
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

    fn insert_wall_at(&mut self, pos: &PosVec) {
        self.insert_at(pos, Wall);
    }

    fn insert_sand_at(&mut self, pos: &PosVec) {
        self.insert_at(pos, FixedSand);
    }

    fn insert_at(&mut self, pos: &PosVec, element: WorldElement) {
        let buffer_pos = self.vec2_to_buffer_pos(pos);
        self.buffer[buffer_pos] = element;
    }

    fn vec2_to_buffer_pos(&self, pos: &PosVec) -> usize {
        let Vec2(x, y) = pos;
        let Vec2(width, height) = &self.size;

        assert!(x < width);
        assert!(y < height);

        (*y * width + *x) as usize
    }

    pub fn is_in_bounds(&self, pos: &PosVec) -> bool {
        let Vec2(x, y) = pos;
        let Vec2(width, height) = &self.size;

        x < width && y < height
    }

    pub fn view_port(&self) -> ViewPort {
        self.view_port_at(&Box::new_with_dimension(Vec2(494, 0), Vec2(503, 9)))
    }

    pub fn view_port_at(&self, view_port: &Box) -> ViewPort {
        ViewPort {
            world: self,
            view_port: view_port.clone(),
        }
    }

    pub fn get_element_at(&self, pos: &PosVec) -> &WorldElement {
        let buffer_pos = self.vec2_to_buffer_pos(&pos);
        &self.buffer[buffer_pos]
    }

    pub fn try_get_element_at(&self, pos: &PosVec) -> Option<&WorldElement> {
        if !self.is_in_bounds(&pos) {
            return None;
        }

        Some(self.get_element_at(pos))
    }

    pub fn drop_sand(&mut self) -> PosVec {
        let mut cur_pos = self.insert_pos.clone();

        while let Some(new_pos) = self.find_next_position(&cur_pos) {
            cur_pos = new_pos;
        }

        self.insert_sand_at(&cur_pos);

        cur_pos
    }

    pub fn find_next_position(&self, pos: &PosVec) -> Option<PosVec> {
        let Vec2(x, y) = pos;
        let try_position = Vec2(*x, y+1);
        if self.try_get_element_at(&try_position) == Some(&Nothing) {
            return Some(try_position);
        }
        let try_position = Vec2(x-1, y+1);
        if self.try_get_element_at(&try_position) == Some(&Nothing) {
            return Some(try_position);
        }
        let try_position = Vec2(x+1, y+1);
        if self.try_get_element_at(&try_position) == Some(&Nothing) {
            return Some(try_position);
        }

        None
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
                let element = self.world.try_get_element_at(&cur).unwrap_or(&Nothing);
                write!(f, "{}", element)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::utils::Vector2;
    use super::*;

    static EXAMPLE : &'static str = include_str!("../res/day14-paths_example.txt");

    #[test]
    fn test_example() {
        let rows: Result<Vec<LineRow>, Error> = EXAMPLE.lines().map(str::trim_end).map(|l| l.parse::<LineRow>()).collect();
        let rows = rows.unwrap();
        println!("{:#?}", rows);

        let bounding_box: Box = rows.iter().collect();
        println!("{:?}", bounding_box);

        let mut world = World::new(Vec2(504, 10), Vec2(500, 0));
        world.insert_lines(&rows);
        let view_port = world.view_port();
        println!("{}", view_port);
    }

    #[test]
    fn test_drop() {
        let rows: Result<Vec<LineRow>, Error> = EXAMPLE.lines().map(str::trim_end).map(|l| l.parse::<LineRow>()).collect();
        let rows = rows.unwrap();
        let bounding_box: Box = rows.iter().collect();
        let Vec2(bx, by) = bounding_box.get_bottom_right();
        let stop_line = by;

        let mut world = World::new(Vec2(bx+1, by+1), Vec2(500, 0));
        world.insert_lines(&rows);
        println!("{}", world.view_port());

        let mut counter = 0;

        loop {
            let end_pos = world.drop_sand();
            if end_pos.get_y() >= &stop_line {
                break;
            }
            counter+=1;
        }

        println!("{}", world.view_port());
        println!("rest: {}", counter);
    }

    #[test]
    fn test_drop_part2() {
        let rows: Result<Vec<LineRow>, Error> = EXAMPLE.lines().map(str::trim_end).map(|l| l.parse::<LineRow>()).collect();
        let rows = rows.unwrap();
        let bounding_box: Box = rows.iter().collect();
        let Vec2(bx, by) = bounding_box.get_bottom_right();

        let insert_pos = Vec2(500, 0);
        let mut world = World::new(Vec2(bx+20, by+2), insert_pos.clone());
        world.insert_lines(&rows);
        let dimension = Box::new_with_dimension(Vec2(450, 0), Vec2(550, 15));
        println!("{}", world.view_port_at(&dimension));

        let mut counter = 0;

        loop {
            counter+=1;
            let end_pos = world.drop_sand();
            if end_pos == insert_pos {
                break;
            }
        }

        println!("{}", world.view_port_at(&dimension));
        println!("rest: {}", counter);
    }
}
