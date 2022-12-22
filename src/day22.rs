use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;
use crate::utils::turtle::*;
use crate::utils::vec2::*;

type Pos = isize;
type PosVec2 = Vec2<Pos>;

#[derive(Debug)]
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
}

impl From<HashMap<PosVec2, Tile>> for Map {
    fn from(map: HashMap<PosVec2, Tile>) -> Self {
        let bounding_box: BoundingBox<&Pos> = map.keys().collect();
        let bounding_box = bounding_box.map(|v| *v);
        Map {
            map,
            bounding_box,
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

#[cfg(test)]
mod test{

    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day22-map_example.txt");

    #[test]
    fn test_parse_map() {
        let map: Map = EXAMPLE.parse().unwrap();

        println!("{}", map);
    }
}