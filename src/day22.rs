use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem::replace;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::{Error, Scored};
use crate::utils::minmax::MinMax;
pub use crate::utils::turtle::*;
use crate::utils::vec2::*;


type Pos = isize;
type PosVec2 = Vec2<Pos>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<PosVec2, Tile>,
    bounding_box: BoundingBox<Pos>,
    row_ranges: HashMap<Pos, RangeInclusive<Pos>>,
    col_ranges: HashMap<Pos, RangeInclusive<Pos>>,
    regions: Option<Vec<Region>>,
}

impl Map {
    pub fn get_starting_position(&self) -> PosVec2 {
        let top_row_idx = *self.bounding_box.y_range().start();
        let top_row_bounds = self.row_ranges[&top_row_idx].clone();
        top_row_bounds
            .map(|b| PosVec2::new(b, top_row_idx))
            .find(|p| self.map.get(p) == Some(&Tile::Floor))
            .expect("no starting position found")
    }

    pub fn set_regions(&mut self, regions: Option<Vec<Region>>) {
        self.regions = regions;
    }

    pub fn wrap_no_region(&self, dir: &Direction, new_pos: &mut PosVec2) {
        match dir {
            Direction::Right => {
                let row_range = &self.row_ranges[new_pos.get_y()];
                new_pos.set_x(*row_range.start());
            }
            Direction::Up => {
                let col_range = &self.col_ranges[new_pos.get_x()];
                new_pos.set_y(*col_range.end());
            }
            Direction::Left => {
                let row_range = &self.row_ranges[new_pos.get_y()];
                new_pos.set_x(*row_range.end());
            }
            Direction::Down => {
                let col_range = &self.col_ranges[new_pos.get_x()];
                new_pos.set_y(*col_range.start());
            }
        }
    }

    pub fn wrap_region(&self, old_pos: &PosVec2, dir: &Direction, new_pos: &mut PosVec2) {
        assert!(self.regions.is_some());

        let regions = self.regions.as_ref().unwrap();

        let source_region = regions.iter().find(|r| r.bounding_box.contains(old_pos))
            .expect(&format!("no region contains {:?}", old_pos));

        let lookup = match dir {
            Direction::Right => 0,
            Direction::Up => 1,
            Direction::Left => 2,
            Direction::Down => 3,
        };

        let new_region = source_region.region_mapping[lookup];
        let dir_in_new_region = source_region.direction_mapping[lookup];

        let new_region = regions.iter().find(|r| r.id == new_region)
            .expect(&format!("invalid region: {:?}", new_region));

        match dir_in_new_region {
            Direction::Right => {}
            Direction::Up => {}
            Direction::Left => {}
            Direction::Down => {}
        }
    }
}

impl From<HashMap<PosVec2, Tile>> for Map {
    fn from(map: HashMap<PosVec2, Tile>) -> Self {
        let bounding_box: BoundingBox<&Pos> = map.keys().collect();
        let bounding_box = bounding_box.map(|v| *v);

        let mut row_ranges = HashMap::new();
        let mut col_ranges = HashMap::new();

        for row in bounding_box.y_range() {
            if let Some((min, max)) = map.keys()
                .filter(|v| v.get_y() == &row)
                .map(|v| v.get_x()).min_max() {
                row_ranges.insert(row, *min..=*max);
            }
        }

        for col in bounding_box.x_range() {
            if let Some((min, max)) = map.keys()
                .filter(|v| v.get_x() == &col)
                .map(|v| v.get_y()).min_max() {
                col_ranges.insert(col, *min..=*max);
            }
        }

        Map {
            map,
            bounding_box,
            row_ranges,
            col_ranges,
            regions: None,
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            if line.trim().len() == 0 {
                break;
            }

            for (x, c) in line.chars().enumerate() {
                let parsed_tile = match c {
                    '.' => Some(Tile::Floor),
                    '#' => Some(Tile::Wall),
                    _ => None,
                };

                if let Some(tile) = parsed_tile {
                    map.insert(PosVec2::new(x as isize + 1, y as isize + 1), tile);
                }
            }
        }

        Ok(map.into())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.bounding_box.y_range() {
            for x in self.bounding_box.x_range() {
                let tile = self.map.get(&PosVec2::new(x, y))
                    .map(|t| t.to_string())
                    .unwrap_or(" ".to_string());
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Instructions(pub Vec<Instruction>);

impl Instructions {

    pub fn simulate<W: World>(&self, turtle: &mut Turtle<'_, W>) {
        let Instructions(instructions) = self;
        for instruction in instructions {
            turtle.eval(instruction).ok();
        }
    }

}

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        let mut number_cache = String::new();

        for c in s.chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => number_cache.push(c),
                'L' | 'R' => {
                    if !number_cache.is_empty() {
                        let number = replace(&mut number_cache, String::new());
                        let number: usize = number.parse()?;
                        instructions.push(Instruction::Step(number));
                        let turn: Turn = c.to_string().parse()?;
                        instructions.push(Instruction::Turn(turn));
                    }
                }
                _ => {
                    return Err(Error::cannot_parse(&c));
                }
            }
        }
        if !number_cache.is_empty() {
            let number: usize = number_cache.parse()?;
            instructions.push(Instruction::Step(number));
        }

        Ok(Instructions(instructions))
    }
}

impl Position for PosVec2 {
    fn get_step_position(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => PosVec2::new(self.get_x() + 1, *self.get_y()),
            Direction::Up => PosVec2::new(*self.get_x(), self.get_y() - 1),
            Direction::Left => PosVec2::new(self.get_x() - 1, *self.get_y()),
            Direction::Down => PosVec2::new(*self.get_x(), self.get_y() + 1),
        }
    }
}

impl World for Map {
    type Position = PosVec2;

    fn wrap_position(&self, _: &Self::Position, dir: &Direction, new_pos: &mut Self::Position) {
        if self.map.contains_key(new_pos) {
            return;
        }

        match dir {
            Direction::Right => {
                let row_range = &self.row_ranges[new_pos.get_y()];
                new_pos.set_x(*row_range.start());
            }
            Direction::Up => {
                let col_range = &self.col_ranges[new_pos.get_x()];
                new_pos.set_y(*col_range.end());
            }
            Direction::Left => {
                let row_range = &self.row_ranges[new_pos.get_y()];
                new_pos.set_x(*row_range.end());
            }
            Direction::Down => {
                let col_range = &self.col_ranges[new_pos.get_x()];
                new_pos.set_y(*col_range.start());
            }
        }
    }

    fn is_accessible(&self, pos: &Self::Position) -> bool {
        self.map.get(pos) != Some(&Tile::Wall)
    }
}

impl Scored for Direction {
    fn get_score(&self) -> u64 {
        match self {
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Left => 2,
            Direction::Down => 1
        }
    }
}

impl Scored for Turtle<'_, Map> {
    fn get_score(&self) -> u64 {
        (1000 * self.turtle_pos().get_y()) as u64
            + (4 * self.turtle_pos().get_x()) as u64
            + self.turtle_dir().get_score()
    }
}

#[derive(Debug)]
pub struct Region {
    id: usize,
    bounding_box: BoundingBox<Pos>,
    region_mapping: [usize; 4],
    direction_mapping: [Direction; 4],
}

impl FromStr for Region {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(":").ok_or(Error::cannot_parse(s))?;
        let (bb, mapping) = rest.split_once(";").ok_or(Error::cannot_parse(rest))?;
        let (start, end) = bb.split_once("-").ok_or(Error::cannot_parse(bb))?;
        let (region_mapping, direction_mapping) = mapping.split_once("-").ok_or(Error::cannot_parse(mapping))?;

        let id: usize = id.parse()?;
        let start: PosVec2 = start.parse()?;
        let end: PosVec2 = end.parse()?;



        let region_mapping: Result<Vec<usize>, ParseIntError> =  region_mapping.chars().map(|c| c.to_string().parse::<usize>()).collect();
        let region_mapping = region_mapping?;

        let region_mapping = [
            region_mapping[0],
            region_mapping[1],
            region_mapping[2],
            region_mapping[3]
        ];

        let direction_mapping: Result<Vec<Direction>, Error> = direction_mapping.chars().map(|c| c.to_string().parse::<Direction>()).collect();
        let direction_mapping = direction_mapping?;

        let direction_mapping = [
            direction_mapping[0],
            direction_mapping[1],
            direction_mapping[2],
            direction_mapping[3],
        ];

        Ok(Region {
            id,
            bounding_box: BoundingBox::new(start, end),
            region_mapping,
            direction_mapping,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day22-map_example.txt");

    #[test]
    fn test_parse_map() {
        let map: Map = EXAMPLE.parse().unwrap();

        println!("{}", map);
    }

    #[test]
    fn test_parse_instructions() {
        let mut lines = EXAMPLE.lines();
        while let Some(l) = lines.next() {
            if l.trim().is_empty() {
                break;
            }
        }
        let instructions = lines.next().unwrap();
        let instructions: Instructions = instructions.parse().unwrap();

        println!("{:#?}", instructions);
    }

    #[test]
    fn test_simulate() {
        let map: Map = EXAMPLE.parse().unwrap();

        let mut lines = EXAMPLE.lines();
        while let Some(l) = lines.next() {
            if l.trim().is_empty() {
                break;
            }
        }
        let instructions = lines.next().unwrap();
        let instructions: Instructions = instructions.parse().unwrap();

        let starting_position = map.get_starting_position();

        let mut turtle = Turtle::new(&map, starting_position, Direction::Right);

        instructions.simulate(&mut turtle);

        assert_eq!(6032, turtle.get_score());
    }
}
